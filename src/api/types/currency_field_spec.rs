pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CurrencyFieldSpec {
    /// Fields that must be provided for this currency.
    #[serde(default)]
    pub required: Vec<String>,
    /// Fields that are accepted but not required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<Vec<String>>,
}

impl CurrencyFieldSpec {
    pub fn builder() -> CurrencyFieldSpecBuilder {
        <CurrencyFieldSpecBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencyFieldSpecBuilder {
    required: Option<Vec<String>>,
    optional: Option<Vec<String>>,
}

impl CurrencyFieldSpecBuilder {
    pub fn required(mut self, value: Vec<String>) -> Self {
        self.required = Some(value);
        self
    }

    pub fn optional(mut self, value: Vec<String>) -> Self {
        self.optional = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrencyFieldSpec`].
    /// This method will fail if any of the following fields are not set:
    /// - [`required`](CurrencyFieldSpecBuilder::required)
    pub fn build(self) -> Result<CurrencyFieldSpec, BuildError> {
        Ok(CurrencyFieldSpec {
            required: self
                .required
                .ok_or_else(|| BuildError::missing_field("required"))?,
            optional: self.optional,
        })
    }
}
