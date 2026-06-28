pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DrawdownChecklistResponse {
    /// The unique identifier of the drawdown checklist item.
    #[serde(default)]
    pub id: String,
    /// The creation timestamp of the checklist item.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The last update timestamp of the checklist item.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The ID of the associated drawdown.
    #[serde(default)]
    pub drawdown_id: String,
    /// The name of the checklist item.
    #[serde(default)]
    pub name: String,
    /// A detailed description of the checklist item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The priority level of the checklist item.
    #[serde(default)]
    pub priority: i64,
    /// Indicates whether the checklist item has been completed.
    #[serde(default)]
    pub is_checked: bool,
}

impl DrawdownChecklistResponse {
    pub fn builder() -> DrawdownChecklistResponseBuilder {
        <DrawdownChecklistResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DrawdownChecklistResponseBuilder {
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    drawdown_id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    priority: Option<i64>,
    is_checked: Option<bool>,
}

impl DrawdownChecklistResponseBuilder {
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

    pub fn drawdown_id(mut self, value: impl Into<String>) -> Self {
        self.drawdown_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn priority(mut self, value: i64) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn is_checked(mut self, value: bool) -> Self {
        self.is_checked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DrawdownChecklistResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DrawdownChecklistResponseBuilder::id)
    /// - [`created_at`](DrawdownChecklistResponseBuilder::created_at)
    /// - [`updated_at`](DrawdownChecklistResponseBuilder::updated_at)
    /// - [`drawdown_id`](DrawdownChecklistResponseBuilder::drawdown_id)
    /// - [`name`](DrawdownChecklistResponseBuilder::name)
    /// - [`priority`](DrawdownChecklistResponseBuilder::priority)
    /// - [`is_checked`](DrawdownChecklistResponseBuilder::is_checked)
    pub fn build(self) -> Result<DrawdownChecklistResponse, BuildError> {
        Ok(DrawdownChecklistResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            drawdown_id: self
                .drawdown_id
                .ok_or_else(|| BuildError::missing_field("drawdown_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            priority: self
                .priority
                .ok_or_else(|| BuildError::missing_field("priority"))?,
            is_checked: self
                .is_checked
                .ok_or_else(|| BuildError::missing_field("is_checked"))?,
        })
    }
}
