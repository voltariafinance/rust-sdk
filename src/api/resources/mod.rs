//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Clients**
//! - **Sandbox**
//! - **Accounts**
//! - **Collections**
//! - **Documents**
//! - **Investors**
//! - **Installments**
//! - **Loans**
//! - **Partners**
//! - **Recoveries**
//! - **Webhooks**
//! - **Repayments**
//! - **Drawdowns**

use crate::{ApiError, ClientConfig};

pub mod accounts;
pub mod clients;
pub mod collections;
pub mod documents;
pub mod drawdowns;
pub mod installments;
pub mod investors;
pub mod loans;
pub mod partners;
pub mod recoveries;
pub mod repayments;
pub mod sandbox;
pub mod webhooks;
pub struct ApiClient {
    pub config: ClientConfig,
    pub clients: ClientsClient,
    pub sandbox: SandboxClient,
    pub accounts: AccountsClient,
    pub collections: CollectionsClient,
    pub documents: DocumentsClient,
    pub investors: InvestorsClient,
    pub installments: InstallmentsClient,
    pub loans: LoansClient,
    pub partners: PartnersClient,
    pub recoveries: RecoveriesClient,
    pub webhooks: WebhooksClient,
    pub repayments: RepaymentsClient,
    pub drawdowns: DrawdownsClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            clients: ClientsClient::new(config.clone())?,
            sandbox: SandboxClient::new(config.clone())?,
            accounts: AccountsClient::new(config.clone())?,
            collections: CollectionsClient::new(config.clone())?,
            documents: DocumentsClient::new(config.clone())?,
            investors: InvestorsClient::new(config.clone())?,
            installments: InstallmentsClient::new(config.clone())?,
            loans: LoansClient::new(config.clone())?,
            partners: PartnersClient::new(config.clone())?,
            recoveries: RecoveriesClient::new(config.clone())?,
            webhooks: WebhooksClient::new(config.clone())?,
            repayments: RepaymentsClient::new(config.clone())?,
            drawdowns: DrawdownsClient::new(config.clone())?,
        })
    }
}

pub use accounts::AccountsClient;
pub use clients::ClientsClient;
pub use collections::CollectionsClient;
pub use documents::DocumentsClient;
pub use drawdowns::DrawdownsClient;
pub use installments::InstallmentsClient;
pub use investors::InvestorsClient;
pub use loans::LoansClient;
pub use partners::PartnersClient;
pub use recoveries::RecoveriesClient;
pub use repayments::RepaymentsClient;
pub use sandbox::SandboxClient;
pub use webhooks::WebhooksClient;
