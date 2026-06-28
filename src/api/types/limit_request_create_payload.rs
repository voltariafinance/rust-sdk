pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LimitRequestCreatePayload {
    /// The ID of the client for which the limit request is being created
    #[serde(default)]
    pub client_id: String,
    /// The requested credit limit amount
    pub requested_limit: LimitRequestCreatePayloadRequestedLimit,
    /// The reason for the limit request
    #[serde(default)]
    pub reason: String,
    /// Whether a special waiver is requested alongside this limit request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiver_request: Option<bool>,
}

impl LimitRequestCreatePayload {
    pub fn builder() -> LimitRequestCreatePayloadBuilder {
        <LimitRequestCreatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LimitRequestCreatePayloadBuilder {
    client_id: Option<String>,
    requested_limit: Option<LimitRequestCreatePayloadRequestedLimit>,
    reason: Option<String>,
    waiver_request: Option<bool>,
}

impl LimitRequestCreatePayloadBuilder {
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn requested_limit(mut self, value: LimitRequestCreatePayloadRequestedLimit) -> Self {
        self.requested_limit = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn waiver_request(mut self, value: bool) -> Self {
        self.waiver_request = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LimitRequestCreatePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_id`](LimitRequestCreatePayloadBuilder::client_id)
    /// - [`requested_limit`](LimitRequestCreatePayloadBuilder::requested_limit)
    /// - [`reason`](LimitRequestCreatePayloadBuilder::reason)
    pub fn build(self) -> Result<LimitRequestCreatePayload, BuildError> {
        Ok(LimitRequestCreatePayload {
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            requested_limit: self
                .requested_limit
                .ok_or_else(|| BuildError::missing_field("requested_limit"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            waiver_request: self.waiver_request,
        })
    }
}
