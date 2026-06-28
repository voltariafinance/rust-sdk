pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkLoanTaskResponse {
    /// Task ID for tracking progress
    #[serde(default)]
    pub task_id: String,
    /// Number of loans to process
    #[serde(default)]
    pub total_loans: i64,
    /// Estimated completion time
    #[serde(default)]
    pub estimated_completion_time: String,
}

impl BulkLoanTaskResponse {
    pub fn builder() -> BulkLoanTaskResponseBuilder {
        <BulkLoanTaskResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkLoanTaskResponseBuilder {
    task_id: Option<String>,
    total_loans: Option<i64>,
    estimated_completion_time: Option<String>,
}

impl BulkLoanTaskResponseBuilder {
    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    pub fn total_loans(mut self, value: i64) -> Self {
        self.total_loans = Some(value);
        self
    }

    pub fn estimated_completion_time(mut self, value: impl Into<String>) -> Self {
        self.estimated_completion_time = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkLoanTaskResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`task_id`](BulkLoanTaskResponseBuilder::task_id)
    /// - [`total_loans`](BulkLoanTaskResponseBuilder::total_loans)
    /// - [`estimated_completion_time`](BulkLoanTaskResponseBuilder::estimated_completion_time)
    pub fn build(self) -> Result<BulkLoanTaskResponse, BuildError> {
        Ok(BulkLoanTaskResponse {
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
            total_loans: self
                .total_loans
                .ok_or_else(|| BuildError::missing_field("total_loans"))?,
            estimated_completion_time: self
                .estimated_completion_time
                .ok_or_else(|| BuildError::missing_field("estimated_completion_time"))?,
        })
    }
}
