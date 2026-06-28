pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LimitRequestResponse {
    /// The ID of the limit request
    #[serde(default)]
    pub id: String,
    /// The ID of the client associated with the limit request
    #[serde(default)]
    pub client_id: String,
    /// The status of the limit request. One of the following: pending, approved, rejected
    pub status: LimitRequestStatusEnum,
    /// The requested limit amount
    #[serde(default)]
    pub requested_limit: String,
    /// The reason for the limit request
    #[serde(default)]
    pub reason: String,
    /// The response to the limit request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    /// The ID of the waiver associated with this limit request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiver_id: Option<String>,
    /// The timestamp when the limit request was created
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl LimitRequestResponse {
    pub fn builder() -> LimitRequestResponseBuilder {
        <LimitRequestResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LimitRequestResponseBuilder {
    id: Option<String>,
    client_id: Option<String>,
    status: Option<LimitRequestStatusEnum>,
    requested_limit: Option<String>,
    reason: Option<String>,
    response: Option<String>,
    waiver_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl LimitRequestResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: LimitRequestStatusEnum) -> Self {
        self.status = Some(value);
        self
    }

    pub fn requested_limit(mut self, value: impl Into<String>) -> Self {
        self.requested_limit = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn response(mut self, value: impl Into<String>) -> Self {
        self.response = Some(value.into());
        self
    }

    pub fn waiver_id(mut self, value: impl Into<String>) -> Self {
        self.waiver_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LimitRequestResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LimitRequestResponseBuilder::id)
    /// - [`client_id`](LimitRequestResponseBuilder::client_id)
    /// - [`status`](LimitRequestResponseBuilder::status)
    /// - [`requested_limit`](LimitRequestResponseBuilder::requested_limit)
    /// - [`reason`](LimitRequestResponseBuilder::reason)
    /// - [`created_at`](LimitRequestResponseBuilder::created_at)
    pub fn build(self) -> Result<LimitRequestResponse, BuildError> {
        Ok(LimitRequestResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            requested_limit: self
                .requested_limit
                .ok_or_else(|| BuildError::missing_field("requested_limit"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            response: self.response,
            waiver_id: self.waiver_id,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
