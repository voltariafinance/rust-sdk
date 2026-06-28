use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    #[serde(rename = "sandbox")]
    Sandbox,
    #[serde(rename = "production")]
    Production,
}
impl Environment {
    pub fn url(&self) -> &'static str {
        match self {
            Self::Sandbox => "https://api.sandbox.voltaria.io",
            Self::Production => "https://api.voltaria.io",
        }
    }
}
impl Default for Environment {
    fn default() -> Self {
        Self::Sandbox
    }
}
