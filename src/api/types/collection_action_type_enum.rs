pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CollectionActionTypeEnum {
    Email,
    Sms,
    PhoneCall,
    PushNotification,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CollectionActionTypeEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Email => serializer.serialize_str("email"),
            Self::Sms => serializer.serialize_str("sms"),
            Self::PhoneCall => serializer.serialize_str("phone_call"),
            Self::PushNotification => serializer.serialize_str("push_notification"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CollectionActionTypeEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "email" => Ok(Self::Email),
            "sms" => Ok(Self::Sms),
            "phone_call" => Ok(Self::PhoneCall),
            "push_notification" => Ok(Self::PushNotification),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CollectionActionTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Email => write!(f, "email"),
            Self::Sms => write!(f, "sms"),
            Self::PhoneCall => write!(f, "phone_call"),
            Self::PushNotification => write!(f, "push_notification"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
