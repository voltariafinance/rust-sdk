use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PartnersClient {
    pub http_client: HttpClient,
}

impl PartnersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Use this endpoint to check the available funding capacity in your dedicated lending account before initiating a new loan or submitting a drawdown request.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_available_funding(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<AvailableFunding>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/partners/available-funding",
                None,
                None,
                options,
            )
            .await
    }

    /// Upload supplementary partner information, such as bank account balance, accounting figures, or other relevant details.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_partner_data(
        &self,
        request: &PartnerDataCreatePayload,
        options: Option<RequestOptions>,
    ) -> Result<PartnerDataResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/partners/data",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Use this endpoint to get the list of waterfalls for your dedicated lending account.
    ///
    /// # Arguments
    ///
    /// * `order_by` - Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    /// * `q` - Query string for filtering. Format: "field:operator:value;...". Supported fields: id, name, date, status, created_at, updated_at. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_partner_waterfalls(
        &self,
        request: &ListPartnerWaterfallsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseWaterfallResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/partners/waterfalls",
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
