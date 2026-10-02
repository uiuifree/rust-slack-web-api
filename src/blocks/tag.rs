use std::fmt;
use std::marker::PhantomData;

use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize, Serializer};

/// A type with a fixed value for its JSON discriminator key (such as `type`).
pub(super) trait Tagged {
    const TAG: &'static str;
}

/// A field that only reads and writes the discriminator value. It holds no data, so the kind can never be mismatched.
pub(super) struct Tag<T>(PhantomData<fn() -> T>);

impl<T> Default for Tag<T> {
    fn default() -> Self {
        Tag(PhantomData)
    }
}

impl<T> Clone for Tag<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Tag<T> {}

impl<T> PartialEq for Tag<T> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<T: Tagged> fmt::Debug for Tag<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(T::TAG)
    }
}

impl<T: Tagged> Serialize for Tag<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(T::TAG)
    }
}

impl<'de, T: Tagged> Deserialize<'de> for Tag<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct TagVisitor<T>(PhantomData<fn() -> T>);

        impl<T: Tagged> Visitor<'_> for TagVisitor<T> {
            type Value = Tag<T>;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "\"{}\"", T::TAG)
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                if v == T::TAG {
                    Ok(Tag::default())
                } else {
                    Err(E::invalid_value(de::Unexpected::Str(v), &self))
                }
            }
        }

        deserializer.deserialize_str(TagVisitor(PhantomData))
    }
}

/// Defines an enum dispatched by its discriminator key. Unknown kinds are kept in `Unknown` as the original JSON.
macro_rules! tagged_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident($key:literal) {
            $($variant:ident($ty:ty),)*
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq)]
        $vis enum $name {
            $($variant($ty),)*
            /// A kind this crate does not know. Keeps the received JSON and writes it back unchanged.
            Unknown(::serde_json::Value),
        }

        $(
            impl From<$ty> for $name {
                fn from(value: $ty) -> Self {
                    $name::$variant(value)
                }
            }
        )*

        impl ::serde::Serialize for $name {
            fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                match self {
                    $($name::$variant(value) => value.serialize(serializer),)*
                    $name::Unknown(value) => value.serialize(serializer),
                }
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                // 未知の種別を元の JSON のまま残すため一度 Value で受ける。
                // derive の内部タグ方式も内部で同様に全体をバッファするので、既知種別の負担は増えない。
                let value = ::serde_json::Value::deserialize(deserializer)?;
                let Some(tag) = value.get($key).and_then(::serde_json::Value::as_str) else {
                    return Ok($name::Unknown(value));
                };
                $(
                    if tag == <$ty as super::tag::Tagged>::TAG {
                        return ::serde_json::from_value(value)
                            .map($name::$variant)
                            .map_err(::serde::de::Error::custom);
                    }
                )*
                Ok($name::Unknown(value))
            }
        }
    };
}

pub(super) use tagged_enum;
