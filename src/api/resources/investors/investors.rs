use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct InvestorsClient {
    pub http_client: HttpClient,
}

impl InvestorsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve all clients with at least one loan funded by this investor.
    ///
    /// # Arguments
    ///
    /// * `q` - Query string for filtering. Format: "field:operator:value;...". Supported fields: id, name, correlation_id, company_number, status. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn investor_list_clients(
        &self,
        request: &InvestorListClientsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseClientInvestorResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/investors/clients",
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

    /// Retrieve a specific client that has a loan funded by this investor.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn investor_get_client(
        &self,
        client_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ClientInvestorResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/investors/clients/{}", client_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieve all loans funded by the current investor.
    ///
    /// # Arguments
    ///
    /// * `q` - Query string for filtering. Format: "field:operator:value;...". Supported fields: id, partner_id, client_id, status, loan_date, currency, partner.name, client.name. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn investor_list_loans(
        &self,
        request: &InvestorListLoansQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseLoanInvestorResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/investors/loans",
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

    /// Retrieve a specific loan funded by the current investor, with its installments.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn investor_get_loan(
        &self,
        loan_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<LoanResponseWithInstallments, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/investors/loans/{}", loan_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieve all installments for loans funded by the current investor.
    ///
    /// # Arguments
    ///
    /// * `q` - Query string for filtering. Format: "field:operator:value;...". Supported fields: id, client_id, loan_id, status, client.name, expected_repayment_at. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn investor_list_installments(
        &self,
        request: &InvestorListInstallmentsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseInstallmentResponseWithClientInfo, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/investors/installments",
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

    /// Retrieve a specific installment for a loan funded by the current investor.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn investor_get_installment(
        &self,
        installment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<InstallmentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/investors/installments/{}", installment_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieve all repayment logs for loans funded by the current investor.
    ///
    /// # Arguments
    ///
    /// * `q` - Query string for filtering. Format: "field:operator:value;...". Supported fields: id, client_id, loan_id, installment_id, created_at, client.name, client.correlation_id. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn investor_list_repayments(
        &self,
        request: &InvestorListRepaymentsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseRepaymentResponseWithClientInfo, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/investors/repayments",
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
}
