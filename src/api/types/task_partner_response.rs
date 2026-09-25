pub use crate::prelude::*;

/// A task shared with your partner account.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaskPartnerResponse {
    /// The ID of the task.
    #[serde(default)]
    pub id: String,
    /// Short title of the task.
    #[serde(default)]
    pub title: String,
    /// Longer description of what needs to be done.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The status of the task. One of the following: active, in_progress, blocked, review_needed, done, cancelled
    pub status: TaskStatusEnum,
    /// Task priority. One of the following: low, medium, high, urgent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<TaskPriorityEnum>,
    /// The user on your team this task is assigned to. Null when nobody on your team has it — either it is unassigned, or Voltaria is handling it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_id: Option<String>,
    /// When the task is due.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_at: Option<DateTime<FixedOffset>>,
    /// When the task was completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<FixedOffset>>,
    /// When the task was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// When the task was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// Your partner account the task belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    /// Client this task relates to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Loan this task relates to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_id: Option<String>,
    /// Installment this task relates to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installment_id: Option<String>,
    /// Waterfall this task relates to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waterfall_id: Option<String>,
}

impl TaskPartnerResponse {
    pub fn builder() -> TaskPartnerResponseBuilder {
        <TaskPartnerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskPartnerResponseBuilder {
    id: Option<String>,
    title: Option<String>,
    description: Option<String>,
    status: Option<TaskStatusEnum>,
    priority: Option<TaskPriorityEnum>,
    assignee_id: Option<String>,
    due_at: Option<DateTime<FixedOffset>>,
    completed_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    partner_id: Option<String>,
    client_id: Option<String>,
    loan_id: Option<String>,
    installment_id: Option<String>,
    waterfall_id: Option<String>,
}

impl TaskPartnerResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn status(mut self, value: TaskStatusEnum) -> Self {
        self.status = Some(value);
        self
    }

    pub fn priority(mut self, value: TaskPriorityEnum) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn assignee_id(mut self, value: impl Into<String>) -> Self {
        self.assignee_id = Some(value.into());
        self
    }

    pub fn due_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.due_at = Some(value);
        self
    }

    pub fn completed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.completed_at = Some(value);
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

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn loan_id(mut self, value: impl Into<String>) -> Self {
        self.loan_id = Some(value.into());
        self
    }

    pub fn installment_id(mut self, value: impl Into<String>) -> Self {
        self.installment_id = Some(value.into());
        self
    }

    pub fn waterfall_id(mut self, value: impl Into<String>) -> Self {
        self.waterfall_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskPartnerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TaskPartnerResponseBuilder::id)
    /// - [`title`](TaskPartnerResponseBuilder::title)
    /// - [`status`](TaskPartnerResponseBuilder::status)
    /// - [`created_at`](TaskPartnerResponseBuilder::created_at)
    /// - [`updated_at`](TaskPartnerResponseBuilder::updated_at)
    pub fn build(self) -> Result<TaskPartnerResponse, BuildError> {
        Ok(TaskPartnerResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            description: self.description,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            priority: self.priority,
            assignee_id: self.assignee_id,
            due_at: self.due_at,
            completed_at: self.completed_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            partner_id: self.partner_id,
            client_id: self.client_id,
            loan_id: self.loan_id,
            installment_id: self.installment_id,
            waterfall_id: self.waterfall_id,
        })
    }
}
