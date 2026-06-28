use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct ClientsClient {
    pub http_client: HttpClient,
}

impl ClientsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a list of all clients associated with your partner account.
    ///
    /// # Arguments
    ///
    /// * `order_by` - Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    /// * `q` - Query string for filtering. Format: "field:operator:value;...". Supported fields: id, name, correlation_id, company_number, status. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_clients(
        &self,
        request: &ListClientsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseClientResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/clients",
                None,
                QueryBuilder::new()
                    .serialize("page", request.page.clone())
                    .serialize("page_size", request.page_size.clone())
                    .serialize("order_by", request.order_by.clone())
                    .serialize("q", request.q.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new client under your partner account. The client will remain in a pending state until reviewed by Winyield.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_client(
        &self,
        request: &ClientCreatePayload,
        options: Option<RequestOptions>,
    ) -> Result<ClientResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/clients",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Upload supplementary client information, such as bank account balance, accounting figures, or other relevant details.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_client_data(
        &self,
        request: &ClientDataCreatePayload,
        options: Option<RequestOptions>,
    ) -> Result<ClientDataResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/clients/data",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve a list of all limit requests associated with your partner account.
    ///
    /// # Arguments
    ///
    /// * `client_id` - Filter by client ID
    /// * `order_by` - Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    /// * `q` - Query string for filtering. Format: "field:operator:value;...". Supported fields: id, client_id. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_limit_requests(
        &self,
        request: &ListLimitRequestsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseLimitRequestResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/clients/limit-requests",
                None,
                QueryBuilder::new()
                    .serialize("client_id", request.client_id.clone())
                    .serialize("page", request.page.clone())
                    .serialize("page_size", request.page_size.clone())
                    .serialize("order_by", request.order_by.clone())
                    .serialize("q", request.q.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a limit review request for a client. The request will remain in a pending state until reviewed by Winyield.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_limit_request(
        &self,
        request: &LimitRequestCreatePayload,
        options: Option<RequestOptions>,
    ) -> Result<LimitRequestResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/clients/limit-requests",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve a specific limit request by its ID.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_limit_request(
        &self,
        request_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<LimitRequestResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/clients/limit-requests/{}", request_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieve all clients that have self-registered via the portal and are awaiting partner approval.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_onboarding_clients(
        &self,
        request: &ListOnboardingClientsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseClientResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/clients/onboarding",
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .serialize("page_size", request.page_size.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Accept a self-onboarded client. The client status moves to 'pending' and the owner's portal account is activated so they can begin submitting documents.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn approve_onboarding(
        &self,
        client_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ClientResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/clients/onboarding/{}/approve", client_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Reject a self-onboarded client. The client record is kept with 'rejected' status for audit history and is not deleted.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn reject_onboarding(
        &self,
        client_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ClientResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/clients/onboarding/{}/reject", client_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Invite a new user to a client's portal account. The invited user will receive an email with a one-time link to set their password. Partner can assign any role: 'owner', 'admin', or 'viewer'.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_client_portal_user(
        &self,
        client_id: &str,
        request: &ClientUserInviteRequest,
        options: Option<RequestOptions>,
    ) -> Result<ClientUserResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/clients/{}/users", client_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve all waivers associated with a specific client.
    ///
    /// # Arguments
    ///
    /// * `order_by` - Field to order the results by, e.g., 'created_at:desc'
    /// * `q` - Query string for filtering. Format: "field:operator:value;...". Supported fields: id, client_id, status. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_client_waivers(
        &self,
        client_id: &str,
        request: &ListClientWaiversQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseWaiverResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/clients/{}/waivers", client_id),
                None,
                QueryBuilder::new()
                    .serialize("page", request.page.clone())
                    .serialize("page_size", request.page_size.clone())
                    .serialize("order_by", request.order_by.clone())
                    .serialize("q", request.q.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve detailed information for a specific client using their client ID.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_client_by_id(
        &self,
        client_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ClientResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/clients/{}", client_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a client by ID. Only clients with 'pending' status can be deleted. All client's loans must also be in 'pending' status.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_client(
        &self,
        client_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Option<HashMap<String, serde_json::Value>>, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v2/clients/{}", client_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieve the checklist summaries for one of your clients, including the AI description and item completion counts.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_client_checklist_summaries(
        &self,
        client_id: &str,
        request: &ListClientChecklistSummariesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseChecklistSummaryPartnerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/clients/{}/checklist-summaries", client_id),
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .serialize("page_size", request.page_size.clone())
                    .build(),
                options,
            )
            .await
    }
}
