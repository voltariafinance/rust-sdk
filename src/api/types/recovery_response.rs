pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RecoveryResponse {
    /// The ID of the recovery.
    #[serde(default)]
    pub id: String,
    /// When the recovery record was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// When the recovery record was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The ID of the partner this recovery belongs to.
    #[serde(default)]
    pub partner_id: String,
    /// The ID of the client this recovery is associated with.
    #[serde(default)]
    pub client_id: String,
    /// The ID of the loan this recovery is associated with.
    #[serde(default)]
    pub loan_id: String,
    /// The amount recovered.
    #[serde(default)]
    pub amount: String,
    /// The currency of the recovered amount.
    pub currency: CurrencyEnum,
    /// The date the recovery was made.
    #[serde(default)]
    pub recovery_date: NaiveDate,
    /// Optional notes about the recovery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl RecoveryResponse {
    pub fn builder() -> RecoveryResponseBuilder {
        <RecoveryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecoveryResponseBuilder {
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    partner_id: Option<String>,
    client_id: Option<String>,
    loan_id: Option<String>,
    amount: Option<String>,
    currency: Option<CurrencyEnum>,
    recovery_date: Option<NaiveDate>,
    notes: Option<String>,
}

impl RecoveryResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn loan_id(mut self, value: impl Into<String>) -> Self {
        self.loan_id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
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

    /// Consumes the builder and constructs a [`RecoveryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RecoveryResponseBuilder::id)
    /// - [`created_at`](RecoveryResponseBuilder::created_at)
    /// - [`updated_at`](RecoveryResponseBuilder::updated_at)
    /// - [`partner_id`](RecoveryResponseBuilder::partner_id)
    /// - [`client_id`](RecoveryResponseBuilder::client_id)
    /// - [`loan_id`](RecoveryResponseBuilder::loan_id)
    /// - [`amount`](RecoveryResponseBuilder::amount)
    /// - [`currency`](RecoveryResponseBuilder::currency)
    /// - [`recovery_date`](RecoveryResponseBuilder::recovery_date)
    pub fn build(self) -> Result<RecoveryResponse, BuildError> {
        Ok(RecoveryResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
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
