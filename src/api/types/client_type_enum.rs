pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClientTypeEnum {
    Corporate,
    Individual,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ClientTypeEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Corporate => serializer.serialize_str("corporate"),
            Self::Individual => serializer.serialize_str("individual"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ClientTypeEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "corporate" => Ok(Self::Corporate),
            "individual" => Ok(Self::Individual),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ClientTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Corporate => write!(f, "corporate"),
            Self::Individual => write!(f, "individual"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
