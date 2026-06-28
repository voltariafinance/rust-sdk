pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkRepaymentResult {
    /// Number of successfully processed repayments
    #[serde(default)]
    pub success_count: i64,
    /// Number of failed repayments
    #[serde(default)]
    pub failure_count: i64,
    /// Total number of repayments processed
    #[serde(default)]
    pub total_processed: i64,
    /// Time taken to process all repayments
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub processing_time_seconds: f64,
    /// Detailed results for each repayment
    #[serde(default)]
    pub results: Vec<BulkRepaymentItemResult>,
}

impl BulkRepaymentResult {
    pub fn builder() -> BulkRepaymentResultBuilder {
        <BulkRepaymentResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkRepaymentResultBuilder {
    success_count: Option<i64>,
    failure_count: Option<i64>,
    total_processed: Option<i64>,
    processing_time_seconds: Option<f64>,
    results: Option<Vec<BulkRepaymentItemResult>>,
}

impl BulkRepaymentResultBuilder {
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

    pub fn results(mut self, value: Vec<BulkRepaymentItemResult>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkRepaymentResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success_count`](BulkRepaymentResultBuilder::success_count)
    /// - [`failure_count`](BulkRepaymentResultBuilder::failure_count)
    /// - [`total_processed`](BulkRepaymentResultBuilder::total_processed)
    /// - [`processing_time_seconds`](BulkRepaymentResultBuilder::processing_time_seconds)
    /// - [`results`](BulkRepaymentResultBuilder::results)
    pub fn build(self) -> Result<BulkRepaymentResult, BuildError> {
        Ok(BulkRepaymentResult {
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
