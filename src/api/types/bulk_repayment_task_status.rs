pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkRepaymentTaskStatus {
    /// Task ID
    #[serde(default)]
    pub task_id: String,
    /// Task status
    #[serde(default)]
    pub status: String,
    /// Current progress count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<i64>,
    /// Total items to process
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Final result when task is completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<BulkRepaymentResult>,
    /// Error message if task failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl BulkRepaymentTaskStatus {
    pub fn builder() -> BulkRepaymentTaskStatusBuilder {
        <BulkRepaymentTaskStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkRepaymentTaskStatusBuilder {
    task_id: Option<String>,
    status: Option<String>,
    current: Option<i64>,
    total: Option<i64>,
    result: Option<BulkRepaymentResult>,
    error: Option<String>,
}

impl BulkRepaymentTaskStatusBuilder {
    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn current(mut self, value: i64) -> Self {
        self.current = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn result(mut self, value: BulkRepaymentResult) -> Self {
        self.result = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkRepaymentTaskStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`task_id`](BulkRepaymentTaskStatusBuilder::task_id)
    /// - [`status`](BulkRepaymentTaskStatusBuilder::status)
    pub fn build(self) -> Result<BulkRepaymentTaskStatus, BuildError> {
        Ok(BulkRepaymentTaskStatus {
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            current: self.current,
            total: self.total,
            result: self.result,
            error: self.error,
        })
    }
}
