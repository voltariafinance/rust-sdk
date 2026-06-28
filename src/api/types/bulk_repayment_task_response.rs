pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkRepaymentTaskResponse {
    /// Task ID for tracking progress
    #[serde(default)]
    pub task_id: String,
    /// Number of repayments to process
    #[serde(default)]
    pub total_repayments: i64,
    /// Estimated completion time
    #[serde(default)]
    pub estimated_completion_time: String,
}

impl BulkRepaymentTaskResponse {
    pub fn builder() -> BulkRepaymentTaskResponseBuilder {
        <BulkRepaymentTaskResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkRepaymentTaskResponseBuilder {
    task_id: Option<String>,
    total_repayments: Option<i64>,
    estimated_completion_time: Option<String>,
}

impl BulkRepaymentTaskResponseBuilder {
    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    pub fn total_repayments(mut self, value: i64) -> Self {
        self.total_repayments = Some(value);
        self
    }

    pub fn estimated_completion_time(mut self, value: impl Into<String>) -> Self {
        self.estimated_completion_time = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkRepaymentTaskResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`task_id`](BulkRepaymentTaskResponseBuilder::task_id)
    /// - [`total_repayments`](BulkRepaymentTaskResponseBuilder::total_repayments)
    /// - [`estimated_completion_time`](BulkRepaymentTaskResponseBuilder::estimated_completion_time)
    pub fn build(self) -> Result<BulkRepaymentTaskResponse, BuildError> {
        Ok(BulkRepaymentTaskResponse {
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
            total_repayments: self
                .total_repayments
                .ok_or_else(|| BuildError::missing_field("total_repayments"))?,
            estimated_completion_time: self
                .estimated_completion_time
                .ok_or_else(|| BuildError::missing_field("estimated_completion_time"))?,
        })
    }
}
