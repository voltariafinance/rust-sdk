pub use crate::prelude::*;

/// Who made a change: the partner's own team, or Voltaria.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaskPublicActorTypeEnum {
    Partner,
    Support,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TaskPublicActorTypeEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Partner => serializer.serialize_str("partner"),
            Self::Support => serializer.serialize_str("support"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TaskPublicActorTypeEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "partner" => Ok(Self::Partner),
            "support" => Ok(Self::Support),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TaskPublicActorTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Partner => write!(f, "partner"),
            Self::Support => write!(f, "support"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
