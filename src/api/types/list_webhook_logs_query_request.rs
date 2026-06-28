pub use crate::prelude::*;

/// Query parameters for list_webhook_logs
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListWebhookLogsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl ListWebhookLogsQueryRequest {
    pub fn builder() -> ListWebhookLogsQueryRequestBuilder {
        <ListWebhookLogsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWebhookLogsQueryRequestBuilder {
    webhook_id: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
}

impl ListWebhookLogsQueryRequestBuilder {
    pub fn webhook_id(mut self, value: impl Into<String>) -> Self {
        self.webhook_id = Some(value.into());
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListWebhookLogsQueryRequest`].
    pub fn build(self) -> Result<ListWebhookLogsQueryRequest, BuildError> {
        Ok(ListWebhookLogsQueryRequest {
            webhook_id: self.webhook_id,
            page: self.page,
            page_size: self.page_size,
        })
    }
}
