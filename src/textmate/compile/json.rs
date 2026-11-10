//! A grammar's JSON as a tree that keeps the order of every object's keys.
//!
//! `serde_json::Value` sorts an object's keys unless `serde_json`'s
//! `preserve_order` feature is on, which adds crates.
//! The order of a grammar's `injections` keys sets the order of its
//! injections, as vscode-textmate reads them.
//! So the compiler reads the text through a visitor of its own.
//!
//! Shiki's grammars are read by JavaScript.
//! So the tree answers the compiler's questions as JavaScript does:
//! whether a value is truthy, and what `String(value)` makes of it.

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt;

use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};

/// A JSON value.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Json {
    /// `null`.
    Null,
    /// `true` or `false`.
    Bool(bool),
    /// A number, as the parser read it.
    Number(Number),
    /// A string.
    String(String),
    /// An array.
    Array(Vec<Json>),
    /// An object, its keys in the order of the text.
    /// A key written twice keeps its first place and its last value, as
    /// `JSON.parse` does.
    Object(Vec<(String, Json)>),
}

/// A JSON number. It is kept in the type the parser read it as, so no
/// conversion can lose a digit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Number {
    /// A whole number of zero or more.
    Unsigned(u64),
    /// A negative whole number.
    Signed(i64),
    /// A number with a fraction or an exponent.
    Float(f64),
}

impl Json {
    /// Reads `text` as JSON.
    pub(crate) fn parse(text: &str) -> Result<Json, serde_json::Error> {
        serde_json::from_str(text)
    }

    /// The value of `key`, when the value is an object that has it.
    pub(crate) fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Object(entries) => entries.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            Json::Null | Json::Bool(_) | Json::Number(_) | Json::String(_) | Json::Array(_) => None,
        }
    }

    /// The value of `key` unless it is missing or `null`, as JavaScript's
    /// `??` reads it.
    pub(crate) fn present(&self, key: &str) -> Option<&Json> {
        self.get(key).filter(|value| !matches!(value, Json::Null))
    }

    /// The value of `key` when it is truthy.
    pub(crate) fn truthy(&self, key: &str) -> Option<&Json> {
        self.get(key).filter(|value| value.is_truthy())
    }

    /// The string value of `key`, when it is a string.
    pub(crate) fn str(&self, key: &str) -> Option<&str> {
        match self.get(key)? {
            Json::String(text) => Some(text),
            Json::Null | Json::Bool(_) | Json::Number(_) | Json::Array(_) | Json::Object(_) => None,
        }
    }

    /// Whether JavaScript reads the value as true in a condition.
    pub(crate) fn is_truthy(&self) -> bool {
        match self {
            Json::Null => false,
            Json::Bool(value) => *value,
            Json::Number(Number::Unsigned(n)) => *n != 0,
            Json::Number(Number::Signed(n)) => *n != 0,
            Json::Number(Number::Float(n)) => n.abs() > 0.0,
            Json::String(text) => !text.is_empty(),
            Json::Array(_) | Json::Object(_) => true,
        }
    }

    /// Whether the value is the number one or `true`, which is how
    /// `applyEndPatternLast` turns on.
    pub(crate) fn is_one_or_true(&self) -> bool {
        match self {
            Json::Bool(value) => *value,
            Json::Number(Number::Unsigned(n)) => *n == 1,
            Json::Number(Number::Signed(n)) => *n == 1,
            Json::Number(Number::Float(n)) => n.total_cmp(&1.0).is_eq(),
            Json::Null | Json::String(_) | Json::Array(_) | Json::Object(_) => false,
        }
    }

    /// What JavaScript's `String(value)` makes of a string, a number, a
    /// boolean or `null`.
    ///
    /// An array or an object gives `[object Object]`.
    /// No grammar writes a pattern that way.
    pub(crate) fn js_string(&self) -> String {
        match self {
            Json::Null => "null".to_owned(),
            Json::Bool(value) => value.to_string(),
            Json::Number(Number::Unsigned(n)) => n.to_string(),
            Json::Number(Number::Signed(n)) => n.to_string(),
            Json::Number(Number::Float(n)) => n.to_string(),
            Json::String(text) => text.clone(),
            Json::Array(_) | Json::Object(_) => "[object Object]".to_owned(),
        }
    }
}

impl<'de> Deserialize<'de> for Json {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Json, D::Error> {
        deserializer.deserialize_any(JsonVisitor)
    }
}

/// Builds a [`Json`] from the parser's events.
struct JsonVisitor;

impl<'de> Visitor<'de> for JsonVisitor {
    type Value = Json;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON value")
    }

    fn visit_unit<E>(self) -> Result<Json, E> {
        Ok(Json::Null)
    }

    fn visit_bool<E>(self, value: bool) -> Result<Json, E> {
        Ok(Json::Bool(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Json, E> {
        Ok(Json::Number(Number::Unsigned(value)))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Json, E> {
        Ok(Json::Number(Number::Signed(value)))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Json, E> {
        Ok(Json::Number(Number::Float(value)))
    }

    fn visit_str<E>(self, value: &str) -> Result<Json, E> {
        Ok(Json::String(value.to_owned()))
    }

    fn visit_string<E>(self, value: String) -> Result<Json, E> {
        Ok(Json::String(value))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Json, A::Error> {
        let mut items = Vec::new();
        while let Some(item) = seq.next_element()? {
            items.push(item);
        }
        Ok(Json::Array(items))
    }

    /// A map from each key to its place finds a repeated key in one lookup.
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Json, A::Error> {
        let mut entries: Vec<(String, Json)> = Vec::new();
        let mut places: HashMap<String, usize> = HashMap::new();
        while let Some((key, value)) = map.next_entry::<String, Json>()? {
            match places.entry(key) {
                Entry::Occupied(place) => {
                    if let Some(entry) = entries.get_mut(*place.get()) {
                        entry.1 = value;
                    }
                }
                Entry::Vacant(place) => {
                    let at = entries.len();
                    entries.push((place.key().clone(), value));
                    place.insert(at);
                }
            }
        }
        Ok(Json::Object(entries))
    }
}
