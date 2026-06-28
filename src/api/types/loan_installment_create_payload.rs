pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoanInstallmentCreatePayload {
    /// The expected repayment date for this installment
    #[serde(default)]
    pub expected_repayment_at: NaiveDate,
    /// The amount due for this installment
    pub amount: LoanInstallmentCreatePayloadAmount,
}

impl LoanInstallmentCreatePayload {
    pub fn builder() -> LoanInstallmentCreatePayloadBuilder {
        <LoanInstallmentCreatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LoanInstallmentCreatePayloadBuilder {
    expected_repayment_at: Option<NaiveDate>,
    amount: Option<LoanInstallmentCreatePayloadAmount>,
}

impl LoanInstallmentCreatePayloadBuilder {
    pub fn expected_repayment_at(mut self, value: NaiveDate) -> Self {
        self.expected_repayment_at = Some(value);
        self
    }

    pub fn amount(mut self, value: LoanInstallmentCreatePayloadAmount) -> Self {
        self.amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LoanInstallmentCreatePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expected_repayment_at`](LoanInstallmentCreatePayloadBuilder::expected_repayment_at)
    /// - [`amount`](LoanInstallmentCreatePayloadBuilder::amount)
    pub fn build(self) -> Result<LoanInstallmentCreatePayload, BuildError> {
        Ok(LoanInstallmentCreatePayload {
            expected_repayment_at: self
                .expected_repayment_at
                .ok_or_else(|| BuildError::missing_field("expected_repayment_at"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
