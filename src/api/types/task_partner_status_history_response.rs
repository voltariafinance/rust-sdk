pub use crate::prelude::*;

/// One status change on a task: when it happened, what it moved from and to, and
/// who made it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaskPartnerStatusHistoryResponse {
    /// When the status changed.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The status before the change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_status: Option<TaskStatusEnum>,
    /// The status after the change.
    pub new_status: TaskStatusEnum,
    /// Who made the change. One of the following: partner, support
    pub actor_type: TaskPublicActorTypeEnum,
}

impl TaskPartnerStatusHistoryResponse {
    pub fn builder() -> TaskPartnerStatusHistoryResponseBuilder {
        <TaskPartnerStatusHistoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskPartnerStatusHistoryResponseBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    old_status: Option<TaskStatusEnum>,
    new_status: Option<TaskStatusEnum>,
    actor_type: Option<TaskPublicActorTypeEnum>,
}

impl TaskPartnerStatusHistoryResponseBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn old_status(mut self, value: TaskStatusEnum) -> Self {
        self.old_status = Some(value);
        self
    }

    pub fn new_status(mut self, value: TaskStatusEnum) -> Self {
        self.new_status = Some(value);
        self
    }

    pub fn actor_type(mut self, value: TaskPublicActorTypeEnum) -> Self {
        self.actor_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskPartnerStatusHistoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](TaskPartnerStatusHistoryResponseBuilder::created_at)
    /// - [`new_status`](TaskPartnerStatusHistoryResponseBuilder::new_status)
    /// - [`actor_type`](TaskPartnerStatusHistoryResponseBuilder::actor_type)
    pub fn build(self) -> Result<TaskPartnerStatusHistoryResponse, BuildError> {
        Ok(TaskPartnerStatusHistoryResponse {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            old_status: self.old_status,
            new_status: self
                .new_status
                .ok_or_else(|| BuildError::missing_field("new_status"))?,
            actor_type: self
                .actor_type
                .ok_or_else(|| BuildError::missing_field("actor_type"))?,
        })
    }
}
