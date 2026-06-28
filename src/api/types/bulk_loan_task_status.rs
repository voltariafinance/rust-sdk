pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkLoanTaskStatus {
    /// Task ID
    #[serde(default)]
    pub task_id: String,
    /// Task status (PENDING, PROGRESS, SUCCESS, FAILURE)
    #[serde(default)]
    pub status: String,
    /// Current loan being processed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<i64>,
    /// Total loans to process
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Final result if completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<BulkLoanResult>,
    /// Error message if task failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl BulkLoanTaskStatus {
    pub fn builder() -> BulkLoanTaskStatusBuilder {
        <BulkLoanTaskStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkLoanTaskStatusBuilder {
    task_id: Option<String>,
    status: Option<String>,
    current: Option<i64>,
    total: Option<i64>,
    result: Option<BulkLoanResult>,
    error: Option<String>,
}

impl BulkLoanTaskStatusBuilder {
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

    pub fn result(mut self, value: BulkLoanResult) -> Self {
        self.result = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkLoanTaskStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`task_id`](BulkLoanTaskStatusBuilder::task_id)
    /// - [`status`](BulkLoanTaskStatusBuilder::status)
    pub fn build(self) -> Result<BulkLoanTaskStatus, BuildError> {
        Ok(BulkLoanTaskStatus {
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
