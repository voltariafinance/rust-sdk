pub use crate::prelude::*;

/// Query parameters for investor_list_clients
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvestorListClientsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// Query string for filtering. Format: "field:operator:value;...". Supported fields: id, name, correlation_id, company_number, status. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
}

impl InvestorListClientsQueryRequest {
    pub fn builder() -> InvestorListClientsQueryRequestBuilder {
        <InvestorListClientsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvestorListClientsQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    order_by: Option<String>,
    q: Option<String>,
}

impl InvestorListClientsQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`InvestorListClientsQueryRequest`].
    pub fn build(self) -> Result<InvestorListClientsQueryRequest, BuildError> {
        Ok(InvestorListClientsQueryRequest {
            page: self.page,
            page_size: self.page_size,
            order_by: self.order_by,
            q: self.q,
        })
    }
}
