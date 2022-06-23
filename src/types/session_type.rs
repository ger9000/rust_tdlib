use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents the type of a session
pub trait TDSessionType: Debug + RObject {}

/// Represents the type of a session
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SessionType {
    #[doc(hidden)]
    _Default,
    /// The session is running on an Android device
    #[serde(rename(deserialize = "sessionTypeAndroid"))]
    Android(SessionTypeAndroid),
    /// The session is running on a generic Apple device
    #[serde(rename(deserialize = "sessionTypeApple"))]
    Apple(SessionTypeApple),
    /// The session is running on the Brave browser
    #[serde(rename(deserialize = "sessionTypeBrave"))]
    Brave(SessionTypeBrave),
    /// The session is running on the Chrome browser
    #[serde(rename(deserialize = "sessionTypeChrome"))]
    Chrome(SessionTypeChrome),
    /// The session is running on the Edge browser
    #[serde(rename(deserialize = "sessionTypeEdge"))]
    Edge(SessionTypeEdge),
    /// The session is running on the Firefox browser
    #[serde(rename(deserialize = "sessionTypeFirefox"))]
    Firefox(SessionTypeFirefox),
    /// The session is running on an iPad device
    #[serde(rename(deserialize = "sessionTypeIpad"))]
    Ipad(SessionTypeIpad),
    /// The session is running on an iPhone device
    #[serde(rename(deserialize = "sessionTypeIphone"))]
    Iphone(SessionTypeIphone),
    /// The session is running on a Linux device
    #[serde(rename(deserialize = "sessionTypeLinux"))]
    Linux(SessionTypeLinux),
    /// The session is running on a Mac device
    #[serde(rename(deserialize = "sessionTypeMac"))]
    Mac(SessionTypeMac),
    /// The session is running on the Opera browser
    #[serde(rename(deserialize = "sessionTypeOpera"))]
    Opera(SessionTypeOpera),
    /// The session is running on the Safari browser
    #[serde(rename(deserialize = "sessionTypeSafari"))]
    Safari(SessionTypeSafari),
    /// The session is running on an Ubuntu device
    #[serde(rename(deserialize = "sessionTypeUbuntu"))]
    Ubuntu(SessionTypeUbuntu),
    /// The session is running on an unknown type of device
    #[serde(rename(deserialize = "sessionTypeUnknown"))]
    Unknown(SessionTypeUnknown),
    /// The session is running on the Vivaldi browser
    #[serde(rename(deserialize = "sessionTypeVivaldi"))]
    Vivaldi(SessionTypeVivaldi),
    /// The session is running on a Windows device
    #[serde(rename(deserialize = "sessionTypeWindows"))]
    Windows(SessionTypeWindows),
    /// The session is running on an Xbox console
    #[serde(rename(deserialize = "sessionTypeXbox"))]
    Xbox(SessionTypeXbox),
}

impl Default for SessionType {
    fn default() -> Self {
        SessionType::_Default
    }
}

impl RObject for SessionType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            SessionType::Android(t) => t.extra(),
            SessionType::Apple(t) => t.extra(),
            SessionType::Brave(t) => t.extra(),
            SessionType::Chrome(t) => t.extra(),
            SessionType::Edge(t) => t.extra(),
            SessionType::Firefox(t) => t.extra(),
            SessionType::Ipad(t) => t.extra(),
            SessionType::Iphone(t) => t.extra(),
            SessionType::Linux(t) => t.extra(),
            SessionType::Mac(t) => t.extra(),
            SessionType::Opera(t) => t.extra(),
            SessionType::Safari(t) => t.extra(),
            SessionType::Ubuntu(t) => t.extra(),
            SessionType::Unknown(t) => t.extra(),
            SessionType::Vivaldi(t) => t.extra(),
            SessionType::Windows(t) => t.extra(),
            SessionType::Xbox(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            SessionType::Android(t) => t.client_id(),
            SessionType::Apple(t) => t.client_id(),
            SessionType::Brave(t) => t.client_id(),
            SessionType::Chrome(t) => t.client_id(),
            SessionType::Edge(t) => t.client_id(),
            SessionType::Firefox(t) => t.client_id(),
            SessionType::Ipad(t) => t.client_id(),
            SessionType::Iphone(t) => t.client_id(),
            SessionType::Linux(t) => t.client_id(),
            SessionType::Mac(t) => t.client_id(),
            SessionType::Opera(t) => t.client_id(),
            SessionType::Safari(t) => t.client_id(),
            SessionType::Ubuntu(t) => t.client_id(),
            SessionType::Unknown(t) => t.client_id(),
            SessionType::Vivaldi(t) => t.client_id(),
            SessionType::Windows(t) => t.client_id(),
            SessionType::Xbox(t) => t.client_id(),

            _ => None,
        }
    }
}

impl SessionType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, SessionType::_Default)
    }
}

impl AsRef<SessionType> for SessionType {
    fn as_ref(&self) -> &SessionType {
        self
    }
}

/// The session is running on an Android device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTypeAndroid {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SessionTypeAndroid {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSessionType for SessionTypeAndroid {}

impl SessionTypeAndroid {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSessionTypeAndroidBuilder {
        let mut inner = SessionTypeAndroid::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSessionTypeAndroidBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSessionTypeAndroidBuilder {
    inner: SessionTypeAndroid,
}

impl RTDSessionTypeAndroidBuilder {
    pub fn build(&self) -> SessionTypeAndroid {
        self.inner.clone()
    }
}

impl AsRef<SessionTypeAndroid> for SessionTypeAndroid {
    fn as_ref(&self) -> &SessionTypeAndroid {
        self
    }
}

impl AsRef<SessionTypeAndroid> for RTDSessionTypeAndroidBuilder {
    fn as_ref(&self) -> &SessionTypeAndroid {
        &self.inner
    }
}

/// The session is running on a generic Apple device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTypeApple {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SessionTypeApple {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSessionType for SessionTypeApple {}

impl SessionTypeApple {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSessionTypeAppleBuilder {
        let mut inner = SessionTypeApple::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSessionTypeAppleBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSessionTypeAppleBuilder {
    inner: SessionTypeApple,
}

impl RTDSessionTypeAppleBuilder {
    pub fn build(&self) -> SessionTypeApple {
        self.inner.clone()
    }
}

impl AsRef<SessionTypeApple> for SessionTypeApple {
    fn as_ref(&self) -> &SessionTypeApple {
        self
    }
}

impl AsRef<SessionTypeApple> for RTDSessionTypeAppleBuilder {
    fn as_ref(&self) -> &SessionTypeApple {
        &self.inner
    }
}

/// The session is running on the Brave browser
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTypeBrave {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SessionTypeBrave {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSessionType for SessionTypeBrave {}

impl SessionTypeBrave {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSessionTypeBraveBuilder {
        let mut inner = SessionTypeBrave::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSessionTypeBraveBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSessionTypeBraveBuilder {
    inner: SessionTypeBrave,
}

impl RTDSessionTypeBraveBuilder {
    pub fn build(&self) -> SessionTypeBrave {
        self.inner.clone()
    }
}

impl AsRef<SessionTypeBrave> for SessionTypeBrave {
    fn as_ref(&self) -> &SessionTypeBrave {
        self
    }
}

impl AsRef<SessionTypeBrave> for RTDSessionTypeBraveBuilder {
    fn as_ref(&self) -> &SessionTypeBrave {
        &self.inner
    }
}

/// The session is running on the Chrome browser
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTypeChrome {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SessionTypeChrome {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSessionType for SessionTypeChrome {}

impl SessionTypeChrome {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSessionTypeChromeBuilder {
        let mut inner = SessionTypeChrome::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSessionTypeChromeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSessionTypeChromeBuilder {
    inner: SessionTypeChrome,
}

impl RTDSessionTypeChromeBuilder {
    pub fn build(&self) -> SessionTypeChrome {
        self.inner.clone()
    }
}

impl AsRef<SessionTypeChrome> for SessionTypeChrome {
    fn as_ref(&self) -> &SessionTypeChrome {
        self
    }
}

impl AsRef<SessionTypeChrome> for RTDSessionTypeChromeBuilder {
    fn as_ref(&self) -> &SessionTypeChrome {
        &self.inner
    }
}

/// The session is running on the Edge browser
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTypeEdge {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SessionTypeEdge {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSessionType for SessionTypeEdge {}

impl SessionTypeEdge {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSessionTypeEdgeBuilder {
        let mut inner = SessionTypeEdge::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSessionTypeEdgeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSessionTypeEdgeBuilder {
    inner: SessionTypeEdge,
}

impl RTDSessionTypeEdgeBuilder {
    pub fn build(&self) -> SessionTypeEdge {
        self.inner.clone()
    }
}

impl AsRef<SessionTypeEdge> for SessionTypeEdge {
    fn as_ref(&self) -> &SessionTypeEdge {
        self
    }
}

impl AsRef<SessionTypeEdge> for RTDSessionTypeEdgeBuilder {
    fn as_ref(&self) -> &SessionTypeEdge {
        &self.inner
    }
}

/// The session is running on the Firefox browser
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTypeFirefox {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SessionTypeFirefox {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSessionType for SessionTypeFirefox {}

impl SessionTypeFirefox {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSessionTypeFirefoxBuilder {
        let mut inner = SessionTypeFirefox::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSessionTypeFirefoxBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSessionTypeFirefoxBuilder {
    inner: SessionTypeFirefox,
}

impl RTDSessionTypeFirefoxBuilder {
    pub fn build(&self) -> SessionTypeFirefox {
        self.inner.clone()
    }
}

impl AsRef<SessionTypeFirefox> for SessionTypeFirefox {
    fn as_ref(&self) -> &SessionTypeFirefox {
        self
    }
}

impl AsRef<SessionTypeFirefox> for RTDSessionTypeFirefoxBuilder {
    fn as_ref(&self) -> &SessionTypeFirefox {
        &self.inner
    }
}

/// The session is running on an iPad device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTypeIpad {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SessionTypeIpad {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSessionType for SessionTypeIpad {}

impl SessionTypeIpad {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSessionTypeIpadBuilder {
        let mut inner = SessionTypeIpad::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSessionTypeIpadBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSessionTypeIpadBuilder {
    inner: SessionTypeIpad,
}

impl RTDSessionTypeIpadBuilder {
    pub fn build(&self) -> SessionTypeIpad {
        self.inner.clone()
    }
}

impl AsRef<SessionTypeIpad> for SessionTypeIpad {
    fn as_ref(&self) -> &SessionTypeIpad {
        self
    }
}

impl AsRef<SessionTypeIpad> for RTDSessionTypeIpadBuilder {
    fn as_ref(&self) -> &SessionTypeIpad {
        &self.inner
    }
}

/// The session is running on an iPhone device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTypeIphone {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SessionTypeIphone {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSessionType for SessionTypeIphone {}

impl SessionTypeIphone {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSessionTypeIphoneBuilder {
        let mut inner = SessionTypeIphone::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSessionTypeIphoneBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSessionTypeIphoneBuilder {
    inner: SessionTypeIphone,
}

impl RTDSessionTypeIphoneBuilder {
    pub fn build(&self) -> SessionTypeIphone {
        self.inner.clone()
    }
}

impl AsRef<SessionTypeIphone> for SessionTypeIphone {
    fn as_ref(&self) -> &SessionTypeIphone {
        self
    }
}

impl AsRef<SessionTypeIphone> for RTDSessionTypeIphoneBuilder {
    fn as_ref(&self) -> &SessionTypeIphone {
        &self.inner
    }
}

/// The session is running on a Linux device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTypeLinux {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SessionTypeLinux {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSessionType for SessionTypeLinux {}

impl SessionTypeLinux {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSessionTypeLinuxBuilder {
        let mut inner = SessionTypeLinux::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSessionTypeLinuxBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSessionTypeLinuxBuilder {
    inner: SessionTypeLinux,
}

impl RTDSessionTypeLinuxBuilder {
    pub fn build(&self) -> SessionTypeLinux {
        self.inner.clone()
    }
}

impl AsRef<SessionTypeLinux> for SessionTypeLinux {
    fn as_ref(&self) -> &SessionTypeLinux {
        self
    }
}

impl AsRef<SessionTypeLinux> for RTDSessionTypeLinuxBuilder {
    fn as_ref(&self) -> &SessionTypeLinux {
        &self.inner
    }
}

/// The session is running on a Mac device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTypeMac {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SessionTypeMac {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSessionType for SessionTypeMac {}

impl SessionTypeMac {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSessionTypeMacBuilder {
        let mut inner = SessionTypeMac::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSessionTypeMacBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSessionTypeMacBuilder {
    inner: SessionTypeMac,
}

impl RTDSessionTypeMacBuilder {
    pub fn build(&self) -> SessionTypeMac {
        self.inner.clone()
    }
}

impl AsRef<SessionTypeMac> for SessionTypeMac {
    fn as_ref(&self) -> &SessionTypeMac {
        self
    }
}

impl AsRef<SessionTypeMac> for RTDSessionTypeMacBuilder {
    fn as_ref(&self) -> &SessionTypeMac {
        &self.inner
    }
}

/// The session is running on the Opera browser
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTypeOpera {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SessionTypeOpera {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSessionType for SessionTypeOpera {}

impl SessionTypeOpera {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSessionTypeOperaBuilder {
        let mut inner = SessionTypeOpera::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSessionTypeOperaBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSessionTypeOperaBuilder {
    inner: SessionTypeOpera,
}

impl RTDSessionTypeOperaBuilder {
    pub fn build(&self) -> SessionTypeOpera {
        self.inner.clone()
    }
}

impl AsRef<SessionTypeOpera> for SessionTypeOpera {
    fn as_ref(&self) -> &SessionTypeOpera {
        self
    }
}

impl AsRef<SessionTypeOpera> for RTDSessionTypeOperaBuilder {
    fn as_ref(&self) -> &SessionTypeOpera {
        &self.inner
    }
}

/// The session is running on the Safari browser
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTypeSafari {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SessionTypeSafari {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSessionType for SessionTypeSafari {}

impl SessionTypeSafari {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSessionTypeSafariBuilder {
        let mut inner = SessionTypeSafari::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSessionTypeSafariBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSessionTypeSafariBuilder {
    inner: SessionTypeSafari,
}

impl RTDSessionTypeSafariBuilder {
    pub fn build(&self) -> SessionTypeSafari {
        self.inner.clone()
    }
}

impl AsRef<SessionTypeSafari> for SessionTypeSafari {
    fn as_ref(&self) -> &SessionTypeSafari {
        self
    }
}

impl AsRef<SessionTypeSafari> for RTDSessionTypeSafariBuilder {
    fn as_ref(&self) -> &SessionTypeSafari {
        &self.inner
    }
}

/// The session is running on an Ubuntu device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTypeUbuntu {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SessionTypeUbuntu {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSessionType for SessionTypeUbuntu {}

impl SessionTypeUbuntu {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSessionTypeUbuntuBuilder {
        let mut inner = SessionTypeUbuntu::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSessionTypeUbuntuBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSessionTypeUbuntuBuilder {
    inner: SessionTypeUbuntu,
}

impl RTDSessionTypeUbuntuBuilder {
    pub fn build(&self) -> SessionTypeUbuntu {
        self.inner.clone()
    }
}

impl AsRef<SessionTypeUbuntu> for SessionTypeUbuntu {
    fn as_ref(&self) -> &SessionTypeUbuntu {
        self
    }
}

impl AsRef<SessionTypeUbuntu> for RTDSessionTypeUbuntuBuilder {
    fn as_ref(&self) -> &SessionTypeUbuntu {
        &self.inner
    }
}

/// The session is running on an unknown type of device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTypeUnknown {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SessionTypeUnknown {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSessionType for SessionTypeUnknown {}

impl SessionTypeUnknown {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSessionTypeUnknownBuilder {
        let mut inner = SessionTypeUnknown::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSessionTypeUnknownBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSessionTypeUnknownBuilder {
    inner: SessionTypeUnknown,
}

impl RTDSessionTypeUnknownBuilder {
    pub fn build(&self) -> SessionTypeUnknown {
        self.inner.clone()
    }
}

impl AsRef<SessionTypeUnknown> for SessionTypeUnknown {
    fn as_ref(&self) -> &SessionTypeUnknown {
        self
    }
}

impl AsRef<SessionTypeUnknown> for RTDSessionTypeUnknownBuilder {
    fn as_ref(&self) -> &SessionTypeUnknown {
        &self.inner
    }
}

/// The session is running on the Vivaldi browser
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTypeVivaldi {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SessionTypeVivaldi {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSessionType for SessionTypeVivaldi {}

impl SessionTypeVivaldi {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSessionTypeVivaldiBuilder {
        let mut inner = SessionTypeVivaldi::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSessionTypeVivaldiBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSessionTypeVivaldiBuilder {
    inner: SessionTypeVivaldi,
}

impl RTDSessionTypeVivaldiBuilder {
    pub fn build(&self) -> SessionTypeVivaldi {
        self.inner.clone()
    }
}

impl AsRef<SessionTypeVivaldi> for SessionTypeVivaldi {
    fn as_ref(&self) -> &SessionTypeVivaldi {
        self
    }
}

impl AsRef<SessionTypeVivaldi> for RTDSessionTypeVivaldiBuilder {
    fn as_ref(&self) -> &SessionTypeVivaldi {
        &self.inner
    }
}

/// The session is running on a Windows device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTypeWindows {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SessionTypeWindows {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSessionType for SessionTypeWindows {}

impl SessionTypeWindows {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSessionTypeWindowsBuilder {
        let mut inner = SessionTypeWindows::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSessionTypeWindowsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSessionTypeWindowsBuilder {
    inner: SessionTypeWindows,
}

impl RTDSessionTypeWindowsBuilder {
    pub fn build(&self) -> SessionTypeWindows {
        self.inner.clone()
    }
}

impl AsRef<SessionTypeWindows> for SessionTypeWindows {
    fn as_ref(&self) -> &SessionTypeWindows {
        self
    }
}

impl AsRef<SessionTypeWindows> for RTDSessionTypeWindowsBuilder {
    fn as_ref(&self) -> &SessionTypeWindows {
        &self.inner
    }
}

/// The session is running on an Xbox console
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTypeXbox {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SessionTypeXbox {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSessionType for SessionTypeXbox {}

impl SessionTypeXbox {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSessionTypeXboxBuilder {
        let mut inner = SessionTypeXbox::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSessionTypeXboxBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSessionTypeXboxBuilder {
    inner: SessionTypeXbox,
}

impl RTDSessionTypeXboxBuilder {
    pub fn build(&self) -> SessionTypeXbox {
        self.inner.clone()
    }
}

impl AsRef<SessionTypeXbox> for SessionTypeXbox {
    fn as_ref(&self) -> &SessionTypeXbox {
        self
    }
}

impl AsRef<SessionTypeXbox> for RTDSessionTypeXboxBuilder {
    fn as_ref(&self) -> &SessionTypeXbox {
        &self.inner
    }
}