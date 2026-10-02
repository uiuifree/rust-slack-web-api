// リクエストを `application/x-www-form-urlencoded` に直列化する。
//
// Slack の全メソッドがこの形式を受け付ける（JSON 本文は一部のメソッドが受け付けない）。
// 値の扱いは公式 SDK と同じ:
// - 文字列・数値・真偽値はそのまま
// - 文字列などの配列はカンマ区切り（`U1,U2`）
// - オブジェクト・オブジェクトの配列（blocks・attachments 等）は JSON 文字列
// - `None` は送らない
//
// `serde_json::Value` の木を経由せず、フィールドごとに直接書き出す。

use serde::ser::{self, Impossible, Serialize};
use std::fmt;

// 構造体（またはマップ）をフォーム本文にする
pub(crate) fn to_form<T: Serialize + ?Sized>(value: &T) -> Result<String, Error> {
    let mut out = form_urlencoded::Serializer::new(String::new());
    value.serialize(TopSerializer { out: &mut out })?;
    Ok(out.finish())
}

/// `serialize_with` helper that sends a list as a JSON array string instead of a comma-separated list.
/// Used by arguments whose documented example looks like `["C1","C2"]`.
///
/// # Errors
///
/// Fails when `value` cannot be serialized to JSON (for example, a map with non-string keys).
pub fn as_json<T: Serialize, S: ser::Serializer>(
    value: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let json = serde_json::to_string(value).map_err(ser::Error::custom)?;
    serializer.serialize_str(&json)
}

#[derive(Debug)]
pub(crate) enum Error {
    // フィールドの値がスカラーで表せない（JSON にフォールバックする合図）
    NotScalar,
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotScalar => f.write_str("value is not a scalar"),
            Error::Custom(msg) => f.write_str(msg),
        }
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

type Out = form_urlencoded::Serializer<'static, String>;

// 1フィールド分の値を書き出す。スカラー・スカラー配列ならそのまま、それ以外は JSON 文字列
fn write_field<T: Serialize + ?Sized>(out: &mut Out, key: &str, value: &T) -> Result<(), Error> {
    match value.serialize(ScalarSerializer) {
        Ok(Some(text)) => {
            out.append_pair(key, &text);
            Ok(())
        }
        Ok(None) => Ok(()),
        Err(Error::NotScalar) => {
            let json = serde_json::to_string(value).map_err(ser::Error::custom)?;
            out.append_pair(key, &json);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

struct TopSerializer<'a> {
    out: &'a mut Out,
}

macro_rules! top_level_only {
    ($($name:ident($($arg:ty),*) -> $ret:ty;)*) => {
        $(fn $name(self, $(_: $arg),*) -> Result<$ret, Error> {
            Err(Error::Custom("form body must be a struct or map".into()))
        })*
    };
}

impl<'a> ser::Serializer for TopSerializer<'a> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = TopMap<'a>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self, Error> {
        Ok(self)
    }
    fn serialize_map(self, _: Option<usize>) -> Result<TopMap<'a>, Error> {
        Ok(TopMap {
            out: self.out,
            key: None,
        })
    }
    fn serialize_unit_struct(self, _: &'static str) -> Result<(), Error> {
        Ok(())
    }
    fn serialize_unit(self) -> Result<(), Error> {
        Ok(())
    }
    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        value.serialize(self)
    }

    top_level_only! {
        serialize_bool(bool) -> ();
        serialize_i8(i8) -> ();
        serialize_i16(i16) -> ();
        serialize_i32(i32) -> ();
        serialize_i64(i64) -> ();
        serialize_u8(u8) -> ();
        serialize_u16(u16) -> ();
        serialize_u32(u32) -> ();
        serialize_u64(u64) -> ();
        serialize_f32(f32) -> ();
        serialize_f64(f64) -> ();
        serialize_char(char) -> ();
        serialize_str(&str) -> ();
        serialize_bytes(&[u8]) -> ();
        serialize_none() -> ();
        serialize_unit_variant(&'static str, u32, &'static str) -> ();
        serialize_seq(Option<usize>) -> Self::SerializeSeq;
        serialize_tuple(usize) -> Self::SerializeTuple;
        serialize_tuple_struct(&'static str, usize) -> Self::SerializeTupleStruct;
        serialize_tuple_variant(&'static str, u32, &'static str, usize) -> Self::SerializeTupleVariant;
        serialize_struct_variant(&'static str, u32, &'static str, usize) -> Self::SerializeStructVariant;
    }
    fn serialize_some<T: Serialize + ?Sized>(self, _: &T) -> Result<(), Error> {
        Err(Error::Custom("form body must be a struct or map".into()))
    }
    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<(), Error> {
        Err(Error::Custom("form body must be a struct or map".into()))
    }
}

impl ser::SerializeStruct for TopSerializer<'_> {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        write_field(self.out, key, value)
    }
    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}

struct TopMap<'a> {
    out: &'a mut Out,
    key: Option<String>,
}

impl ser::SerializeMap for TopMap<'_> {
    type Ok = ();
    type Error = Error;
    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<(), Error> {
        match key.serialize(ScalarSerializer)? {
            Some(k) => {
                self.key = Some(k);
                Ok(())
            }
            None => Err(Error::Custom("map key must not be null".into())),
        }
    }
    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        let key = self
            .key
            .take()
            .ok_or_else(|| Error::Custom("map value without key".into()))?;
        write_field(self.out, &key, value)
    }
    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}

// スカラーを文字列にする。None/unit は Ok(None)（送らない）。構造を持つ値は NotScalar
struct ScalarSerializer;

macro_rules! scalar_display {
    ($($name:ident($ty:ty);)*) => {
        $(fn $name(self, v: $ty) -> Result<Option<String>, Error> {
            Ok(Some(v.to_string()))
        })*
    };
}

impl ser::Serializer for ScalarSerializer {
    type Ok = Option<String>;
    type Error = Error;
    type SerializeSeq = JoinSeq;
    type SerializeTuple = Impossible<Option<String>, Error>;
    type SerializeTupleStruct = Impossible<Option<String>, Error>;
    type SerializeTupleVariant = Impossible<Option<String>, Error>;
    type SerializeMap = Impossible<Option<String>, Error>;
    type SerializeStruct = Impossible<Option<String>, Error>;
    type SerializeStructVariant = Impossible<Option<String>, Error>;

    scalar_display! {
        serialize_bool(bool);
        serialize_i8(i8);
        serialize_i16(i16);
        serialize_i32(i32);
        serialize_i64(i64);
        serialize_u8(u8);
        serialize_u16(u16);
        serialize_u32(u32);
        serialize_u64(u64);
        serialize_f32(f32);
        serialize_f64(f64);
        serialize_char(char);
    }
    fn serialize_str(self, v: &str) -> Result<Option<String>, Error> {
        Ok(Some(v.to_owned()))
    }
    fn serialize_bytes(self, _: &[u8]) -> Result<Option<String>, Error> {
        Err(Error::NotScalar)
    }
    fn serialize_none(self) -> Result<Option<String>, Error> {
        Ok(None)
    }
    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<Option<String>, Error> {
        value.serialize(self)
    }
    fn serialize_unit(self) -> Result<Option<String>, Error> {
        Ok(None)
    }
    fn serialize_unit_struct(self, _: &'static str) -> Result<Option<String>, Error> {
        Ok(None)
    }
    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
    ) -> Result<Option<String>, Error> {
        Ok(Some(variant.to_owned()))
    }
    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<Option<String>, Error> {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<Option<String>, Error> {
        Err(Error::NotScalar)
    }
    fn serialize_seq(self, _: Option<usize>) -> Result<JoinSeq, Error> {
        Ok(JoinSeq {
            joined: String::new(),
            first: true,
        })
    }
    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Error> {
        Err(Error::NotScalar)
    }
    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        Err(Error::NotScalar)
    }
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        Err(Error::NotScalar)
    }
    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Error> {
        Err(Error::NotScalar)
    }
    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Error> {
        Err(Error::NotScalar)
    }
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        Err(Error::NotScalar)
    }
}

// スカラーの配列をカンマ区切りにする。要素がスカラーでなければ NotScalar（JSON にフォールバック）
struct JoinSeq {
    joined: String,
    first: bool,
}

impl ser::SerializeSeq for JoinSeq {
    type Ok = Option<String>;
    type Error = Error;
    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        // 配列の中の null はカンマ区切りで表せない
        let text = value.serialize(ScalarSerializer)?.ok_or(Error::NotScalar)?;
        if !self.first {
            self.joined.push(',');
        }
        self.first = false;
        self.joined.push_str(&text);
        Ok(())
    }
    fn end(self) -> Result<Option<String>, Error> {
        Ok(Some(self.joined))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use serde_json::json;
    use std::collections::BTreeMap;

    #[derive(Serialize)]
    struct Req {
        channel: String,
        limit: Option<i64>,
        inclusive: Option<bool>,
        ratio: f64,
        users: Vec<String>,
        blocks: Vec<serde_json::Value>,
        skipped: Option<String>,
        #[serde(serialize_with = "as_json")]
        ids: Vec<String>,
        letter: char,
        kind: Kind,
        unit: (),
        wrapped: Wrapped,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "snake_case")]
    enum Kind {
        PublicChannel,
    }

    #[derive(Serialize)]
    struct Wrapped(u8);

    fn sample() -> Req {
        Req {
            channel: "C1 #x&y".into(),
            limit: Some(200),
            inclusive: Some(true),
            ratio: 1.5,
            users: vec!["U1".into(), "U2".into()],
            blocks: vec![json!({"type": "divider"})],
            skipped: None,
            ids: vec!["A".into(), "B".into()],
            letter: 'z',
            kind: Kind::PublicChannel,
            unit: (),
            wrapped: Wrapped(7),
        }
    }

    fn decode(body: &str) -> Vec<(String, String)> {
        form_urlencoded::parse(body.as_bytes())
            .into_owned()
            .collect()
    }

    #[test]
    fn struct_fields_follow_slack_conventions() {
        let pairs = decode(&to_form(&sample()).unwrap());
        assert_eq!(
            pairs,
            vec![
                ("channel".into(), "C1 #x&y".into()),
                ("limit".into(), "200".into()),
                ("inclusive".into(), "true".into()),
                ("ratio".into(), "1.5".into()),
                ("users".into(), "U1,U2".into()),
                ("blocks".into(), r#"[{"type":"divider"}]"#.into()),
                ("ids".into(), r#"["A","B"]"#.into()),
                ("letter".into(), "z".into()),
                ("kind".into(), "public_channel".into()),
                ("wrapped".into(), "7".into()),
            ]
        );
    }

    #[test]
    fn integer_widths_are_written_as_numbers() {
        #[derive(Serialize)]
        struct Ints {
            a: i8,
            b: i16,
            c: i32,
            d: u16,
            e: u32,
            f: u64,
            g: f32,
        }
        let body = to_form(&Ints {
            a: -1,
            b: 2,
            c: 3,
            d: 4,
            e: 5,
            f: 6,
            g: 0.5,
        })
        .unwrap();
        assert_eq!(body, "a=-1&b=2&c=3&d=4&e=5&f=6&g=0.5");
    }

    #[test]
    fn map_and_value_bodies() {
        let mut map = BTreeMap::new();
        map.insert("channel", json!("C1"));
        map.insert("nested", json!({"a": 1}));
        map.insert("null", serde_json::Value::Null);
        map.insert("list", json!(["x", 1, true]));
        map.insert("mixed", json!(["x", null]));
        assert_eq!(
            decode(&to_form(&map).unwrap()),
            vec![
                ("channel".into(), "C1".into()),
                ("list".into(), "x,1,true".into()),
                ("mixed".into(), r#"["x",null]"#.into()),
                ("nested".into(), r#"{"a":1}"#.into()),
            ]
        );
        assert_eq!(to_form(&json!({"a": "b"})).unwrap(), "a=b");
    }

    #[test]
    fn non_scalar_shapes_fall_back_to_json() {
        #[derive(Serialize)]
        enum Shape {
            New(u8),
            Tuple(u8, u8),
            Struct { x: u8 },
        }
        #[derive(Serialize)]
        struct TupleStruct(u8, u8);
        #[derive(Serialize)]
        struct Inner {
            x: u8,
        }
        #[derive(Serialize)]
        struct Req<'a> {
            new: Shape,
            tuple_variant: Shape,
            struct_variant: Shape,
            tuple: (u8, u8),
            tuple_struct: TupleStruct,
            inner: Inner,
            #[serde(with = "serde_bytes_like")]
            bytes: &'a [u8],
        }
        mod serde_bytes_like {
            pub fn serialize<S: serde::Serializer>(v: &&[u8], s: S) -> Result<S::Ok, S::Error> {
                s.serialize_bytes(v)
            }
        }
        let body = to_form(&Req {
            new: Shape::New(1),
            tuple_variant: Shape::Tuple(1, 2),
            struct_variant: Shape::Struct { x: 3 },
            tuple: (4, 5),
            tuple_struct: TupleStruct(6, 7),
            inner: Inner { x: 8 },
            bytes: b"hi",
        })
        .unwrap();
        assert_eq!(
            decode(&body),
            vec![
                ("new".into(), r#"{"New":1}"#.into()),
                ("tuple_variant".into(), r#"{"Tuple":[1,2]}"#.into()),
                ("struct_variant".into(), r#"{"Struct":{"x":3}}"#.into()),
                ("tuple".into(), "[4,5]".into()),
                ("tuple_struct".into(), "[6,7]".into()),
                ("inner".into(), r#"{"x":8}"#.into()),
                ("bytes".into(), "[104,105]".into()),
            ]
        );
    }

    #[test]
    fn unit_struct_fields_are_skipped() {
        #[derive(Serialize)]
        struct Marker;
        #[derive(Serialize)]
        struct Req {
            a: Marker,
            b: u8,
        }
        assert_eq!(to_form(&Req { a: Marker, b: 1 }).unwrap(), "b=1");
    }

    #[test]
    fn unit_and_newtype_top_levels() {
        #[derive(Serialize)]
        struct Empty;
        #[derive(Serialize)]
        struct Newtype(BTreeMap<&'static str, &'static str>);
        assert_eq!(to_form(&Empty).unwrap(), "");
        assert_eq!(to_form(&()).unwrap(), "");
        assert_eq!(
            to_form(&Newtype(BTreeMap::from([("a", "b")]))).unwrap(),
            "a=b"
        );
    }

    #[test]
    fn scalar_top_levels_are_rejected() {
        #[derive(Serialize)]
        enum E {
            Unit,
            New(u8),
            Tuple(u8),
            Struct { x: u8 },
        }
        #[derive(Serialize)]
        struct TupleStruct(u8, u8);
        let rejected: Vec<Result<String, Error>> = vec![
            to_form(&true),
            to_form(&1i8),
            to_form(&1i16),
            to_form(&1i32),
            to_form(&1i64),
            to_form(&1u8),
            to_form(&1u16),
            to_form(&1u32),
            to_form(&1u64),
            to_form(&1f32),
            to_form(&1f64),
            to_form(&'c'),
            to_form("s"),
            to_form(&Some(1)),
            to_form(&E::Unit),
            to_form(&E::New(1)),
            to_form(&E::Struct { x: 1 }),
            to_form(&vec![1]),
            to_form(&(1, 2)),
            to_form(&TupleStruct(1, 2)),
            to_form(&E::Tuple(1)),
            to_form(&Option::<u8>::None),
        ];
        for r in rejected {
            let err = r.unwrap_err();
            assert_eq!(err.to_string(), "form body must be a struct or map");
        }
        assert!(to_form(serde_bytes_ref(b"x")).is_err());
    }

    struct BytesRef<'a>(&'a [u8]);
    impl Serialize for BytesRef<'_> {
        fn serialize<S: ser::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            s.serialize_bytes(self.0)
        }
    }
    fn serde_bytes_ref(b: &[u8]) -> &BytesRef<'_> {
        Box::leak(Box::new(BytesRef(b)))
    }

    #[test]
    fn map_key_errors() {
        let mut null_key = BTreeMap::new();
        null_key.insert(Option::<&str>::None, 1);
        assert_eq!(
            to_form(&null_key).unwrap_err().to_string(),
            "map key must not be null"
        );

        let mut struct_key = BTreeMap::new();
        struct_key.insert(vec![("a", 1)], 1);
        // スカラーでないキー（タプルの配列）は NotScalar のまま返る
        assert_eq!(
            to_form(&struct_key).unwrap_err().to_string(),
            "value is not a scalar"
        );
    }

    #[test]
    fn value_without_key_is_an_error() {
        use ser::SerializeMap;
        let mut out = form_urlencoded::Serializer::new(String::new());
        let mut map = TopMap {
            out: &mut out,
            key: None,
        };
        assert_eq!(
            map.serialize_value(&1).unwrap_err().to_string(),
            "map value without key"
        );
    }

    #[test]
    fn custom_errors_propagate() {
        struct Fails;
        impl Serialize for Fails {
            fn serialize<S: ser::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
                Err(ser::Error::custom("boom"))
            }
        }
        #[derive(Serialize)]
        struct Req {
            f: Fails,
        }
        assert_eq!(to_form(&Req { f: Fails }).unwrap_err().to_string(), "boom");

        // JSON へのフォールバックでも失敗するもの（キーが文字列でないマップ）
        #[derive(Serialize)]
        struct Bad {
            m: BTreeMap<Vec<u8>, u8>,
        }
        let err = to_form(&Bad {
            m: BTreeMap::from([(vec![1], 1)]),
        })
        .unwrap_err();
        assert!(matches!(err, Error::Custom(_)));

        #[derive(Serialize)]
        struct BadJson {
            #[serde(serialize_with = "as_json")]
            m: BTreeMap<Vec<u8>, u8>,
        }
        assert!(to_form(&BadJson {
            m: BTreeMap::from([(vec![1], 1)])
        })
        .is_err());
    }
}
