use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct InstallmentsClient {
    pub http_client: HttpClient,
}

impl InstallmentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a list of all loan installments under your partner account. Supports optional filtering by loan or client.
    ///
    /// # Arguments
    ///
    /// * `order_by` - Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    /// * `q` - Query string for filtering. Format: "field:operator:value;...". Supported fields: id, client_id, loan_id, status, client.name, expected_repayment_at. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_installments(
        &self,
        request: &ListInstallmentsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseInstallmentResponseWithClientInfo, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/installments",
                None,
                QueryBuilder::new()
                    .serialize("page", request.page.clone())
                    .serialize("page_size", request.page_size.clone())
                    .serialize("client_id", request.client_id.clone())
                    .serialize("loan_id", request.loan_id.clone())
                    .serialize("order_by", request.order_by.clone())
                    .serialize("q", request.q.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add new installments to a loan with its specific loan ID. This endpoint is available to select partners and will trigger the recalculation of the IRR and interest amounts for all installments of the loan.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_installment(
        &self,
        request: &InstallmentCreatePayload,
        options: Option<RequestOptions>,
    ) -> Result<Vec<InstallmentResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/installments",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve a list of payment promises recorded for installments under your partner account. Supports optional filtering by loan or client.
    ///
    /// # Arguments
    ///
    /// * `order_by` - Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    /// * `q` - Query string for filtering. Format: "field:operator:value;...". Supported fields: id, installment_id, status, promised_date, created_at. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_payment_promises(
        &self,
        request: &ListPaymentPromisesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponsePaymentPromiseResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/installments/payment-promises",
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .serialize("page_size", request.page_size.clone())
                    .serialize("order_by", request.order_by.clone())
                    .serialize("q", request.q.clone())
                    .serialize("loan_id", request.loan_id.clone())
                    .serialize("client_id", request.client_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Record a payment promise made by a client for one of their installments. The promised date must be today or in the future.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_payment_promise(
        &self,
        request: &PaymentPromiseCreatePayload,
        options: Option<RequestOptions>,
    ) -> Result<PaymentPromiseResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/installments/payment-promises",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve detailed information for a specific installment using its installment ID.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_installment_by_id(
        &self,
        installment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<InstallmentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/installments/{}", installment_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an installment's amount and expected repayment date with its specific installment ID. This endpoint is available to select partners and will trigger the recalculation of the IRR and interest amounts for all installments of the loan.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn edit_installment(
        &self,
        installment_id: &str,
        request: &InstallmentEditPayload,
        options: Option<RequestOptions>,
    ) -> Result<InstallmentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v2/installments/{}", installment_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete an installment with its specific installment ID. This endpoint is available to select partners and will trigger the recalculation of the IRR and interest amounts for all installments of the loan.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_installment(
        &self,
        installment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v2/installments/{}", installment_id),
                None,
                None,
                options,
            )
            .await
    }
}
