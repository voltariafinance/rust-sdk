pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoanInvestorResponse {
    /// The ID of the loan
    #[serde(default)]
    pub id: String,
    /// The creation date of the loan
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The last update date of the loan
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The ID of the partner
    #[serde(default)]
    pub partner_id: String,
    /// The ID of the client
    #[serde(default)]
    pub client_id: String,
    /// The currency of the loan
    pub currency: CurrencyEnum,
    /// The amount of the loan
    #[serde(default)]
    pub amount: String,
    /// The status of the loan
    pub status: LoanStatusEnum,
    /// The internal rate of return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub irr: Option<String>,
    /// The date of the loan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_date: Option<NaiveDate>,
    /// The currency rate conversion to EUR at the time of the loan
    #[serde(default)]
    pub currency_rate: String,
    /// The correlation ID provided at the creation of the loan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    /// The payment status of the loan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_status: Option<LoanPaymentStatusEnum>,
    /// The payment reference for the loan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_reference: Option<String>,
    /// The date of early settlement if the loan was settled early
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_settlement_date: Option<NaiveDate>,
    /// The settlement amount at early settlement if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_settlement_amount: Option<String>,
    /// Additional data related to the loan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,
    /// The client details associated with the loan
    pub client: ClientBaseInfo,
    /// Whether the loan disbursement is paid directly to the client (as opposed to the partner).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_to_the_client: Option<bool>,
}

impl LoanInvestorResponse {
    pub fn builder() -> LoanInvestorResponseBuilder {
        <LoanInvestorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LoanInvestorResponseBuilder {
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    partner_id: Option<String>,
    client_id: Option<String>,
    currency: Option<CurrencyEnum>,
    amount: Option<String>,
    status: Option<LoanStatusEnum>,
    irr: Option<String>,
    loan_date: Option<NaiveDate>,
    currency_rate: Option<String>,
    correlation_id: Option<String>,
    payment_status: Option<LoanPaymentStatusEnum>,
    payment_reference: Option<String>,
    early_settlement_date: Option<NaiveDate>,
    early_settlement_amount: Option<String>,
    data: Option<HashMap<String, serde_json::Value>>,
    client: Option<ClientBaseInfo>,
    payment_to_the_client: Option<bool>,
}

impl LoanInvestorResponseBuilder {
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

    pub fn currency(mut self, value: CurrencyEnum) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn status(mut self, value: LoanStatusEnum) -> Self {
        self.status = Some(value);
        self
    }

    pub fn irr(mut self, value: impl Into<String>) -> Self {
        self.irr = Some(value.into());
        self
    }

    pub fn loan_date(mut self, value: NaiveDate) -> Self {
        self.loan_date = Some(value);
        self
    }

    pub fn currency_rate(mut self, value: impl Into<String>) -> Self {
        self.currency_rate = Some(value.into());
        self
    }

    pub fn correlation_id(mut self, value: impl Into<String>) -> Self {
        self.correlation_id = Some(value.into());
        self
    }

    pub fn payment_status(mut self, value: LoanPaymentStatusEnum) -> Self {
        self.payment_status = Some(value);
        self
    }

    pub fn payment_reference(mut self, value: impl Into<String>) -> Self {
        self.payment_reference = Some(value.into());
        self
    }

    pub fn early_settlement_date(mut self, value: NaiveDate) -> Self {
        self.early_settlement_date = Some(value);
        self
    }

    pub fn early_settlement_amount(mut self, value: impl Into<String>) -> Self {
        self.early_settlement_amount = Some(value.into());
        self
    }

    pub fn data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn client(mut self, value: ClientBaseInfo) -> Self {
        self.client = Some(value);
        self
    }

    pub fn payment_to_the_client(mut self, value: bool) -> Self {
        self.payment_to_the_client = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LoanInvestorResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LoanInvestorResponseBuilder::id)
    /// - [`created_at`](LoanInvestorResponseBuilder::created_at)
    /// - [`updated_at`](LoanInvestorResponseBuilder::updated_at)
    /// - [`partner_id`](LoanInvestorResponseBuilder::partner_id)
    /// - [`client_id`](LoanInvestorResponseBuilder::client_id)
    /// - [`currency`](LoanInvestorResponseBuilder::currency)
    /// - [`amount`](LoanInvestorResponseBuilder::amount)
    /// - [`status`](LoanInvestorResponseBuilder::status)
    /// - [`currency_rate`](LoanInvestorResponseBuilder::currency_rate)
    /// - [`client`](LoanInvestorResponseBuilder::client)
    pub fn build(self) -> Result<LoanInvestorResponse, BuildError> {
        Ok(LoanInvestorResponse {
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
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            irr: self.irr,
            loan_date: self.loan_date,
            currency_rate: self
                .currency_rate
                .ok_or_else(|| BuildError::missing_field("currency_rate"))?,
            correlation_id: self.correlation_id,
            payment_status: self.payment_status,
            payment_reference: self.payment_reference,
            early_settlement_date: self.early_settlement_date,
            early_settlement_amount: self.early_settlement_amount,
            data: self.data,
            client: self
                .client
                .ok_or_else(|| BuildError::missing_field("client"))?,
            payment_to_the_client: self.payment_to_the_client,
        })
    }
}
