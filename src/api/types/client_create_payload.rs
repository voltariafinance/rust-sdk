pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClientCreatePayload {
    /// The correlation ID you provided at the creation of the client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    /// The name of the client
    #[serde(default)]
    pub name: String,
    /// The type of the client, must be one of `individual`,`corporate`. Default is `corporate` if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ClientTypeEnum>,
    /// The jurisdiction of the client, must be one of `eu`, `us`, `uk`
    pub jurisdiction: JurisdictionEnum,
    /// The company number of the client if type is `corporate`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_number: Option<String>,
    /// Additional data associated with the client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,
}

impl ClientCreatePayload {
    pub fn builder() -> ClientCreatePayloadBuilder {
        <ClientCreatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientCreatePayloadBuilder {
    correlation_id: Option<String>,
    name: Option<String>,
    r#type: Option<ClientTypeEnum>,
    jurisdiction: Option<JurisdictionEnum>,
    company_number: Option<String>,
    data: Option<HashMap<String, serde_json::Value>>,
}

impl ClientCreatePayloadBuilder {
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

    pub fn data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClientCreatePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ClientCreatePayloadBuilder::name)
    /// - [`jurisdiction`](ClientCreatePayloadBuilder::jurisdiction)
    pub fn build(self) -> Result<ClientCreatePayload, BuildError> {
        Ok(ClientCreatePayload {
            correlation_id: self.correlation_id,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type,
            jurisdiction: self
                .jurisdiction
                .ok_or_else(|| BuildError::missing_field("jurisdiction"))?,
            company_number: self.company_number,
            data: self.data,
        })
    }
}
