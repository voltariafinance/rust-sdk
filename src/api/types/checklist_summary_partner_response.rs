pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ChecklistSummaryPartnerResponse {
    /// Unique checklist summary identifier.
    #[serde(default)]
    pub id: String,
    /// When the summary was generated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Checklist type.
    pub r#type: ChecklistTypeEnum,
    /// AI-generated summary of the checklist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_description: Option<String>,
    /// Total number of checklist items.
    #[serde(default)]
    pub total_items: i64,
    /// Number of completed checklist items.
    #[serde(default)]
    pub completed_items: i64,
}

impl ChecklistSummaryPartnerResponse {
    pub fn builder() -> ChecklistSummaryPartnerResponseBuilder {
        <ChecklistSummaryPartnerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChecklistSummaryPartnerResponseBuilder {
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    r#type: Option<ChecklistTypeEnum>,
    ai_description: Option<String>,
    total_items: Option<i64>,
    completed_items: Option<i64>,
}

impl ChecklistSummaryPartnerResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn r#type(mut self, value: ChecklistTypeEnum) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn ai_description(mut self, value: impl Into<String>) -> Self {
        self.ai_description = Some(value.into());
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    pub fn completed_items(mut self, value: i64) -> Self {
        self.completed_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChecklistSummaryPartnerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ChecklistSummaryPartnerResponseBuilder::id)
    /// - [`created_at`](ChecklistSummaryPartnerResponseBuilder::created_at)
    /// - [`r#type`](ChecklistSummaryPartnerResponseBuilder::r#type)
    /// - [`total_items`](ChecklistSummaryPartnerResponseBuilder::total_items)
    /// - [`completed_items`](ChecklistSummaryPartnerResponseBuilder::completed_items)
    pub fn build(self) -> Result<ChecklistSummaryPartnerResponse, BuildError> {
        Ok(ChecklistSummaryPartnerResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            ai_description: self.ai_description,
            total_items: self
                .total_items
                .ok_or_else(|| BuildError::missing_field("total_items"))?,
            completed_items: self
                .completed_items
                .ok_or_else(|| BuildError::missing_field("completed_items"))?,
        })
    }
}
