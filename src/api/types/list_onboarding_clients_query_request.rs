pub use crate::prelude::*;

/// Query parameters for list_onboarding_clients
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListOnboardingClientsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl ListOnboardingClientsQueryRequest {
    pub fn builder() -> ListOnboardingClientsQueryRequestBuilder {
        <ListOnboardingClientsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOnboardingClientsQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
}

impl ListOnboardingClientsQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOnboardingClientsQueryRequest`].
    pub fn build(self) -> Result<ListOnboardingClientsQueryRequest, BuildError> {
        Ok(ListOnboardingClientsQueryRequest {
            page: self.page,
            page_size: self.page_size,
        })
    }
}
