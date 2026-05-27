use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};

use crate::{GString, GStringError, Validator};
use core::{fmt, marker::PhantomData};

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::string::String;

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> Serialize
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

struct GStringVisitor<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>(
    PhantomData<V>,
);

impl<'de, V, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> Visitor<'de>
    for GStringVisitor<V, MIN, MAX, ASCII_ONLY>
where
    V: Validator,
    GStringError<V::Err>: fmt::Display,
{
    type Value = GString<V, MIN, MAX, ASCII_ONLY>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a string with length between {} and {}{}",
            MIN,
            MAX,
            if ASCII_ONLY { " (ASCII only)" } else { "" }
        )
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        GString::try_new(v).map_err(de::Error::custom)
    }

    #[cfg(feature = "alloc")]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        GString::try_from(v).map_err(de::Error::custom)
    }
}

impl<'de, V, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> Deserialize<'de>
    for GString<V, MIN, MAX, ASCII_ONLY>
where
    V: Validator,
    GStringError<V::Err>: fmt::Display,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(GStringVisitor(PhantomData))
    }
}

// GSecret serde
#[cfg(feature = "secret")]
mod secret_serde {
    use crate::{GSecret, GStringError, Validator};
    use core::{fmt, marker::PhantomData};
    use serde::{
        Deserialize, Deserializer,
        de::{self, Visitor},
    };

    struct GSecretVisitor<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>(
        PhantomData<V>,
    );

    impl<'de, V, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> Visitor<'de>
        for GSecretVisitor<V, MIN, MAX, ASCII_ONLY>
    where
        V: Validator,
        GStringError<V::Err>: fmt::Display,
    {
        type Value = GSecret<V, MIN, MAX, ASCII_ONLY>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(
                formatter,
                "a secret string between {} and {}{}",
                MIN,
                MAX,
                if ASCII_ONLY { " (ASCII only)" } else { "" }
            )
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            GSecret::try_new(v).map_err(de::Error::custom)
        }

        #[cfg(feature = "alloc")]
        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            GSecret::try_from(v).map_err(de::Error::custom)
        }
    }

    impl<'de, V, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> Deserialize<'de>
        for GSecret<V, MIN, MAX, ASCII_ONLY>
    where
        V: Validator,
        GStringError<V::Err>: fmt::Display,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(GSecretVisitor(PhantomData))
        }
    }
}
