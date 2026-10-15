pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ClientCounterpartyUpdatePayload {
    /// The counterparty's legal or trading name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// ISO 3166-1 alpha-2 code of the country the counterparty is registered in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The counterparty's company registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_number: Option<String>,
    /// Free-text notes about the counterparty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl ClientCounterpartyUpdatePayload {
    pub fn builder() -> ClientCounterpartyUpdatePayloadBuilder {
        <ClientCounterpartyUpdatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientCounterpartyUpdatePayloadBuilder {
    name: Option<String>,
    country: Option<String>,
    company_number: Option<String>,
    notes: Option<String>,
}

impl ClientCounterpartyUpdatePayloadBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn company_number(mut self, value: impl Into<String>) -> Self {
        self.company_number = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ClientCounterpartyUpdatePayload`].
    pub fn build(self) -> Result<ClientCounterpartyUpdatePayload, BuildError> {
        Ok(ClientCounterpartyUpdatePayload {
            name: self.name,
            country: self.country,
            company_number: self.company_number,
            notes: self.notes,
        })
    }
}
