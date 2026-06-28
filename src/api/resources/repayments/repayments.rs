use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct RepaymentsClient {
    pub http_client: HttpClient,
}

impl RepaymentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve all repayments made under your partner account. Supports filtering by client, loan, or installments.
    ///
    /// # Arguments
    ///
    /// * `order_by` - Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    /// * `q` - Query string for filtering. Format: "field:operator:value;...". Supported fields: id, client_id, loan_id, installment_id, created_at, client.name, client.correlation_id. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_repayment_logs(
        &self,
        request: &ListRepaymentLogsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseRepaymentResponseWithClientInfo, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/repayments",
                None,
                QueryBuilder::new()
                    .serialize("client_id", request.client_id.clone())
                    .serialize("loan_id", request.loan_id.clone())
                    .serialize("installment_id", request.installment_id.clone())
                    .serialize("page", request.page.clone())
                    .serialize("page_size", request.page_size.clone())
                    .serialize("order_by", request.order_by.clone())
                    .serialize("q", request.q.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new repayment log for an installment. Requires the installment ID, loan ID or loan correlation ID.
    ///
    /// # Arguments
    ///
    /// * `installment_id` - ID of the installment
    /// * `loan_id` - ID of the associated Loan
    /// * `correlation_id` - Correlation ID of associated loan
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_repayment(
        &self,
        request: &RepaymentCreatePayload,
        options: Option<RequestOptions>,
    ) -> Result<RepaymentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/repayments",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("installment_id", request.installment_id.clone())
                    .serialize("loan_id", request.loan_id.clone())
                    .serialize("correlation_id", request.correlation_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Initiate processing of up to 10000 repayment logs. Returns task_id for tracking progress.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_bulk_repayments(
        &self,
        request: &BulkRepaymentCreatePayload,
        options: Option<RequestOptions>,
    ) -> Result<BulkRepaymentTaskResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/repayments/bulk",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Check the progress and results of a bulk repayment processing task.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_bulk_repayment_status(
        &self,
        task_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<BulkRepaymentTaskStatus, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/repayments/bulk/{}", task_id),
                None,
                None,
                options,
            )
            .await
    }
}
