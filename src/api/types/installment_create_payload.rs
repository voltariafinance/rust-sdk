pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InstallmentCreatePayload {
    /// The loan ID to add the installments to
    #[serde(default)]
    pub loan_id: String,
    /// List of installments to add to the loan
    #[serde(default)]
    pub installments: Vec<LoanInstallmentCreatePayload>,
}

impl InstallmentCreatePayload {
    pub fn builder() -> InstallmentCreatePayloadBuilder {
        <InstallmentCreatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InstallmentCreatePayloadBuilder {
    loan_id: Option<String>,
    installments: Option<Vec<LoanInstallmentCreatePayload>>,
}

impl InstallmentCreatePayloadBuilder {
    pub fn loan_id(mut self, value: impl Into<String>) -> Self {
        self.loan_id = Some(value.into());
        self
    }

    pub fn installments(mut self, value: Vec<LoanInstallmentCreatePayload>) -> Self {
        self.installments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InstallmentCreatePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`loan_id`](InstallmentCreatePayloadBuilder::loan_id)
    /// - [`installments`](InstallmentCreatePayloadBuilder::installments)
    pub fn build(self) -> Result<InstallmentCreatePayload, BuildError> {
        Ok(InstallmentCreatePayload {
            loan_id: self
                .loan_id
                .ok_or_else(|| BuildError::missing_field("loan_id"))?,
            installments: self
                .installments
                .ok_or_else(|| BuildError::missing_field("installments"))?,
        })
    }
}
