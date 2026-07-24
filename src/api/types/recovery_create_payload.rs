pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecoveryCreatePayload {
    /// The ID of the loan this recovery is associated with.
    #[serde(default)]
    pub loan_id: String,
    /// The amount recovered (must be > 0).
    pub amount: RecoveryCreatePayloadAmount,
    /// The currency of the recovered amount, must be one of the supported currencies: eur, gbp, usd, czk, pln, isk
    pub currency: CurrencyEnum,
    /// The date the recovery was made.
    #[serde(default)]
    pub recovery_date: NaiveDate,
    /// Optional notes about the recovery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl RecoveryCreatePayload {
    pub fn builder() -> RecoveryCreatePayloadBuilder {
        <RecoveryCreatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecoveryCreatePayloadBuilder {
    loan_id: Option<String>,
    amount: Option<RecoveryCreatePayloadAmount>,
    currency: Option<CurrencyEnum>,
    recovery_date: Option<NaiveDate>,
    notes: Option<String>,
}

impl RecoveryCreatePayloadBuilder {
    pub fn loan_id(mut self, value: impl Into<String>) -> Self {
        self.loan_id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: RecoveryCreatePayloadAmount) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn currency(mut self, value: CurrencyEnum) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn recovery_date(mut self, value: NaiveDate) -> Self {
        self.recovery_date = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RecoveryCreatePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`loan_id`](RecoveryCreatePayloadBuilder::loan_id)
    /// - [`amount`](RecoveryCreatePayloadBuilder::amount)
    /// - [`currency`](RecoveryCreatePayloadBuilder::currency)
    /// - [`recovery_date`](RecoveryCreatePayloadBuilder::recovery_date)
    pub fn build(self) -> Result<RecoveryCreatePayload, BuildError> {
        Ok(RecoveryCreatePayload {
            loan_id: self
                .loan_id
                .ok_or_else(|| BuildError::missing_field("loan_id"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            recovery_date: self
                .recovery_date
                .ok_or_else(|| BuildError::missing_field("recovery_date"))?,
            notes: self.notes,
        })
    }
}
