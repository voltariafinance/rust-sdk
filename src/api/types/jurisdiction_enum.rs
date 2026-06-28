pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JurisdictionEnum {
    Eu,
    Uk,
    Us,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for JurisdictionEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Eu => serializer.serialize_str("eu"),
            Self::Uk => serializer.serialize_str("uk"),
            Self::Us => serializer.serialize_str("us"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for JurisdictionEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "eu" => Ok(Self::Eu),
            "uk" => Ok(Self::Uk),
            "us" => Ok(Self::Us),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for JurisdictionEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eu => write!(f, "eu"),
            Self::Uk => write!(f, "uk"),
            Self::Us => write!(f, "us"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
