pub use crate::prelude::*;

/// A note, with the name and email of whoever wrote it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NoteResponse {
    /// Note ID.
    #[serde(default)]
    pub id: String,
    /// When the note was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// When the note was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The note content.
    #[serde(default)]
    pub content: String,
    /// Related loan ID, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_id: Option<String>,
    /// Related installment ID, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installment_id: Option<String>,
    /// First name of the note author.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_first_name: Option<String>,
    /// Last name of the note author.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_last_name: Option<String>,
    /// Email of the note author. Null if the author was deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_email: Option<String>,
}

impl NoteResponse {
    pub fn builder() -> NoteResponseBuilder {
        <NoteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NoteResponseBuilder {
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    content: Option<String>,
    loan_id: Option<String>,
    installment_id: Option<String>,
    author_first_name: Option<String>,
    author_last_name: Option<String>,
    author_email: Option<String>,
}

impl NoteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
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

    pub fn author_first_name(mut self, value: impl Into<String>) -> Self {
        self.author_first_name = Some(value.into());
        self
    }

    pub fn author_last_name(mut self, value: impl Into<String>) -> Self {
        self.author_last_name = Some(value.into());
        self
    }

    pub fn author_email(mut self, value: impl Into<String>) -> Self {
        self.author_email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NoteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](NoteResponseBuilder::id)
    /// - [`created_at`](NoteResponseBuilder::created_at)
    /// - [`updated_at`](NoteResponseBuilder::updated_at)
    /// - [`content`](NoteResponseBuilder::content)
    pub fn build(self) -> Result<NoteResponse, BuildError> {
        Ok(NoteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            loan_id: self.loan_id,
            installment_id: self.installment_id,
            author_first_name: self.author_first_name,
            author_last_name: self.author_last_name,
            author_email: self.author_email,
        })
    }
}
