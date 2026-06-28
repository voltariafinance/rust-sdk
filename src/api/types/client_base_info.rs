pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ClientBaseInfo {
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

impl ClientBaseInfo {
    pub fn builder() -> ClientBaseInfoBuilder {
        <ClientBaseInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientBaseInfoBuilder {
    name: Option<String>,
    r#type: Option<ClientTypeEnum>,
    jurisdiction: Option<JurisdictionEnum>,
    company_number: Option<String>,
    status: Option<ClientStatusEnum>,
}

impl ClientBaseInfoBuilder {
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

    /// Consumes the builder and constructs a [`ClientBaseInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ClientBaseInfoBuilder::name)
    /// - [`r#type`](ClientBaseInfoBuilder::r#type)
    /// - [`jurisdiction`](ClientBaseInfoBuilder::jurisdiction)
    /// - [`status`](ClientBaseInfoBuilder::status)
    pub fn build(self) -> Result<ClientBaseInfo, BuildError> {
        Ok(ClientBaseInfo {
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
