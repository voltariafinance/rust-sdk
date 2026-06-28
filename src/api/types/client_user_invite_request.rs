pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ClientUserInviteRequest {
    #[serde(default)]
    pub first_name: String,
    #[serde(default)]
    pub last_name: String,
    #[serde(default)]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default)]
    pub role_type: String,
}

impl ClientUserInviteRequest {
    pub fn builder() -> ClientUserInviteRequestBuilder {
        <ClientUserInviteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientUserInviteRequestBuilder {
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    role_type: Option<String>,
}

impl ClientUserInviteRequestBuilder {
    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn role_type(mut self, value: impl Into<String>) -> Self {
        self.role_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ClientUserInviteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`first_name`](ClientUserInviteRequestBuilder::first_name)
    /// - [`last_name`](ClientUserInviteRequestBuilder::last_name)
    /// - [`email`](ClientUserInviteRequestBuilder::email)
    /// - [`role_type`](ClientUserInviteRequestBuilder::role_type)
    pub fn build(self) -> Result<ClientUserInviteRequest, BuildError> {
        Ok(ClientUserInviteRequest {
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            phone: self.phone,
            role_type: self
                .role_type
                .ok_or_else(|| BuildError::missing_field("role_type"))?,
        })
    }
}
