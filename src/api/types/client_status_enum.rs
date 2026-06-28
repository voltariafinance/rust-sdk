pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClientStatusEnum {
    Active,
    Rejected,
    Deactivated,
    Pending,
    PendingOnboarding,
    PreApproved,
    Deleted,
    Inactive,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ClientStatusEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("active"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::Deactivated => serializer.serialize_str("deactivated"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::PendingOnboarding => serializer.serialize_str("pending_onboarding"),
            Self::PreApproved => serializer.serialize_str("pre_approved"),
            Self::Deleted => serializer.serialize_str("deleted"),
            Self::Inactive => serializer.serialize_str("inactive"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ClientStatusEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "active" => Ok(Self::Active),
            "rejected" => Ok(Self::Rejected),
            "deactivated" => Ok(Self::Deactivated),
            "pending" => Ok(Self::Pending),
            "pending_onboarding" => Ok(Self::PendingOnboarding),
            "pre_approved" => Ok(Self::PreApproved),
            "deleted" => Ok(Self::Deleted),
            "inactive" => Ok(Self::Inactive),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ClientStatusEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Rejected => write!(f, "rejected"),
            Self::Deactivated => write!(f, "deactivated"),
            Self::Pending => write!(f, "pending"),
            Self::PendingOnboarding => write!(f, "pending_onboarding"),
            Self::PreApproved => write!(f, "pre_approved"),
            Self::Deleted => write!(f, "deleted"),
            Self::Inactive => write!(f, "inactive"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
