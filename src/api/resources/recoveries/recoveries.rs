use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct RecoveriesClient {
    pub http_client: HttpClient,
}

impl RecoveriesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve recoveries recorded against your loans. Supports filtering by client or loan.
    ///
    /// # Arguments
    ///
    /// * `order_by` - Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    /// * `q` - Query string for filtering. Format: "field:operator:value;...". Supported fields: id, client_id, loan_id, currency, recovery_date, created_at. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_recoveries(
        &self,
        request: &ListRecoveriesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseRecoveryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/recoveries",
                None,
                QueryBuilder::new()
                    .serialize("client_id", request.client_id.clone())
                    .serialize("loan_id", request.loan_id.clone())
                    .serialize("page", request.page.clone())
                    .serialize("page_size", request.page_size.clone())
                    .serialize("order_by", request.order_by.clone())
                    .serialize("q", request.q.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Record a new recovery against one of your loans.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_recovery(
        &self,
        request: &RecoveryCreatePayload,
        options: Option<RequestOptions>,
    ) -> Result<RecoveryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/recoveries",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
