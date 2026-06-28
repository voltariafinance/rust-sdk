pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkLoanCreatePayload {
    /// List of loans to create (max 1000)
    #[serde(default)]
    pub loans: Vec<BulkLoanItemPayload>,
}

impl BulkLoanCreatePayload {
    pub fn builder() -> BulkLoanCreatePayloadBuilder {
        <BulkLoanCreatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkLoanCreatePayloadBuilder {
    loans: Option<Vec<BulkLoanItemPayload>>,
}

impl BulkLoanCreatePayloadBuilder {
    pub fn loans(mut self, value: Vec<BulkLoanItemPayload>) -> Self {
        self.loans = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkLoanCreatePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`loans`](BulkLoanCreatePayloadBuilder::loans)
    pub fn build(self) -> Result<BulkLoanCreatePayload, BuildError> {
        Ok(BulkLoanCreatePayload {
            loans: self
                .loans
                .ok_or_else(|| BuildError::missing_field("loans"))?,
        })
    }
}
