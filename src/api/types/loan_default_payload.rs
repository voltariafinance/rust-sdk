pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoanDefaultPayload {
    /// Date the loan is marked as defaulted.
    #[serde(default)]
    pub default_date: NaiveDate,
    /// Amount recovered when the defaulted loan is sold.
    pub sold_amount: LoanDefaultPayloadSoldAmount,
}

impl LoanDefaultPayload {
    pub fn builder() -> LoanDefaultPayloadBuilder {
        <LoanDefaultPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LoanDefaultPayloadBuilder {
    default_date: Option<NaiveDate>,
    sold_amount: Option<LoanDefaultPayloadSoldAmount>,
}

impl LoanDefaultPayloadBuilder {
    pub fn default_date(mut self, value: NaiveDate) -> Self {
        self.default_date = Some(value);
        self
    }

    pub fn sold_amount(mut self, value: LoanDefaultPayloadSoldAmount) -> Self {
        self.sold_amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LoanDefaultPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`default_date`](LoanDefaultPayloadBuilder::default_date)
    /// - [`sold_amount`](LoanDefaultPayloadBuilder::sold_amount)
    pub fn build(self) -> Result<LoanDefaultPayload, BuildError> {
        Ok(LoanDefaultPayload {
            default_date: self
                .default_date
                .ok_or_else(|| BuildError::missing_field("default_date"))?,
            sold_amount: self
                .sold_amount
                .ok_or_else(|| BuildError::missing_field("sold_amount"))?,
        })
    }
}
