pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KycStatusEnum {
    NotStarted,
    Triggered,
    Verified,
    Rejected,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for KycStatusEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NotStarted => serializer.serialize_str("not_started"),
            Self::Triggered => serializer.serialize_str("triggered"),
            Self::Verified => serializer.serialize_str("verified"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for KycStatusEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "not_started" => Ok(Self::NotStarted),
            "triggered" => Ok(Self::Triggered),
            "verified" => Ok(Self::Verified),
            "rejected" => Ok(Self::Rejected),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for KycStatusEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotStarted => write!(f, "not_started"),
            Self::Triggered => write!(f, "triggered"),
            Self::Verified => write!(f, "verified"),
            Self::Rejected => write!(f, "rejected"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
