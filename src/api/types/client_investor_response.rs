pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ClientInvestorResponse {
    /// The ID of the client
    #[serde(default)]
    pub id: String,
    /// The creation date of the client
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The last update date of the client
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The correlation ID you provided at the creation of the client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    /// The name of the client
    #[serde(default)]
    pub name: String,
    /// The type of the client, must be one of `individual`, `corporate`
    pub r#type: ClientTypeEnum,
    /// The jurisdiction of the client, must be one of `eu`, `us`, `uk`
    pub jurisdiction: JurisdictionEnum,
    /// The company number of the client if type is `corporate`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_number: Option<String>,
    /// The status of the client. One of the following: `active, rejected, deactivated, pending, pending_onboarding, pre_approved, deleted, inactive`
    pub status: ClientStatusEnum,
}

impl ClientInvestorResponse {
    pub fn builder() -> ClientInvestorResponseBuilder {
        <ClientInvestorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientInvestorResponseBuilder {
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    correlation_id: Option<String>,
    name: Option<String>,
    r#type: Option<ClientTypeEnum>,
    jurisdiction: Option<JurisdictionEnum>,
    company_number: Option<String>,
    status: Option<ClientStatusEnum>,
}

impl ClientInvestorResponseBuilder {
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

    pub fn correlation_id(mut self, value: impl Into<String>) -> Self {
        self.correlation_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ClientTypeEnum) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn jurisdiction(mut self, value: JurisdictionEnum) -> Self {
        self.jurisdiction = Some(value);
        self
    }

    pub fn company_number(mut self, value: impl Into<String>) -> Self {
        self.company_number = Some(value.into());
        self
    }

    pub fn status(mut self, value: ClientStatusEnum) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClientInvestorResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ClientInvestorResponseBuilder::id)
    /// - [`created_at`](ClientInvestorResponseBuilder::created_at)
    /// - [`updated_at`](ClientInvestorResponseBuilder::updated_at)
    /// - [`name`](ClientInvestorResponseBuilder::name)
    /// - [`r#type`](ClientInvestorResponseBuilder::r#type)
    /// - [`jurisdiction`](ClientInvestorResponseBuilder::jurisdiction)
    /// - [`status`](ClientInvestorResponseBuilder::status)
    pub fn build(self) -> Result<ClientInvestorResponse, BuildError> {
        Ok(ClientInvestorResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            correlation_id: self.correlation_id,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            jurisdiction: self
                .jurisdiction
                .ok_or_else(|| BuildError::missing_field("jurisdiction"))?,
            company_number: self.company_number,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
