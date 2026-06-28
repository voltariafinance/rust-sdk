pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RoleResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#type: String,
}

impl RoleResponse {
    pub fn builder() -> RoleResponseBuilder {
        <RoleResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RoleResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    r#type: Option<String>,
}

impl RoleResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RoleResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RoleResponseBuilder::id)
    /// - [`name`](RoleResponseBuilder::name)
    /// - [`r#type`](RoleResponseBuilder::r#type)
    pub fn build(self) -> Result<RoleResponse, BuildError> {
        Ok(RoleResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
