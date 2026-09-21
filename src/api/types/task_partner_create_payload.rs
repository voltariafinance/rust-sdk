pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskPartnerCreatePayload {
    /// Short title of the task.
    #[serde(default)]
    pub title: String,
    /// Optional longer description of what needs to be done.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Task priority. One of the following: low, medium, high, urgent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<TaskPriorityEnum>,
    /// Optional due date for the task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_at: Option<DateTime<FixedOffset>>,
    /// Client this task relates to. Must belong to your partner account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Loan this task relates to. Must belong to your partner account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_id: Option<String>,
    /// Installment this task relates to. Must belong to your partner account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installment_id: Option<String>,
    /// Waterfall this task relates to. Must belong to your partner account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waterfall_id: Option<String>,
}

impl TaskPartnerCreatePayload {
    pub fn builder() -> TaskPartnerCreatePayloadBuilder {
        <TaskPartnerCreatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskPartnerCreatePayloadBuilder {
    title: Option<String>,
    description: Option<String>,
    priority: Option<TaskPriorityEnum>,
    due_at: Option<DateTime<FixedOffset>>,
    client_id: Option<String>,
    loan_id: Option<String>,
    installment_id: Option<String>,
    waterfall_id: Option<String>,
}

impl TaskPartnerCreatePayloadBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn priority(mut self, value: TaskPriorityEnum) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn due_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.due_at = Some(value);
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

    /// Consumes the builder and constructs a [`TaskPartnerCreatePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](TaskPartnerCreatePayloadBuilder::title)
    pub fn build(self) -> Result<TaskPartnerCreatePayload, BuildError> {
        Ok(TaskPartnerCreatePayload {
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            description: self.description,
            priority: self.priority,
            due_at: self.due_at,
            client_id: self.client_id,
            loan_id: self.loan_id,
            installment_id: self.installment_id,
            waterfall_id: self.waterfall_id,
        })
    }
}
