pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EarlySettlementResponse {
    /// The ID of the loan
    #[serde(default)]
    pub loan_id: String,
    /// The date of early settlement
    #[serde(default)]
    pub settlement_date: NaiveDate,
    /// The settlement amount at early settlement
    #[serde(default)]
    pub settlement_amount: String,
    /// The internal rate of return at early settlement
    #[serde(default)]
    pub settlement_irr: String,
    /// The original internal rate of return before early settlement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_irr: Option<String>,
    /// The minimum fee applicable at early settlement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_fee: Option<String>,
}

impl EarlySettlementResponse {
    pub fn builder() -> EarlySettlementResponseBuilder {
        <EarlySettlementResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EarlySettlementResponseBuilder {
    loan_id: Option<String>,
    settlement_date: Option<NaiveDate>,
    settlement_amount: Option<String>,
    settlement_irr: Option<String>,
    original_irr: Option<String>,
    minimum_fee: Option<String>,
}

impl EarlySettlementResponseBuilder {
    pub fn loan_id(mut self, value: impl Into<String>) -> Self {
        self.loan_id = Some(value.into());
        self
    }

    pub fn settlement_date(mut self, value: NaiveDate) -> Self {
        self.settlement_date = Some(value);
        self
    }

    pub fn settlement_amount(mut self, value: impl Into<String>) -> Self {
        self.settlement_amount = Some(value.into());
        self
    }

    pub fn settlement_irr(mut self, value: impl Into<String>) -> Self {
        self.settlement_irr = Some(value.into());
        self
    }

    pub fn original_irr(mut self, value: impl Into<String>) -> Self {
        self.original_irr = Some(value.into());
        self
    }

    pub fn minimum_fee(mut self, value: impl Into<String>) -> Self {
        self.minimum_fee = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EarlySettlementResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`loan_id`](EarlySettlementResponseBuilder::loan_id)
    /// - [`settlement_date`](EarlySettlementResponseBuilder::settlement_date)
    /// - [`settlement_amount`](EarlySettlementResponseBuilder::settlement_amount)
    /// - [`settlement_irr`](EarlySettlementResponseBuilder::settlement_irr)
    pub fn build(self) -> Result<EarlySettlementResponse, BuildError> {
        Ok(EarlySettlementResponse {
            loan_id: self
                .loan_id
                .ok_or_else(|| BuildError::missing_field("loan_id"))?,
            settlement_date: self
                .settlement_date
                .ok_or_else(|| BuildError::missing_field("settlement_date"))?,
            settlement_amount: self
                .settlement_amount
                .ok_or_else(|| BuildError::missing_field("settlement_amount"))?,
            settlement_irr: self
                .settlement_irr
                .ok_or_else(|| BuildError::missing_field("settlement_irr"))?,
            original_irr: self.original_irr,
            minimum_fee: self.minimum_fee,
        })
    }
}
