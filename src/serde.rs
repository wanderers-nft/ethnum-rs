use crate::U256;
use serde::{de, Deserialize, Serialize};

/// This serializes a U256 number to a decimal string representation.
impl Serialize for U256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// This deserializes a decimal string into U256.
impl<'de> Deserialize<'de> for U256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse::<U256>()
            .map_err(de::Error::custom)
    }
}
