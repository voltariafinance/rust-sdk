//! API client and types for the Voltaria API
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints
//! - [`types`] - Request, response, and model types

pub mod resources;
pub mod types;

pub use resources::{
    AccountsClient, ApiClient, ClientsClient, DocumentsClient, DrawdownsClient, InstallmentsClient,
    InvestorsClient, LoansClient, PartnersClient, RepaymentsClient, SandboxClient, WebhooksClient,
};
pub use types::*;
