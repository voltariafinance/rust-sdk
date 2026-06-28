pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentPromiseResponse {
    /// The ID of the payment promise
    #[serde(default)]
    pub id: String,
    /// When the promise was created
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// When the promise was last updated
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The ID of the installment this promise relates to
    #[serde(default)]
    pub installment_id: String,
    /// The ID of the partner the installment belongs to
    #[serde(default)]
    pub partner_id: String,
    /// The ID of the client the installment belongs to
    #[serde(default)]
    pub client_id: String,
    /// The ID of the loan the installment belongs to
    #[serde(default)]
    pub loan_id: String,
    /// The amount the client has promised to pay
    #[serde(default)]
    pub amount: String,
    /// The date the client has committed to pay by
    #[serde(default)]
    pub promised_date: NaiveDate,
    /// The status of the promise
    pub status: PaymentPromiseStatusEnum,
    /// Optional notes captured during the collections interaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PaymentPromiseResponse {
    pub fn builder() -> PaymentPromiseResponseBuilder {
        <PaymentPromiseResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentPromiseResponseBuilder {
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    installment_id: Option<String>,
    partner_id: Option<String>,
    client_id: Option<String>,
    loan_id: Option<String>,
    amount: Option<String>,
    promised_date: Option<NaiveDate>,
    status: Option<PaymentPromiseStatusEnum>,
    notes: Option<String>,
}

impl PaymentPromiseResponseBuilder {
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

    pub fn installment_id(mut self, value: impl Into<String>) -> Self {
        self.installment_id = Some(value.into());
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

    pub fn promised_date(mut self, value: NaiveDate) -> Self {
        self.promised_date = Some(value);
        self
    }

    pub fn status(mut self, value: PaymentPromiseStatusEnum) -> Self {
        self.status = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentPromiseResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PaymentPromiseResponseBuilder::id)
    /// - [`created_at`](PaymentPromiseResponseBuilder::created_at)
    /// - [`updated_at`](PaymentPromiseResponseBuilder::updated_at)
    /// - [`installment_id`](PaymentPromiseResponseBuilder::installment_id)
    /// - [`partner_id`](PaymentPromiseResponseBuilder::partner_id)
    /// - [`client_id`](PaymentPromiseResponseBuilder::client_id)
    /// - [`loan_id`](PaymentPromiseResponseBuilder::loan_id)
    /// - [`amount`](PaymentPromiseResponseBuilder::amount)
    /// - [`promised_date`](PaymentPromiseResponseBuilder::promised_date)
    /// - [`status`](PaymentPromiseResponseBuilder::status)
    pub fn build(self) -> Result<PaymentPromiseResponse, BuildError> {
        Ok(PaymentPromiseResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            installment_id: self
                .installment_id
                .ok_or_else(|| BuildError::missing_field("installment_id"))?,
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
            promised_date: self
                .promised_date
                .ok_or_else(|| BuildError::missing_field("promised_date"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            notes: self.notes,
        })
    }
}
