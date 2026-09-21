pub use crate::prelude::*;

/// Our own verdict, mapped from whichever code a CoP provider returns.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CopStatusEnum {
    Matched,
    CloseMatch,
    NotMatched,
    AccountNotFound,
    Unavailable,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CopStatusEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Matched => serializer.serialize_str("matched"),
            Self::CloseMatch => serializer.serialize_str("close_match"),
            Self::NotMatched => serializer.serialize_str("not_matched"),
            Self::AccountNotFound => serializer.serialize_str("account_not_found"),
            Self::Unavailable => serializer.serialize_str("unavailable"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CopStatusEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "matched" => Ok(Self::Matched),
            "close_match" => Ok(Self::CloseMatch),
            "not_matched" => Ok(Self::NotMatched),
            "account_not_found" => Ok(Self::AccountNotFound),
            "unavailable" => Ok(Self::Unavailable),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CopStatusEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Matched => write!(f, "matched"),
            Self::CloseMatch => write!(f, "close_match"),
            Self::NotMatched => write!(f, "not_matched"),
            Self::AccountNotFound => write!(f, "account_not_found"),
            Self::Unavailable => write!(f, "unavailable"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
