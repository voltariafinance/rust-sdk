pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PartnerClientAccountCreateRequest {
    /// Full name of the account holder.
    #[serde(default)]
    pub account_holder_name: String,
    /// Optional label / nickname for the account (max 50 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Account holder type. One of: `business`, `private`.
    pub account_holder_type: AccountHolderTypeEnum,
    /// ISO 4217 currency code. Use `/accounts/fields` to get required fields per currency.
    pub currency: CurrencyEnum,
    /// Sort code (required for GBP).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
    /// Account number (required for GBP and USD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// IBAN (required for EUR, CZK, PLN).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    /// BIC / SWIFT code (optional for EUR).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    /// ABA routing number (required for USD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
    /// Account type (required for USD). E.g. `checking` or `savings`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Account holder address (required for USD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AccountAddress>,
    /// Account status. `active` demotes any existing active account in the same currency to `passive`; `passive` is added as a backup. Defaults to `active`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PartnerClientAccountCreateRequestStatus>,
}

impl PartnerClientAccountCreateRequest {
    pub fn builder() -> PartnerClientAccountCreateRequestBuilder {
        <PartnerClientAccountCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnerClientAccountCreateRequestBuilder {
    account_holder_name: Option<String>,
    label: Option<String>,
    account_holder_type: Option<AccountHolderTypeEnum>,
    currency: Option<CurrencyEnum>,
    sort_code: Option<String>,
    account_number: Option<String>,
    iban: Option<String>,
    bic: Option<String>,
    routing_number: Option<String>,
    account_type: Option<String>,
    address: Option<AccountAddress>,
    status: Option<PartnerClientAccountCreateRequestStatus>,
}

impl PartnerClientAccountCreateRequestBuilder {
    pub fn account_holder_name(mut self, value: impl Into<String>) -> Self {
        self.account_holder_name = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn account_holder_type(mut self, value: AccountHolderTypeEnum) -> Self {
        self.account_holder_type = Some(value);
        self
    }

    pub fn currency(mut self, value: CurrencyEnum) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn sort_code(mut self, value: impl Into<String>) -> Self {
        self.sort_code = Some(value.into());
        self
    }

    pub fn account_number(mut self, value: impl Into<String>) -> Self {
        self.account_number = Some(value.into());
        self
    }

    pub fn iban(mut self, value: impl Into<String>) -> Self {
        self.iban = Some(value.into());
        self
    }

    pub fn bic(mut self, value: impl Into<String>) -> Self {
        self.bic = Some(value.into());
        self
    }

    pub fn routing_number(mut self, value: impl Into<String>) -> Self {
        self.routing_number = Some(value.into());
        self
    }

    pub fn account_type(mut self, value: impl Into<String>) -> Self {
        self.account_type = Some(value.into());
        self
    }

    pub fn address(mut self, value: AccountAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn status(mut self, value: PartnerClientAccountCreateRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PartnerClientAccountCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_holder_name`](PartnerClientAccountCreateRequestBuilder::account_holder_name)
    /// - [`account_holder_type`](PartnerClientAccountCreateRequestBuilder::account_holder_type)
    /// - [`currency`](PartnerClientAccountCreateRequestBuilder::currency)
    pub fn build(self) -> Result<PartnerClientAccountCreateRequest, BuildError> {
        Ok(PartnerClientAccountCreateRequest {
            account_holder_name: self
                .account_holder_name
                .ok_or_else(|| BuildError::missing_field("account_holder_name"))?,
            label: self.label,
            account_holder_type: self
                .account_holder_type
                .ok_or_else(|| BuildError::missing_field("account_holder_type"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            sort_code: self.sort_code,
            account_number: self.account_number,
            iban: self.iban,
            bic: self.bic,
            routing_number: self.routing_number,
            account_type: self.account_type,
            address: self.address,
            status: self.status,
        })
    }
}
