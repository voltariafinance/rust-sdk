pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WaiverStatusEnum {
    Pending,
    Active,
    Paid,
    Rejected,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WaiverStatusEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Active => serializer.serialize_str("active"),
            Self::Paid => serializer.serialize_str("paid"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WaiverStatusEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "active" => Ok(Self::Active),
            "paid" => Ok(Self::Paid),
            "rejected" => Ok(Self::Rejected),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WaiverStatusEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Active => write!(f, "active"),
            Self::Paid => write!(f, "paid"),
            Self::Rejected => write!(f, "rejected"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
