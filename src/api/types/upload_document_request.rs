pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UploadDocumentRequest {
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub file: Vec<u8>,
    #[serde(skip_serializing)]
    pub client_id: Option<String>,
    #[serde(skip_serializing)]
    pub loan_id: Option<String>,
    #[serde(skip_serializing)]
    pub installment_id: Option<String>,
    #[serde(skip_serializing)]
    pub waterfall_id: Option<String>,
}
impl UploadDocumentRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        form = form.part(
            "file",
            reqwest::multipart::Part::bytes(self.file.clone())
                .file_name("file")
                .mime_str("application/octet-stream")
                .unwrap(),
        );

        if let Ok(json_str) = serde_json::to_string(&self.category) {
            form = form.text("category", json_str);
        }

        if let Ok(json_str) = serde_json::to_string(&self.file_name) {
            form = form.text("file_name", json_str);
        }

        form
    }
}

impl UploadDocumentRequest {
    pub fn builder() -> UploadDocumentRequestBuilder {
        <UploadDocumentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UploadDocumentRequestBuilder {
    category: Option<String>,
    file_name: Option<String>,
    file: Option<Vec<u8>>,
    client_id: Option<String>,
    loan_id: Option<String>,
    installment_id: Option<String>,
    waterfall_id: Option<String>,
}

impl UploadDocumentRequestBuilder {
    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
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

    pub fn waterfall_id(mut self, value: impl Into<String>) -> Self {
        self.waterfall_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UploadDocumentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](UploadDocumentRequestBuilder::category)
    /// - [`file_name`](UploadDocumentRequestBuilder::file_name)
    /// - [`file`](UploadDocumentRequestBuilder::file)
    pub fn build(self) -> Result<UploadDocumentRequest, BuildError> {
        Ok(UploadDocumentRequest {
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
            client_id: self.client_id,
            loan_id: self.loan_id,
            installment_id: self.installment_id,
            waterfall_id: self.waterfall_id,
        })
    }
}
