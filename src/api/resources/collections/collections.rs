use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CollectionsClient {
    pub http_client: HttpClient,
}

impl CollectionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve all collection actions configured for your partner account.
    ///
    /// # Arguments
    ///
    /// * `order_by` - Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    /// * `q` - Query string for filtering. Format: "field:operator:value;...". Supported fields: id, name, action_type, is_active, timing. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_collection_actions(
        &self,
        request: &ListCollectionActionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseCollectionActionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/collection-actions",
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

    /// Retrieve collection action logs for your partner account. Supports filtering by client, loan, installment, status, or action type.
    ///
    /// # Arguments
    ///
    /// * `order_by` - Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    /// * `q` - Query string for filtering. Format: "field:operator:value;...". Supported fields: id, collection_action_id, action_type, status, client_id, loan_id, installment_id, scheduled_for. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_collection_action_logs(
        &self,
        request: &ListCollectionActionLogsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseCollectionActionLogResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/collection-actions/logs",
                None,
                QueryBuilder::new()
                    .serialize("client_id", request.client_id.clone())
                    .serialize("loan_id", request.loan_id.clone())
                    .serialize("installment_id", request.installment_id.clone())
                    .serialize("status", request.status.clone())
                    .serialize("action_type", request.action_type.clone())
                    .serialize("page", request.page.clone())
                    .serialize("page_size", request.page_size.clone())
                    .serialize("order_by", request.order_by.clone())
                    .serialize("q", request.q.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Update the status and notes of a collection action log.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_collection_action_log(
        &self,
        log_id: &str,
        request: &CollectionActionLogUpdatePayload,
        options: Option<RequestOptions>,
    ) -> Result<CollectionActionLogResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v2/collection-actions/logs/{}", log_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
