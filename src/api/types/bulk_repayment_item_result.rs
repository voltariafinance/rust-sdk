pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkRepaymentItemResult {
    /// Index of the repayment in the original request
    #[serde(default)]
    pub index: i64,
    /// Whether the repayment was processed successfully
    #[serde(default)]
    pub success: bool,
    /// ID of the created repayment log if successful
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repayment_log_id: Option<String>,
    /// Error message if processing failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// ID of the associated installment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installment_id: Option<String>,
    /// ID of the associated loan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_id: Option<String>,
    /// Amount of the repayment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

impl BulkRepaymentItemResult {
    pub fn builder() -> BulkRepaymentItemResultBuilder {
        <BulkRepaymentItemResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkRepaymentItemResultBuilder {
    index: Option<i64>,
    success: Option<bool>,
    repayment_log_id: Option<String>,
    error: Option<String>,
    installment_id: Option<String>,
    loan_id: Option<String>,
    amount: Option<String>,
}

impl BulkRepaymentItemResultBuilder {
    pub fn index(mut self, value: i64) -> Self {
        self.index = Some(value);
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn repayment_log_id(mut self, value: impl Into<String>) -> Self {
        self.repayment_log_id = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn installment_id(mut self, value: impl Into<String>) -> Self {
        self.installment_id = Some(value.into());
        self
    }

    pub fn loan_id(mut self, value: impl Into<String>) -> Self {
        self.loan_id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkRepaymentItemResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`index`](BulkRepaymentItemResultBuilder::index)
    /// - [`success`](BulkRepaymentItemResultBuilder::success)
    pub fn build(self) -> Result<BulkRepaymentItemResult, BuildError> {
        Ok(BulkRepaymentItemResult {
            index: self
                .index
                .ok_or_else(|| BuildError::missing_field("index"))?,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            repayment_log_id: self.repayment_log_id,
            error: self.error,
            installment_id: self.installment_id,
            loan_id: self.loan_id,
            amount: self.amount,
        })
    }
}
