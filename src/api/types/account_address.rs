pub use crate::prelude::*;

/// Address structure for account holder (used for USD accounts).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountAddress {
    /// ISO 3166-1 alpha-2 country code.
    #[serde(default)]
    pub country: String,
    /// City.
    #[serde(default)]
    pub city: String,
    /// Postal / ZIP code.
    #[serde(default)]
    pub postal_code: String,
    /// State or province code (required for USD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Street address, line 1.
    #[serde(default)]
    pub line1: String,
}

impl AccountAddress {
    pub fn builder() -> AccountAddressBuilder {
        <AccountAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountAddressBuilder {
    country: Option<String>,
    city: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
    line1: Option<String>,
}

impl AccountAddressBuilder {
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn line1(mut self, value: impl Into<String>) -> Self {
        self.line1 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountAddress`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country`](AccountAddressBuilder::country)
    /// - [`city`](AccountAddressBuilder::city)
    /// - [`postal_code`](AccountAddressBuilder::postal_code)
    /// - [`line1`](AccountAddressBuilder::line1)
    pub fn build(self) -> Result<AccountAddress, BuildError> {
        Ok(AccountAddress {
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            postal_code: self
                .postal_code
                .ok_or_else(|| BuildError::missing_field("postal_code"))?,
            state: self.state,
            line1: self
                .line1
                .ok_or_else(|| BuildError::missing_field("line1"))?,
        })
    }
}
