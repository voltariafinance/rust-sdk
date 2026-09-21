pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskNoteCreatePayload {
    /// The note content.
    #[serde(default)]
    pub content: String,
}

impl TaskNoteCreatePayload {
    pub fn builder() -> TaskNoteCreatePayloadBuilder {
        <TaskNoteCreatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskNoteCreatePayloadBuilder {
    content: Option<String>,
}

impl TaskNoteCreatePayloadBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskNoteCreatePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](TaskNoteCreatePayloadBuilder::content)
    pub fn build(self) -> Result<TaskNoteCreatePayload, BuildError> {
        Ok(TaskNoteCreatePayload {
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}
