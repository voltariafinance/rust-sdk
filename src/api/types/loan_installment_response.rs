pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LoanInstallmentResponse {
    /// The ID of the installment
    #[serde(default)]
    pub id: String,
    /// The amount of the installment
    #[serde(default)]
    pub amount: String,
    /// The installment number in sequence
    #[serde(default)]
    pub installment_number: i64,
    /// The expected repayment date of the installment
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expected_repayment_at: DateTime<FixedOffset>,
    /// The status of the installment
    pub status: LoanStatusEnum,
    /// The creation date of the installment
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The last update date of the installment
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl LoanInstallmentResponse {
    pub fn builder() -> LoanInstallmentResponseBuilder {
        <LoanInstallmentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LoanInstallmentResponseBuilder {
    id: Option<String>,
    amount: Option<String>,
    installment_number: Option<i64>,
    expected_repayment_at: Option<DateTime<FixedOffset>>,
    status: Option<LoanStatusEnum>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl LoanInstallmentResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn installment_number(mut self, value: i64) -> Self {
        self.installment_number = Some(value);
        self
    }

    pub fn expected_repayment_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expected_repayment_at = Some(value);
        self
    }

    pub fn status(mut self, value: LoanStatusEnum) -> Self {
        self.status = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LoanInstallmentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LoanInstallmentResponseBuilder::id)
    /// - [`amount`](LoanInstallmentResponseBuilder::amount)
    /// - [`installment_number`](LoanInstallmentResponseBuilder::installment_number)
    /// - [`expected_repayment_at`](LoanInstallmentResponseBuilder::expected_repayment_at)
    /// - [`status`](LoanInstallmentResponseBuilder::status)
    /// - [`created_at`](LoanInstallmentResponseBuilder::created_at)
    /// - [`updated_at`](LoanInstallmentResponseBuilder::updated_at)
    pub fn build(self) -> Result<LoanInstallmentResponse, BuildError> {
        Ok(LoanInstallmentResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            installment_number: self
                .installment_number
                .ok_or_else(|| BuildError::missing_field("installment_number"))?,
            expected_repayment_at: self
                .expected_repayment_at
                .ok_or_else(|| BuildError::missing_field("expected_repayment_at"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
