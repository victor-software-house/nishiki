//! giallo, a `TextMate` engine in Rust on Shiki's grammars. It is EUPL-1.2,
//! so this module builds only with the `giallo` feature, on the bench box,
//! and never ships (bench/compare/README.md).

use std::hint::black_box;

use giallo::{HighlightOptions, Registry, ThemeVariant};
use nishiki::Language;

use crate::engines::{Colours, Engine};

/// giallo's built-in registry, with Shiki's grammars and themes.
pub struct Giallo {
    registry: Registry,
    options: HighlightOptions,
}

impl Giallo {
    pub fn new(language: Language) -> Option<Giallo> {
        let mut registry = Registry::builtin().ok()?;
        registry.link_grammars();
        let options = HighlightOptions::new(language.id(), ThemeVariant::Single("github-dark"));
        // A language giallo lacks fails here, during setup.
        registry.highlight("", &options).ok()?;
        Some(Giallo { registry, options })
    }
}

impl Engine for Giallo {
    fn run(&mut self, text: &str) -> usize {
        match self.registry.highlight(text, &self.options) {
            Ok(code) => {
                let count = code.tokens.iter().map(Vec::len).sum();
                black_box(code);
                count
            }
            Err(_) => 0,
        }
    }

    fn colours(&mut self, text: &str) -> Colours {
        let Ok(code) = self.registry.highlight(text, &self.options) else {
            return Colours::new();
        };
        code.tokens
            .iter()
            .map(|line| {
                let mut end = 0;
                line.iter()
                    .map(|token| {
                        end += token.text.len();
                        let colour = match &token.style {
                            ThemeVariant::Single(style) => style.foreground.as_hex(),
                            ThemeVariant::Dual { dark, .. } => dark.foreground.as_hex(),
                        };
                        (end, colour.to_lowercase())
                    })
                    .collect()
            })
            .collect()
    }
}
