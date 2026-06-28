pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhookUpdatePayload {
    /// The URL to send webhooks to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Description of this webhook endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Event type to subscribe toPossible values: loan_updated, installment_updated, client_updated, client_limit_updated, partner_limit_updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<WebhookEventTypeEnum>,
    /// Status of the webhook subscriptionPossible values: active, paused, disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WebhookStatusEnum>,
    /// Whether to retry failed webhooks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<bool>,
    /// Secret for signing webhook payloads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl WebhookUpdatePayload {
    pub fn builder() -> WebhookUpdatePayloadBuilder {
        <WebhookUpdatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookUpdatePayloadBuilder {
    url: Option<String>,
    description: Option<String>,
    event_type: Option<WebhookEventTypeEnum>,
    status: Option<WebhookStatusEnum>,
    retry: Option<bool>,
    secret: Option<String>,
}

impl WebhookUpdatePayloadBuilder {
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

    pub fn status(mut self, value: WebhookStatusEnum) -> Self {
        self.status = Some(value);
        self
    }

    pub fn retry(mut self, value: bool) -> Self {
        self.retry = Some(value);
        self
    }

    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebhookUpdatePayload`].
    pub fn build(self) -> Result<WebhookUpdatePayload, BuildError> {
        Ok(WebhookUpdatePayload {
            url: self.url,
            description: self.description,
            event_type: self.event_type,
            status: self.status,
            retry: self.retry,
            secret: self.secret,
        })
    }
}
