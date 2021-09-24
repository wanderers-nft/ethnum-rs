//! Serde support.

mod buffer;

use self::buffer::Buffer;
use crate::U256;
use core::{
    fmt::{self, Formatter, Write},
    mem::MaybeUninit,
};
use serde::{
    de::{self, Deserialize, Deserializer, Visitor},
    ser::{Serialize, Serializer},
};

impl Serialize for U256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let zeros = (self.leading_zeros() / 8) as usize;
        let bytes = self.to_be_bytes();

        // SAFETY: Cannot have more leading zeros than there are bits.
        #[cfg(debug_assertions)]
        let slice = &bytes[zeros..];
        #[cfg(not(debug_assertions))]
        let slice = unsafe { bytes.get_unchecked(zeros..) };

        serializer.serialize_bytes(slice)
    }
}

impl<'de> Deserialize<'de> for U256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;
        impl Visitor<'_> for V {
            type Value = U256;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("up to 32 bytes of integer data")
            }

            fn visit_borrowed_bytes<E>(self, v: &'_ [u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_bytes(v)
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let mut bytes = [0; 32];
                let start = 32_usize
                    .checked_sub(dbg!(v.len()))
                    .ok_or_else(|| de::Error::custom("more than 32 bytes of integer data"))?;

                // SAFETY: `start` is less than 32 and `bytes[start..]` has the
                // same length as `v`. Additionally, we `bytes` and `v` can
                // never be overlapping.
                #[cfg(debug_assertions)]
                bytes[start..].copy_from_slice(v);
                #[cfg(not(debug_assertions))]
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        v.as_ptr(),
                        bytes.get_unchecked_mut(start..).as_mut_ptr(),
                        v.len(),
                    );
                };

                Ok(U256::from_be_bytes(bytes))
            }
        }

        deserializer.deserialize_str(V)
    }
}

macro_rules! serde_fmt_impl {
    (
        $format:literal,
        $radix:literal,
        $expecting:literal,
        $prefix:literal $(,)?
    ) => {
        use super::*;

        /// Serialize 256-bit integer as a formatted string.
        pub fn serialize<S>(x: &U256, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            // Maximum length for an unsigned 256-bit integer is 78 bytes.
            let mut bytes = [MaybeUninit::uninit(); 78];
            let mut buf = Buffer::new(&mut bytes);
            if let Err(_) = write!(buf, $format, x) {
                unreachable!("formatting u256 is infallible");
            }
            serializer.serialize_str(buf.as_str())
        }

        /// Deserialize 256-bit integer from a formatted string.
        pub fn deserialize<'de, D>(deserializer: D) -> Result<U256, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct V;
            impl Visitor<'_> for V {
                type Value = U256;

                fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                    f.write_str(concat!(
                        "expecting a 256-bit integer formatted as a ",
                        $expecting,
                        " string",
                    ))
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    U256::from_str_radix(v, $radix).map_err(de::Error::custom)
                }
            }

            deserializer.deserialize_str(V)
        }
    };
}

/// Module used for serializing 256-integers as a decimal string.
///
/// # Examples
///
/// Basic usage
///
/// ```
/// # use ethnum::U256;
/// # use serde_derive::{Deserialize, Serialize};
/// #[derive(Deserialize, Serialize)]
/// struct MyData {
///    #[serde(with = "ethnum::serde::dec")]
///    value: U256,
/// }
/// ```
pub mod dec {
    serde_fmt_impl!("{}", 10, "decimal", "");
}

/// Module used for serializing 256-integers as a hexadecimal string, compatible
/// with Ethereum JSON RPC spec.
///
/// # Examples
///
/// Basic usage
///
/// ```
/// # use ethnum::U256;
/// # use serde_derive::{Deserialize, Serialize};
/// #[derive(Deserialize, Serialize)]
/// struct MyData {
///    #[serde(with = "ethnum::serde::hex")]
///    value: U256,
/// }
/// ```
pub mod hex {
    serde_fmt_impl!("{:#x}", 16, "hexadecimal", "0x");
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_derive::{Deserialize, Serialize};
    use serde_json::json;

    #[test]
    fn default_serialization() {
        assert_eq!(
            serde_json::to_value(&U256::new(0x12345)).unwrap(),
            json!([0x1, 0x23, 0x45]),
        );
        assert_eq!(
            serde_json::from_slice::<U256>(b"\"\x01\x02\x03\"").unwrap(),
            U256::new(0x12345),
        );
    }

    #[test]
    fn formatted_serialization() {
        #[derive(Debug, Deserialize, PartialEq, Serialize)]
        struct Data {
            #[serde(with = "dec")]
            d: U256,
            #[serde(with = "hex")]
            h: U256,
        }

        assert_eq!(
            serde_json::to_value(&Data {
                d: U256::new(1337),
                h: U256::new(0x123),
            })
            .unwrap(),
            json!({
                "d": "1337",
                "h": "0x123",
            }),
        );

        assert_eq!(
            serde_json::from_value::<Data>(json!({
                "d": "1337",
                "h": "0x123",
            }))
            .unwrap(),
            Data {
                d: U256::new(1337),
                h: U256::new(0x123),
            },
        );
    }
}
