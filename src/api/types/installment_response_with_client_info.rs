pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InstallmentResponseWithClientInfo {
    /// The ID of the installment
    #[serde(default)]
    pub id: String,
    /// The creation date of the installment
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The last update date of the installment
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The partner ID
    #[serde(default)]
    pub partner_id: String,
    /// The client ID
    #[serde(default)]
    pub client_id: String,
    /// The currency of the installment
    pub currency: CurrencyEnum,
    /// The status of the installment (possible values: active, overdue, default, sold, restructured, repaid, pending)
    pub status: InstallmentStatusEnum,
    /// The associated loan ID
    #[serde(default)]
    pub loan_id: String,
    /// The total amount of the installment
    #[serde(default)]
    pub amount: String,
    /// The expected repayment date
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expected_repayment_at: DateTime<FixedOffset>,
    /// The installment number in sequence
    #[serde(default)]
    pub installment_number: i64,
    /// The total number of installments
    #[serde(default)]
    pub installments: i64,
    /// The principal amount of the installment
    #[serde(default)]
    pub principal: String,
    /// The interest amount of the installment
    #[serde(default)]
    pub interest: String,
    /// The amount repaid so far for this installment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repayment_amount: Option<String>,
    /// The actual repayment date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repayment_at: Option<DateTime<FixedOffset>>,
    /// The client details associated with the installment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<ClientBaseInfo>,
}

impl InstallmentResponseWithClientInfo {
    pub fn builder() -> InstallmentResponseWithClientInfoBuilder {
        <InstallmentResponseWithClientInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InstallmentResponseWithClientInfoBuilder {
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    partner_id: Option<String>,
    client_id: Option<String>,
    currency: Option<CurrencyEnum>,
    status: Option<InstallmentStatusEnum>,
    loan_id: Option<String>,
    amount: Option<String>,
    expected_repayment_at: Option<DateTime<FixedOffset>>,
    installment_number: Option<i64>,
    installments: Option<i64>,
    principal: Option<String>,
    interest: Option<String>,
    repayment_amount: Option<String>,
    repayment_at: Option<DateTime<FixedOffset>>,
    client: Option<ClientBaseInfo>,
}

impl InstallmentResponseWithClientInfoBuilder {
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

    pub fn status(mut self, value: InstallmentStatusEnum) -> Self {
        self.status = Some(value);
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

    pub fn expected_repayment_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expected_repayment_at = Some(value);
        self
    }

    pub fn installment_number(mut self, value: i64) -> Self {
        self.installment_number = Some(value);
        self
    }

    pub fn installments(mut self, value: i64) -> Self {
        self.installments = Some(value);
        self
    }

    pub fn principal(mut self, value: impl Into<String>) -> Self {
        self.principal = Some(value.into());
        self
    }

    pub fn interest(mut self, value: impl Into<String>) -> Self {
        self.interest = Some(value.into());
        self
    }

    pub fn repayment_amount(mut self, value: impl Into<String>) -> Self {
        self.repayment_amount = Some(value.into());
        self
    }

    pub fn repayment_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.repayment_at = Some(value);
        self
    }

    pub fn client(mut self, value: ClientBaseInfo) -> Self {
        self.client = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InstallmentResponseWithClientInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InstallmentResponseWithClientInfoBuilder::id)
    /// - [`created_at`](InstallmentResponseWithClientInfoBuilder::created_at)
    /// - [`updated_at`](InstallmentResponseWithClientInfoBuilder::updated_at)
    /// - [`partner_id`](InstallmentResponseWithClientInfoBuilder::partner_id)
    /// - [`client_id`](InstallmentResponseWithClientInfoBuilder::client_id)
    /// - [`currency`](InstallmentResponseWithClientInfoBuilder::currency)
    /// - [`status`](InstallmentResponseWithClientInfoBuilder::status)
    /// - [`loan_id`](InstallmentResponseWithClientInfoBuilder::loan_id)
    /// - [`amount`](InstallmentResponseWithClientInfoBuilder::amount)
    /// - [`expected_repayment_at`](InstallmentResponseWithClientInfoBuilder::expected_repayment_at)
    /// - [`installment_number`](InstallmentResponseWithClientInfoBuilder::installment_number)
    /// - [`installments`](InstallmentResponseWithClientInfoBuilder::installments)
    /// - [`principal`](InstallmentResponseWithClientInfoBuilder::principal)
    /// - [`interest`](InstallmentResponseWithClientInfoBuilder::interest)
    pub fn build(self) -> Result<InstallmentResponseWithClientInfo, BuildError> {
        Ok(InstallmentResponseWithClientInfo {
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
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            loan_id: self
                .loan_id
                .ok_or_else(|| BuildError::missing_field("loan_id"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            expected_repayment_at: self
                .expected_repayment_at
                .ok_or_else(|| BuildError::missing_field("expected_repayment_at"))?,
            installment_number: self
                .installment_number
                .ok_or_else(|| BuildError::missing_field("installment_number"))?,
            installments: self
                .installments
                .ok_or_else(|| BuildError::missing_field("installments"))?,
            principal: self
                .principal
                .ok_or_else(|| BuildError::missing_field("principal"))?,
            interest: self
                .interest
                .ok_or_else(|| BuildError::missing_field("interest"))?,
            repayment_amount: self.repayment_amount,
            repayment_at: self.repayment_at,
            client: self.client,
        })
    }
}
