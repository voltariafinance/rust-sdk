pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaginatedResponseClientResponse {
    #[serde(default)]
    pub items: Vec<ClientResponse>,
    /// Current page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Number of items per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Number of items in the current page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_in_page: Option<i64>,
    /// Total number of items across all pages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
    /// Total number of pages available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i64>,
    /// Indicates if there is a next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_next: Option<bool>,
    /// Indicates if there is a previous page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_previous: Option<bool>,
}

impl PaginatedResponseClientResponse {
    pub fn builder() -> PaginatedResponseClientResponseBuilder {
        <PaginatedResponseClientResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginatedResponseClientResponseBuilder {
    items: Option<Vec<ClientResponse>>,
    page: Option<i64>,
    page_size: Option<i64>,
    items_in_page: Option<i64>,
    total_items: Option<i64>,
    total_pages: Option<i64>,
    has_next: Option<bool>,
    has_previous: Option<bool>,
}

impl PaginatedResponseClientResponseBuilder {
    pub fn items(mut self, value: Vec<ClientResponse>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn items_in_page(mut self, value: i64) -> Self {
        self.items_in_page = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    pub fn total_pages(mut self, value: i64) -> Self {
        self.total_pages = Some(value);
        self
    }

    pub fn has_next(mut self, value: bool) -> Self {
        self.has_next = Some(value);
        self
    }

    pub fn has_previous(mut self, value: bool) -> Self {
        self.has_previous = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaginatedResponseClientResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](PaginatedResponseClientResponseBuilder::items)
    pub fn build(self) -> Result<PaginatedResponseClientResponse, BuildError> {
        Ok(PaginatedResponseClientResponse {
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
            page: self.page,
            page_size: self.page_size,
            items_in_page: self.items_in_page,
            total_items: self.total_items,
            total_pages: self.total_pages,
            has_next: self.has_next,
            has_previous: self.has_previous,
        })
    }
}
