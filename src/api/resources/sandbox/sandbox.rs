use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct SandboxClient {
    pub http_client: HttpClient,
}

impl SandboxClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Update an existing client's status or credit limit using their client ID.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_client(
        &self,
        client_id: &str,
        request: &ClientUpdateSandbox,
        options: Option<RequestOptions>,
    ) -> Result<ClientResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v2/sandbox/clients/{}", client_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Update the status of a specific loan using its unique loan ID.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_loan(
        &self,
        loan_id: &str,
        request: &LoanUpdateSandbox,
        options: Option<RequestOptions>,
    ) -> Result<LoanResponseWithInstallments, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v2/sandbox/loans/{}", loan_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Test a webhook subscription by ID or trigger all by event type.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn webhook_test(
        &self,
        request: &WebhookTestSandbox,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/sandbox/webhooks/trigger",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
