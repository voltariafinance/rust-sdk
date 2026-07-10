pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LoanReviewRequestResponse {
    /// The ID of the loan review request
    #[serde(default)]
    pub id: String,
    /// The ID of the loan associated with the review request
    #[serde(default)]
    pub loan_id: String,
    /// The ID of the client associated with the review request
    #[serde(default)]
    pub client_id: String,
    /// The status of the review request. One of the following: pending, approved, rejected
    pub status: LoanReviewRequestStatusEnum,
    /// The requester's note for the review request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// The reviewer's note explaining the approval or rejection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    /// The timestamp when the review request was approved or rejected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<DateTime<FixedOffset>>,
    /// The timestamp when the review request was created
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The timestamp when the review request was last updated
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl LoanReviewRequestResponse {
    pub fn builder() -> LoanReviewRequestResponseBuilder {
        <LoanReviewRequestResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LoanReviewRequestResponseBuilder {
    id: Option<String>,
    loan_id: Option<String>,
    client_id: Option<String>,
    status: Option<LoanReviewRequestStatusEnum>,
    notes: Option<String>,
    response: Option<String>,
    reviewed_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl LoanReviewRequestResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn loan_id(mut self, value: impl Into<String>) -> Self {
        self.loan_id = Some(value.into());
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: LoanReviewRequestStatusEnum) -> Self {
        self.status = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn response(mut self, value: impl Into<String>) -> Self {
        self.response = Some(value.into());
        self
    }

    pub fn reviewed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.reviewed_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LoanReviewRequestResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LoanReviewRequestResponseBuilder::id)
    /// - [`loan_id`](LoanReviewRequestResponseBuilder::loan_id)
    /// - [`client_id`](LoanReviewRequestResponseBuilder::client_id)
    /// - [`status`](LoanReviewRequestResponseBuilder::status)
    /// - [`created_at`](LoanReviewRequestResponseBuilder::created_at)
    /// - [`updated_at`](LoanReviewRequestResponseBuilder::updated_at)
    pub fn build(self) -> Result<LoanReviewRequestResponse, BuildError> {
        Ok(LoanReviewRequestResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            loan_id: self
                .loan_id
                .ok_or_else(|| BuildError::missing_field("loan_id"))?,
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            notes: self.notes,
            response: self.response,
            reviewed_at: self.reviewed_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
