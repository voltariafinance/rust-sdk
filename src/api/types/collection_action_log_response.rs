pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CollectionActionLogResponse {
    /// The ID of the collection action log
    #[serde(default)]
    pub id: String,
    /// The ID of the collection action this log belongs to
    #[serde(default)]
    pub collection_action_id: String,
    /// The channel used for this action
    pub action_type: CollectionActionTypeEnum,
    /// The name of the action at the time it was triggered
    #[serde(default)]
    pub action_name: String,
    /// The current status of the action
    pub status: CollectionActionStatusEnum,
    /// The ID of the client this action targets
    #[serde(default)]
    pub client_id: String,
    /// The ID of the loan this action targets
    #[serde(default)]
    pub loan_id: String,
    /// The ID of the installment this action targets
    #[serde(default)]
    pub installment_id: String,
    /// Whether this action needs manual follow-up
    #[serde(default)]
    pub flag: bool,
    /// Notes about this action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// When this action is/was scheduled to run
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub scheduled_for: DateTime<FixedOffset>,
}

impl CollectionActionLogResponse {
    pub fn builder() -> CollectionActionLogResponseBuilder {
        <CollectionActionLogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CollectionActionLogResponseBuilder {
    id: Option<String>,
    collection_action_id: Option<String>,
    action_type: Option<CollectionActionTypeEnum>,
    action_name: Option<String>,
    status: Option<CollectionActionStatusEnum>,
    client_id: Option<String>,
    loan_id: Option<String>,
    installment_id: Option<String>,
    flag: Option<bool>,
    notes: Option<String>,
    scheduled_for: Option<DateTime<FixedOffset>>,
}

impl CollectionActionLogResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn collection_action_id(mut self, value: impl Into<String>) -> Self {
        self.collection_action_id = Some(value.into());
        self
    }

    pub fn action_type(mut self, value: CollectionActionTypeEnum) -> Self {
        self.action_type = Some(value);
        self
    }

    pub fn action_name(mut self, value: impl Into<String>) -> Self {
        self.action_name = Some(value.into());
        self
    }

    pub fn status(mut self, value: CollectionActionStatusEnum) -> Self {
        self.status = Some(value);
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn loan_id(mut self, value: impl Into<String>) -> Self {
        self.loan_id = Some(value.into());
        self
    }

    pub fn installment_id(mut self, value: impl Into<String>) -> Self {
        self.installment_id = Some(value.into());
        self
    }

    pub fn flag(mut self, value: bool) -> Self {
        self.flag = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn scheduled_for(mut self, value: DateTime<FixedOffset>) -> Self {
        self.scheduled_for = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CollectionActionLogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CollectionActionLogResponseBuilder::id)
    /// - [`collection_action_id`](CollectionActionLogResponseBuilder::collection_action_id)
    /// - [`action_type`](CollectionActionLogResponseBuilder::action_type)
    /// - [`action_name`](CollectionActionLogResponseBuilder::action_name)
    /// - [`status`](CollectionActionLogResponseBuilder::status)
    /// - [`client_id`](CollectionActionLogResponseBuilder::client_id)
    /// - [`loan_id`](CollectionActionLogResponseBuilder::loan_id)
    /// - [`installment_id`](CollectionActionLogResponseBuilder::installment_id)
    /// - [`flag`](CollectionActionLogResponseBuilder::flag)
    /// - [`scheduled_for`](CollectionActionLogResponseBuilder::scheduled_for)
    pub fn build(self) -> Result<CollectionActionLogResponse, BuildError> {
        Ok(CollectionActionLogResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            collection_action_id: self
                .collection_action_id
                .ok_or_else(|| BuildError::missing_field("collection_action_id"))?,
            action_type: self
                .action_type
                .ok_or_else(|| BuildError::missing_field("action_type"))?,
            action_name: self
                .action_name
                .ok_or_else(|| BuildError::missing_field("action_name"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            loan_id: self
                .loan_id
                .ok_or_else(|| BuildError::missing_field("loan_id"))?,
            installment_id: self
                .installment_id
                .ok_or_else(|| BuildError::missing_field("installment_id"))?,
            flag: self.flag.ok_or_else(|| BuildError::missing_field("flag"))?,
            notes: self.notes,
            scheduled_for: self
                .scheduled_for
                .ok_or_else(|| BuildError::missing_field("scheduled_for"))?,
        })
    }
}
