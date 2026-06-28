pub use crate::prelude::*;

/// Query parameters for list_drawdown_checklists
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDrawdownChecklistsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// Query string for filtering. Format: "field:operator:value;...". Supported fields: . Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
}

impl ListDrawdownChecklistsQueryRequest {
    pub fn builder() -> ListDrawdownChecklistsQueryRequestBuilder {
        <ListDrawdownChecklistsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDrawdownChecklistsQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    order_by: Option<String>,
    q: Option<String>,
}

impl ListDrawdownChecklistsQueryRequestBuilder {
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

    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.q = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListDrawdownChecklistsQueryRequest`].
    pub fn build(self) -> Result<ListDrawdownChecklistsQueryRequest, BuildError> {
        Ok(ListDrawdownChecklistsQueryRequest {
            page: self.page,
            page_size: self.page_size,
            order_by: self.order_by,
            q: self.q,
        })
    }
}
