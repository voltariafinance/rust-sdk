pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AvailableDocumentCategoriesResponse {
    /// Document categories available for upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
}

impl AvailableDocumentCategoriesResponse {
    pub fn builder() -> AvailableDocumentCategoriesResponseBuilder {
        <AvailableDocumentCategoriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AvailableDocumentCategoriesResponseBuilder {
    categories: Option<Vec<String>>,
}

impl AvailableDocumentCategoriesResponseBuilder {
    pub fn categories(mut self, value: Vec<String>) -> Self {
        self.categories = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AvailableDocumentCategoriesResponse`].
    pub fn build(self) -> Result<AvailableDocumentCategoriesResponse, BuildError> {
        Ok(AvailableDocumentCategoriesResponse {
            categories: self.categories,
        })
    }
}
