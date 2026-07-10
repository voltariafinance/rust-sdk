pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LoanReviewRequestCreatePayload {
    /// The ID of the loan to be reviewed. Must be a not-yet-disbursed (pending or pre-approved) loan belonging to the current partner
    #[serde(default)]
    pub loan_id: String,
    /// Optional note from the requester explaining the review request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl LoanReviewRequestCreatePayload {
    pub fn builder() -> LoanReviewRequestCreatePayloadBuilder {
        <LoanReviewRequestCreatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LoanReviewRequestCreatePayloadBuilder {
    loan_id: Option<String>,
    notes: Option<String>,
}

impl LoanReviewRequestCreatePayloadBuilder {
    pub fn loan_id(mut self, value: impl Into<String>) -> Self {
        self.loan_id = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LoanReviewRequestCreatePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`loan_id`](LoanReviewRequestCreatePayloadBuilder::loan_id)
    pub fn build(self) -> Result<LoanReviewRequestCreatePayload, BuildError> {
        Ok(LoanReviewRequestCreatePayload {
            loan_id: self
                .loan_id
                .ok_or_else(|| BuildError::missing_field("loan_id"))?,
            notes: self.notes,
        })
    }
}
