pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ClientUserResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub partner_id: String,
    #[serde(default)]
    pub client_id: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub role_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<RoleResponse>,
    pub status: ClientUserStatusEnum,
    #[serde(default)]
    pub is_email_verified: bool,
    pub kyc_status: KycStatusEnum,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "is_2fa_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is2fa_enabled: Option<bool>,
    #[serde(rename = "is_2fa_required")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is2fa_required: Option<bool>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl ClientUserResponse {
    pub fn builder() -> ClientUserResponseBuilder {
        <ClientUserResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientUserResponseBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    client_id: Option<String>,
    email: Option<String>,
    role_id: Option<String>,
    role: Option<RoleResponse>,
    status: Option<ClientUserStatusEnum>,
    is_email_verified: Option<bool>,
    kyc_status: Option<KycStatusEnum>,
    first_name: Option<String>,
    last_name: Option<String>,
    phone: Option<String>,
    is2fa_enabled: Option<bool>,
    is2fa_required: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ClientUserResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn role_id(mut self, value: impl Into<String>) -> Self {
        self.role_id = Some(value.into());
        self
    }

    pub fn role(mut self, value: RoleResponse) -> Self {
        self.role = Some(value);
        self
    }

    pub fn status(mut self, value: ClientUserStatusEnum) -> Self {
        self.status = Some(value);
        self
    }

    pub fn is_email_verified(mut self, value: bool) -> Self {
        self.is_email_verified = Some(value);
        self
    }

    pub fn kyc_status(mut self, value: KycStatusEnum) -> Self {
        self.kyc_status = Some(value);
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn is2fa_enabled(mut self, value: bool) -> Self {
        self.is2fa_enabled = Some(value);
        self
    }

    pub fn is2fa_required(mut self, value: bool) -> Self {
        self.is2fa_required = Some(value);
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

    /// Consumes the builder and constructs a [`ClientUserResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ClientUserResponseBuilder::id)
    /// - [`partner_id`](ClientUserResponseBuilder::partner_id)
    /// - [`client_id`](ClientUserResponseBuilder::client_id)
    /// - [`email`](ClientUserResponseBuilder::email)
    /// - [`role_id`](ClientUserResponseBuilder::role_id)
    /// - [`status`](ClientUserResponseBuilder::status)
    /// - [`is_email_verified`](ClientUserResponseBuilder::is_email_verified)
    /// - [`kyc_status`](ClientUserResponseBuilder::kyc_status)
    /// - [`created_at`](ClientUserResponseBuilder::created_at)
    /// - [`updated_at`](ClientUserResponseBuilder::updated_at)
    pub fn build(self) -> Result<ClientUserResponse, BuildError> {
        Ok(ClientUserResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            role_id: self
                .role_id
                .ok_or_else(|| BuildError::missing_field("role_id"))?,
            role: self.role,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            is_email_verified: self
                .is_email_verified
                .ok_or_else(|| BuildError::missing_field("is_email_verified"))?,
            kyc_status: self
                .kyc_status
                .ok_or_else(|| BuildError::missing_field("kyc_status"))?,
            first_name: self.first_name,
            last_name: self.last_name,
            phone: self.phone,
            is2fa_enabled: self.is2fa_enabled,
            is2fa_required: self.is2fa_required,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
