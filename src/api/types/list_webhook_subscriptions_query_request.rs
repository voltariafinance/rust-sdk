pub use crate::prelude::*;

/// Query parameters for list_webhook_subscriptions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListWebhookSubscriptionsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<WebhookEventTypeEnum>,
}

impl ListWebhookSubscriptionsQueryRequest {
    pub fn builder() -> ListWebhookSubscriptionsQueryRequestBuilder {
        <ListWebhookSubscriptionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWebhookSubscriptionsQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    event_type: Option<WebhookEventTypeEnum>,
}

impl ListWebhookSubscriptionsQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn event_type(mut self, value: WebhookEventTypeEnum) -> Self {
        self.event_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListWebhookSubscriptionsQueryRequest`].
    pub fn build(self) -> Result<ListWebhookSubscriptionsQueryRequest, BuildError> {
        Ok(ListWebhookSubscriptionsQueryRequest {
            page: self.page,
            page_size: self.page_size,
            event_type: self.event_type,
        })
    }
}
