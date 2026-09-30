pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ClientLimitHistoryResponse {
    /// The unique identifier of the limit record
    #[serde(default)]
    pub id: String,
    /// When the limit was granted
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The currency the limit is denominated in
    pub currency: CurrencyEnum,
    /// The credit limit granted to the client
    #[serde(default)]
    pub limit: String,
}

impl ClientLimitHistoryResponse {
    pub fn builder() -> ClientLimitHistoryResponseBuilder {
        <ClientLimitHistoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientLimitHistoryResponseBuilder {
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<CurrencyEnum>,
    limit: Option<String>,
}

impl ClientLimitHistoryResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: CurrencyEnum) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn limit(mut self, value: impl Into<String>) -> Self {
        self.limit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ClientLimitHistoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ClientLimitHistoryResponseBuilder::id)
    /// - [`created_at`](ClientLimitHistoryResponseBuilder::created_at)
    /// - [`currency`](ClientLimitHistoryResponseBuilder::currency)
    /// - [`limit`](ClientLimitHistoryResponseBuilder::limit)
    pub fn build(self) -> Result<ClientLimitHistoryResponse, BuildError> {
        Ok(ClientLimitHistoryResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            limit: self
                .limit
                .ok_or_else(|| BuildError::missing_field("limit"))?,
        })
    }
}
