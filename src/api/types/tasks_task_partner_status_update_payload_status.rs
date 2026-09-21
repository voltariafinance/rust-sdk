pub use crate::prelude::*;

/// The new status of the task. One of the following: active, in_progress, blocked, done. You can move a task to any of these at any time, so one closed by mistake can be reopened. Every change is kept in the task's status history.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaskPartnerStatusUpdatePayloadStatus {
    Active,
    InProgress,
    Blocked,
    Done,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TaskPartnerStatusUpdatePayloadStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("active"),
            Self::InProgress => serializer.serialize_str("in_progress"),
            Self::Blocked => serializer.serialize_str("blocked"),
            Self::Done => serializer.serialize_str("done"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TaskPartnerStatusUpdatePayloadStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "active" => Ok(Self::Active),
            "in_progress" => Ok(Self::InProgress),
            "blocked" => Ok(Self::Blocked),
            "done" => Ok(Self::Done),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TaskPartnerStatusUpdatePayloadStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Blocked => write!(f, "blocked"),
            Self::Done => write!(f, "done"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
