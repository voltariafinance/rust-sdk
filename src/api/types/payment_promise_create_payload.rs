pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentPromiseCreatePayload {
    /// The ID of the installment this promise relates to
    #[serde(default)]
    pub installment_id: String,
    /// The amount the client has promised to pay (must be > 0)
    pub amount: PaymentPromiseCreatePayloadAmount,
    /// The date the client has committed to pay by (today or future)
    #[serde(default)]
    pub promised_date: NaiveDate,
    /// Optional notes captured during the collections interaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PaymentPromiseCreatePayload {
    pub fn builder() -> PaymentPromiseCreatePayloadBuilder {
        <PaymentPromiseCreatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentPromiseCreatePayloadBuilder {
    installment_id: Option<String>,
    amount: Option<PaymentPromiseCreatePayloadAmount>,
    promised_date: Option<NaiveDate>,
    notes: Option<String>,
}

impl PaymentPromiseCreatePayloadBuilder {
    pub fn installment_id(mut self, value: impl Into<String>) -> Self {
        self.installment_id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: PaymentPromiseCreatePayloadAmount) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn promised_date(mut self, value: NaiveDate) -> Self {
        self.promised_date = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentPromiseCreatePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`installment_id`](PaymentPromiseCreatePayloadBuilder::installment_id)
    /// - [`amount`](PaymentPromiseCreatePayloadBuilder::amount)
    /// - [`promised_date`](PaymentPromiseCreatePayloadBuilder::promised_date)
    pub fn build(self) -> Result<PaymentPromiseCreatePayload, BuildError> {
        Ok(PaymentPromiseCreatePayload {
            installment_id: self
                .installment_id
                .ok_or_else(|| BuildError::missing_field("installment_id"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            promised_date: self
                .promised_date
                .ok_or_else(|| BuildError::missing_field("promised_date"))?,
            notes: self.notes,
        })
    }
}
