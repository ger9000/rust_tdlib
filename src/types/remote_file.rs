
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Represents a remote file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoteFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Remote file identifier; may be empty. Can be used across application restarts or even from other devices for the current user. Uniquely identifies a file, but a file can have a lot of different valid identifiers. If the ID starts with "http://" or "https://", it represents the HTTP URL of the file. TDLib is currently unable to download files if only their URL is known. If downloadFile is called on such a file or if it is sent to a secret chat, TDLib starts a file generation process by sending updateFileGenerationStart to the client with the HTTP URL in the original_path and "#url#" as the conversion string. Clients should generate the file by downloading it to the specified location
  id: String,
  /// Unique file identifier; may be empty if unknown. The unique file identifier which is the same for the same file even for different users and is persistent over time
  unique_id: String,
  /// True, if the file is currently being uploaded (or a remote copy is being generated by some other means)
  is_uploading_active: bool,
  /// True, if a remote copy is fully available
  is_uploading_completed: bool,
  /// Size of the remote available part of the file; 0 if unknown
  uploaded_size: i64,
  
}

impl RObject for RemoteFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "remoteFile" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl RemoteFile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRemoteFileBuilder {
    let mut inner = RemoteFile::default();
    inner.td_name = "remoteFile".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRemoteFileBuilder { inner }
  }

  pub fn id(&self) -> &String { &self.id }

  pub fn unique_id(&self) -> &String { &self.unique_id }

  pub fn is_uploading_active(&self) -> bool { self.is_uploading_active }

  pub fn is_uploading_completed(&self) -> bool { self.is_uploading_completed }

  pub fn uploaded_size(&self) -> i64 { self.uploaded_size }

}

#[doc(hidden)]
pub struct RTDRemoteFileBuilder {
  inner: RemoteFile
}

impl RTDRemoteFileBuilder {
  pub fn build(&self) -> RemoteFile { self.inner.clone() }

   
  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

   
  pub fn unique_id<T: AsRef<str>>(&mut self, unique_id: T) -> &mut Self {
    self.inner.unique_id = unique_id.as_ref().to_string();
    self
  }

   
  pub fn is_uploading_active(&mut self, is_uploading_active: bool) -> &mut Self {
    self.inner.is_uploading_active = is_uploading_active;
    self
  }

   
  pub fn is_uploading_completed(&mut self, is_uploading_completed: bool) -> &mut Self {
    self.inner.is_uploading_completed = is_uploading_completed;
    self
  }

   
  pub fn uploaded_size(&mut self, uploaded_size: i64) -> &mut Self {
    self.inner.uploaded_size = uploaded_size;
    self
  }

}

impl AsRef<RemoteFile> for RemoteFile {
  fn as_ref(&self) -> &RemoteFile { self }
}

impl AsRef<RemoteFile> for RTDRemoteFileBuilder {
  fn as_ref(&self) -> &RemoteFile { &self.inner }
}


