use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the reason why a chat is reported
pub trait TDChatReportReason: Debug + RObject {}

/// Describes the reason why a chat is reported
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatReportReason {
    #[doc(hidden)]
    _Default(()),
    /// The chat has child abuse related content
    ChildAbuse(ChatReportReasonChildAbuse),
    /// The chat contains copyrighted content
    Copyright(ChatReportReasonCopyright),
    /// A custom reason provided by the user
    Custom(ChatReportReasonCustom),
    /// The chat contains pornographic messages
    Pornography(ChatReportReasonPornography),
    /// The chat contains spam messages
    Spam(ChatReportReasonSpam),
    /// The location-based chat is unrelated to its stated location
    UnrelatedLocation(ChatReportReasonUnrelatedLocation),
    /// The chat promotes violence
    Violence(ChatReportReasonViolence),
}

impl Default for ChatReportReason {
    fn default() -> Self {
        ChatReportReason::_Default(())
    }
}

impl<'de> Deserialize<'de> for ChatReportReason {
    fn deserialize<D>(deserializer: D) -> Result<ChatReportReason, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          ChatReportReason,
          (chatReportReasonChildAbuse, ChildAbuse);
          (chatReportReasonCopyright, Copyright);
          (chatReportReasonCustom, Custom);
          (chatReportReasonPornography, Pornography);
          (chatReportReasonSpam, Spam);
          (chatReportReasonUnrelatedLocation, UnrelatedLocation);
          (chatReportReasonViolence, Violence);

        )(deserializer)
    }
}

impl RObject for ChatReportReason {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            ChatReportReason::ChildAbuse(t) => t.td_name(),
            ChatReportReason::Copyright(t) => t.td_name(),
            ChatReportReason::Custom(t) => t.td_name(),
            ChatReportReason::Pornography(t) => t.td_name(),
            ChatReportReason::Spam(t) => t.td_name(),
            ChatReportReason::UnrelatedLocation(t) => t.td_name(),
            ChatReportReason::Violence(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            ChatReportReason::ChildAbuse(t) => t.extra(),
            ChatReportReason::Copyright(t) => t.extra(),
            ChatReportReason::Custom(t) => t.extra(),
            ChatReportReason::Pornography(t) => t.extra(),
            ChatReportReason::Spam(t) => t.extra(),
            ChatReportReason::UnrelatedLocation(t) => t.extra(),
            ChatReportReason::Violence(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl ChatReportReason {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        if let ChatReportReason::_Default(_) = self {
            true
        } else {
            false
        }
    }
}

impl AsRef<ChatReportReason> for ChatReportReason {
    fn as_ref(&self) -> &ChatReportReason {
        self
    }
}

/// The chat has child abuse related content
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatReportReasonChildAbuse {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for ChatReportReasonChildAbuse {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "chatReportReasonChildAbuse"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDChatReportReason for ChatReportReasonChildAbuse {}

impl ChatReportReasonChildAbuse {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatReportReasonChildAbuseBuilder {
        let mut inner = ChatReportReasonChildAbuse::default();
        inner.td_name = "chatReportReasonChildAbuse".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDChatReportReasonChildAbuseBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatReportReasonChildAbuseBuilder {
    inner: ChatReportReasonChildAbuse,
}

impl RTDChatReportReasonChildAbuseBuilder {
    pub fn build(&self) -> ChatReportReasonChildAbuse {
        self.inner.clone()
    }
}

impl AsRef<ChatReportReasonChildAbuse> for ChatReportReasonChildAbuse {
    fn as_ref(&self) -> &ChatReportReasonChildAbuse {
        self
    }
}

impl AsRef<ChatReportReasonChildAbuse> for RTDChatReportReasonChildAbuseBuilder {
    fn as_ref(&self) -> &ChatReportReasonChildAbuse {
        &self.inner
    }
}

/// The chat contains copyrighted content
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatReportReasonCopyright {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for ChatReportReasonCopyright {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "chatReportReasonCopyright"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDChatReportReason for ChatReportReasonCopyright {}

impl ChatReportReasonCopyright {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatReportReasonCopyrightBuilder {
        let mut inner = ChatReportReasonCopyright::default();
        inner.td_name = "chatReportReasonCopyright".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDChatReportReasonCopyrightBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatReportReasonCopyrightBuilder {
    inner: ChatReportReasonCopyright,
}

impl RTDChatReportReasonCopyrightBuilder {
    pub fn build(&self) -> ChatReportReasonCopyright {
        self.inner.clone()
    }
}

impl AsRef<ChatReportReasonCopyright> for ChatReportReasonCopyright {
    fn as_ref(&self) -> &ChatReportReasonCopyright {
        self
    }
}

impl AsRef<ChatReportReasonCopyright> for RTDChatReportReasonCopyrightBuilder {
    fn as_ref(&self) -> &ChatReportReasonCopyright {
        &self.inner
    }
}

/// A custom reason provided by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatReportReasonCustom {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Report text
    text: String,
}

impl RObject for ChatReportReasonCustom {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "chatReportReasonCustom"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDChatReportReason for ChatReportReasonCustom {}

impl ChatReportReasonCustom {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatReportReasonCustomBuilder {
        let mut inner = ChatReportReasonCustom::default();
        inner.td_name = "chatReportReasonCustom".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDChatReportReasonCustomBuilder { inner }
    }

    pub fn text(&self) -> &String {
        &self.text
    }
}

#[doc(hidden)]
pub struct RTDChatReportReasonCustomBuilder {
    inner: ChatReportReasonCustom,
}

impl RTDChatReportReasonCustomBuilder {
    pub fn build(&self) -> ChatReportReasonCustom {
        self.inner.clone()
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }
}

impl AsRef<ChatReportReasonCustom> for ChatReportReasonCustom {
    fn as_ref(&self) -> &ChatReportReasonCustom {
        self
    }
}

impl AsRef<ChatReportReasonCustom> for RTDChatReportReasonCustomBuilder {
    fn as_ref(&self) -> &ChatReportReasonCustom {
        &self.inner
    }
}

/// The chat contains pornographic messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatReportReasonPornography {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for ChatReportReasonPornography {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "chatReportReasonPornography"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDChatReportReason for ChatReportReasonPornography {}

impl ChatReportReasonPornography {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatReportReasonPornographyBuilder {
        let mut inner = ChatReportReasonPornography::default();
        inner.td_name = "chatReportReasonPornography".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDChatReportReasonPornographyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatReportReasonPornographyBuilder {
    inner: ChatReportReasonPornography,
}

impl RTDChatReportReasonPornographyBuilder {
    pub fn build(&self) -> ChatReportReasonPornography {
        self.inner.clone()
    }
}

impl AsRef<ChatReportReasonPornography> for ChatReportReasonPornography {
    fn as_ref(&self) -> &ChatReportReasonPornography {
        self
    }
}

impl AsRef<ChatReportReasonPornography> for RTDChatReportReasonPornographyBuilder {
    fn as_ref(&self) -> &ChatReportReasonPornography {
        &self.inner
    }
}

/// The chat contains spam messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatReportReasonSpam {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for ChatReportReasonSpam {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "chatReportReasonSpam"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDChatReportReason for ChatReportReasonSpam {}

impl ChatReportReasonSpam {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatReportReasonSpamBuilder {
        let mut inner = ChatReportReasonSpam::default();
        inner.td_name = "chatReportReasonSpam".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDChatReportReasonSpamBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatReportReasonSpamBuilder {
    inner: ChatReportReasonSpam,
}

impl RTDChatReportReasonSpamBuilder {
    pub fn build(&self) -> ChatReportReasonSpam {
        self.inner.clone()
    }
}

impl AsRef<ChatReportReasonSpam> for ChatReportReasonSpam {
    fn as_ref(&self) -> &ChatReportReasonSpam {
        self
    }
}

impl AsRef<ChatReportReasonSpam> for RTDChatReportReasonSpamBuilder {
    fn as_ref(&self) -> &ChatReportReasonSpam {
        &self.inner
    }
}

/// The location-based chat is unrelated to its stated location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatReportReasonUnrelatedLocation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for ChatReportReasonUnrelatedLocation {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "chatReportReasonUnrelatedLocation"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDChatReportReason for ChatReportReasonUnrelatedLocation {}

impl ChatReportReasonUnrelatedLocation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatReportReasonUnrelatedLocationBuilder {
        let mut inner = ChatReportReasonUnrelatedLocation::default();
        inner.td_name = "chatReportReasonUnrelatedLocation".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDChatReportReasonUnrelatedLocationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatReportReasonUnrelatedLocationBuilder {
    inner: ChatReportReasonUnrelatedLocation,
}

impl RTDChatReportReasonUnrelatedLocationBuilder {
    pub fn build(&self) -> ChatReportReasonUnrelatedLocation {
        self.inner.clone()
    }
}

impl AsRef<ChatReportReasonUnrelatedLocation> for ChatReportReasonUnrelatedLocation {
    fn as_ref(&self) -> &ChatReportReasonUnrelatedLocation {
        self
    }
}

impl AsRef<ChatReportReasonUnrelatedLocation> for RTDChatReportReasonUnrelatedLocationBuilder {
    fn as_ref(&self) -> &ChatReportReasonUnrelatedLocation {
        &self.inner
    }
}

/// The chat promotes violence
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatReportReasonViolence {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for ChatReportReasonViolence {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "chatReportReasonViolence"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDChatReportReason for ChatReportReasonViolence {}

impl ChatReportReasonViolence {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatReportReasonViolenceBuilder {
        let mut inner = ChatReportReasonViolence::default();
        inner.td_name = "chatReportReasonViolence".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDChatReportReasonViolenceBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatReportReasonViolenceBuilder {
    inner: ChatReportReasonViolence,
}

impl RTDChatReportReasonViolenceBuilder {
    pub fn build(&self) -> ChatReportReasonViolence {
        self.inner.clone()
    }
}

impl AsRef<ChatReportReasonViolence> for ChatReportReasonViolence {
    fn as_ref(&self) -> &ChatReportReasonViolence {
        self
    }
}

impl AsRef<ChatReportReasonViolence> for RTDChatReportReasonViolenceBuilder {
    fn as_ref(&self) -> &ChatReportReasonViolence {
        &self.inner
    }
}
