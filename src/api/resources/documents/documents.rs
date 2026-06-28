use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DocumentsClient {
    pub http_client: HttpClient,
}

impl DocumentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve all documents linked to a client.
    ///
    /// # Arguments
    ///
    /// * `order_by` - Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    /// * `q` - Query string for filtering. Format: "field:operator:value;...". Supported fields: id, client_id, loan_id, installment_id, waterfall_id, category, file_name, document_date, folder_path. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_documents(
        &self,
        request: &ListDocumentsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseDocumentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/documents",
                None,
                QueryBuilder::new()
                    .serialize("client_id", request.client_id.clone())
                    .serialize("loan_id", request.loan_id.clone())
                    .serialize("installment_id", request.installment_id.clone())
                    .serialize("waterfall_id", request.waterfall_id.clone())
                    .serialize("page", request.page.clone())
                    .serialize("page_size", request.page_size.clone())
                    .serialize("order_by", request.order_by.clone())
                    .serialize("q", request.q.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Upload a new document related to a client or loan, such as financial statements or KYC files.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn upload_document(
        &self,
        request: &UploadDocumentRequest,
        options: Option<RequestOptions>,
    ) -> Result<DocumentResponse, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v2/documents",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .serialize("client_id", request.client_id.clone())
                    .serialize("loan_id", request.loan_id.clone())
                    .serialize("installment_id", request.installment_id.clone())
                    .serialize("waterfall_id", request.waterfall_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve all available document categories.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_available_document_categories(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<AvailableDocumentCategoriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/documents/available-categories",
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieve details for a specific document using its document ID.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_document_by_id(
        &self,
        document_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DocumentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/documents/{}", document_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a specific document by using its document ID.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    pub async fn delete_document(
        &self,
        document_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v2/documents/{}", document_id),
                None,
                None,
                options,
            )
            .await
    }
}
