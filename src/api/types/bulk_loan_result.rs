pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkLoanResult {
    /// Number of successfully created loans
    #[serde(default)]
    pub success_count: i64,
    /// Number of failed loans
    #[serde(default)]
    pub failure_count: i64,
    /// Total number of loans processed
    #[serde(default)]
    pub total_processed: i64,
    /// Time taken to process all loans
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub processing_time_seconds: f64,
    /// Detailed results for each loan
    #[serde(default)]
    pub results: Vec<BulkLoanItemResult>,
}

impl BulkLoanResult {
    pub fn builder() -> BulkLoanResultBuilder {
        <BulkLoanResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkLoanResultBuilder {
    success_count: Option<i64>,
    failure_count: Option<i64>,
    total_processed: Option<i64>,
    processing_time_seconds: Option<f64>,
    results: Option<Vec<BulkLoanItemResult>>,
}

impl BulkLoanResultBuilder {
    pub fn success_count(mut self, value: i64) -> Self {
        self.success_count = Some(value);
        self
    }

    pub fn failure_count(mut self, value: i64) -> Self {
        self.failure_count = Some(value);
        self
    }

    pub fn total_processed(mut self, value: i64) -> Self {
        self.total_processed = Some(value);
        self
    }

    pub fn processing_time_seconds(mut self, value: f64) -> Self {
        self.processing_time_seconds = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<BulkLoanItemResult>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkLoanResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success_count`](BulkLoanResultBuilder::success_count)
    /// - [`failure_count`](BulkLoanResultBuilder::failure_count)
    /// - [`total_processed`](BulkLoanResultBuilder::total_processed)
    /// - [`processing_time_seconds`](BulkLoanResultBuilder::processing_time_seconds)
    /// - [`results`](BulkLoanResultBuilder::results)
    pub fn build(self) -> Result<BulkLoanResult, BuildError> {
        Ok(BulkLoanResult {
            success_count: self
                .success_count
                .ok_or_else(|| BuildError::missing_field("success_count"))?,
            failure_count: self
                .failure_count
                .ok_or_else(|| BuildError::missing_field("failure_count"))?,
            total_processed: self
                .total_processed
                .ok_or_else(|| BuildError::missing_field("total_processed"))?,
            processing_time_seconds: self
                .processing_time_seconds
                .ok_or_else(|| BuildError::missing_field("processing_time_seconds"))?,
            results: self
                .results
                .ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
