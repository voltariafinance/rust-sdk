pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LoanUpdateSandbox {
    /// The status of the loan. One of the following: `pending, overdue, active, default, sold, restructured, repaid, pre_approved, rejected, inactive`. Use the delete loan endpoint to delete a loan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<LoanStatusEnum>,
    /// The date the loan was sold. Required when the status changes to `sold`; can't be later than today or earlier than the loan date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sold_at: Option<DateTime<FixedOffset>>,
    /// The date the loan was restructured. Required when the status changes to `restructured`; can't be later than today or earlier than the loan date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restructured_at: Option<DateTime<FixedOffset>>,
    /// The date the loan defaulted. Required when the status changes to `default`; can't be later than today or earlier than the loan date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaulted_at: Option<DateTime<FixedOffset>>,
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
    sold_at: Option<DateTime<FixedOffset>>,
    restructured_at: Option<DateTime<FixedOffset>>,
    defaulted_at: Option<DateTime<FixedOffset>>,
}

impl LoanUpdateSandboxBuilder {
    pub fn status(mut self, value: LoanStatusEnum) -> Self {
        self.status = Some(value);
        self
    }

    pub fn sold_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.sold_at = Some(value);
        self
    }

    pub fn restructured_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.restructured_at = Some(value);
        self
    }

    pub fn defaulted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.defaulted_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LoanUpdateSandbox`].
    pub fn build(self) -> Result<LoanUpdateSandbox, BuildError> {
        Ok(LoanUpdateSandbox {
            status: self.status,
            sold_at: self.sold_at,
            restructured_at: self.restructured_at,
            defaulted_at: self.defaulted_at,
        })
    }
}
