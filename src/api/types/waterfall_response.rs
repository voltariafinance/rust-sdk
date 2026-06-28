pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WaterfallResponse {
    /// The ID of the waterfall
    #[serde(default)]
    pub id: String,
    /// The partner ID
    #[serde(default)]
    pub partner_id: String,
    /// The name of the waterfall
    #[serde(default)]
    pub name: String,
    /// The date of the waterfall
    #[serde(default)]
    pub date: NaiveDate,
    /// The status of the waterfall
    pub status: WaterfallStatusEnum,
    /// The cash balance associated with the waterfall
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_balance: Option<String>,
    /// The currency of the cash balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_balance_currency: Option<String>,
    /// The date of the cash balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_balance_date: Option<NaiveDate>,
    /// The Presigned URL of the file. This is a temporary URL that allows you to download the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    /// The date and time when the waterfall was created
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The date and time when the waterfall was last updated
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl WaterfallResponse {
    pub fn builder() -> WaterfallResponseBuilder {
        <WaterfallResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WaterfallResponseBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    name: Option<String>,
    date: Option<NaiveDate>,
    status: Option<WaterfallStatusEnum>,
    cash_balance: Option<String>,
    cash_balance_currency: Option<String>,
    cash_balance_date: Option<NaiveDate>,
    file_url: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl WaterfallResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn status(mut self, value: WaterfallStatusEnum) -> Self {
        self.status = Some(value);
        self
    }

    pub fn cash_balance(mut self, value: impl Into<String>) -> Self {
        self.cash_balance = Some(value.into());
        self
    }

    pub fn cash_balance_currency(mut self, value: impl Into<String>) -> Self {
        self.cash_balance_currency = Some(value.into());
        self
    }

    pub fn cash_balance_date(mut self, value: NaiveDate) -> Self {
        self.cash_balance_date = Some(value);
        self
    }

    pub fn file_url(mut self, value: impl Into<String>) -> Self {
        self.file_url = Some(value.into());
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

    /// Consumes the builder and constructs a [`WaterfallResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WaterfallResponseBuilder::id)
    /// - [`partner_id`](WaterfallResponseBuilder::partner_id)
    /// - [`name`](WaterfallResponseBuilder::name)
    /// - [`date`](WaterfallResponseBuilder::date)
    /// - [`status`](WaterfallResponseBuilder::status)
    /// - [`created_at`](WaterfallResponseBuilder::created_at)
    /// - [`updated_at`](WaterfallResponseBuilder::updated_at)
    pub fn build(self) -> Result<WaterfallResponse, BuildError> {
        Ok(WaterfallResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            cash_balance: self.cash_balance,
            cash_balance_currency: self.cash_balance_currency,
            cash_balance_date: self.cash_balance_date,
            file_url: self.file_url,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
