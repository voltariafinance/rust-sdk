pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookSubscriptionResponse {
    /// The ID of the webhook subscription
    #[serde(default)]
    pub id: String,
    /// The URL to send webhooks to
    #[serde(default)]
    pub url: String,
    /// Description of this webhook endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Event type subscribed toPossible values: loan_updated, installment_updated, client_updated, client_limit_updated, partner_limit_updated
    pub event_type: WebhookEventTypeEnum,
    /// Status of the webhook subscriptionPossible values: active, paused, disabled
    pub status: WebhookStatusEnum,
    /// Whether to retry failed webhooks
    #[serde(default)]
    pub retry: bool,
    /// Secret for signing webhook payloads
    #[serde(default)]
    pub secret: String,
    /// Creation timestamp
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Last update timestamp
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl WebhookSubscriptionResponse {
    pub fn builder() -> WebhookSubscriptionResponseBuilder {
        <WebhookSubscriptionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookSubscriptionResponseBuilder {
    id: Option<String>,
    url: Option<String>,
    description: Option<String>,
    event_type: Option<WebhookEventTypeEnum>,
    status: Option<WebhookStatusEnum>,
    retry: Option<bool>,
    secret: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl WebhookSubscriptionResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhookSubscriptionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WebhookSubscriptionResponseBuilder::id)
    /// - [`url`](WebhookSubscriptionResponseBuilder::url)
    /// - [`event_type`](WebhookSubscriptionResponseBuilder::event_type)
    /// - [`status`](WebhookSubscriptionResponseBuilder::status)
    /// - [`retry`](WebhookSubscriptionResponseBuilder::retry)
    /// - [`secret`](WebhookSubscriptionResponseBuilder::secret)
    /// - [`created_at`](WebhookSubscriptionResponseBuilder::created_at)
    /// - [`updated_at`](WebhookSubscriptionResponseBuilder::updated_at)
    pub fn build(self) -> Result<WebhookSubscriptionResponse, BuildError> {
        Ok(WebhookSubscriptionResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            description: self.description,
            event_type: self
                .event_type
                .ok_or_else(|| BuildError::missing_field("event_type"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            retry: self
                .retry
                .ok_or_else(|| BuildError::missing_field("retry"))?,
            secret: self
                .secret
                .ok_or_else(|| BuildError::missing_field("secret"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
