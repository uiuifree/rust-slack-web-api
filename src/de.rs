//! Lenient deserializers used by the generated response types.
//!
//! Slack returns some fields as a string in one place and a number in another (`ts`, `date_created`,
//! `post_at`, ...). These functions accept such variations so that one unexpected type does not fail
//! the whole response. They read values directly through a `Visitor`, without an intermediate tree.

use serde::de::{self, Deserializer, IgnoredAny, Visitor};
use serde::Deserialize;
use std::fmt;
use std::marker::PhantomData;

/// Reads a string; numbers and booleans are converted to strings.
///
/// # Errors
///
/// Only when the input itself is malformed.
pub fn opt_string<'de, D: Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    d.deserialize_any(OptVisitor(PhantomData::<String>))
}

/// Reads an integer; numeric strings and floats (truncated) are accepted.
///
/// # Errors
///
/// Only when the input itself is malformed.
pub fn opt_i64<'de, D: Deserializer<'de>>(d: D) -> Result<Option<i64>, D::Error> {
    d.deserialize_any(OptVisitor(PhantomData::<i64>))
}

/// Reads a float; numeric strings are accepted.
///
/// # Errors
///
/// Only when the input itself is malformed.
pub fn opt_f64<'de, D: Deserializer<'de>>(d: D) -> Result<Option<f64>, D::Error> {
    d.deserialize_any(OptVisitor(PhantomData::<f64>))
}

/// Reads a boolean; `0`/`1` and `"true"`/`"false"` are accepted.
///
/// # Errors
///
/// Only when the input itself is malformed.
pub fn opt_bool<'de, D: Deserializer<'de>>(d: D) -> Result<Option<bool>, D::Error> {
    d.deserialize_any(OptVisitor(PhantomData::<bool>))
}

/// Reads an array; `null` and non-array values become an empty `Vec`.
///
/// # Errors
///
/// When an element cannot be read as `T`.
pub fn vec<'de, D: Deserializer<'de>, T: Deserialize<'de>>(d: D) -> Result<Vec<T>, D::Error> {
    d.deserialize_any(VecVisitor(PhantomData))
}

/// Reads an object; `null` and non-object values (`""`, `false`, ...) become `None`.
///
/// # Errors
///
/// When the object cannot be read as `T`.
pub fn opt_object<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    d: D,
) -> Result<Option<T>, D::Error> {
    d.deserialize_any(ObjectVisitor(PhantomData))
}

// 型ごとの変換規則。読めない値は None（項目が無いのと同じ扱い）にする
trait Lenient: Sized {
    const EXPECTING: &'static str;
    fn from_str(v: &str) -> Option<Self>;
    fn from_i64(v: i64) -> Option<Self>;
    fn from_u64(v: u64) -> Option<Self>;
    fn from_f64(v: f64) -> Option<Self>;
    fn from_bool(v: bool) -> Option<Self>;
}

impl Lenient for String {
    const EXPECTING: &'static str = "a string";
    fn from_str(v: &str) -> Option<Self> {
        Some(v.to_owned())
    }
    fn from_i64(v: i64) -> Option<Self> {
        Some(v.to_string())
    }
    fn from_u64(v: u64) -> Option<Self> {
        Some(v.to_string())
    }
    fn from_f64(v: f64) -> Option<Self> {
        Some(v.to_string())
    }
    fn from_bool(v: bool) -> Option<Self> {
        Some(v.to_string())
    }
}

impl Lenient for i64 {
    const EXPECTING: &'static str = "an integer";
    fn from_str(v: &str) -> Option<Self> {
        v.parse::<i64>()
            .ok()
            .or_else(|| v.parse::<f64>().ok().map(|f| f as i64))
    }
    fn from_i64(v: i64) -> Option<Self> {
        Some(v)
    }
    fn from_u64(v: u64) -> Option<Self> {
        i64::try_from(v).ok()
    }
    fn from_f64(v: f64) -> Option<Self> {
        Some(v as i64)
    }
    fn from_bool(v: bool) -> Option<Self> {
        Some(i64::from(v))
    }
}

impl Lenient for f64 {
    const EXPECTING: &'static str = "a number";
    fn from_str(v: &str) -> Option<Self> {
        v.parse().ok()
    }
    fn from_i64(v: i64) -> Option<Self> {
        Some(v as f64)
    }
    fn from_u64(v: u64) -> Option<Self> {
        Some(v as f64)
    }
    fn from_f64(v: f64) -> Option<Self> {
        Some(v)
    }
    fn from_bool(v: bool) -> Option<Self> {
        Some(if v { 1.0 } else { 0.0 })
    }
}

impl Lenient for bool {
    const EXPECTING: &'static str = "a boolean";
    fn from_str(v: &str) -> Option<Self> {
        match v {
            "true" | "1" => Some(true),
            "false" | "0" | "" => Some(false),
            _ => None,
        }
    }
    fn from_i64(v: i64) -> Option<Self> {
        Some(v != 0)
    }
    fn from_u64(v: u64) -> Option<Self> {
        Some(v != 0)
    }
    fn from_f64(v: f64) -> Option<Self> {
        Some(v != 0.0)
    }
    fn from_bool(v: bool) -> Option<Self> {
        Some(v)
    }
}

struct OptVisitor<T>(PhantomData<T>);

impl<'de, T: Lenient> Visitor<'de> for OptVisitor<T> {
    type Value = Option<T>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(T::EXPECTING)
    }
    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(T::from_str(v))
    }
    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(T::from_i64(v))
    }
    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(T::from_u64(v))
    }
    fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
        Ok(T::from_f64(v))
    }
    fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(T::from_bool(v))
    }
    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }
    // スカラーのはずの項目に配列・オブジェクトが来ても応答全体は失敗させない
    fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        while seq.next_element::<IgnoredAny>()?.is_some() {}
        Ok(None)
    }
    fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        while map.next_entry::<IgnoredAny, IgnoredAny>()?.is_some() {}
        Ok(None)
    }
}

// 期待した形でない値を読み飛ばすための共通の受け口
macro_rules! skip_other_values {
    ($empty:expr) => {
        fn visit_str<E: de::Error>(self, _: &str) -> Result<Self::Value, E> {
            Ok($empty)
        }
        fn visit_i64<E: de::Error>(self, _: i64) -> Result<Self::Value, E> {
            Ok($empty)
        }
        fn visit_u64<E: de::Error>(self, _: u64) -> Result<Self::Value, E> {
            Ok($empty)
        }
        fn visit_f64<E: de::Error>(self, _: f64) -> Result<Self::Value, E> {
            Ok($empty)
        }
        fn visit_bool<E: de::Error>(self, _: bool) -> Result<Self::Value, E> {
            Ok($empty)
        }
        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok($empty)
        }
    };
}

struct VecVisitor<T>(PhantomData<T>);

impl<'de, T: Deserialize<'de>> Visitor<'de> for VecVisitor<T> {
    type Value = Vec<T>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("an array")
    }
    fn visit_seq<A: de::SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
        Vec::deserialize(de::value::SeqAccessDeserializer::new(seq))
    }
    fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        while map.next_entry::<IgnoredAny, IgnoredAny>()?.is_some() {}
        Ok(Vec::new())
    }
    skip_other_values!(Vec::new());
}

struct ObjectVisitor<T>(PhantomData<T>);

impl<'de, T: Deserialize<'de>> Visitor<'de> for ObjectVisitor<T> {
    type Value = Option<T>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("an object")
    }
    fn visit_map<A: de::MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        T::deserialize(de::value::MapAccessDeserializer::new(map)).map(Some)
    }
    fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        while seq.next_element::<IgnoredAny>()?.is_some() {}
        Ok(None)
    }
    skip_other_values!(None);
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Row {
        #[serde(default, deserialize_with = "opt_string")]
        s: Option<String>,
        #[serde(default, deserialize_with = "opt_i64")]
        i: Option<i64>,
        #[serde(default, deserialize_with = "opt_f64")]
        f: Option<f64>,
        #[serde(default, deserialize_with = "opt_bool")]
        b: Option<bool>,
        #[serde(default, deserialize_with = "vec")]
        v: Vec<u8>,
    }

    fn row(v: serde_json::Value) -> Row {
        serde_json::from_value(v).unwrap()
    }

    #[test]
    fn strings_accept_scalars() {
        assert_eq!(row(json!({"s": "x"})).s.as_deref(), Some("x"));
        assert_eq!(row(json!({"s": -1})).s.as_deref(), Some("-1"));
        assert_eq!(row(json!({"s": 1})).s.as_deref(), Some("1"));
        assert_eq!(row(json!({"s": 1.5})).s.as_deref(), Some("1.5"));
        assert_eq!(row(json!({"s": true})).s.as_deref(), Some("true"));
        assert_eq!(row(json!({"s": null})).s, None);
        assert_eq!(row(json!({})).s, None);
    }

    #[test]
    fn integers_accept_strings_and_floats() {
        assert_eq!(row(json!({"i": 5})).i, Some(5));
        assert_eq!(row(json!({"i": -5})).i, Some(-5));
        assert_eq!(row(json!({"i": "1616127454"})).i, Some(1616127454));
        assert_eq!(row(json!({"i": "1.9"})).i, Some(1));
        assert_eq!(row(json!({"i": "x"})).i, None);
        assert_eq!(row(json!({"i": 2.7})).i, Some(2));
        assert_eq!(row(json!({"i": u64::MAX})).i, None);
        assert_eq!(row(json!({"i": true})).i, Some(1));
    }

    #[test]
    fn floats_accept_strings_and_integers() {
        assert_eq!(row(json!({"f": 0.5})).f, Some(0.5));
        assert_eq!(row(json!({"f": "0.25"})).f, Some(0.25));
        assert_eq!(row(json!({"f": -2})).f, Some(-2.0));
        assert_eq!(row(json!({"f": 3})).f, Some(3.0));
        assert_eq!(row(json!({"f": true})).f, Some(1.0));
        assert_eq!(row(json!({"f": false})).f, Some(0.0));
    }

    #[test]
    fn booleans_accept_numbers_and_strings() {
        assert_eq!(row(json!({"b": true})).b, Some(true));
        assert_eq!(row(json!({"b": "true"})).b, Some(true));
        assert_eq!(row(json!({"b": "1"})).b, Some(true));
        assert_eq!(row(json!({"b": "false"})).b, Some(false));
        assert_eq!(row(json!({"b": "0"})).b, Some(false));
        assert_eq!(row(json!({"b": ""})).b, Some(false));
        assert_eq!(row(json!({"b": "maybe"})).b, None);
        assert_eq!(row(json!({"b": 0})).b, Some(false));
        assert_eq!(row(json!({"b": -1})).b, Some(true));
        assert_eq!(row(json!({"b": 0.0})).b, Some(false));
    }

    #[test]
    fn structured_values_in_scalar_fields_are_dropped() {
        let r = row(json!({"s": [1, 2], "i": {"a": 1}, "v": [1]}));
        assert_eq!(r.s, None);
        assert_eq!(r.i, None);
        assert_eq!(r.v, vec![1]);
    }

    #[test]
    fn non_array_vec_becomes_empty() {
        for v in [
            json!(null),
            json!("x"),
            json!(-1),
            json!(1),
            json!(1.5),
            json!(true),
            json!({"a": 1}),
        ] {
            assert!(row(json!({ "v": v })).v.is_empty());
        }
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Inner {
        a: u8,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Outer {
        #[serde(default, deserialize_with = "opt_object")]
        o: Option<Inner>,
    }

    #[test]
    fn objects_accept_only_maps() {
        let o = |v: serde_json::Value| {
            serde_json::from_value::<Outer>(json!({ "o": v }))
                .unwrap()
                .o
        };
        assert_eq!(o(json!({"a": 1})), Some(Inner { a: 1 }));
        for v in [
            json!(null),
            json!(""),
            json!(-1),
            json!(1),
            json!(1.5),
            json!(false),
            json!([1, 2]),
        ] {
            assert_eq!(o(v), None);
        }
        assert!(serde_json::from_value::<Outer>(json!({"o": {"a": "x"}})).is_err());
    }

    #[test]
    fn expecting_message_names_the_type() {
        // deserialize_any を使わない形式（bincode 等）では expecting が使われる。文言だけ確かめる
        struct Probe<T>(PhantomData<T>);
        impl<T: Lenient> fmt::Display for Probe<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                OptVisitor::<T>(PhantomData).expecting(f)
            }
        }
        assert_eq!(Probe::<String>(PhantomData).to_string(), "a string");
        assert_eq!(Probe::<i64>(PhantomData).to_string(), "an integer");
        assert_eq!(Probe::<f64>(PhantomData).to_string(), "a number");
        assert_eq!(Probe::<bool>(PhantomData).to_string(), "a boolean");
        struct Expect<V>(V);
        impl<'de, V: Visitor<'de>> fmt::Display for Expect<V> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.expecting(f)
            }
        }
        assert_eq!(
            Expect(VecVisitor::<u8>(PhantomData)).to_string(),
            "an array"
        );
        assert_eq!(
            Expect(ObjectVisitor::<u8>(PhantomData)).to_string(),
            "an object"
        );
    }

    #[test]
    fn bytes_and_char_inputs_are_rejected_with_type_message() {
        // serde_json は bytes を出さないので、IntoDeserializer で確かめる
        use serde::de::value::{BytesDeserializer, Error};
        let err = opt_string(BytesDeserializer::<Error>::new(b"x")).unwrap_err();
        assert!(err.to_string().contains("a string"));
    }
}
