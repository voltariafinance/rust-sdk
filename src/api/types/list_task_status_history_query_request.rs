pub use crate::prelude::*;

/// Query parameters for list_task_status_history
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListTaskStatusHistoryQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Field to order the results by, e.g., 'created_at:asc'. Defaults to 'created_at:desc'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
}

impl ListTaskStatusHistoryQueryRequest {
    pub fn builder() -> ListTaskStatusHistoryQueryRequestBuilder {
        <ListTaskStatusHistoryQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTaskStatusHistoryQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    order_by: Option<String>,
}

impl ListTaskStatusHistoryQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn order_by(mut self, value: impl Into<String>) -> Self {
        self.order_by = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListTaskStatusHistoryQueryRequest`].
    pub fn build(self) -> Result<ListTaskStatusHistoryQueryRequest, BuildError> {
        Ok(ListTaskStatusHistoryQueryRequest {
            page: self.page,
            page_size: self.page_size,
            order_by: self.order_by,
        })
    }
}
