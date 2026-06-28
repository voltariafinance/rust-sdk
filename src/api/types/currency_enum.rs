pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CurrencyEnum {
    Eur,
    Gbp,
    Usd,
    Czk,
    Pln,
    Isk,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CurrencyEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Eur => serializer.serialize_str("eur"),
            Self::Gbp => serializer.serialize_str("gbp"),
            Self::Usd => serializer.serialize_str("usd"),
            Self::Czk => serializer.serialize_str("czk"),
            Self::Pln => serializer.serialize_str("pln"),
            Self::Isk => serializer.serialize_str("isk"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CurrencyEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "eur" => Ok(Self::Eur),
            "gbp" => Ok(Self::Gbp),
            "usd" => Ok(Self::Usd),
            "czk" => Ok(Self::Czk),
            "pln" => Ok(Self::Pln),
            "isk" => Ok(Self::Isk),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CurrencyEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eur => write!(f, "eur"),
            Self::Gbp => write!(f, "gbp"),
            Self::Usd => write!(f, "usd"),
            Self::Czk => write!(f, "czk"),
            Self::Pln => write!(f, "pln"),
            Self::Isk => write!(f, "isk"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
