pub use crate::prelude::*;

/// Query parameters for list_client_checklist_summaries
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListClientChecklistSummariesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl ListClientChecklistSummariesQueryRequest {
    pub fn builder() -> ListClientChecklistSummariesQueryRequestBuilder {
        <ListClientChecklistSummariesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListClientChecklistSummariesQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
}

impl ListClientChecklistSummariesQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListClientChecklistSummariesQueryRequest`].
    pub fn build(self) -> Result<ListClientChecklistSummariesQueryRequest, BuildError> {
        Ok(ListClientChecklistSummariesQueryRequest {
            page: self.page,
            page_size: self.page_size,
        })
    }
}
