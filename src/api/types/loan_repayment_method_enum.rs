pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoanRepaymentMethodEnum {
    Pmt,
    ConstantPrincipal,
    Custom,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LoanRepaymentMethodEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pmt => serializer.serialize_str("pmt"),
            Self::ConstantPrincipal => serializer.serialize_str("constant_principal"),
            Self::Custom => serializer.serialize_str("custom"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LoanRepaymentMethodEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pmt" => Ok(Self::Pmt),
            "constant_principal" => Ok(Self::ConstantPrincipal),
            "custom" => Ok(Self::Custom),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LoanRepaymentMethodEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pmt => write!(f, "pmt"),
            Self::ConstantPrincipal => write!(f, "constant_principal"),
            Self::Custom => write!(f, "custom"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
