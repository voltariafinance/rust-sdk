pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebhookLogResponse {
    /// The ID of the webhook log
    #[serde(default)]
    pub id: String,
    /// The ID of the webhook subscription
    #[serde(default)]
    pub webhook_id: String,
    /// Creation timestamp
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Last update timestamp
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The type of event
    pub event_type: WebhookEventTypeEnum,
    /// The URL of the webhook subscription
    #[serde(default)]
    pub webhook_url: String,
    /// Whether the webhook was delivered successfully
    #[serde(default)]
    pub success: bool,
    /// The HTTP status code returned by the server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
    /// Error message if the webhook failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// The time taken to deliver the webhook in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_duration_ms: Option<i64>,
    /// The request body sent to the webhook URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<WebhookLogResponseRequestBody>,
}

impl WebhookLogResponse {
    pub fn builder() -> WebhookLogResponseBuilder {
        <WebhookLogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookLogResponseBuilder {
    id: Option<String>,
    webhook_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    event_type: Option<WebhookEventTypeEnum>,
    webhook_url: Option<String>,
    success: Option<bool>,
    status_code: Option<i64>,
    error_message: Option<String>,
    request_duration_ms: Option<i64>,
    request_body: Option<WebhookLogResponseRequestBody>,
}

impl WebhookLogResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn webhook_id(mut self, value: impl Into<String>) -> Self {
        self.webhook_id = Some(value.into());
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

    pub fn event_type(mut self, value: WebhookEventTypeEnum) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn webhook_url(mut self, value: impl Into<String>) -> Self {
        self.webhook_url = Some(value.into());
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn status_code(mut self, value: i64) -> Self {
        self.status_code = Some(value);
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn request_duration_ms(mut self, value: i64) -> Self {
        self.request_duration_ms = Some(value);
        self
    }

    pub fn request_body(mut self, value: WebhookLogResponseRequestBody) -> Self {
        self.request_body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhookLogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WebhookLogResponseBuilder::id)
    /// - [`webhook_id`](WebhookLogResponseBuilder::webhook_id)
    /// - [`created_at`](WebhookLogResponseBuilder::created_at)
    /// - [`updated_at`](WebhookLogResponseBuilder::updated_at)
    /// - [`event_type`](WebhookLogResponseBuilder::event_type)
    /// - [`webhook_url`](WebhookLogResponseBuilder::webhook_url)
    /// - [`success`](WebhookLogResponseBuilder::success)
    pub fn build(self) -> Result<WebhookLogResponse, BuildError> {
        Ok(WebhookLogResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            webhook_id: self
                .webhook_id
                .ok_or_else(|| BuildError::missing_field("webhook_id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            event_type: self
                .event_type
                .ok_or_else(|| BuildError::missing_field("event_type"))?,
            webhook_url: self
                .webhook_url
                .ok_or_else(|| BuildError::missing_field("webhook_url"))?,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            status_code: self.status_code,
            error_message: self.error_message,
            request_duration_ms: self.request_duration_ms,
            request_body: self.request_body,
        })
    }
}
