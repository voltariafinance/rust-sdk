pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoanCreatePayload {
    /// The ID of the client for this loan
    #[serde(default)]
    pub client_id: String,
    /// The currency of the loan, must be one of the supported currencies: eur, gbp, usd, czk, pln, isk
    pub currency: CurrencyEnum,
    /// The amount of the loan
    pub amount: LoanCreatePayloadAmount,
    /// The correlation ID you provided at the creation of the loan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    /// Please provide the loan_date if it differs from the loan creation time (created_at). Otherwise, it will be automatically set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_date: Option<NaiveDate>,
    /// List of installments for the loan, each with a due date and amount
    #[serde(default)]
    pub installments: Vec<LoanInstallmentCreatePayload>,
    /// Additional data related to the loan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,
}

impl LoanCreatePayload {
    pub fn builder() -> LoanCreatePayloadBuilder {
        <LoanCreatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LoanCreatePayloadBuilder {
    client_id: Option<String>,
    currency: Option<CurrencyEnum>,
    amount: Option<LoanCreatePayloadAmount>,
    correlation_id: Option<String>,
    loan_date: Option<NaiveDate>,
    installments: Option<Vec<LoanInstallmentCreatePayload>>,
    data: Option<HashMap<String, serde_json::Value>>,
}

impl LoanCreatePayloadBuilder {
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn currency(mut self, value: CurrencyEnum) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn amount(mut self, value: LoanCreatePayloadAmount) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn correlation_id(mut self, value: impl Into<String>) -> Self {
        self.correlation_id = Some(value.into());
        self
    }

    pub fn loan_date(mut self, value: NaiveDate) -> Self {
        self.loan_date = Some(value);
        self
    }

    pub fn installments(mut self, value: Vec<LoanInstallmentCreatePayload>) -> Self {
        self.installments = Some(value);
        self
    }

    pub fn data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LoanCreatePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_id`](LoanCreatePayloadBuilder::client_id)
    /// - [`currency`](LoanCreatePayloadBuilder::currency)
    /// - [`amount`](LoanCreatePayloadBuilder::amount)
    /// - [`installments`](LoanCreatePayloadBuilder::installments)
    pub fn build(self) -> Result<LoanCreatePayload, BuildError> {
        Ok(LoanCreatePayload {
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            correlation_id: self.correlation_id,
            loan_date: self.loan_date,
            installments: self
                .installments
                .ok_or_else(|| BuildError::missing_field("installments"))?,
            data: self.data,
        })
    }
}
