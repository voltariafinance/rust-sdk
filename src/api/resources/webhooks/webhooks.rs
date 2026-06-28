use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct WebhooksClient {
    pub http_client: HttpClient,
}

impl WebhooksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List all webhook subscriptions for your partner account.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_webhook_subscriptions(
        &self,
        request: &ListWebhookSubscriptionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseWebhookSubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/webhooks/subscriptions",
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .serialize("page_size", request.page_size.clone())
                    .serialize("event_type", request.event_type.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new webhook subscription for a specific event type.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_webhook_subscription(
        &self,
        request: &WebhookCreatePayload,
        options: Option<RequestOptions>,
    ) -> Result<WebhookSubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/webhooks/subscriptions",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve details for a specific webhook subscription with its webhook ID.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_webhook_subscription(
        &self,
        webhook_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WebhookSubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/webhooks/subscriptions/{}", webhook_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a webhook subscription with its specific webhook ID.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_webhook_subscription(
        &self,
        webhook_id: &str,
        request: &WebhookUpdatePayload,
        options: Option<RequestOptions>,
    ) -> Result<WebhookSubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v2/webhooks/subscriptions/{}", webhook_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a specific webhook subscription.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_webhook_subscription(
        &self,
        webhook_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v2/webhooks/subscriptions/{}", webhook_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieve all webhook logs linked to your partner account.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_webhook_logs(
        &self,
        request: &ListWebhookLogsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResponseWebhookLogResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/webhooks/logs",
                None,
                QueryBuilder::new()
                    .serialize("webhook_id", request.webhook_id.clone())
                    .int("page", request.page.clone())
                    .serialize("page_size", request.page_size.clone())
                    .build(),
                options,
            )
            .await
    }
}
