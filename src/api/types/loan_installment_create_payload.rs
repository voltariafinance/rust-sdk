pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoanInstallmentCreatePayload {
    /// The expected repayment date for this installment
    #[serde(default)]
    pub expected_repayment_at: NaiveDate,
    /// The amount due for this installment
    pub amount: LoanInstallmentCreatePayloadAmount,
    /// The principal part of the installment amount. Only when creating a loan with repayment_method custom, together with interest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<LoanInstallmentCreatePayloadPrincipal>,
    /// The interest part of the installment amount. Only when creating a loan with repayment_method custom, together with principal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest: Option<LoanInstallmentCreatePayloadInterest>,
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
    principal: Option<LoanInstallmentCreatePayloadPrincipal>,
    interest: Option<LoanInstallmentCreatePayloadInterest>,
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

    pub fn principal(mut self, value: LoanInstallmentCreatePayloadPrincipal) -> Self {
        self.principal = Some(value);
        self
    }

    pub fn interest(mut self, value: LoanInstallmentCreatePayloadInterest) -> Self {
        self.interest = Some(value);
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
            principal: self.principal,
            interest: self.interest,
        })
    }
}
