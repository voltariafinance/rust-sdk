pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DrawdownResponse {
    /// The unique identifier of the drawdown.
    #[serde(default)]
    pub id: String,
    /// The creation timestamp of the drawdown.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The last update timestamp of the drawdown.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The currency of the drawdown.
    pub currency: CurrencyEnum,
    /// The amount of the drawdown.
    #[serde(default)]
    pub amount: String,
    /// The status of the drawdown.
    pub status: DrawdownStatusEnum,
    /// The date of the drawdown.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub drawdown_date: DateTime<FixedOffset>,
}

impl DrawdownResponse {
    pub fn builder() -> DrawdownResponseBuilder {
        <DrawdownResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DrawdownResponseBuilder {
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    currency: Option<CurrencyEnum>,
    amount: Option<String>,
    status: Option<DrawdownStatusEnum>,
    drawdown_date: Option<DateTime<FixedOffset>>,
}

impl DrawdownResponseBuilder {
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

    pub fn currency(mut self, value: CurrencyEnum) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn status(mut self, value: DrawdownStatusEnum) -> Self {
        self.status = Some(value);
        self
    }

    pub fn drawdown_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.drawdown_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DrawdownResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DrawdownResponseBuilder::id)
    /// - [`created_at`](DrawdownResponseBuilder::created_at)
    /// - [`updated_at`](DrawdownResponseBuilder::updated_at)
    /// - [`currency`](DrawdownResponseBuilder::currency)
    /// - [`amount`](DrawdownResponseBuilder::amount)
    /// - [`status`](DrawdownResponseBuilder::status)
    /// - [`drawdown_date`](DrawdownResponseBuilder::drawdown_date)
    pub fn build(self) -> Result<DrawdownResponse, BuildError> {
        Ok(DrawdownResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            drawdown_date: self
                .drawdown_date
                .ok_or_else(|| BuildError::missing_field("drawdown_date"))?,
        })
    }
}
