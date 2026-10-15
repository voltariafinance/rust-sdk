pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ClientCounterpartyResponse {
    /// The ID of the counterparty
    #[serde(default)]
    pub id: String,
    /// The counterparty's legal or trading name
    #[serde(default)]
    pub name: String,
    /// ISO 3166-1 alpha-2 code of the country the counterparty is registered in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The counterparty's company registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_number: Option<String>,
    /// The review status of the counterparty. One of: pending, active, inactive
    pub status: ClientCounterpartyStatusEnum,
    /// The date and time the counterparty was created
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The date and time the counterparty was last updated
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The ID of the client the counterparty belongs to
    #[serde(default)]
    pub client_id: String,
    /// Free-text notes about the counterparty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl ClientCounterpartyResponse {
    pub fn builder() -> ClientCounterpartyResponseBuilder {
        <ClientCounterpartyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientCounterpartyResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    country: Option<String>,
    company_number: Option<String>,
    status: Option<ClientCounterpartyStatusEnum>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    client_id: Option<String>,
    notes: Option<String>,
}

impl ClientCounterpartyResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn company_number(mut self, value: impl Into<String>) -> Self {
        self.company_number = Some(value.into());
        self
    }

    pub fn status(mut self, value: ClientCounterpartyStatusEnum) -> Self {
        self.status = Some(value);
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

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ClientCounterpartyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ClientCounterpartyResponseBuilder::id)
    /// - [`name`](ClientCounterpartyResponseBuilder::name)
    /// - [`status`](ClientCounterpartyResponseBuilder::status)
    /// - [`created_at`](ClientCounterpartyResponseBuilder::created_at)
    /// - [`updated_at`](ClientCounterpartyResponseBuilder::updated_at)
    /// - [`client_id`](ClientCounterpartyResponseBuilder::client_id)
    pub fn build(self) -> Result<ClientCounterpartyResponse, BuildError> {
        Ok(ClientCounterpartyResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            country: self.country,
            company_number: self.company_number,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            notes: self.notes,
        })
    }
}
