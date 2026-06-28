pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClientDataResponse {
    /// The unique identifier for the client data entry
    #[serde(default)]
    pub id: String,
    /// The timestamp when the client data entry was created
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The unique identifier for the client associated with the data
    #[serde(default)]
    pub client_id: String,
    /// The actual data associated with the client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,
}

impl ClientDataResponse {
    pub fn builder() -> ClientDataResponseBuilder {
        <ClientDataResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientDataResponseBuilder {
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    client_id: Option<String>,
    data: Option<HashMap<String, serde_json::Value>>,
}

impl ClientDataResponseBuilder {
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

    pub fn data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClientDataResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ClientDataResponseBuilder::id)
    /// - [`created_at`](ClientDataResponseBuilder::created_at)
    /// - [`client_id`](ClientDataResponseBuilder::client_id)
    pub fn build(self) -> Result<ClientDataResponse, BuildError> {
        Ok(ClientDataResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            data: self.data,
        })
    }
}
