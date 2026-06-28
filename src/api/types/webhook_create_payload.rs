pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookCreatePayload {
    /// The URL to send webhooks to
    #[serde(default)]
    pub url: String,
    /// Optional description of this webhook endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Event type to subscribe toPossible values: loan_updated, installment_updated, client_updated, client_limit_updated, partner_limit_updated
    pub event_type: WebhookEventTypeEnum,
    /// Secret for signing webhook payloads
    #[serde(default)]
    pub secret: String,
    /// Whether to retry failed webhooks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<bool>,
    /// Status of the webhook subscription. Defaults to 'active'.Possible values: active, paused, disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WebhookStatusEnum>,
}

impl WebhookCreatePayload {
    pub fn builder() -> WebhookCreatePayloadBuilder {
        <WebhookCreatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookCreatePayloadBuilder {
    url: Option<String>,
    description: Option<String>,
    event_type: Option<WebhookEventTypeEnum>,
    secret: Option<String>,
    retry: Option<bool>,
    status: Option<WebhookStatusEnum>,
}

impl WebhookCreatePayloadBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn event_type(mut self, value: WebhookEventTypeEnum) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
        self
    }

    pub fn retry(mut self, value: bool) -> Self {
        self.retry = Some(value);
        self
    }

    pub fn status(mut self, value: WebhookStatusEnum) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhookCreatePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](WebhookCreatePayloadBuilder::url)
    /// - [`event_type`](WebhookCreatePayloadBuilder::event_type)
    /// - [`secret`](WebhookCreatePayloadBuilder::secret)
    pub fn build(self) -> Result<WebhookCreatePayload, BuildError> {
        Ok(WebhookCreatePayload {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            description: self.description,
            event_type: self
                .event_type
                .ok_or_else(|| BuildError::missing_field("event_type"))?,
            secret: self
                .secret
                .ok_or_else(|| BuildError::missing_field("secret"))?,
            retry: self.retry,
            status: self.status,
        })
    }
}
