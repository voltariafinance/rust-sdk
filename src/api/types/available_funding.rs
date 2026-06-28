pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AvailableFunding {
    /// Currency of the limit
    pub currency: CurrencyEnum,
    #[serde(default)]
    pub limit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub later: Option<String>,
    #[serde(default)]
    pub max_maturity_days: i64,
    #[serde(default)]
    pub rate: String,
    #[serde(default)]
    pub outstanding: String,
    #[serde(default)]
    pub available: String,
}

impl AvailableFunding {
    pub fn builder() -> AvailableFundingBuilder {
        <AvailableFundingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AvailableFundingBuilder {
    currency: Option<CurrencyEnum>,
    limit: Option<String>,
    later: Option<String>,
    max_maturity_days: Option<i64>,
    rate: Option<String>,
    outstanding: Option<String>,
    available: Option<String>,
}

impl AvailableFundingBuilder {
    pub fn currency(mut self, value: CurrencyEnum) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn limit(mut self, value: impl Into<String>) -> Self {
        self.limit = Some(value.into());
        self
    }

    pub fn later(mut self, value: impl Into<String>) -> Self {
        self.later = Some(value.into());
        self
    }

    pub fn max_maturity_days(mut self, value: i64) -> Self {
        self.max_maturity_days = Some(value);
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

    /// Consumes the builder and constructs a [`AvailableFunding`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](AvailableFundingBuilder::currency)
    /// - [`limit`](AvailableFundingBuilder::limit)
    /// - [`max_maturity_days`](AvailableFundingBuilder::max_maturity_days)
    /// - [`rate`](AvailableFundingBuilder::rate)
    /// - [`outstanding`](AvailableFundingBuilder::outstanding)
    /// - [`available`](AvailableFundingBuilder::available)
    pub fn build(self) -> Result<AvailableFunding, BuildError> {
        Ok(AvailableFunding {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            limit: self
                .limit
                .ok_or_else(|| BuildError::missing_field("limit"))?,
            later: self.later,
            max_maturity_days: self
                .max_maturity_days
                .ok_or_else(|| BuildError::missing_field("max_maturity_days"))?,
            rate: self.rate.ok_or_else(|| BuildError::missing_field("rate"))?,
            outstanding: self
                .outstanding
                .ok_or_else(|| BuildError::missing_field("outstanding"))?,
            available: self
                .available
                .ok_or_else(|| BuildError::missing_field("available"))?,
        })
    }
}
