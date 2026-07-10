pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LimitRequestSourceEnum {
    Partner,
    Internal,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LimitRequestSourceEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Partner => serializer.serialize_str("partner"),
            Self::Internal => serializer.serialize_str("internal"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LimitRequestSourceEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "partner" => Ok(Self::Partner),
            "internal" => Ok(Self::Internal),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LimitRequestSourceEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Partner => write!(f, "partner"),
            Self::Internal => write!(f, "internal"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
