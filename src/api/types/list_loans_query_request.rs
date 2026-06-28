pub use crate::prelude::*;

/// Query parameters for list_loans
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListLoansQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// Query string for filtering. Format: "field:operator:value;...". Supported fields: id, client_id, status, client.name, correlation_id. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
}

impl ListLoansQueryRequest {
    pub fn builder() -> ListLoansQueryRequestBuilder {
        <ListLoansQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListLoansQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    client_id: Option<String>,
    order_by: Option<String>,
    q: Option<String>,
}

impl ListLoansQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`ListLoansQueryRequest`].
    pub fn build(self) -> Result<ListLoansQueryRequest, BuildError> {
        Ok(ListLoansQueryRequest {
            page: self.page,
            page_size: self.page_size,
            client_id: self.client_id,
            order_by: self.order_by,
            q: self.q,
        })
    }
}
