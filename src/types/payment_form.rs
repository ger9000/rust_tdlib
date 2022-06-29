use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about an invoice payment form
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentForm {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The payment form identifier

    #[serde(deserialize_with = "super::_common::number_from_string")]
    #[serde(default)]
    id: i64,
    /// Full information about the invoice
    invoice: Invoice,
    /// User identifier of the seller bot

    #[serde(default)]
    seller_bot_user_id: i64,
    /// User identifier of the payment provider bot

    #[serde(default)]
    payment_provider_user_id: i64,
    /// Information about the payment provider

    #[serde(skip_serializing_if = "PaymentProvider::_is_default")]
    payment_provider: PaymentProvider,
    /// Saved server-side order information; may be null
    saved_order_info: Option<OrderInfo>,
    /// Information about saved card credentials; may be null
    saved_credentials: Option<SavedCredentials>,
    /// True, if the user can choose to save credentials

    #[serde(default)]
    can_save_credentials: bool,
    /// True, if the user will be able to save credentials protected by a password they set up

    #[serde(default)]
    need_password: bool,
    /// Product title

    #[serde(default)]
    product_title: String,
    /// Product description
    product_description: FormattedText,
    /// Product photo; may be null
    product_photo: Option<Photo>,
}

impl RObject for PaymentForm {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PaymentForm {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPaymentFormBuilder {
        let mut inner = PaymentForm::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPaymentFormBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn invoice(&self) -> &Invoice {
        &self.invoice
    }

    pub fn seller_bot_user_id(&self) -> i64 {
        self.seller_bot_user_id
    }

    pub fn payment_provider_user_id(&self) -> i64 {
        self.payment_provider_user_id
    }

    pub fn payment_provider(&self) -> &PaymentProvider {
        &self.payment_provider
    }

    pub fn saved_order_info(&self) -> &Option<OrderInfo> {
        &self.saved_order_info
    }

    pub fn saved_credentials(&self) -> &Option<SavedCredentials> {
        &self.saved_credentials
    }

    pub fn can_save_credentials(&self) -> bool {
        self.can_save_credentials
    }

    pub fn need_password(&self) -> bool {
        self.need_password
    }

    pub fn product_title(&self) -> &String {
        &self.product_title
    }

    pub fn product_description(&self) -> &FormattedText {
        &self.product_description
    }

    pub fn product_photo(&self) -> &Option<Photo> {
        &self.product_photo
    }
}

#[doc(hidden)]
pub struct RTDPaymentFormBuilder {
    inner: PaymentForm,
}

impl RTDPaymentFormBuilder {
    pub fn build(&self) -> PaymentForm {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn invoice<T: AsRef<Invoice>>(&mut self, invoice: T) -> &mut Self {
        self.inner.invoice = invoice.as_ref().clone();
        self
    }

    pub fn seller_bot_user_id(&mut self, seller_bot_user_id: i64) -> &mut Self {
        self.inner.seller_bot_user_id = seller_bot_user_id;
        self
    }

    pub fn payment_provider_user_id(&mut self, payment_provider_user_id: i64) -> &mut Self {
        self.inner.payment_provider_user_id = payment_provider_user_id;
        self
    }

    pub fn payment_provider<T: AsRef<PaymentProvider>>(
        &mut self,
        payment_provider: T,
    ) -> &mut Self {
        self.inner.payment_provider = payment_provider.as_ref().clone();
        self
    }

    pub fn saved_order_info<T: AsRef<OrderInfo>>(&mut self, saved_order_info: T) -> &mut Self {
        self.inner.saved_order_info = Some(saved_order_info.as_ref().clone());
        self
    }

    pub fn saved_credentials<T: AsRef<SavedCredentials>>(
        &mut self,
        saved_credentials: T,
    ) -> &mut Self {
        self.inner.saved_credentials = Some(saved_credentials.as_ref().clone());
        self
    }

    pub fn can_save_credentials(&mut self, can_save_credentials: bool) -> &mut Self {
        self.inner.can_save_credentials = can_save_credentials;
        self
    }

    pub fn need_password(&mut self, need_password: bool) -> &mut Self {
        self.inner.need_password = need_password;
        self
    }

    pub fn product_title<T: AsRef<str>>(&mut self, product_title: T) -> &mut Self {
        self.inner.product_title = product_title.as_ref().to_string();
        self
    }

    pub fn product_description<T: AsRef<FormattedText>>(
        &mut self,
        product_description: T,
    ) -> &mut Self {
        self.inner.product_description = product_description.as_ref().clone();
        self
    }

    pub fn product_photo<T: AsRef<Photo>>(&mut self, product_photo: T) -> &mut Self {
        self.inner.product_photo = Some(product_photo.as_ref().clone());
        self
    }
}

impl AsRef<PaymentForm> for PaymentForm {
    fn as_ref(&self) -> &PaymentForm {
        self
    }
}

impl AsRef<PaymentForm> for RTDPaymentFormBuilder {
    fn as_ref(&self) -> &PaymentForm {
        &self.inner
    }
}
