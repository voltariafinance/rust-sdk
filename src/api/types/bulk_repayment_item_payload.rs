pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BulkRepaymentItemPayload {
    /// The amount of payment made for installment
    pub amount: BulkRepaymentItemPayloadAmount,
    /// Please provide the repayment_date if it differs from the time you log this repayment. Otherwise, it will be automatically set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repayment_date: Option<DateTime<FixedOffset>>,
    /// Additional metadata related to the repayment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,
    /// Indicates if this repayment is for early settlement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_early_settlement: Option<bool>,
    /// ID of the installment to make payment for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installment_id: Option<String>,
    /// ID of the associated loan (alternative to installment_id)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_id: Option<String>,
    /// Correlation ID of associated loan (alternative identifier)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
}

impl BulkRepaymentItemPayload {
    pub fn builder() -> BulkRepaymentItemPayloadBuilder {
        <BulkRepaymentItemPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkRepaymentItemPayloadBuilder {
    amount: Option<BulkRepaymentItemPayloadAmount>,
    repayment_date: Option<DateTime<FixedOffset>>,
    data: Option<HashMap<String, serde_json::Value>>,
    is_early_settlement: Option<bool>,
    installment_id: Option<String>,
    loan_id: Option<String>,
    correlation_id: Option<String>,
}

impl BulkRepaymentItemPayloadBuilder {
    pub fn amount(mut self, value: BulkRepaymentItemPayloadAmount) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn repayment_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.repayment_date = Some(value);
        self
    }

    pub fn data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn is_early_settlement(mut self, value: bool) -> Self {
        self.is_early_settlement = Some(value);
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

    pub fn correlation_id(mut self, value: impl Into<String>) -> Self {
        self.correlation_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkRepaymentItemPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](BulkRepaymentItemPayloadBuilder::amount)
    pub fn build(self) -> Result<BulkRepaymentItemPayload, BuildError> {
        Ok(BulkRepaymentItemPayload {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            repayment_date: self.repayment_date,
            data: self.data,
            is_early_settlement: self.is_early_settlement,
            installment_id: self.installment_id,
            loan_id: self.loan_id,
            correlation_id: self.correlation_id,
        })
    }
}
