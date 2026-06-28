pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkLoanItemResult {
    /// Index of loan in original request
    #[serde(default)]
    pub index: i64,
    /// Whether loan was created successfully
    #[serde(default)]
    pub success: bool,
    /// ID of created loan if successful
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_id: Option<String>,
    /// Error message if creation failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// ID of associated client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Correlation ID if provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    /// Loan amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

impl BulkLoanItemResult {
    pub fn builder() -> BulkLoanItemResultBuilder {
        <BulkLoanItemResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkLoanItemResultBuilder {
    index: Option<i64>,
    success: Option<bool>,
    loan_id: Option<String>,
    error: Option<String>,
    client_id: Option<String>,
    correlation_id: Option<String>,
    amount: Option<String>,
}

impl BulkLoanItemResultBuilder {
    pub fn index(mut self, value: i64) -> Self {
        self.index = Some(value);
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn loan_id(mut self, value: impl Into<String>) -> Self {
        self.loan_id = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn correlation_id(mut self, value: impl Into<String>) -> Self {
        self.correlation_id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkLoanItemResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`index`](BulkLoanItemResultBuilder::index)
    /// - [`success`](BulkLoanItemResultBuilder::success)
    pub fn build(self) -> Result<BulkLoanItemResult, BuildError> {
        Ok(BulkLoanItemResult {
            index: self
                .index
                .ok_or_else(|| BuildError::missing_field("index"))?,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            loan_id: self.loan_id,
            error: self.error,
            client_id: self.client_id,
            correlation_id: self.correlation_id,
            amount: self.amount,
        })
    }
}
