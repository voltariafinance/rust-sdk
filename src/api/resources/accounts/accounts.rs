use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct AccountsClient {
    pub http_client: HttpClient,
}

impl AccountsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Return the required and optional bank account fields for each supported currency. Fetch once and cache; use it to build the create-account request.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_client_account_fields(
        &self,
        client_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, CurrencyFieldSpec>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/clients/{}/accounts/fields", client_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieve all bank accounts for one of your clients.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_client_accounts(
        &self,
        client_id: &str,
        request: &ListClientAccountsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseClientAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/clients/{}/accounts", client_id),
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .serialize("page_size", request.page_size.clone())
                    .serialize("order_by", request.order_by.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a bank account for one of your clients. The account is registered with the payment provider immediately. Use the `status` field to create it as `active` (default; demotes any existing active account in the same currency to `passive`) or `passive`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_client_account(
        &self,
        client_id: &str,
        request: &PartnerClientAccountCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<ClientAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/clients/{}/accounts", client_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve a specific bank account for one of your clients.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_client_account(
        &self,
        client_id: &str,
        account_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ClientAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/clients/{}/accounts/{}", client_id, account_id),
                None,
                None,
                options,
            )
            .await
    }
}
