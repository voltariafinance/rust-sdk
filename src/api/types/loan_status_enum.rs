pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoanStatusEnum {
    Pending,
    Overdue,
    Active,
    Default,
    Sold,
    Restructured,
    Repaid,
    PreApproved,
    Rejected,
    Deleted,
    Inactive,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LoanStatusEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Overdue => serializer.serialize_str("overdue"),
            Self::Active => serializer.serialize_str("active"),
            Self::Default => serializer.serialize_str("default"),
            Self::Sold => serializer.serialize_str("sold"),
            Self::Restructured => serializer.serialize_str("restructured"),
            Self::Repaid => serializer.serialize_str("repaid"),
            Self::PreApproved => serializer.serialize_str("pre_approved"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::Deleted => serializer.serialize_str("deleted"),
            Self::Inactive => serializer.serialize_str("inactive"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LoanStatusEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "overdue" => Ok(Self::Overdue),
            "active" => Ok(Self::Active),
            "default" => Ok(Self::Default),
            "sold" => Ok(Self::Sold),
            "restructured" => Ok(Self::Restructured),
            "repaid" => Ok(Self::Repaid),
            "pre_approved" => Ok(Self::PreApproved),
            "rejected" => Ok(Self::Rejected),
            "deleted" => Ok(Self::Deleted),
            "inactive" => Ok(Self::Inactive),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LoanStatusEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Overdue => write!(f, "overdue"),
            Self::Active => write!(f, "active"),
            Self::Default => write!(f, "default"),
            Self::Sold => write!(f, "sold"),
            Self::Restructured => write!(f, "restructured"),
            Self::Repaid => write!(f, "repaid"),
            Self::PreApproved => write!(f, "pre_approved"),
            Self::Rejected => write!(f, "rejected"),
            Self::Deleted => write!(f, "deleted"),
            Self::Inactive => write!(f, "inactive"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
