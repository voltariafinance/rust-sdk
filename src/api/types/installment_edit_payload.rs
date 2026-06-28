pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InstallmentEditPayload {
    /// The new amount for the installment
    pub amount: InstallmentEditPayloadAmount,
    /// The new expected repayment date
    #[serde(default)]
    pub expected_repayment_at: NaiveDate,
}

impl InstallmentEditPayload {
    pub fn builder() -> InstallmentEditPayloadBuilder {
        <InstallmentEditPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InstallmentEditPayloadBuilder {
    amount: Option<InstallmentEditPayloadAmount>,
    expected_repayment_at: Option<NaiveDate>,
}

impl InstallmentEditPayloadBuilder {
    pub fn amount(mut self, value: InstallmentEditPayloadAmount) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn expected_repayment_at(mut self, value: NaiveDate) -> Self {
        self.expected_repayment_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InstallmentEditPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](InstallmentEditPayloadBuilder::amount)
    /// - [`expected_repayment_at`](InstallmentEditPayloadBuilder::expected_repayment_at)
    pub fn build(self) -> Result<InstallmentEditPayload, BuildError> {
        Ok(InstallmentEditPayload {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            expected_repayment_at: self
                .expected_repayment_at
                .ok_or_else(|| BuildError::missing_field("expected_repayment_at"))?,
        })
    }
}
