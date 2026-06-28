//! Hand-written prefix-routing wrapper around the Fern-generated client.
//!
//! [`Voltaria`] resolves the API base URL automatically from the API key
//! prefix, so callers don't have to pick an [`Environment`] manually:
//!
//! - keys starting with `live_`    -> [`Environment::Production`] (`https://api.voltaria.io`)
//! - keys starting with `sandbox_` -> [`Environment::Sandbox`] (`https://api.sandbox.voltaria.io`)
//! - anything else (including empty) -> [`VoltariaError::InvalidApiKey`]
//!
//! An explicitly supplied environment or base URL always overrides the
//! prefix-based routing.
//!
//! ```no_run
//! use voltaria_api::voltaria::Voltaria;
//!
//! // Routed to production from the `live_` prefix.
//! let client = Voltaria::new("live_sk_123").expect("valid key");
//! ```

use crate::api::resources::ApiClient;
use crate::{ApiError, ClientConfig, Environment};

/// Errors produced while constructing a prefix-routed client.
#[derive(Debug)]
pub enum VoltariaError {
    /// The API key did not start with a recognised environment prefix
    /// (`live_` or `sandbox_`) and no explicit environment or base URL was
    /// supplied to override the prefix routing.
    InvalidApiKey,
    /// The underlying generated client failed to build.
    Build(ApiError),
}

impl std::fmt::Display for VoltariaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidApiKey => write!(
                f,
                "invalid API key: expected a `live_` or `sandbox_` prefix, \
                 or an explicit environment / base URL"
            ),
            Self::Build(e) => write!(f, "failed to build client: {e}"),
        }
    }
}

impl std::error::Error for VoltariaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Build(e) => Some(e),
            Self::InvalidApiKey => None,
        }
    }
}

impl From<ApiError> for VoltariaError {
    fn from(e: ApiError) -> Self {
        Self::Build(e)
    }
}

/// Resolve the base URL for an API key, honouring explicit overrides.
///
/// Mirrors the reference behaviour exactly:
/// 1. an explicit `environment` wins,
/// 2. otherwise an explicit `base_url` wins,
/// 3. otherwise the `live_` / `sandbox_` prefix selects the environment,
/// 4. otherwise [`VoltariaError::InvalidApiKey`].
fn resolve_base_url(
    api_key: &str,
    environment: Option<Environment>,
    base_url: Option<&str>,
) -> Result<String, VoltariaError> {
    if let Some(env) = environment {
        return Ok(env.url().to_string());
    }
    if let Some(url) = base_url {
        return Ok(url.to_string());
    }
    if api_key.starts_with("live_") {
        return Ok(Environment::Production.url().to_string());
    }
    if api_key.starts_with("sandbox_") {
        return Ok(Environment::Sandbox.url().to_string());
    }
    Err(VoltariaError::InvalidApiKey)
}

/// Prefix-routing entry point for building a Voltaria [`ApiClient`].
pub struct Voltaria;

impl Voltaria {
    /// Build a client, deriving the environment from the API key prefix.
    ///
    /// The key is also used as the bearer token on the generated client.
    /// Returns [`VoltariaError::InvalidApiKey`] when the prefix is not
    /// recognised.
    pub fn new(api_key: &str) -> Result<ApiClient, VoltariaError> {
        VoltariaBuilder::new(api_key).build()
    }

    /// Start a builder for customising overrides before construction.
    pub fn builder(api_key: &str) -> VoltariaBuilder {
        VoltariaBuilder::new(api_key)
    }
}

/// Builder form that allows overriding the prefix-derived routing.
pub struct VoltariaBuilder {
    api_key: String,
    environment: Option<Environment>,
    base_url: Option<String>,
}

impl VoltariaBuilder {
    /// Create a builder for the given API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            environment: None,
            base_url: None,
        }
    }

    /// Force a specific environment, overriding the key prefix.
    pub fn environment(mut self, environment: Environment) -> Self {
        self.environment = Some(environment);
        self
    }

    /// Force a specific base URL, overriding the key prefix.
    ///
    /// An explicit environment, if also set, takes precedence over this.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Resolve the routing and build the generated client.
    pub fn build(self) -> Result<ApiClient, VoltariaError> {
        let base_url =
            resolve_base_url(&self.api_key, self.environment, self.base_url.as_deref())?;
        let config = ClientConfig {
            base_url,
            token: Some(self.api_key),
            ..Default::default()
        };
        Ok(ApiClient::new(config)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROD_URL: &str = "https://api.voltaria.io";
    const SANDBOX_URL: &str = "https://api.sandbox.voltaria.io";

    #[test]
    fn live_prefix_routes_to_production() {
        let client = Voltaria::new("live_abc123").expect("live key should build");
        assert_eq!(client.config.base_url, PROD_URL);
        assert_eq!(client.config.token.as_deref(), Some("live_abc123"));
    }

    #[test]
    fn sandbox_prefix_routes_to_sandbox() {
        let client = Voltaria::new("sandbox_abc123").expect("sandbox key should build");
        assert_eq!(client.config.base_url, SANDBOX_URL);
        assert_eq!(client.config.token.as_deref(), Some("sandbox_abc123"));
    }

    #[test]
    fn unknown_prefix_is_invalid() {
        // `ApiClient` is not `Debug`, so match on the result rather than
        // using `expect_err` (which would require `Debug` on the Ok type).
        match Voltaria::new("nope_abc123") {
            Err(VoltariaError::InvalidApiKey) => {}
            Err(other) => panic!("expected InvalidApiKey, got {other:?}"),
            Ok(_) => panic!("expected InvalidApiKey, got Ok"),
        }
    }

    #[test]
    fn empty_key_is_invalid() {
        match Voltaria::new("") {
            Err(VoltariaError::InvalidApiKey) => {}
            Err(other) => panic!("expected InvalidApiKey, got {other:?}"),
            Ok(_) => panic!("expected InvalidApiKey, got Ok"),
        }
    }

    #[test]
    fn explicit_base_url_overrides_prefix() {
        // Unknown prefix would normally error, but base_url override rescues it.
        let client = Voltaria::builder("nope_abc123")
            .base_url("https://localhost:8000")
            .build()
            .expect("base_url override should build");
        assert_eq!(client.config.base_url, "https://localhost:8000");
    }

    #[test]
    fn explicit_environment_overrides_prefix() {
        // live_ prefix would route to production, but the env override wins.
        let client = Voltaria::builder("live_abc123")
            .environment(Environment::Sandbox)
            .build()
            .expect("environment override should build");
        assert_eq!(client.config.base_url, SANDBOX_URL);
    }

    #[test]
    fn environment_takes_precedence_over_base_url() {
        let client = Voltaria::builder("sandbox_abc123")
            .base_url("https://localhost:8000")
            .environment(Environment::Production)
            .build()
            .expect("environment should win over base_url");
        assert_eq!(client.config.base_url, PROD_URL);
    }
}
