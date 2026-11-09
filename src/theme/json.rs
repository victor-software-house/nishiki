//! Theme files in Shiki's JSON format.
//!
//! `settings` wins over `tokenColors` when a file has both, as in Shiki.
//! Fields are read leniently, as VS Code reads them: a field of the wrong
//! type is ignored rather than rejected.
//! Any theme VS Code or Shiki loads therefore loads here.

use std::{error, fmt};

use serde_json::{Map, Value};

use super::{RuleScope, ThemeKind, ThemeRule, ThemeSource};

/// A theme file that could not be read.
#[derive(Debug)]
pub struct ThemeError(Reason);

#[derive(Debug)]
enum Reason {
    Json(serde_json::Error),
    NotAnObject,
}

impl fmt::Display for ThemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Reason::Json(_) => f.write_str("the theme is not valid JSON"),
            Reason::NotAnObject => f.write_str("the theme is not a JSON object"),
        }
    }
}

impl error::Error for ThemeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.0 {
            Reason::Json(cause) => Some(cause),
            Reason::NotAnObject => None,
        }
    }
}

fn text(object: &Map<String, Value>, key: &str) -> Option<String> {
    object
        .get(key)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}

fn rule(entry: &Value) -> Option<ThemeRule> {
    let entry = entry.as_object()?;
    let settings = entry.get("settings")?.as_object()?;
    let scope = match entry.get("scope") {
        Some(Value::String(selectors)) => RuleScope::Selectors(selectors.clone()),
        Some(Value::Array(list)) => RuleScope::List(
            list.iter()
                .filter_map(Value::as_str)
                .map(ToOwned::to_owned)
                .collect(),
        ),
        _ => RuleScope::Defaults,
    };
    Some(ThemeRule {
        name: text(entry, "name"),
        scope,
        foreground: text(settings, "foreground"),
        background: text(settings, "background"),
        font_style: text(settings, "fontStyle"),
    })
}

pub(super) fn source(json: &str) -> Result<ThemeSource, ThemeError> {
    let value: Value =
        serde_json::from_str(json).map_err(|cause| ThemeError(Reason::Json(cause)))?;
    let theme = value.as_object().ok_or(ThemeError(Reason::NotAnObject))?;
    let colors = theme.get("colors").and_then(Value::as_object);
    let rules = theme
        .get("settings")
        .or_else(|| theme.get("tokenColors"))
        .and_then(Value::as_array)
        .map(|entries| entries.iter().filter_map(rule).collect())
        .unwrap_or_default();
    Ok(ThemeSource {
        name: text(theme, "name"),
        kind: match theme.get("type").and_then(Value::as_str) {
            Some("light") => Some(ThemeKind::Light),
            Some("dark") => Some(ThemeKind::Dark),
            _ => None,
        },
        editor_foreground: colors.and_then(|colors| text(colors, "editor.foreground")),
        editor_background: colors.and_then(|colors| text(colors, "editor.background")),
        rules,
    })
}
