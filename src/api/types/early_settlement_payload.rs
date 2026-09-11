pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EarlySettlementPayload {
    /// Date the loan would be settled. Must be today or later. Defaults to today when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_date: Option<NaiveDate>,
}

impl EarlySettlementPayload {
    pub fn builder() -> EarlySettlementPayloadBuilder {
        <EarlySettlementPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EarlySettlementPayloadBuilder {
    settlement_date: Option<NaiveDate>,
}

impl EarlySettlementPayloadBuilder {
    pub fn settlement_date(mut self, value: NaiveDate) -> Self {
        self.settlement_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EarlySettlementPayload`].
    pub fn build(self) -> Result<EarlySettlementPayload, BuildError> {
        Ok(EarlySettlementPayload {
            settlement_date: self.settlement_date,
        })
    }
}
