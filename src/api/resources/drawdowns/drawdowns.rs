use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DrawdownsClient {
    pub http_client: HttpClient,
}

impl DrawdownsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a list of drawdowns.
    ///
    /// # Arguments
    ///
    /// * `order_by` - Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    /// * `q` - Query string for filtering. Format: "field:operator:value;...". Supported fields: . Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_drawdowns(
        &self,
        request: &ListDrawdownsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseDrawdownResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/drawdowns",
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

    /// Create a new drawdown request.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_drawdown_request(
        &self,
        request: &DrawdownCreatePayload,
        options: Option<RequestOptions>,
    ) -> Result<DrawdownResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/drawdowns",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve all checklist items for a specific drawdown
    ///
    /// # Arguments
    ///
    /// * `order_by` - Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    /// * `q` - Query string for filtering. Format: "field:operator:value;...". Supported fields: . Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_drawdown_checklists(
        &self,
        drawdown_id: &str,
        request: &ListDrawdownChecklistsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseDrawdownChecklistResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/drawdowns/{}/checklists", drawdown_id),
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
}
