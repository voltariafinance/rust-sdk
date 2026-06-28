pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WaiverResponse {
    /// The ID of the waiver
    #[serde(default)]
    pub id: String,
    /// The ID of the client associated with the waiver
    #[serde(default)]
    pub client_id: String,
    /// The ID of the loan associated with the waiver
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_id: Option<String>,
    /// The ID of the limit request associated with the waiver
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_request_id: Option<String>,
    /// The status of the waiver. One of: pending, active, paid, rejected
    pub status: WaiverStatusEnum,
    /// The date of the waiver
    #[serde(default)]
    pub waiver_date: NaiveDate,
    /// The currency of the waiver amount
    pub currency: CurrencyEnum,
    /// The waiver amount
    #[serde(default)]
    pub amount: String,
    /// Additional notes about the waiver
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// The timestamp when the waiver was created
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl WaiverResponse {
    pub fn builder() -> WaiverResponseBuilder {
        <WaiverResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WaiverResponseBuilder {
    id: Option<String>,
    client_id: Option<String>,
    loan_id: Option<String>,
    limit_request_id: Option<String>,
    status: Option<WaiverStatusEnum>,
    waiver_date: Option<NaiveDate>,
    currency: Option<CurrencyEnum>,
    amount: Option<String>,
    notes: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl WaiverResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn limit_request_id(mut self, value: impl Into<String>) -> Self {
        self.limit_request_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: WaiverStatusEnum) -> Self {
        self.status = Some(value);
        self
    }

    pub fn waiver_date(mut self, value: NaiveDate) -> Self {
        self.waiver_date = Some(value);
        self
    }

    pub fn currency(mut self, value: CurrencyEnum) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WaiverResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WaiverResponseBuilder::id)
    /// - [`client_id`](WaiverResponseBuilder::client_id)
    /// - [`status`](WaiverResponseBuilder::status)
    /// - [`waiver_date`](WaiverResponseBuilder::waiver_date)
    /// - [`currency`](WaiverResponseBuilder::currency)
    /// - [`amount`](WaiverResponseBuilder::amount)
    /// - [`created_at`](WaiverResponseBuilder::created_at)
    pub fn build(self) -> Result<WaiverResponse, BuildError> {
        Ok(WaiverResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            loan_id: self.loan_id,
            limit_request_id: self.limit_request_id,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            waiver_date: self
                .waiver_date
                .ok_or_else(|| BuildError::missing_field("waiver_date"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
