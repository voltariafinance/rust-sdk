pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LoanUpdateSandbox {
    /// The status of the client. One of the following: `pending, overdue, active, default, sold, restructured, repaid, pre_approved, rejected, deleted, inactive`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<LoanStatusEnum>,
}

impl LoanUpdateSandbox {
    pub fn builder() -> LoanUpdateSandboxBuilder {
        <LoanUpdateSandboxBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LoanUpdateSandboxBuilder {
    status: Option<LoanStatusEnum>,
}

impl LoanUpdateSandboxBuilder {
    pub fn status(mut self, value: LoanStatusEnum) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LoanUpdateSandbox`].
    pub fn build(self) -> Result<LoanUpdateSandbox, BuildError> {
        Ok(LoanUpdateSandbox {
            status: self.status,
        })
    }
}
