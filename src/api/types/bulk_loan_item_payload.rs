pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BulkLoanItemPayload {
    /// The ID of the client for this loan
    #[serde(default)]
    pub client_id: String,
    /// The currency of the loan, must be one of the supported currencies: eur, gbp, usd, czk, pln, isk
    pub currency: CurrencyEnum,
    /// The amount of the loan
    pub amount: BulkLoanItemPayloadAmount,
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

impl BulkLoanItemPayload {
    pub fn builder() -> BulkLoanItemPayloadBuilder {
        <BulkLoanItemPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkLoanItemPayloadBuilder {
    client_id: Option<String>,
    currency: Option<CurrencyEnum>,
    amount: Option<BulkLoanItemPayloadAmount>,
    correlation_id: Option<String>,
    loan_date: Option<NaiveDate>,
    installments: Option<Vec<LoanInstallmentCreatePayload>>,
    data: Option<HashMap<String, serde_json::Value>>,
}

impl BulkLoanItemPayloadBuilder {
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn currency(mut self, value: CurrencyEnum) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn amount(mut self, value: BulkLoanItemPayloadAmount) -> Self {
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

    /// Consumes the builder and constructs a [`BulkLoanItemPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_id`](BulkLoanItemPayloadBuilder::client_id)
    /// - [`currency`](BulkLoanItemPayloadBuilder::currency)
    /// - [`amount`](BulkLoanItemPayloadBuilder::amount)
    /// - [`installments`](BulkLoanItemPayloadBuilder::installments)
    pub fn build(self) -> Result<BulkLoanItemPayload, BuildError> {
        Ok(BulkLoanItemPayload {
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
