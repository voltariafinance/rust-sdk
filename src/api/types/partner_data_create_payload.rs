pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PartnerDataCreatePayload {
    #[serde(default)]
    pub data: HashMap<String, serde_json::Value>,
}

impl PartnerDataCreatePayload {
    pub fn builder() -> PartnerDataCreatePayloadBuilder {
        <PartnerDataCreatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnerDataCreatePayloadBuilder {
    data: Option<HashMap<String, serde_json::Value>>,
}

impl PartnerDataCreatePayloadBuilder {
    pub fn data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PartnerDataCreatePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](PartnerDataCreatePayloadBuilder::data)
    pub fn build(self) -> Result<PartnerDataCreatePayload, BuildError> {
        Ok(PartnerDataCreatePayload {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
