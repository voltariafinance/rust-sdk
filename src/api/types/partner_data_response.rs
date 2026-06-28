pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PartnerDataResponse {
    /// The unique identifier for the partner data entry
    #[serde(default)]
    pub id: String,
    /// The timestamp when the partner data entry was created
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The actual data associated with the partner
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,
}

impl PartnerDataResponse {
    pub fn builder() -> PartnerDataResponseBuilder {
        <PartnerDataResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnerDataResponseBuilder {
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    data: Option<HashMap<String, serde_json::Value>>,
}

impl PartnerDataResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PartnerDataResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PartnerDataResponseBuilder::id)
    /// - [`created_at`](PartnerDataResponseBuilder::created_at)
    pub fn build(self) -> Result<PartnerDataResponse, BuildError> {
        Ok(PartnerDataResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            data: self.data,
        })
    }
}
