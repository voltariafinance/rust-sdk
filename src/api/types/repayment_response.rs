pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RepaymentResponse {
    /// The ID of the repayment log
    #[serde(default)]
    pub id: String,
    /// The creation date of the repayment log
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The ID of the client who made the payment
    #[serde(default)]
    pub client_id: String,
    /// The ID of the loan for which the payment was made
    #[serde(default)]
    pub loan_id: String,
    /// The ID of the installment for which the payment was made
    #[serde(default)]
    pub installment_id: String,
    /// The amount of payment made for installment
    #[serde(default)]
    pub amount: String,
    /// The date of the payment made for installment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repayment_date: Option<DateTime<FixedOffset>>,
    /// The type of repayment sent in data field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Additional metadata related to the repayment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,
    /// Indicates if this repayment is for early settlement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_early_settlement: Option<bool>,
    /// Principal portion of the repayment, when the partner provides a breakdown of the payment components
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_amount: Option<String>,
    /// Interest portion of the repayment, when the partner provides a breakdown of the payment components
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_amount: Option<String>,
    /// Late fee portion of the repayment, when the partner provides a breakdown of the payment components
    #[serde(skip_serializing_if = "Option::is_none")]
    pub late_fee_amount: Option<String>,
}

impl RepaymentResponse {
    pub fn builder() -> RepaymentResponseBuilder {
        <RepaymentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RepaymentResponseBuilder {
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    client_id: Option<String>,
    loan_id: Option<String>,
    installment_id: Option<String>,
    amount: Option<String>,
    repayment_date: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
    data: Option<HashMap<String, serde_json::Value>>,
    is_early_settlement: Option<bool>,
    principal_amount: Option<String>,
    interest_amount: Option<String>,
    late_fee_amount: Option<String>,
}

impl RepaymentResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
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

    pub fn installment_id(mut self, value: impl Into<String>) -> Self {
        self.installment_id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn repayment_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.repayment_date = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
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

    pub fn principal_amount(mut self, value: impl Into<String>) -> Self {
        self.principal_amount = Some(value.into());
        self
    }

    pub fn interest_amount(mut self, value: impl Into<String>) -> Self {
        self.interest_amount = Some(value.into());
        self
    }

    pub fn late_fee_amount(mut self, value: impl Into<String>) -> Self {
        self.late_fee_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RepaymentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RepaymentResponseBuilder::id)
    /// - [`created_at`](RepaymentResponseBuilder::created_at)
    /// - [`client_id`](RepaymentResponseBuilder::client_id)
    /// - [`loan_id`](RepaymentResponseBuilder::loan_id)
    /// - [`installment_id`](RepaymentResponseBuilder::installment_id)
    /// - [`amount`](RepaymentResponseBuilder::amount)
    pub fn build(self) -> Result<RepaymentResponse, BuildError> {
        Ok(RepaymentResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            loan_id: self
                .loan_id
                .ok_or_else(|| BuildError::missing_field("loan_id"))?,
            installment_id: self
                .installment_id
                .ok_or_else(|| BuildError::missing_field("installment_id"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            repayment_date: self.repayment_date,
            r#type: self.r#type,
            data: self.data,
            is_early_settlement: self.is_early_settlement,
            principal_amount: self.principal_amount,
            interest_amount: self.interest_amount,
            late_fee_amount: self.late_fee_amount,
        })
    }
}
