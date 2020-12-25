use std::sync::{Arc, Mutex, Condvar};
use async_trait::async_trait;
use rtdlib_sys::Tdlib;
use super::observer::OBSERVER;

use super::api::Api;
use crate::{
    errors::RTDError,
    types::from_json,
    types::TdType,
    types::{SetTdlibParameters, UpdateAuthorizationState, CheckAuthenticationCode, CheckDatabaseEncryptionKey, AuthorizationStateWaitCode, AuthorizationStateWaitEncryptionKey, AuthorizationStateWaitPassword, CheckAuthenticationPassword, SetAuthenticationPhoneNumber, TdlibParameters},
};
use tokio::{
    sync::mpsc,
    task::JoinHandle
};
use crate::types::{AuthorizationState, AuthorizationStateWaitPhoneNumber, AuthorizationStateWaitRegistration};
use crate::errors::RTDResult;
use std::io;

/// `AuthStateHandler` trait provides methods, that returns data, required for authentication
#[async_trait]
pub trait AuthStateHandler {
    /// Returns wait code
    async fn handle_wait_code(&self, wait_code: &AuthorizationStateWaitCode) -> String;
    /// Returns database encryption key
    async fn handle_encryption_key(&self, wait_encryption_key: &AuthorizationStateWaitEncryptionKey) -> String;
    /// Returns password
    async fn handle_wait_password(&self, wait_password: &AuthorizationStateWaitPassword) -> String;
    /// Returns phone number
    async fn handle_wait_phone_number(&self, wait_phone_number: &AuthorizationStateWaitPhoneNumber) -> String;
    async fn handle_wait_registration(&self, wait_registration: &AuthorizationStateWaitRegistration) -> String;
}

/// Provides minimum implementation of `AuthStateHandler`.
/// All required methods wait for stdin input
pub struct TypeInAuthStateHandler {}

impl TypeInAuthStateHandler {
    fn type_in() -> String {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => input.trim().to_string(),
            Err(e) => panic!("Can not get input value: {:?}", e),
        }
    }
}


#[async_trait]
impl AuthStateHandler for TypeInAuthStateHandler {
    async fn handle_wait_code(&self, _wait_code: &AuthorizationStateWaitCode) -> String {
        eprintln!("wait for auth code");
        TypeInAuthStateHandler::type_in()
    }

    async fn handle_encryption_key(&self, _wait_encryption_key: &AuthorizationStateWaitEncryptionKey) -> String {
        eprintln!("wait for encryption key");
        TypeInAuthStateHandler::type_in()
    }

    async fn handle_wait_password(&self, _wait_password: &AuthorizationStateWaitPassword) -> String {
        eprintln!("wait for password");
        TypeInAuthStateHandler::type_in()
    }

    async fn handle_wait_phone_number(&self, _wait_phone_number: &AuthorizationStateWaitPhoneNumber) -> String {
        eprintln!("wait for phone number");
        TypeInAuthStateHandler::type_in()
    }

    async fn handle_wait_registration(&self, _wait_registration: &AuthorizationStateWaitRegistration) -> String {
        unimplemented!()
    }
}


/// `Client` is a high-level abstraction of TDLib.
/// Before start any API interactions you must call `start().await`.
#[derive(Clone)]
pub struct Client<A>
where A: AuthStateHandler + Send + Sync + 'static
{
    stop_flag: Arc<Mutex<bool>>,
    api: Api,
    updates_sender: Option<mpsc::Sender<TdType>>,
    auth_state_handler: Arc<A>,
    tdlib_parameters: Arc<TdlibParameters>,
    have_auth: Arc<(Mutex<bool>, Condvar)>

}

impl<A> Client<A> where A: AuthStateHandler + Send + Sync + 'static{
    pub fn api(&self) -> &Api {
        &self.api
    }

    pub fn new(tdlib: Tdlib, auth_state_handler: A, tdlib_parameters: TdlibParameters) -> Self {
        let stop_flag = Arc::new(Mutex::new(false));
        Self {
            stop_flag,
            tdlib_parameters: Arc::new(tdlib_parameters),
            api: Api::new(tdlib),
            auth_state_handler: Arc::new(auth_state_handler),
            have_auth: Arc::new((Mutex::new(false), Condvar::new())),
            updates_sender: None,
        }
    }
    /// If you want to receive Telegram updates (messages, channels, etc; see `crate::types::TdType`),
    /// you must set mpsc::Sender here.
    pub fn set_updates_sender(&mut self, updates_sender: mpsc::Sender<TdType>) {
        self.updates_sender = Some(updates_sender)
    }

    /// Starts interaction with TDLib.
    /// Method blocks until authorization performed.
    pub async fn start(&mut self) -> Result<JoinHandle<()>, RTDError> {
        let stop_flag = self.stop_flag.clone();
        let api = self.api.clone();

        let updates_sender = self.updates_sender.clone();

        let auth_state_handler = self.auth_state_handler.clone();
        let tdlib_params = self.tdlib_parameters.clone();
        let (sx, mut rx) = mpsc::channel::<()>(3);
        let (auth_sx, mut auth_rx) = mpsc::channel::<UpdateAuthorizationState>(10);
        let auth_api = self.api.clone();

        let handle = tokio::spawn(async move {
            let current = tokio::runtime::Handle::try_current().unwrap();
            while !*stop_flag.lock().unwrap() {
                let rec_api = api.clone();
                if let Some(json) = current.spawn_blocking(move||rec_api.receive(2.0)).await.unwrap() {
                    match from_json::<TdType>(&json) {
                        Ok(t) => match OBSERVER.notify(t) {
                            None => {}
                            Some(t) => {
                                match t {
                                    TdType::UpdateAuthorizationState(auth_state) => {
                                        auth_sx.send(auth_state).await.unwrap();
                                    },
                                    _ => {
                                        match &updates_sender {
                                            None => {}
                                            Some(sender) => {
                                                sender.send(t).await.unwrap();
                                            }
                                        }
                                    }
                                }
                            },
                        },
                        Err(e) => {panic!("{}", e)}
                    };
                }
            }
        });

        // TODO: store it within Client instance?
        let _auth_handle = tokio::spawn(async move {
            while let Some(auth_state) = auth_rx.recv().await {
                handle_auth_state(
                                &auth_api,
                                auth_state_handler.clone(),
                                auth_state,
                                sx.clone(),
                                tdlib_params.clone()
                            ).await.unwrap();
            }
        });

        rx.recv().await.unwrap();
        Ok(handle)
    }
}

async fn handle_auth_state<A: AuthStateHandler>(api: &Api, auth_state_handler: Arc<A>, state: UpdateAuthorizationState, sender: mpsc::Sender<()>, tdlib_parameters: Arc<TdlibParameters>) -> RTDResult<()>{
    match state.authorization_state() {
        AuthorizationState::_Default(_) => {unreachable!()}
        AuthorizationState::Closed(_) => {todo!()}
        AuthorizationState::Closing(_) => {todo!()}
        AuthorizationState::LoggingOut(_) => {todo!()}
        AuthorizationState::Ready(_) => {
            sender.send(()).await.unwrap();
            Ok(())
        }
        AuthorizationState::WaitCode(wait_code) => {
            let code = auth_state_handler.handle_wait_code(wait_code).await;
            api.check_authentication_code(CheckAuthenticationCode::builder().code(code).build()).await?;
            Ok(())
        }
        AuthorizationState::WaitEncryptionKey(wait_encryption_key) => {
            let key = auth_state_handler.handle_encryption_key(wait_encryption_key).await;
            api.check_database_encryption_key(CheckDatabaseEncryptionKey::builder().encryption_key(key).build()).await?;
            Ok(())
        }
        AuthorizationState::WaitOtherDeviceConfirmation(_) => {todo!()}
        AuthorizationState::WaitPassword(wait_password) => {
            let password = auth_state_handler.handle_wait_password(wait_password).await;
            api.check_authentication_password(CheckAuthenticationPassword::builder().password(password).build()).await?;
            Ok(())
        }
        AuthorizationState::WaitPhoneNumber(wait_phone_number) => {
            let phone_number = auth_state_handler.handle_wait_phone_number(wait_phone_number).await;
            api.set_authentication_phone_number(SetAuthenticationPhoneNumber::builder().phone_number(phone_number).build()).await?;
            Ok(())
        }
        AuthorizationState::WaitRegistration(_) => {todo!()}
        AuthorizationState::WaitTdlibParameters(_) => {
            api.set_tdlib_parameters(SetTdlibParameters::builder().parameters(tdlib_parameters).build()).await?;
            Ok(())
        }
        AuthorizationState::GetAuthorizationState(_) => {todo!()}
    }
}
