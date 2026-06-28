pub use crate::prelude::*;

/// Query parameters for list_repayment_logs
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListRepaymentLogsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// Query string for filtering. Format: "field:operator:value;...". Supported fields: id, client_id, loan_id, installment_id, created_at, client.name, client.correlation_id. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
}

impl ListRepaymentLogsQueryRequest {
    pub fn builder() -> ListRepaymentLogsQueryRequestBuilder {
        <ListRepaymentLogsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListRepaymentLogsQueryRequestBuilder {
    client_id: Option<String>,
    loan_id: Option<String>,
    installment_id: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
    order_by: Option<String>,
    q: Option<String>,
}

impl ListRepaymentLogsQueryRequestBuilder {
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn loan_id(mut self, value: impl Into<String>) -> Self {
        self.loan_id = Some(value.into());
        self
    }

    pub fn installment_id(mut self, value: impl Into<String>) -> Self {
        self.installment_id = Some(value.into());
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

    pub fn order_by(mut self, value: impl Into<String>) -> Self {
        self.order_by = Some(value.into());
        self
    }

    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.q = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListRepaymentLogsQueryRequest`].
    pub fn build(self) -> Result<ListRepaymentLogsQueryRequest, BuildError> {
        Ok(ListRepaymentLogsQueryRequest {
            client_id: self.client_id,
            loan_id: self.loan_id,
            installment_id: self.installment_id,
            page: self.page,
            page_size: self.page_size,
            order_by: self.order_by,
            q: self.q,
        })
    }
}
