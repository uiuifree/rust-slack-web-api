use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use super::*;

mod api_examples;
mod builders;
mod doc_examples;
mod enums;
mod structs;

/// 値を JSON にすると `expected` と一致し、`expected` を読むと元の値に戻ることを確かめる。
fn assert_json<T: Serialize + DeserializeOwned + PartialEq + Debug>(value: &T, expected: &str) {
    assert_eq!(&serde_json::from_str::<T>(expected).unwrap(), value);
    let expected: Value = serde_json::from_str(expected).unwrap();
    assert_eq!(serde_json::to_value(value).unwrap(), expected);
}

/// 未知の種別として読まれ、書き出すと元の JSON に戻ることを確かめる。
fn assert_unknown<T: Serialize + DeserializeOwned + Debug>(json: &str, is_unknown: fn(&T) -> bool) {
    let value: T = serde_json::from_str(json).unwrap();
    let original: Value = serde_json::from_str(json).unwrap();
    assert!(is_unknown(&value), "{value:?}");
    assert_eq!(serde_json::to_value(&value).unwrap(), original);
}

/// ドキュメントや API の JSON 例を読み、既知の型だけで読めて（Unknown が混ざらず）、書き戻すと同じ意味になることを確かめる。
fn roundtrip<T: Serialize + DeserializeOwned + Debug>(json: &str) -> T {
    let value: T = serde_json::from_str(json).unwrap();
    let original: Value = serde_json::from_str(json).unwrap();
    assert!(!format!("{value:?}").contains("Unknown("), "{value:?}");
    assert_eq!(
        normalize(serde_json::to_value(&value).unwrap()),
        normalize(original)
    );
    value
}

/// `{"blocks": [...]}` 形式の例を `Vec<Block>` として確かめる。
fn roundtrip_blocks(json: &str) -> Vec<Block> {
    let mut original: Value = serde_json::from_str(json).unwrap();
    let blocks = original.as_object_mut().unwrap().remove("blocks").unwrap();
    assert_eq!(original, Value::Object(Default::default()));
    roundtrip(&blocks.to_string())
}

/// 意味の同じ JSON を同じ形にそろえる。`null` のキーは省略と同じに、数値は 45 と 45.0 を同じに扱う。
fn normalize(value: Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(
            map.into_iter()
                .filter(|(_, v)| !v.is_null())
                .map(|(k, v)| (k, normalize(v)))
                .collect(),
        ),
        Value::Array(items) => Value::Array(items.into_iter().map(normalize).collect()),
        Value::Number(n) => Value::from(n.as_f64().unwrap()),
        other => other,
    }
}
