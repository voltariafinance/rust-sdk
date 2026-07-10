pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CollectionActionResponse {
    /// The ID of the collection action
    #[serde(default)]
    pub id: String,
    /// The name of the collection action
    #[serde(default)]
    pub name: String,
    /// The channel used for this action
    pub action_type: CollectionActionTypeEnum,
    /// Whether this action is currently active
    #[serde(default)]
    pub is_active: bool,
    /// A description of the collection action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Timing offset relative to the installment due date, e.g. 'd-5' (5 days before) or 'd+3' (3 days after)
    #[serde(default)]
    pub timing: String,
}

impl CollectionActionResponse {
    pub fn builder() -> CollectionActionResponseBuilder {
        <CollectionActionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CollectionActionResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    action_type: Option<CollectionActionTypeEnum>,
    is_active: Option<bool>,
    description: Option<String>,
    timing: Option<String>,
}

impl CollectionActionResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn action_type(mut self, value: CollectionActionTypeEnum) -> Self {
        self.action_type = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn timing(mut self, value: impl Into<String>) -> Self {
        self.timing = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CollectionActionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CollectionActionResponseBuilder::id)
    /// - [`name`](CollectionActionResponseBuilder::name)
    /// - [`action_type`](CollectionActionResponseBuilder::action_type)
    /// - [`is_active`](CollectionActionResponseBuilder::is_active)
    /// - [`timing`](CollectionActionResponseBuilder::timing)
    pub fn build(self) -> Result<CollectionActionResponse, BuildError> {
        Ok(CollectionActionResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            action_type: self
                .action_type
                .ok_or_else(|| BuildError::missing_field("action_type"))?,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            description: self.description,
            timing: self
                .timing
                .ok_or_else(|| BuildError::missing_field("timing"))?,
        })
    }
}
