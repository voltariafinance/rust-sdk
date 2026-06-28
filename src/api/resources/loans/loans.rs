use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct LoansClient {
    pub http_client: HttpClient,
}

impl LoansClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve all loans associated with your partner account. Supports optional filtering by client ID.
    ///
    /// # Arguments
    ///
    /// * `order_by` - Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    /// * `q` - Query string for filtering. Format: "field:operator:value;...". Supported fields: id, client_id, status, client.name, correlation_id. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_loans(
        &self,
        request: &ListLoansQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseLoanResponseWithClientInfo, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/loans",
                None,
                QueryBuilder::new()
                    .serialize("page", request.page.clone())
                    .serialize("page_size", request.page_size.clone())
                    .serialize("client_id", request.client_id.clone())
                    .serialize("order_by", request.order_by.clone())
                    .serialize("q", request.q.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new loan for an approved client with an active credit limit.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_loan(
        &self,
        request: &LoanCreatePayload,
        options: Option<RequestOptions>,
    ) -> Result<LoanResponseWithInstallments, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/loans",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve detailed information about a specific loan by its loan ID.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_loan_by_id(
        &self,
        loan_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<LoanResponseWithInstallments, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/loans/{}", loan_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a loan by ID. Only loans with 'pending' status can be deleted.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_loan(
        &self,
        loan_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Option<HashMap<String, serde_json::Value>>, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v2/loans/{}", loan_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Create multiple loans in a single request. Processing happens asynchronously. Returns a task ID for tracking progress.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_bulk_loans(
        &self,
        request: &BulkLoanCreatePayload,
        options: Option<RequestOptions>,
    ) -> Result<BulkLoanTaskResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/loans/bulk",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Check the status of a bulk loan creation task and retrieve results when completed.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_bulk_loan_status(
        &self,
        task_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<BulkLoanTaskStatus, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/loans/bulk/{}", task_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Mark a loan as defaulted, recording the default date and the amount recovered from selling it. Defaults the loan's active and overdue installments and updates the loan status accordingly.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn set_loan_default(
        &self,
        loan_id: &str,
        request: &LoanDefaultPayload,
        options: Option<RequestOptions>,
    ) -> Result<LoanResponseWithInstallments, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/loans/{}/set-default", loan_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
