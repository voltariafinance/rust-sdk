pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClientUpdateSandbox {
    /// The status of the client. One of the following: `active, rejected, deactivated, pending, pending_onboarding, pre_approved, deleted, inactive`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ClientStatusEnum>,
    /// The limit to set for the client. This will override the existing limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<ClientUpdateSandboxLimit>,
}

impl ClientUpdateSandbox {
    pub fn builder() -> ClientUpdateSandboxBuilder {
        <ClientUpdateSandboxBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientUpdateSandboxBuilder {
    status: Option<ClientStatusEnum>,
    limit: Option<ClientUpdateSandboxLimit>,
}

impl ClientUpdateSandboxBuilder {
    pub fn status(mut self, value: ClientStatusEnum) -> Self {
        self.status = Some(value);
        self
    }

    pub fn limit(mut self, value: ClientUpdateSandboxLimit) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClientUpdateSandbox`].
    pub fn build(self) -> Result<ClientUpdateSandbox, BuildError> {
        Ok(ClientUpdateSandbox {
            status: self.status,
            limit: self.limit,
        })
    }
}
