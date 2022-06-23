use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the message TTL in a chat. Requires can_delete_messages administrator right in basic groups, supergroups and channels Message TTL can't be changed in a chat with the current user (Saved Messages) and the chat 777000 (Telegram).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatMessageTtl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// New TTL value, in seconds; unless the chat is secret, it must be from 0 up to 365 * 86400 and be divisible by 86400
    ttl: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetChatMessageTtl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetChatMessageTtl {}

impl SetChatMessageTtl {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetChatMessageTtlBuilder {
        let mut inner = SetChatMessageTtl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setChatMessageTtl".to_string();

        RTDSetChatMessageTtlBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn ttl(&self) -> i32 {
        self.ttl
    }
}

#[doc(hidden)]
pub struct RTDSetChatMessageTtlBuilder {
    inner: SetChatMessageTtl,
}

impl RTDSetChatMessageTtlBuilder {
    pub fn build(&self) -> SetChatMessageTtl {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn ttl(&mut self, ttl: i32) -> &mut Self {
        self.inner.ttl = ttl;
        self
    }
}

impl AsRef<SetChatMessageTtl> for SetChatMessageTtl {
    fn as_ref(&self) -> &SetChatMessageTtl {
        self
    }
}

impl AsRef<SetChatMessageTtl> for RTDSetChatMessageTtlBuilder {
    fn as_ref(&self) -> &SetChatMessageTtl {
        &self.inner
    }
}