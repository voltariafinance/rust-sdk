pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountStatusEnum {
    Active,
    Passive,
    Pending,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountStatusEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("active"),
            Self::Passive => serializer.serialize_str("passive"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountStatusEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "active" => Ok(Self::Active),
            "passive" => Ok(Self::Passive),
            "pending" => Ok(Self::Pending),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountStatusEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Passive => write!(f, "passive"),
            Self::Pending => write!(f, "pending"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
