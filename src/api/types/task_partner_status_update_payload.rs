pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaskPartnerStatusUpdatePayload {
    /// The new status of the task. One of the following: active, in_progress, blocked, done. You can move a task to any of these at any time, so one closed by mistake can be reopened. Every change is kept in the task's status history.
    pub status: TaskPartnerStatusUpdatePayloadStatus,
}

impl TaskPartnerStatusUpdatePayload {
    pub fn builder() -> TaskPartnerStatusUpdatePayloadBuilder {
        <TaskPartnerStatusUpdatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskPartnerStatusUpdatePayloadBuilder {
    status: Option<TaskPartnerStatusUpdatePayloadStatus>,
}

impl TaskPartnerStatusUpdatePayloadBuilder {
    pub fn status(mut self, value: TaskPartnerStatusUpdatePayloadStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskPartnerStatusUpdatePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](TaskPartnerStatusUpdatePayloadBuilder::status)
    pub fn build(self) -> Result<TaskPartnerStatusUpdatePayload, BuildError> {
        Ok(TaskPartnerStatusUpdatePayload {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
