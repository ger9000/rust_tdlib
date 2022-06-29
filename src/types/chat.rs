use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A chat. (Can be a private chat, basic group, supergroup, or secret chat)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Chat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat unique identifier

    #[serde(default)]
    id: i64,
    /// Type of the chat

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "ChatType::_is_default")]
    type_: ChatType,
    /// Chat title

    #[serde(default)]
    title: String,
    /// Chat photo; may be null
    photo: Option<ChatPhotoInfo>,
    /// Actions that non-administrator chat members are allowed to take in the chat
    permissions: ChatPermissions,
    /// Last message in the chat; may be null
    last_message: Option<Message>,
    /// Positions of the chat in chat lists

    #[serde(default)]
    positions: Option<Vec<ChatPosition>>,
    /// Identifier of a user or chat that is selected to send messages in the chat; may be null if the user can't change message sender
    message_sender_id: Option<MessageSender>,
    /// True, if chat content can't be saved locally, forwarded, or copied

    #[serde(default)]
    has_protected_content: bool,
    /// True, if the chat is marked as unread

    #[serde(default)]
    is_marked_as_unread: bool,
    /// True, if the chat is blocked by the current user and private messages from the chat can't be received

    #[serde(default)]
    is_blocked: bool,
    /// True, if the chat has scheduled messages

    #[serde(default)]
    has_scheduled_messages: bool,
    /// True, if the chat messages can be deleted only for the current user while other users will continue to see the messages

    #[serde(default)]
    can_be_deleted_only_for_self: bool,
    /// True, if the chat messages can be deleted for all users

    #[serde(default)]
    can_be_deleted_for_all_users: bool,
    /// True, if the chat can be reported to Telegram moderators through reportChat or reportChatPhoto

    #[serde(default)]
    can_be_reported: bool,
    /// Default value of the disable_notification parameter, used when a message is sent to the chat

    #[serde(default)]
    default_disable_notification: bool,
    /// Number of unread messages in the chat

    #[serde(default)]
    unread_count: i32,
    /// Identifier of the last read incoming message

    #[serde(default)]
    last_read_inbox_message_id: i64,
    /// Identifier of the last read outgoing message

    #[serde(default)]
    last_read_outbox_message_id: i64,
    /// Number of unread messages with a mention/reply in the chat

    #[serde(default)]
    unread_mention_count: i32,
    /// Number of messages with unread reactions in the chat

    #[serde(default)]
    unread_reaction_count: i32,
    /// Notification settings for the chat
    notification_settings: ChatNotificationSettings,
    /// List of reactions, available in the chat

    #[serde(default)]
    available_reactions: Vec<String>,
    /// Current message Time To Live setting (self-destruct timer) for the chat; 0 if not defined. TTL is counted from the time message or its content is viewed in secret chats and from the send date in other chats

    #[serde(default)]
    message_ttl: i32,
    /// If non-empty, name of a theme, set for the chat

    #[serde(default)]
    theme_name: String,
    /// Information about actions which must be possible to do through the chat action bar; may be null
    action_bar: Option<ChatActionBar>,
    /// Information about video chat of the chat
    video_chat: VideoChat,
    /// Information about pending join requests; may be null
    pending_join_requests: Option<ChatJoinRequestsInfo>,
    /// Identifier of the message from which reply markup needs to be used; 0 if there is no default custom reply markup in the chat

    #[serde(default)]
    reply_markup_message_id: i64,
    /// A draft of a message in the chat; may be null
    draft_message: Option<DraftMessage>,
    /// Application-specific data associated with the chat. (For example, the chat scroll position or local chat notification settings can be stored here.) Persistent if the message database is used

    #[serde(default)]
    client_data: String,
}

impl RObject for Chat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Chat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatBuilder {
        let mut inner = Chat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn type_(&self) -> &ChatType {
        &self.type_
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn photo(&self) -> &Option<ChatPhotoInfo> {
        &self.photo
    }

    pub fn permissions(&self) -> &ChatPermissions {
        &self.permissions
    }

    pub fn last_message(&self) -> &Option<Message> {
        &self.last_message
    }

    pub fn positions(&self) -> &Option<Vec<ChatPosition>> {
        &self.positions
    }

    pub fn message_sender_id(&self) -> &Option<MessageSender> {
        &self.message_sender_id
    }

    pub fn has_protected_content(&self) -> bool {
        self.has_protected_content
    }

    pub fn is_marked_as_unread(&self) -> bool {
        self.is_marked_as_unread
    }

    pub fn is_blocked(&self) -> bool {
        self.is_blocked
    }

    pub fn has_scheduled_messages(&self) -> bool {
        self.has_scheduled_messages
    }

    pub fn can_be_deleted_only_for_self(&self) -> bool {
        self.can_be_deleted_only_for_self
    }

    pub fn can_be_deleted_for_all_users(&self) -> bool {
        self.can_be_deleted_for_all_users
    }

    pub fn can_be_reported(&self) -> bool {
        self.can_be_reported
    }

    pub fn default_disable_notification(&self) -> bool {
        self.default_disable_notification
    }

    pub fn unread_count(&self) -> i32 {
        self.unread_count
    }

    pub fn last_read_inbox_message_id(&self) -> i64 {
        self.last_read_inbox_message_id
    }

    pub fn last_read_outbox_message_id(&self) -> i64 {
        self.last_read_outbox_message_id
    }

    pub fn unread_mention_count(&self) -> i32 {
        self.unread_mention_count
    }

    pub fn unread_reaction_count(&self) -> i32 {
        self.unread_reaction_count
    }

    pub fn notification_settings(&self) -> &ChatNotificationSettings {
        &self.notification_settings
    }

    pub fn available_reactions(&self) -> &Vec<String> {
        &self.available_reactions
    }

    pub fn message_ttl(&self) -> i32 {
        self.message_ttl
    }

    pub fn theme_name(&self) -> &String {
        &self.theme_name
    }

    pub fn action_bar(&self) -> &Option<ChatActionBar> {
        &self.action_bar
    }

    pub fn video_chat(&self) -> &VideoChat {
        &self.video_chat
    }

    pub fn pending_join_requests(&self) -> &Option<ChatJoinRequestsInfo> {
        &self.pending_join_requests
    }

    pub fn reply_markup_message_id(&self) -> i64 {
        self.reply_markup_message_id
    }

    pub fn draft_message(&self) -> &Option<DraftMessage> {
        &self.draft_message
    }

    pub fn client_data(&self) -> &String {
        &self.client_data
    }
}

#[doc(hidden)]
pub struct RTDChatBuilder {
    inner: Chat,
}

impl RTDChatBuilder {
    pub fn build(&self) -> Chat {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn type_<T: AsRef<ChatType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn photo<T: AsRef<ChatPhotoInfo>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = Some(photo.as_ref().clone());
        self
    }

    pub fn permissions<T: AsRef<ChatPermissions>>(&mut self, permissions: T) -> &mut Self {
        self.inner.permissions = permissions.as_ref().clone();
        self
    }

    pub fn last_message<T: AsRef<Message>>(&mut self, last_message: T) -> &mut Self {
        self.inner.last_message = Some(last_message.as_ref().clone());
        self
    }

    pub fn positions(&mut self, positions: Vec<ChatPosition>) -> &mut Self {
        self.inner.positions = Some(positions);
        self
    }

    pub fn message_sender_id<T: AsRef<MessageSender>>(
        &mut self,
        message_sender_id: T,
    ) -> &mut Self {
        self.inner.message_sender_id = Some(message_sender_id.as_ref().clone());
        self
    }

    pub fn has_protected_content(&mut self, has_protected_content: bool) -> &mut Self {
        self.inner.has_protected_content = has_protected_content;
        self
    }

    pub fn is_marked_as_unread(&mut self, is_marked_as_unread: bool) -> &mut Self {
        self.inner.is_marked_as_unread = is_marked_as_unread;
        self
    }

    pub fn is_blocked(&mut self, is_blocked: bool) -> &mut Self {
        self.inner.is_blocked = is_blocked;
        self
    }

    pub fn has_scheduled_messages(&mut self, has_scheduled_messages: bool) -> &mut Self {
        self.inner.has_scheduled_messages = has_scheduled_messages;
        self
    }

    pub fn can_be_deleted_only_for_self(
        &mut self,
        can_be_deleted_only_for_self: bool,
    ) -> &mut Self {
        self.inner.can_be_deleted_only_for_self = can_be_deleted_only_for_self;
        self
    }

    pub fn can_be_deleted_for_all_users(
        &mut self,
        can_be_deleted_for_all_users: bool,
    ) -> &mut Self {
        self.inner.can_be_deleted_for_all_users = can_be_deleted_for_all_users;
        self
    }

    pub fn can_be_reported(&mut self, can_be_reported: bool) -> &mut Self {
        self.inner.can_be_reported = can_be_reported;
        self
    }

    pub fn default_disable_notification(
        &mut self,
        default_disable_notification: bool,
    ) -> &mut Self {
        self.inner.default_disable_notification = default_disable_notification;
        self
    }

    pub fn unread_count(&mut self, unread_count: i32) -> &mut Self {
        self.inner.unread_count = unread_count;
        self
    }

    pub fn last_read_inbox_message_id(&mut self, last_read_inbox_message_id: i64) -> &mut Self {
        self.inner.last_read_inbox_message_id = last_read_inbox_message_id;
        self
    }

    pub fn last_read_outbox_message_id(&mut self, last_read_outbox_message_id: i64) -> &mut Self {
        self.inner.last_read_outbox_message_id = last_read_outbox_message_id;
        self
    }

    pub fn unread_mention_count(&mut self, unread_mention_count: i32) -> &mut Self {
        self.inner.unread_mention_count = unread_mention_count;
        self
    }

    pub fn unread_reaction_count(&mut self, unread_reaction_count: i32) -> &mut Self {
        self.inner.unread_reaction_count = unread_reaction_count;
        self
    }

    pub fn notification_settings<T: AsRef<ChatNotificationSettings>>(
        &mut self,
        notification_settings: T,
    ) -> &mut Self {
        self.inner.notification_settings = notification_settings.as_ref().clone();
        self
    }

    pub fn available_reactions(&mut self, available_reactions: Vec<String>) -> &mut Self {
        self.inner.available_reactions = available_reactions;
        self
    }

    pub fn message_ttl(&mut self, message_ttl: i32) -> &mut Self {
        self.inner.message_ttl = message_ttl;
        self
    }

    pub fn theme_name<T: AsRef<str>>(&mut self, theme_name: T) -> &mut Self {
        self.inner.theme_name = theme_name.as_ref().to_string();
        self
    }

    pub fn action_bar<T: AsRef<ChatActionBar>>(&mut self, action_bar: T) -> &mut Self {
        self.inner.action_bar = Some(action_bar.as_ref().clone());
        self
    }

    pub fn video_chat<T: AsRef<VideoChat>>(&mut self, video_chat: T) -> &mut Self {
        self.inner.video_chat = video_chat.as_ref().clone();
        self
    }

    pub fn pending_join_requests<T: AsRef<ChatJoinRequestsInfo>>(
        &mut self,
        pending_join_requests: T,
    ) -> &mut Self {
        self.inner.pending_join_requests = Some(pending_join_requests.as_ref().clone());
        self
    }

    pub fn reply_markup_message_id(&mut self, reply_markup_message_id: i64) -> &mut Self {
        self.inner.reply_markup_message_id = reply_markup_message_id;
        self
    }

    pub fn draft_message<T: AsRef<DraftMessage>>(&mut self, draft_message: T) -> &mut Self {
        self.inner.draft_message = Some(draft_message.as_ref().clone());
        self
    }

    pub fn client_data<T: AsRef<str>>(&mut self, client_data: T) -> &mut Self {
        self.inner.client_data = client_data.as_ref().to_string();
        self
    }
}

impl AsRef<Chat> for Chat {
    fn as_ref(&self) -> &Chat {
        self
    }
}

impl AsRef<Chat> for RTDChatBuilder {
    fn as_ref(&self) -> &Chat {
        &self.inner
    }
}
