pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookTestSandbox {
    /// The ID of the webhook subscription. Only this webhook will be triggered if provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
    /// Event type to trigger for the test. All subscriptions for this event type will be triggered if webhook_id is not provided.Possible values: loan_updated, installment_updated, client_updated, client_limit_updated, partner_limit_updated
    pub event_type: WebhookEventTypeEnum,
}

impl WebhookTestSandbox {
    pub fn builder() -> WebhookTestSandboxBuilder {
        <WebhookTestSandboxBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookTestSandboxBuilder {
    webhook_id: Option<String>,
    event_type: Option<WebhookEventTypeEnum>,
}

impl WebhookTestSandboxBuilder {
    pub fn webhook_id(mut self, value: impl Into<String>) -> Self {
        self.webhook_id = Some(value.into());
        self
    }

    pub fn event_type(mut self, value: WebhookEventTypeEnum) -> Self {
        self.event_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhookTestSandbox`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_type`](WebhookTestSandboxBuilder::event_type)
    pub fn build(self) -> Result<WebhookTestSandbox, BuildError> {
        Ok(WebhookTestSandbox {
            webhook_id: self.webhook_id,
            event_type: self
                .event_type
                .ok_or_else(|| BuildError::missing_field("event_type"))?,
        })
    }
}
