pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClientDataCreatePayload {
    #[serde(default)]
    pub client_id: String,
    #[serde(default)]
    pub data: HashMap<String, serde_json::Value>,
}

impl ClientDataCreatePayload {
    pub fn builder() -> ClientDataCreatePayloadBuilder {
        <ClientDataCreatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientDataCreatePayloadBuilder {
    client_id: Option<String>,
    data: Option<HashMap<String, serde_json::Value>>,
}

impl ClientDataCreatePayloadBuilder {
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClientDataCreatePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_id`](ClientDataCreatePayloadBuilder::client_id)
    /// - [`data`](ClientDataCreatePayloadBuilder::data)
    pub fn build(self) -> Result<ClientDataCreatePayload, BuildError> {
        Ok(ClientDataCreatePayload {
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
