pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkRepaymentCreatePayload {
    /// List of repayments to create (max 10000)
    #[serde(default)]
    pub repayments: Vec<BulkRepaymentItemPayload>,
}

impl BulkRepaymentCreatePayload {
    pub fn builder() -> BulkRepaymentCreatePayloadBuilder {
        <BulkRepaymentCreatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkRepaymentCreatePayloadBuilder {
    repayments: Option<Vec<BulkRepaymentItemPayload>>,
}

impl BulkRepaymentCreatePayloadBuilder {
    pub fn repayments(mut self, value: Vec<BulkRepaymentItemPayload>) -> Self {
        self.repayments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkRepaymentCreatePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`repayments`](BulkRepaymentCreatePayloadBuilder::repayments)
    pub fn build(self) -> Result<BulkRepaymentCreatePayload, BuildError> {
        Ok(BulkRepaymentCreatePayload {
            repayments: self
                .repayments
                .ok_or_else(|| BuildError::missing_field("repayments"))?,
        })
    }
}
