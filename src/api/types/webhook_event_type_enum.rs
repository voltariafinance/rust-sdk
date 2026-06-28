pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WebhookEventTypeEnum {
    LoanUpdated,
    InstallmentUpdated,
    ClientUpdated,
    ClientLimitUpdated,
    PartnerLimitUpdated,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WebhookEventTypeEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::LoanUpdated => serializer.serialize_str("loan.updated"),
            Self::InstallmentUpdated => serializer.serialize_str("installment.updated"),
            Self::ClientUpdated => serializer.serialize_str("client.updated"),
            Self::ClientLimitUpdated => serializer.serialize_str("client.limit.updated"),
            Self::PartnerLimitUpdated => serializer.serialize_str("partner.limit.updated"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WebhookEventTypeEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "loan.updated" => Ok(Self::LoanUpdated),
            "installment.updated" => Ok(Self::InstallmentUpdated),
            "client.updated" => Ok(Self::ClientUpdated),
            "client.limit.updated" => Ok(Self::ClientLimitUpdated),
            "partner.limit.updated" => Ok(Self::PartnerLimitUpdated),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WebhookEventTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LoanUpdated => write!(f, "loan.updated"),
            Self::InstallmentUpdated => write!(f, "installment.updated"),
            Self::ClientUpdated => write!(f, "client.updated"),
            Self::ClientLimitUpdated => write!(f, "client.limit.updated"),
            Self::PartnerLimitUpdated => write!(f, "partner.limit.updated"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
