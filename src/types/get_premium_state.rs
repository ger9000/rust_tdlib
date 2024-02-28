use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns state of Telegram Premium subscription and promotion videos for Premium features
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPremiumState {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetPremiumState {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetPremiumState {}

impl GetPremiumState {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetPremiumStateBuilder {
        let mut inner = GetPremiumState::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getPremiumState".to_string();

        GetPremiumStateBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetPremiumStateBuilder {
    inner: GetPremiumState,
}

#[deprecated]
pub type RTDGetPremiumStateBuilder = GetPremiumStateBuilder;

impl GetPremiumStateBuilder {
    pub fn build(&self) -> GetPremiumState {
        self.inner.clone()
    }
}

impl AsRef<GetPremiumState> for GetPremiumState {
    fn as_ref(&self) -> &GetPremiumState {
        self
    }
}

impl AsRef<GetPremiumState> for GetPremiumStateBuilder {
    fn as_ref(&self) -> &GetPremiumState {
        &self.inner
    }
}