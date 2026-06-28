pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentResponse {
    /// The ID of the document
    #[serde(default)]
    pub id: String,
    /// The category of the document (kyc, financial, etc.)
    #[serde(default)]
    pub category: String,
    /// The name of the uploaded file
    #[serde(default)]
    pub file_name: String,
    /// The content type of the file
    #[serde(default)]
    pub file_type: String,
    /// The associated client ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// The Presigned URL of the file. This is a temporary URL that allows you to download the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    /// The ID of the associated loan, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_id: Option<String>,
    /// The ID of the associated installment, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installment_id: Option<String>,
    /// Slash-delimited folder path used to organise the document, e.g. 'Legal/Contracts'. Null means the document is unfiled (at the root).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<String>,
    /// Optional document date (e.g. the date an investment document was issued)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_date: Option<NaiveDate>,
    /// Optional expiry date of the document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<NaiveDate>,
    /// The date and time when the document was created
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl DocumentResponse {
    pub fn builder() -> DocumentResponseBuilder {
        <DocumentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentResponseBuilder {
    id: Option<String>,
    category: Option<String>,
    file_name: Option<String>,
    file_type: Option<String>,
    client_id: Option<String>,
    file_url: Option<String>,
    loan_id: Option<String>,
    installment_id: Option<String>,
    folder_path: Option<String>,
    document_date: Option<NaiveDate>,
    expiry_date: Option<NaiveDate>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl DocumentResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn file_type(mut self, value: impl Into<String>) -> Self {
        self.file_type = Some(value.into());
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn file_url(mut self, value: impl Into<String>) -> Self {
        self.file_url = Some(value.into());
        self
    }

    pub fn loan_id(mut self, value: impl Into<String>) -> Self {
        self.loan_id = Some(value.into());
        self
    }

    pub fn installment_id(mut self, value: impl Into<String>) -> Self {
        self.installment_id = Some(value.into());
        self
    }

    pub fn folder_path(mut self, value: impl Into<String>) -> Self {
        self.folder_path = Some(value.into());
        self
    }

    pub fn document_date(mut self, value: NaiveDate) -> Self {
        self.document_date = Some(value);
        self
    }

    pub fn expiry_date(mut self, value: NaiveDate) -> Self {
        self.expiry_date = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DocumentResponseBuilder::id)
    /// - [`category`](DocumentResponseBuilder::category)
    /// - [`file_name`](DocumentResponseBuilder::file_name)
    /// - [`file_type`](DocumentResponseBuilder::file_type)
    /// - [`created_at`](DocumentResponseBuilder::created_at)
    pub fn build(self) -> Result<DocumentResponse, BuildError> {
        Ok(DocumentResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            file_type: self
                .file_type
                .ok_or_else(|| BuildError::missing_field("file_type"))?,
            client_id: self.client_id,
            file_url: self.file_url,
            loan_id: self.loan_id,
            installment_id: self.installment_id,
            folder_path: self.folder_path,
            document_date: self.document_date,
            expiry_date: self.expiry_date,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
