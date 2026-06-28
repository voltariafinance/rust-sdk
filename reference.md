# Reference
## Clients
<details><summary><code>client.clients.<a href="/src/api/resources/clients/client.rs">list_clients</a>(page: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, page_size: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, order_by: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, q: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;PaginatedResponseClientResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of all clients associated with your partner account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .clients
        .list_clients(
            &ListClientsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**order_by:** `Option<Option<String>>` — Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    
</dd>
</dl>

<dl>
<dd>

**q:** `Option<Option<String>>` — Query string for filtering. Format: "field:operator:value;...". Supported fields: id, name, correlation_id, company_number, status. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.clients.<a href="/src/api/resources/clients/client.rs">create_client</a>(request: ClientCreatePayload) -> Result&lt;ClientResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new client under your partner account. The client will remain in a pending state until reviewed by Winyield.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .clients
        .create_client(
            &ClientCreatePayload {
                name: "ACME Corp".to_string(),
                jurisdiction: JurisdictionEnum::Eu,
                correlation_id: None,
                r#type: None,
                company_number: None,
                data: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**correlation_id:** `Option<Option<String>>` — The correlation ID you provided at the creation of the client
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — The name of the client
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<Option<ClientTypeEnum>>` — The type of the client, must be one of `individual`,`corporate`. Default is `corporate` if not provided.
    
</dd>
</dl>

<dl>
<dd>

**jurisdiction:** `JurisdictionEnum` — The jurisdiction of the client, must be one of `eu`, `us`, `uk`
    
</dd>
</dl>

<dl>
<dd>

**company_number:** `Option<Option<String>>` — The company number of the client if type is `corporate`
    
</dd>
</dl>

<dl>
<dd>

**data:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>` — Additional data associated with the client
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.clients.<a href="/src/api/resources/clients/client.rs">create_client_data</a>(request: ClientDataCreatePayload) -> Result&lt;ClientDataResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Upload supplementary client information, such as bank account balance, accounting figures, or other relevant details.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .clients
        .create_client_data(
            &ClientDataCreatePayload {
                client_id: "client_123".to_string(),
                data: HashMap::from([
                    ("key1".to_string(), serde_json::json!("value1")),
                    ("key2".to_string(), serde_json::json!("value2")),
                ]),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**data:** `std::collections::HashMap<String, serde_json::Value>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.clients.<a href="/src/api/resources/clients/client.rs">list_limit_requests</a>(client_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, page: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, page_size: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, order_by: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, q: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;PaginatedResponseLimitRequestResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of all limit requests associated with your partner account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .clients
        .list_limit_requests(
            &ListLimitRequestsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `Option<Option<String>>` — Filter by client ID
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**order_by:** `Option<Option<String>>` — Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    
</dd>
</dl>

<dl>
<dd>

**q:** `Option<Option<String>>` — Query string for filtering. Format: "field:operator:value;...". Supported fields: id, client_id. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.clients.<a href="/src/api/resources/clients/client.rs">create_limit_request</a>(request: LimitRequestCreatePayload) -> Result&lt;LimitRequestResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a limit review request for a client. The request will remain in a pending state until reviewed by Winyield.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .clients
        .create_limit_request(
            &LimitRequestCreatePayload {
                client_id: "client_1234567890abcdef".to_string(),
                requested_limit: LimitRequestCreatePayloadRequestedLimit::Double(1.1),
                reason: "Need more credit for business expansion".to_string(),
                waiver_request: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `String` — The ID of the client for which the limit request is being created
    
</dd>
</dl>

<dl>
<dd>

**requested_limit:** `LimitRequestCreatePayloadRequestedLimit` — The requested credit limit amount
    
</dd>
</dl>

<dl>
<dd>

**reason:** `String` — The reason for the limit request
    
</dd>
</dl>

<dl>
<dd>

**waiver_request:** `Option<bool>` — Whether a special waiver is requested alongside this limit request
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.clients.<a href="/src/api/resources/clients/client.rs">get_limit_request</a>(request_id: String) -> Result&lt;LimitRequestResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a specific limit request by its ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .clients
        .get_limit_request(&"request_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**request_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.clients.<a href="/src/api/resources/clients/client.rs">list_onboarding_clients</a>(page: Option&lt;Option&lt;i64&gt;&gt;, page_size: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;PaginatedResponseClientResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve all clients that have self-registered via the portal and are awaiting partner approval.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .clients
        .list_onboarding_clients(
            &ListOnboardingClientsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.clients.<a href="/src/api/resources/clients/client.rs">approve_onboarding</a>(client_id: String) -> Result&lt;ClientResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Accept a self-onboarded client. The client status moves to 'pending' and the owner's portal account is activated so they can begin submitting documents.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .clients
        .approve_onboarding(&"client_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.clients.<a href="/src/api/resources/clients/client.rs">reject_onboarding</a>(client_id: String) -> Result&lt;ClientResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Reject a self-onboarded client. The client record is kept with 'rejected' status for audit history and is not deleted.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .clients
        .reject_onboarding(&"client_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.clients.<a href="/src/api/resources/clients/client.rs">add_client_portal_user</a>(client_id: String, request: ClientUserInviteRequest) -> Result&lt;ClientUserResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Invite a new user to a client's portal account. The invited user will receive an email with a one-time link to set their password. Partner can assign any role: 'owner', 'admin', or 'viewer'.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .clients
        .add_client_portal_user(
            &"client_id".to_string(),
            &ClientUserInviteRequest {
                first_name: "first_name".to_string(),
                last_name: "last_name".to_string(),
                email: "email".to_string(),
                role_type: "role_type".to_string(),
                phone: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**first_name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**last_name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**email:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**phone:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**role_type:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.clients.<a href="/src/api/resources/clients/client.rs">list_client_waivers</a>(client_id: String, page: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, page_size: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, order_by: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, q: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;PaginatedResponseWaiverResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve all waivers associated with a specific client.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .clients
        .list_client_waivers(
            &"client_id".to_string(),
            &ListClientWaiversQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**order_by:** `Option<Option<String>>` — Field to order the results by, e.g., 'created_at:desc'
    
</dd>
</dl>

<dl>
<dd>

**q:** `Option<Option<String>>` — Query string for filtering. Format: "field:operator:value;...". Supported fields: id, client_id, status. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.clients.<a href="/src/api/resources/clients/client.rs">get_client_by_id</a>(client_id: String) -> Result&lt;ClientResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve detailed information for a specific client using their client ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .clients
        .get_client_by_id(&"client_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.clients.<a href="/src/api/resources/clients/client.rs">delete_client</a>(client_id: String) -> Result&lt;Option&lt;std::collections::HashMap&lt;String, serde_json::Value&gt;&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a client by ID. Only clients with 'pending' status can be deleted. All client's loans must also be in 'pending' status.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .clients
        .delete_client(&"client_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.clients.<a href="/src/api/resources/clients/client.rs">list_client_checklist_summaries</a>(client_id: String, page: Option&lt;Option&lt;i64&gt;&gt;, page_size: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;PaginatedResponseChecklistSummaryPartnerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the checklist summaries for one of your clients, including the AI description and item completion counts.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .clients
        .list_client_checklist_summaries(
            &"client_id".to_string(),
            &ListClientChecklistSummariesQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Sandbox
<details><summary><code>client.sandbox.<a href="/src/api/resources/sandbox/client.rs">update_client</a>(client_id: String, request: ClientUpdateSandbox) -> Result&lt;ClientResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update an existing client's status or credit limit using their client ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sandbox
        .update_client(
            &"client_id".to_string(),
            &ClientUpdateSandbox {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<Option<ClientStatusEnum>>` — The status of the client. One of the following: `active, rejected, deactivated, pending, pending_onboarding, pre_approved, deleted, inactive`
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Option<ClientUpdateSandboxLimit>>` — The limit to set for the client. This will override the existing limit.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sandbox.<a href="/src/api/resources/sandbox/client.rs">update_loan</a>(loan_id: String, request: LoanUpdateSandbox) -> Result&lt;LoanResponseWithInstallments, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update the status of a specific loan using its unique loan ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sandbox
        .update_loan(
            &"loan_id".to_string(),
            &LoanUpdateSandbox {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**loan_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<Option<LoanStatusEnum>>` — The status of the client. One of the following: `pending, overdue, active, default, sold, restructured, repaid, pre_approved, rejected, deleted, inactive`
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sandbox.<a href="/src/api/resources/sandbox/client.rs">webhook_test</a>(request: WebhookTestSandbox) -> Result&lt;std::collections::HashMap&lt;String, serde_json::Value&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Test a webhook subscription by ID or trigger all by event type.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sandbox
        .webhook_test(
            &WebhookTestSandbox {
                event_type: WebhookEventTypeEnum::LoanUpdated,
                webhook_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**webhook_id:** `Option<Option<String>>` — The ID of the webhook subscription. Only this webhook will be triggered if provided.
    
</dd>
</dl>

<dl>
<dd>

**event_type:** `WebhookEventTypeEnum` — Event type to trigger for the test. All subscriptions for this event type will be triggered if webhook_id is not provided.Possible values: loan_updated, installment_updated, client_updated, client_limit_updated, partner_limit_updated
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Accounts
<details><summary><code>client.accounts.<a href="/src/api/resources/accounts/client.rs">list_client_account_fields</a>(client_id: String) -> Result&lt;std::collections::HashMap&lt;String, CurrencyFieldSpec&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Return the required and optional bank account fields for each supported currency. Fetch once and cache; use it to build the create-account request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .accounts
        .list_client_account_fields(&"client_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.accounts.<a href="/src/api/resources/accounts/client.rs">list_client_accounts</a>(client_id: String, page: Option&lt;Option&lt;i64&gt;&gt;, page_size: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, order_by: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;PaginatedResponseClientAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve all bank accounts for one of your clients.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .accounts
        .list_client_accounts(
            &"client_id".to_string(),
            &ListClientAccountsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**order_by:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.accounts.<a href="/src/api/resources/accounts/client.rs">create_client_account</a>(client_id: String, request: PartnerClientAccountCreateRequest) -> Result&lt;ClientAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a bank account for one of your clients. The account is registered with the payment provider immediately. Use the `status` field to create it as `active` (default; demotes any existing active account in the same currency to `passive`) or `passive`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .accounts
        .create_client_account(
            &"client_id".to_string(),
            &PartnerClientAccountCreateRequest {
                account_holder_name: "Acme Ltd".to_string(),
                account_holder_type: AccountHolderTypeEnum::Business,
                currency: CurrencyEnum::Gbp,
                sort_code: Some("40-47-84".to_string()),
                account_number: Some("12345678".to_string()),
                label: None,
                iban: None,
                bic: None,
                routing_number: None,
                account_type: None,
                address: None,
                status: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**account_holder_name:** `String` — Full name of the account holder.
    
</dd>
</dl>

<dl>
<dd>

**label:** `Option<Option<String>>` — Optional label / nickname for the account (max 50 characters).
    
</dd>
</dl>

<dl>
<dd>

**account_holder_type:** `AccountHolderTypeEnum` — Account holder type. One of: `business`, `private`.
    
</dd>
</dl>

<dl>
<dd>

**currency:** `CurrencyEnum` — ISO 4217 currency code. Use `/accounts/fields` to get required fields per currency.
    
</dd>
</dl>

<dl>
<dd>

**sort_code:** `Option<Option<String>>` — Sort code (required for GBP).
    
</dd>
</dl>

<dl>
<dd>

**account_number:** `Option<Option<String>>` — Account number (required for GBP and USD).
    
</dd>
</dl>

<dl>
<dd>

**iban:** `Option<Option<String>>` — IBAN (required for EUR, CZK, PLN).
    
</dd>
</dl>

<dl>
<dd>

**bic:** `Option<Option<String>>` — BIC / SWIFT code (optional for EUR).
    
</dd>
</dl>

<dl>
<dd>

**routing_number:** `Option<Option<String>>` — ABA routing number (required for USD).
    
</dd>
</dl>

<dl>
<dd>

**account_type:** `Option<Option<String>>` — Account type (required for USD). E.g. `checking` or `savings`.
    
</dd>
</dl>

<dl>
<dd>

**address:** `Option<Option<AccountAddress>>` — Account holder address (required for USD).
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<PartnerClientAccountCreateRequestStatus>` — Account status. `active` demotes any existing active account in the same currency to `passive`; `passive` is added as a backup. Defaults to `active`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.accounts.<a href="/src/api/resources/accounts/client.rs">get_client_account</a>(client_id: String, account_id: String) -> Result&lt;ClientAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a specific bank account for one of your clients.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .accounts
        .get_client_account(&"client_id".to_string(), &"account_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Documents
<details><summary><code>client.documents.<a href="/src/api/resources/documents/client.rs">list_documents</a>(client_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, loan_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, installment_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, waterfall_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, page: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, page_size: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, order_by: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, q: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;PaginatedResponseDocumentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve all documents linked to a client.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .documents
        .list_documents(
            &ListDocumentsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**loan_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**installment_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**waterfall_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**order_by:** `Option<Option<String>>` — Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    
</dd>
</dl>

<dl>
<dd>

**q:** `Option<Option<String>>` — Query string for filtering. Format: "field:operator:value;...". Supported fields: id, client_id, loan_id, installment_id, waterfall_id, category, file_name, document_date, folder_path. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.documents.<a href="/src/api/resources/documents/client.rs">upload_document</a>(client_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, loan_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, installment_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, waterfall_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;DocumentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Upload a new document related to a client or loan, such as financial statements or KYC files.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .documents
        .upload_document(
            &UploadDocumentRequest {
                file: b"test file content".to_vec(),
                category: "category".to_string(),
                file_name: "file_name".to_string(),
                client_id: None,
                loan_id: None,
                installment_id: None,
                waterfall_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**loan_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**installment_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**waterfall_id:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.documents.<a href="/src/api/resources/documents/client.rs">get_available_document_categories</a>() -> Result&lt;AvailableDocumentCategoriesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve all available document categories.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .documents
        .get_available_document_categories(None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.documents.<a href="/src/api/resources/documents/client.rs">get_document_by_id</a>(document_id: String) -> Result&lt;DocumentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve details for a specific document using its document ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .documents
        .get_document_by_id(&"document_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**document_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.documents.<a href="/src/api/resources/documents/client.rs">delete_document</a>(document_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a specific document by using its document ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .documents
        .delete_document(&"document_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**document_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Investors
<details><summary><code>client.investors.<a href="/src/api/resources/investors/client.rs">investor_list_clients</a>(page: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, page_size: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, order_by: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, q: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;PaginatedResponseClientInvestorResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve all clients with at least one loan funded by this investor.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .investors
        .investor_list_clients(
            &InvestorListClientsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**order_by:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**q:** `Option<Option<String>>` — Query string for filtering. Format: "field:operator:value;...". Supported fields: id, name, correlation_id, company_number, status. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.investors.<a href="/src/api/resources/investors/client.rs">investor_get_client</a>(client_id: String) -> Result&lt;ClientInvestorResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a specific client that has a loan funded by this investor.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .investors
        .investor_get_client(&"client_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.investors.<a href="/src/api/resources/investors/client.rs">investor_list_loans</a>(page: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, page_size: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, client_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, order_by: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, q: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;PaginatedResponseLoanInvestorResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve all loans funded by the current investor.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .investors
        .investor_list_loans(
            &InvestorListLoansQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**client_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**order_by:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**q:** `Option<Option<String>>` — Query string for filtering. Format: "field:operator:value;...". Supported fields: id, partner_id, client_id, status, loan_date, currency, partner.name, client.name. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.investors.<a href="/src/api/resources/investors/client.rs">investor_get_loan</a>(loan_id: String) -> Result&lt;LoanResponseWithInstallments, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a specific loan funded by the current investor, with its installments.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .investors
        .investor_get_loan(&"loan_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**loan_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.investors.<a href="/src/api/resources/investors/client.rs">investor_list_installments</a>(page: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, page_size: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, client_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, loan_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, order_by: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, q: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;PaginatedResponseInstallmentResponseWithClientInfo, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve all installments for loans funded by the current investor.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .investors
        .investor_list_installments(
            &InvestorListInstallmentsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**client_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**loan_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**order_by:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**q:** `Option<Option<String>>` — Query string for filtering. Format: "field:operator:value;...". Supported fields: id, client_id, loan_id, status, client.name, expected_repayment_at. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.investors.<a href="/src/api/resources/investors/client.rs">investor_get_installment</a>(installment_id: String) -> Result&lt;InstallmentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a specific installment for a loan funded by the current investor.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .investors
        .investor_get_installment(&"installment_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**installment_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.investors.<a href="/src/api/resources/investors/client.rs">investor_list_repayments</a>(client_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, loan_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, installment_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, page: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, page_size: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, order_by: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, q: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;PaginatedResponseRepaymentResponseWithClientInfo, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve all repayment logs for loans funded by the current investor.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .investors
        .investor_list_repayments(
            &InvestorListRepaymentsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**loan_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**installment_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**order_by:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**q:** `Option<Option<String>>` — Query string for filtering. Format: "field:operator:value;...". Supported fields: id, client_id, loan_id, installment_id, created_at, client.name, client.correlation_id. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Installments
<details><summary><code>client.installments.<a href="/src/api/resources/installments/client.rs">list_installments</a>(page: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, page_size: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, client_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, loan_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, order_by: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, q: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;PaginatedResponseInstallmentResponseWithClientInfo, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of all loan installments under your partner account. Supports optional filtering by loan or client.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .installments
        .list_installments(
            &ListInstallmentsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**client_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**loan_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**order_by:** `Option<Option<String>>` — Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    
</dd>
</dl>

<dl>
<dd>

**q:** `Option<Option<String>>` — Query string for filtering. Format: "field:operator:value;...". Supported fields: id, client_id, loan_id, status, client.name, expected_repayment_at. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.installments.<a href="/src/api/resources/installments/client.rs">add_installment</a>(request: InstallmentCreatePayload) -> Result&lt;Vec&lt;InstallmentResponse&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add new installments to a loan with its specific loan ID. This endpoint is available to select partners and will trigger the recalculation of the IRR and interest amounts for all installments of the loan.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .installments
        .add_installment(
            &InstallmentCreatePayload {
                loan_id: "loan_12345".to_string(),
                installments: vec![
                    LoanInstallmentCreatePayload {
                        expected_repayment_at: NaiveDate::parse_from_str("2025-12-01", "%Y-%m-%d")
                            .unwrap(),
                        amount: LoanInstallmentCreatePayloadAmount::Double(1000.0),
                    },
                    LoanInstallmentCreatePayload {
                        expected_repayment_at: NaiveDate::parse_from_str("2026-01-01", "%Y-%m-%d")
                            .unwrap(),
                        amount: LoanInstallmentCreatePayloadAmount::Double(1000.0),
                    },
                ],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**loan_id:** `String` — The loan ID to add the installments to
    
</dd>
</dl>

<dl>
<dd>

**installments:** `Vec<LoanInstallmentCreatePayload>` — List of installments to add to the loan
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.installments.<a href="/src/api/resources/installments/client.rs">list_payment_promises</a>(page: Option&lt;Option&lt;i64&gt;&gt;, page_size: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, order_by: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, q: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, loan_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, client_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;PaginatedResponsePaymentPromiseResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of payment promises recorded for installments under your partner account. Supports optional filtering by loan or client.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .installments
        .list_payment_promises(
            &ListPaymentPromisesQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**order_by:** `Option<Option<String>>` — Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    
</dd>
</dl>

<dl>
<dd>

**q:** `Option<Option<String>>` — Query string for filtering. Format: "field:operator:value;...". Supported fields: id, installment_id, status, promised_date, created_at. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    
</dd>
</dl>

<dl>
<dd>

**loan_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**client_id:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.installments.<a href="/src/api/resources/installments/client.rs">create_payment_promise</a>(request: PaymentPromiseCreatePayload) -> Result&lt;PaymentPromiseResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Record a payment promise made by a client for one of their installments. The promised date must be today or in the future.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .installments
        .create_payment_promise(
            &PaymentPromiseCreatePayload {
                installment_id: "inst_12345".to_string(),
                amount: PaymentPromiseCreatePayloadAmount::Double(1.1),
                promised_date: NaiveDate::parse_from_str("2026-05-15", "%Y-%m-%d").unwrap(),
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**installment_id:** `String` — The ID of the installment this promise relates to
    
</dd>
</dl>

<dl>
<dd>

**amount:** `PaymentPromiseCreatePayloadAmount` — The amount the client has promised to pay (must be > 0)
    
</dd>
</dl>

<dl>
<dd>

**promised_date:** `String` — The date the client has committed to pay by (today or future)
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<Option<String>>` — Optional notes captured during the collections interaction
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.installments.<a href="/src/api/resources/installments/client.rs">get_installment_by_id</a>(installment_id: String) -> Result&lt;InstallmentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve detailed information for a specific installment using its installment ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .installments
        .get_installment_by_id(&"installment_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**installment_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.installments.<a href="/src/api/resources/installments/client.rs">edit_installment</a>(installment_id: String, request: InstallmentEditPayload) -> Result&lt;InstallmentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update an installment's amount and expected repayment date with its specific installment ID. This endpoint is available to select partners and will trigger the recalculation of the IRR and interest amounts for all installments of the loan.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .installments
        .edit_installment(
            &"installment_id".to_string(),
            &InstallmentEditPayload {
                amount: InstallmentEditPayloadAmount::Double(1.1),
                expected_repayment_at: NaiveDate::parse_from_str("2025-12-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**installment_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**amount:** `InstallmentEditPayloadAmount` — The new amount for the installment
    
</dd>
</dl>

<dl>
<dd>

**expected_repayment_at:** `String` — The new expected repayment date
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.installments.<a href="/src/api/resources/installments/client.rs">delete_installment</a>(installment_id: String) -> Result&lt;std::collections::HashMap&lt;String, serde_json::Value&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete an installment with its specific installment ID. This endpoint is available to select partners and will trigger the recalculation of the IRR and interest amounts for all installments of the loan.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .installments
        .delete_installment(&"installment_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**installment_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Loans
<details><summary><code>client.loans.<a href="/src/api/resources/loans/client.rs">list_loans</a>(page: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, page_size: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, client_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, order_by: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, q: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;PaginatedResponseLoanResponseWithClientInfo, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve all loans associated with your partner account. Supports optional filtering by client ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .loans
        .list_loans(
            &ListLoansQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**client_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**order_by:** `Option<Option<String>>` — Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    
</dd>
</dl>

<dl>
<dd>

**q:** `Option<Option<String>>` — Query string for filtering. Format: "field:operator:value;...". Supported fields: id, client_id, status, client.name, correlation_id. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.loans.<a href="/src/api/resources/loans/client.rs">create_loan</a>(request: LoanCreatePayload) -> Result&lt;LoanResponseWithInstallments, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new loan for an approved client with an active credit limit.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .loans
        .create_loan(
            &LoanCreatePayload {
                client_id: "client_ACME".to_string(),
                currency: CurrencyEnum::Eur,
                amount: LoanCreatePayloadAmount::Double(1.1),
                installments: vec![LoanInstallmentCreatePayload {
                    expected_repayment_at: NaiveDate::parse_from_str("2025-12-01", "%Y-%m-%d")
                        .unwrap(),
                    amount: LoanInstallmentCreatePayloadAmount::Double(1.1),
                }],
                correlation_id: None,
                loan_date: None,
                data: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `String` — The ID of the client for this loan
    
</dd>
</dl>

<dl>
<dd>

**currency:** `CurrencyEnum` — The currency of the loan, must be one of the supported currencies: eur, gbp, usd, czk, pln, isk
    
</dd>
</dl>

<dl>
<dd>

**amount:** `LoanCreatePayloadAmount` — The amount of the loan
    
</dd>
</dl>

<dl>
<dd>

**correlation_id:** `Option<Option<String>>` — The correlation ID you provided at the creation of the loan
    
</dd>
</dl>

<dl>
<dd>

**loan_date:** `Option<Option<String>>` — Please provide the loan_date if it differs from the loan creation time (created_at). Otherwise, it will be automatically set.
    
</dd>
</dl>

<dl>
<dd>

**installments:** `Vec<LoanInstallmentCreatePayload>` — List of installments for the loan, each with a due date and amount
    
</dd>
</dl>

<dl>
<dd>

**data:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>` — Additional data related to the loan
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.loans.<a href="/src/api/resources/loans/client.rs">get_loan_by_id</a>(loan_id: String) -> Result&lt;LoanResponseWithInstallments, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve detailed information about a specific loan by its loan ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .loans
        .get_loan_by_id(&"loan_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**loan_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.loans.<a href="/src/api/resources/loans/client.rs">delete_loan</a>(loan_id: String) -> Result&lt;Option&lt;std::collections::HashMap&lt;String, serde_json::Value&gt;&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a loan by ID. Only loans with 'pending' status can be deleted.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.loans.delete_loan(&"loan_id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**loan_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.loans.<a href="/src/api/resources/loans/client.rs">create_bulk_loans</a>(request: BulkLoanCreatePayload) -> Result&lt;BulkLoanTaskResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create multiple loans in a single request. Processing happens asynchronously. Returns a task ID for tracking progress.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .loans
        .create_bulk_loans(
            &BulkLoanCreatePayload {
                loans: vec![BulkLoanItemPayload {
                    client_id: "client_123".to_string(),
                    currency: CurrencyEnum::Eur,
                    amount: BulkLoanItemPayloadAmount::Double(50000.0),
                    correlation_id: Some("LOAN_001".to_string()),
                    loan_date: Some(NaiveDate::parse_from_str("2023-05-01", "%Y-%m-%d").unwrap()),
                    installments: vec![
                        LoanInstallmentCreatePayload {
                            expected_repayment_at: NaiveDate::parse_from_str(
                                "2023-06-01",
                                "%Y-%m-%d",
                            )
                            .unwrap(),
                            amount: LoanInstallmentCreatePayloadAmount::Double(26000.0),
                        },
                        LoanInstallmentCreatePayload {
                            expected_repayment_at: NaiveDate::parse_from_str(
                                "2023-07-01",
                                "%Y-%m-%d",
                            )
                            .unwrap(),
                            amount: LoanInstallmentCreatePayloadAmount::Double(26000.0),
                        },
                    ],
                    data: Some(HashMap::from([
                        ("loan_type".to_string(), serde_json::json!("business")),
                        ("purpose".to_string(), serde_json::json!("expansion")),
                    ])),
                }],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**loans:** `Vec<BulkLoanItemPayload>` — List of loans to create (max 1000)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.loans.<a href="/src/api/resources/loans/client.rs">get_bulk_loan_status</a>(task_id: String) -> Result&lt;BulkLoanTaskStatus, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Check the status of a bulk loan creation task and retrieve results when completed.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .loans
        .get_bulk_loan_status(&"task_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.loans.<a href="/src/api/resources/loans/client.rs">set_loan_default</a>(loan_id: String, request: LoanDefaultPayload) -> Result&lt;LoanResponseWithInstallments, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Mark a loan as defaulted, recording the default date and the amount recovered from selling it. Defaults the loan's active and overdue installments and updates the loan status accordingly.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .loans
        .set_loan_default(
            &"loan_id".to_string(),
            &LoanDefaultPayload {
                default_date: NaiveDate::parse_from_str("2026-06-23", "%Y-%m-%d").unwrap(),
                sold_amount: LoanDefaultPayloadSoldAmount::Double(1.1),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**loan_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**default_date:** `String` — Date the loan is marked as defaulted.
    
</dd>
</dl>

<dl>
<dd>

**sold_amount:** `LoanDefaultPayloadSoldAmount` — Amount recovered when the defaulted loan is sold.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Partners
<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">get_available_funding</a>() -> Result&lt;Vec&lt;AvailableFunding&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Use this endpoint to check the available funding capacity in your dedicated lending account before initiating a new loan or submitting a drawdown request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.partners.get_available_funding(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">create_partner_data</a>(request: PartnerDataCreatePayload) -> Result&lt;PartnerDataResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Upload supplementary partner information, such as bank account balance, accounting figures, or other relevant details.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .create_partner_data(
            &PartnerDataCreatePayload {
                data: HashMap::from([
                    ("key1".to_string(), serde_json::json!("value1")),
                    ("key2".to_string(), serde_json::json!("value2")),
                ]),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**data:** `std::collections::HashMap<String, serde_json::Value>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">list_partner_waterfalls</a>(page: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, page_size: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, order_by: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, q: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;PaginatedResponseWaterfallResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Use this endpoint to get the list of waterfalls for your dedicated lending account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .list_partner_waterfalls(
            &ListPartnerWaterfallsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**order_by:** `Option<Option<String>>` — Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    
</dd>
</dl>

<dl>
<dd>

**q:** `Option<Option<String>>` — Query string for filtering. Format: "field:operator:value;...". Supported fields: id, name, date, status, created_at, updated_at. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Webhooks
<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">list_webhook_subscriptions</a>(page: Option&lt;Option&lt;i64&gt;&gt;, page_size: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, event_type: Option&lt;Option&lt;Option&lt;WebhookEventTypeEnum&gt;&gt;&gt;) -> Result&lt;PaginatedResponseWebhookSubscriptionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

List all webhook subscriptions for your partner account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .list_webhook_subscriptions(
            &ListWebhookSubscriptionsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**event_type:** `Option<Option<WebhookEventTypeEnum>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">create_webhook_subscription</a>(request: WebhookCreatePayload) -> Result&lt;WebhookSubscriptionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new webhook subscription for a specific event type.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .create_webhook_subscription(
            &WebhookCreatePayload {
                url: "https://example.com/webhooks".to_string(),
                description: Some("Loan update event".to_string()),
                event_type: WebhookEventTypeEnum::LoanUpdated,
                secret: "whsec_f3o9p8h7g6j5k4l3m2n1o0p9i8u7y6t5".to_string(),
                retry: Some(false),
                status: Some(WebhookStatusEnum::Active),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**url:** `String` — The URL to send webhooks to
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` — Optional description of this webhook endpoint
    
</dd>
</dl>

<dl>
<dd>

**event_type:** `WebhookEventTypeEnum` — Event type to subscribe toPossible values: loan_updated, installment_updated, client_updated, client_limit_updated, partner_limit_updated
    
</dd>
</dl>

<dl>
<dd>

**secret:** `String` — Secret for signing webhook payloads
    
</dd>
</dl>

<dl>
<dd>

**retry:** `Option<bool>` — Whether to retry failed webhooks
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<Option<WebhookStatusEnum>>` — Status of the webhook subscription. Defaults to 'active'.Possible values: active, paused, disabled
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">get_webhook_subscription</a>(webhook_id: String) -> Result&lt;WebhookSubscriptionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve details for a specific webhook subscription with its webhook ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .get_webhook_subscription(&"webhook_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**webhook_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">update_webhook_subscription</a>(webhook_id: String, request: WebhookUpdatePayload) -> Result&lt;WebhookSubscriptionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a webhook subscription with its specific webhook ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .update_webhook_subscription(
            &"webhook_id".to_string(),
            &WebhookUpdatePayload {
                url: Some("https://example.com/webhooks/v2".to_string()),
                description: Some("Updated webhook endpoint".to_string()),
                event_type: Some(WebhookEventTypeEnum::InstallmentUpdated),
                status: Some(WebhookStatusEnum::Paused),
                retry: Some(true),
                secret: Some("whsec_updated_secret_here".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**webhook_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<Option<String>>` — The URL to send webhooks to
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` — Description of this webhook endpoint
    
</dd>
</dl>

<dl>
<dd>

**event_type:** `Option<Option<WebhookEventTypeEnum>>` — Event type to subscribe toPossible values: loan_updated, installment_updated, client_updated, client_limit_updated, partner_limit_updated
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<Option<WebhookStatusEnum>>` — Status of the webhook subscriptionPossible values: active, paused, disabled
    
</dd>
</dl>

<dl>
<dd>

**retry:** `Option<Option<bool>>` — Whether to retry failed webhooks
    
</dd>
</dl>

<dl>
<dd>

**secret:** `Option<Option<String>>` — Secret for signing webhook payloads
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">delete_webhook_subscription</a>(webhook_id: String) -> Result&lt;std::collections::HashMap&lt;String, serde_json::Value&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a specific webhook subscription.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .delete_webhook_subscription(&"webhook_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**webhook_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">list_webhook_logs</a>(webhook_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, page: Option&lt;Option&lt;i64&gt;&gt;, page_size: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;PaginatedResponseWebhookLogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve all webhook logs linked to your partner account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .list_webhook_logs(
            &ListWebhookLogsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**webhook_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Repayments
<details><summary><code>client.repayments.<a href="/src/api/resources/repayments/client.rs">list_repayment_logs</a>(client_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, loan_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, installment_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, page: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, page_size: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, order_by: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, q: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;PaginatedResponseRepaymentResponseWithClientInfo, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve all repayments made under your partner account. Supports filtering by client, loan, or installments.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .repayments
        .list_repayment_logs(
            &ListRepaymentLogsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**loan_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**installment_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**order_by:** `Option<Option<String>>` — Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    
</dd>
</dl>

<dl>
<dd>

**q:** `Option<Option<String>>` — Query string for filtering. Format: "field:operator:value;...". Supported fields: id, client_id, loan_id, installment_id, created_at, client.name, client.correlation_id. Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.repayments.<a href="/src/api/resources/repayments/client.rs">create_repayment</a>(request: RepaymentCreatePayload, installment_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, loan_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, correlation_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;RepaymentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new repayment log for an installment. Requires the installment ID, loan ID or loan correlation ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .repayments
        .create_repayment(
            &RepaymentCreatePayload {
                amount: RepaymentCreatePayloadAmount::Double(1.1),
                installment_id: None,
                loan_id: None,
                correlation_id: None,
                repayment_date: None,
                data: None,
                is_early_settlement: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**amount:** `RepaymentCreatePayloadAmount` — The amount of payment made for installment
    
</dd>
</dl>

<dl>
<dd>

**repayment_date:** `Option<Option<String>>` — Please provide the repayment_date if it differs from the time you log this repayment. Otherwise, it will be automatically set.
    
</dd>
</dl>

<dl>
<dd>

**data:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>` — Additional metadata related to the repayment
    
</dd>
</dl>

<dl>
<dd>

**is_early_settlement:** `Option<bool>` — Indicates if this repayment is for early settlement
    
</dd>
</dl>

<dl>
<dd>

**installment_id:** `Option<Option<String>>` — ID of the installment
    
</dd>
</dl>

<dl>
<dd>

**loan_id:** `Option<Option<String>>` — ID of the associated Loan
    
</dd>
</dl>

<dl>
<dd>

**correlation_id:** `Option<Option<String>>` — Correlation ID of associated loan
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.repayments.<a href="/src/api/resources/repayments/client.rs">create_bulk_repayments</a>(request: BulkRepaymentCreatePayload) -> Result&lt;BulkRepaymentTaskResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Initiate processing of up to 10000 repayment logs. Returns task_id for tracking progress.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .repayments
        .create_bulk_repayments(
            &BulkRepaymentCreatePayload {
                repayments: vec![
                    BulkRepaymentItemPayload {
                        amount: BulkRepaymentItemPayloadAmount::Double(1000.0),
                        repayment_date: Some(
                            DateTime::parse_from_rfc3339("2023-10-01T12:00:00Z").unwrap(),
                        ),
                        data: Some(HashMap::from([
                            ("reference".to_string(), serde_json::json!("TXN-001")),
                            ("type".to_string(), serde_json::json!("transfer")),
                        ])),
                        is_early_settlement: None,
                        installment_id: Some("installment_123".to_string()),
                        loan_id: None,
                        correlation_id: None,
                    },
                    BulkRepaymentItemPayload {
                        amount: BulkRepaymentItemPayloadAmount::Double(500.5),
                        repayment_date: None,
                        data: Some(HashMap::from([
                            (
                                "notes".to_string(),
                                serde_json::json!("Payment received in office"),
                            ),
                            ("type".to_string(), serde_json::json!("cash")),
                        ])),
                        is_early_settlement: None,
                        installment_id: None,
                        loan_id: Some("loan_456".to_string()),
                        correlation_id: None,
                    },
                    BulkRepaymentItemPayload {
                        amount: BulkRepaymentItemPayloadAmount::Double(750.0),
                        repayment_date: Some(
                            DateTime::parse_from_rfc3339("2023-09-30T15:30:00Z").unwrap(),
                        ),
                        data: None,
                        is_early_settlement: None,
                        installment_id: None,
                        loan_id: None,
                        correlation_id: Some("LOAN-789".to_string()),
                    },
                ],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**repayments:** `Vec<BulkRepaymentItemPayload>` — List of repayments to create (max 10000)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.repayments.<a href="/src/api/resources/repayments/client.rs">get_bulk_repayment_status</a>(task_id: String) -> Result&lt;BulkRepaymentTaskStatus, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Check the progress and results of a bulk repayment processing task.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .repayments
        .get_bulk_repayment_status(&"task_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Drawdowns
<details><summary><code>client.drawdowns.<a href="/src/api/resources/drawdowns/client.rs">list_drawdowns</a>(page: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, page_size: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, order_by: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, q: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;PaginatedResponseDrawdownResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of drawdowns.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .drawdowns
        .list_drawdowns(
            &ListDrawdownsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**order_by:** `Option<Option<String>>` — Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    
</dd>
</dl>

<dl>
<dd>

**q:** `Option<Option<String>>` — Query string for filtering. Format: "field:operator:value;...". Supported fields: . Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.drawdowns.<a href="/src/api/resources/drawdowns/client.rs">create_drawdown_request</a>(request: DrawdownCreatePayload) -> Result&lt;DrawdownResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new drawdown request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .drawdowns
        .create_drawdown_request(
            &DrawdownCreatePayload {
                amount: DrawdownCreatePayloadAmount::Double(1.1),
                drawdown_date: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**amount:** `DrawdownCreatePayloadAmount` — The amount for the drawdown.
    
</dd>
</dl>

<dl>
<dd>

**drawdown_date:** `Option<Option<String>>` — The date for the drawdown. If not provided, defaults to the current date and time.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.drawdowns.<a href="/src/api/resources/drawdowns/client.rs">list_drawdown_checklists</a>(drawdown_id: String, page: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, page_size: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, order_by: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, q: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;PaginatedResponseDrawdownChecklistResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve all checklist items for a specific drawdown
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .drawdowns
        .list_drawdown_checklists(
            &"drawdown_id".to_string(),
            &ListDrawdownChecklistsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**drawdown_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**order_by:** `Option<Option<String>>` — Field to order the results by, e.g., 'created_at:desc,updated_at:asc'
    
</dd>
</dl>

<dl>
<dd>

**q:** `Option<Option<String>>` — Query string for filtering. Format: "field:operator:value;...". Supported fields: . Supported operators: is, in, not_in, contains, not_contains, like, not_like, ilike, not_ilike, gt, gte, lt, lte, starts_with, ends_with, is_null, is_not_null.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

