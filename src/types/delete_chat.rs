use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Deletes a chat along with all messages in the corresponding chat for all chat members. For group chats this will release the username and remove all members. Use the field chat.can_be_deleted_for_all_users to find whether the method can be applied to the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteChat {}

impl DeleteChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeleteChatBuilder {
        let mut inner = DeleteChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteChat".to_string();

        RTDDeleteChatBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct RTDDeleteChatBuilder {
    inner: DeleteChat,
}

impl RTDDeleteChatBuilder {
    pub fn build(&self) -> DeleteChat {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<DeleteChat> for DeleteChat {
    fn as_ref(&self) -> &DeleteChat {
        self
    }
}

impl AsRef<DeleteChat> for RTDDeleteChatBuilder {
    fn as_ref(&self) -> &DeleteChat {
        &self.inner
    }
}
