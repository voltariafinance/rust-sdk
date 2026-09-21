use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TasksClient {
    pub http_client: HttpClient,
}

impl TasksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Paginated list of the tasks shared with your partner account, optionally filtered by status or by the client, loan, installment or waterfall they relate to.
    ///
    /// # Arguments
    ///
    /// * `status` - Filter by task status.
    /// * `client_id` - Filter by client.
    /// * `loan_id` - Filter by loan.
    /// * `installment_id` - Filter by installment.
    /// * `waterfall_id` - Filter by waterfall.
    /// * `order_by` - Field to order the results by, e.g., 'due_at:asc,created_at:desc'.
    /// * `q` - Query string for filtering. Format: "field:operator:value;...". Supported fields: id, status, priority, due_at, created_at, client_id, loan_id, installment_id, waterfall_id. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_tasks(
        &self,
        request: &ListTasksQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseTaskPartnerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/tasks",
                None,
                QueryBuilder::new()
                    .serialize("status", request.status.clone())
                    .serialize("client_id", request.client_id.clone())
                    .serialize("loan_id", request.loan_id.clone())
                    .serialize("installment_id", request.installment_id.clone())
                    .serialize("waterfall_id", request.waterfall_id.clone())
                    .int("page", request.page.clone())
                    .serialize("page_size", request.page_size.clone())
                    .serialize("order_by", request.order_by.clone())
                    .serialize("q", request.q.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Open a task for your partner account. Any entity you link to it must belong to you.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_task(
        &self,
        request: &TaskPartnerCreatePayload,
        options: Option<RequestOptions>,
    ) -> Result<TaskPartnerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/tasks",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve one of your tasks by its ID.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_task(
        &self,
        task_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<TaskPartnerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/tasks/{}", task_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Move one of your tasks to another status. Status is the only field you can change. Requires a signed-in user — API keys cannot change a task.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_task_status(
        &self,
        task_id: &str,
        request: &TaskPartnerStatusUpdatePayload,
        options: Option<RequestOptions>,
    ) -> Result<TaskPartnerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v2/tasks/{}/status", task_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// The status transitions of one of your tasks, and whether each one was made by your team or by Voltaria support.
    ///
    /// # Arguments
    ///
    /// * `order_by` - Field to order the results by, e.g., 'created_at:asc'. Defaults to 'created_at:desc'.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_task_status_history(
        &self,
        task_id: &str,
        request: &ListTaskStatusHistoryQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseTaskPartnerStatusHistoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/tasks/{}/status-history", task_id),
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .serialize("page_size", request.page_size.clone())
                    .serialize("order_by", request.order_by.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Notes exchanged with Voltaria on one of your tasks.
    ///
    /// # Arguments
    ///
    /// * `order_by` - Field to order the results by, e.g., 'created_at:desc'. Defaults to 'created_at:desc'.
    /// * `q` - Query string for filtering. Format: "field:operator:value;...". Supported fields: id, loan_id, installment_id, created_at. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_task_notes(
        &self,
        task_id: &str,
        request: &ListTaskNotesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseNoteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/tasks/{}/notes", task_id),
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .serialize("page_size", request.page_size.clone())
                    .serialize("order_by", request.order_by.clone())
                    .serialize("q", request.q.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add a note to one of your tasks. Requires a signed-in user — API keys cannot write notes, because a note needs an author.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_task_note(
        &self,
        task_id: &str,
        request: &TaskNoteCreatePayload,
        options: Option<RequestOptions>,
    ) -> Result<NoteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/tasks/{}/notes", task_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
