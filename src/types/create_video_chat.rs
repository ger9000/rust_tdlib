use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Creates a video chat (a group call bound to a chat). Available only for basic groups, supergroups and channels; requires can_manage_video_chats rights
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateVideoChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of a chat in which the video chat will be created
    chat_id: i64,
    /// Group call title; if empty, chat title will be used
    title: String,
    /// Point in time (Unix timestamp) when the group call is supposed to be started by an administrator; 0 to start the video chat immediately. The date must be at least 10 seconds and at most 8 days in the future
    start_date: i32,
    /// Pass true to create an RTMP stream instead of an ordinary video chat; requires creator privileges
    is_rtmp_stream: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CreateVideoChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CreateVideoChat {}

impl CreateVideoChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCreateVideoChatBuilder {
        let mut inner = CreateVideoChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "createVideoChat".to_string();

        RTDCreateVideoChatBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn start_date(&self) -> i32 {
        self.start_date
    }

    pub fn is_rtmp_stream(&self) -> bool {
        self.is_rtmp_stream
    }
}

#[doc(hidden)]
pub struct RTDCreateVideoChatBuilder {
    inner: CreateVideoChat,
}

impl RTDCreateVideoChatBuilder {
    pub fn build(&self) -> CreateVideoChat {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn start_date(&mut self, start_date: i32) -> &mut Self {
        self.inner.start_date = start_date;
        self
    }

    pub fn is_rtmp_stream(&mut self, is_rtmp_stream: bool) -> &mut Self {
        self.inner.is_rtmp_stream = is_rtmp_stream;
        self
    }
}

impl AsRef<CreateVideoChat> for CreateVideoChat {
    fn as_ref(&self) -> &CreateVideoChat {
        self
    }
}

impl AsRef<CreateVideoChat> for RTDCreateVideoChatBuilder {
    fn as_ref(&self) -> &CreateVideoChat {
        &self.inner
    }
}