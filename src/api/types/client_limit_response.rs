pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ClientLimitResponse {
    pub currency: CurrencyEnum,
    #[serde(default)]
    pub max_maturity_days: i64,
    #[serde(default)]
    pub limit: String,
    #[serde(default)]
    pub rate: String,
    #[serde(default)]
    pub outstanding: String,
    #[serde(default)]
    pub available: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl ClientLimitResponse {
    pub fn builder() -> ClientLimitResponseBuilder {
        <ClientLimitResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientLimitResponseBuilder {
    currency: Option<CurrencyEnum>,
    max_maturity_days: Option<i64>,
    limit: Option<String>,
    rate: Option<String>,
    outstanding: Option<String>,
    available: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ClientLimitResponseBuilder {
    pub fn currency(mut self, value: CurrencyEnum) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn max_maturity_days(mut self, value: i64) -> Self {
        self.max_maturity_days = Some(value);
        self
    }

    pub fn limit(mut self, value: impl Into<String>) -> Self {
        self.limit = Some(value.into());
        self
    }

    pub fn rate(mut self, value: impl Into<String>) -> Self {
        self.rate = Some(value.into());
        self
    }

    pub fn outstanding(mut self, value: impl Into<String>) -> Self {
        self.outstanding = Some(value.into());
        self
    }

    pub fn available(mut self, value: impl Into<String>) -> Self {
        self.available = Some(value.into());
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

    /// Consumes the builder and constructs a [`ClientLimitResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](ClientLimitResponseBuilder::currency)
    /// - [`max_maturity_days`](ClientLimitResponseBuilder::max_maturity_days)
    /// - [`limit`](ClientLimitResponseBuilder::limit)
    /// - [`rate`](ClientLimitResponseBuilder::rate)
    /// - [`outstanding`](ClientLimitResponseBuilder::outstanding)
    /// - [`available`](ClientLimitResponseBuilder::available)
    /// - [`created_at`](ClientLimitResponseBuilder::created_at)
    /// - [`updated_at`](ClientLimitResponseBuilder::updated_at)
    pub fn build(self) -> Result<ClientLimitResponse, BuildError> {
        Ok(ClientLimitResponse {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            max_maturity_days: self
                .max_maturity_days
                .ok_or_else(|| BuildError::missing_field("max_maturity_days"))?,
            limit: self
                .limit
                .ok_or_else(|| BuildError::missing_field("limit"))?,
            rate: self.rate.ok_or_else(|| BuildError::missing_field("rate"))?,
            outstanding: self
                .outstanding
                .ok_or_else(|| BuildError::missing_field("outstanding"))?,
            available: self
                .available
                .ok_or_else(|| BuildError::missing_field("available"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
