pub use crate::prelude::*;

/// Query parameters for list_documents
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDocumentsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waterfall_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// Query string for filtering. Format: "field:operator:value;...". Supported fields: id, client_id, loan_id, installment_id, waterfall_id, category, file_name, document_date, folder_path. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
}

impl ListDocumentsQueryRequest {
    pub fn builder() -> ListDocumentsQueryRequestBuilder {
        <ListDocumentsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDocumentsQueryRequestBuilder {
    client_id: Option<String>,
    loan_id: Option<String>,
    installment_id: Option<String>,
    waterfall_id: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
    order_by: Option<String>,
    q: Option<String>,
}

impl ListDocumentsQueryRequestBuilder {
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

    pub fn waterfall_id(mut self, value: impl Into<String>) -> Self {
        self.waterfall_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`ListDocumentsQueryRequest`].
    pub fn build(self) -> Result<ListDocumentsQueryRequest, BuildError> {
        Ok(ListDocumentsQueryRequest {
            client_id: self.client_id,
            loan_id: self.loan_id,
            installment_id: self.installment_id,
            waterfall_id: self.waterfall_id,
            page: self.page,
            page_size: self.page_size,
            order_by: self.order_by,
            q: self.q,
        })
    }
}
