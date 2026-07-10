pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CollectionActionLogUpdatePayload {
    /// The updated status of the action: 'completed' or 'failed'
    pub status: CollectionActionLogUpdatePayloadStatus,
    /// Notes about this action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl CollectionActionLogUpdatePayload {
    pub fn builder() -> CollectionActionLogUpdatePayloadBuilder {
        <CollectionActionLogUpdatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CollectionActionLogUpdatePayloadBuilder {
    status: Option<CollectionActionLogUpdatePayloadStatus>,
    notes: Option<String>,
}

impl CollectionActionLogUpdatePayloadBuilder {
    pub fn status(mut self, value: CollectionActionLogUpdatePayloadStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CollectionActionLogUpdatePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](CollectionActionLogUpdatePayloadBuilder::status)
    pub fn build(self) -> Result<CollectionActionLogUpdatePayload, BuildError> {
        Ok(CollectionActionLogUpdatePayload {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            notes: self.notes,
        })
    }
}
