//! The matchers: for each pattern of a language on the rule engine, the
//! function that decides whether it matches at a position and where.
//!
//! Each is written from the pattern's parsed tree ([`writer`]).
//! [`shape`] holds the analysis that lets a matcher skip giving characters
//! back.
//! [`patterns`] names the patterns as the language's `Pattern` enum does.

mod chars;
mod patterns;
mod shape;
#[cfg(test)]
mod tests;
mod writer;

use std::collections::BTreeSet;
use std::path::Path;

use chars::Chars;
use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::{format_ident, quote};
use syn::parse::Parser;
use writer::{ClassFunction, Matcher, Refusal, anchor_test, hex, matcher};

use crate::File;
use crate::error::Error;
use crate::header::header;
use crate::source::Source;

/// Every matcher file, generated.
pub(crate) fn files(root: &Path) -> Result<Vec<File>, Error> {
    Ok(vec![language(root, "proto", "src/proto/matchers.rs")?])
}

/// The runtime items a matcher file may call, from
/// `crate::rules::generated`.
const RUNTIME: [&str; 7] = [
    "Found",
    "Skip",
    "byte_at",
    "char_at",
    "in_ranges",
    "prev_boundary",
    "record",
];

/// The lexer methods that come from the `crate::rules::Chars` trait.
const CHARS: [&str; 4] = ["boundary", "space_end", "word_at", "word_before"];

/// The most lines of code clippy's `too_many_lines` lets a function body hold.
const FUNCTION_LINES: usize = 100;

/// The matcher file of the grammar `name`, written to `path`.
///
/// Every pattern must get a matcher: the dispatch matches on every variant.
/// The language keeps scan skips in its lexer's `x` field.
fn language(root: &Path, name: &'static str, path: &'static str) -> Result<File, Error> {
    let source = Source::load(root, name)?;
    let mut arms = Vec::new();
    let mut items = Vec::new();
    let mut fields = Vec::new();
    let mut tables = Tables::default();
    for pattern in patterns::patterns(source)? {
        let field = format_ident!("{}", snake(&pattern.name));
        let function = format_ident!("match_{field}");
        let skips = quote!(lx.x.#field);
        let Matcher {
            items: code,
            classes,
            skips: skipping,
            leading,
        } = matcher(pattern.regex, &function, &pattern.reads, Some(&skips)).map_err(
            |Refusal(why)| {
                Error::Shape(format!(
                    "{name}: {} has no generated matcher: {why}",
                    pattern.name
                ))
            },
        )?;
        let variant = format_ident!("{}", pattern.name);
        let call = quote!(#function(self, p));
        let call = match leading {
            Some(anchor) => {
                let test = anchor_test(anchor, &format_ident!("self"), &format_ident!("p"))
                    .map_err(|Refusal(why)| {
                        Error::Shape(format!(
                            "{name}: {} has no dispatch test: {why}",
                            pattern.name
                        ))
                    })?;
                quote!(if #test { #call } else { None })
            }
            None => call,
        };
        arms.push(quote!(Pattern::#variant => #call,));
        if skipping {
            fields.push(quote!(#field: Skip,));
        }
        items.push(code);
        items.extend(classes.iter().map(|class| tables.function(class)));
    }
    let tables = tables.statics();
    let skips = (!fields.is_empty()).then(|| {
        quote! {
            /// Each skipping pattern's last failure on the line.
            #[derive(Debug, Default)]
            pub(crate) struct Skips {
                #(#fields)*
            }
        }
    });
    let body = quote! {
        #skips

        impl Lexer<'_> {
            /// Whether `pattern` matches at `p`, and where its match ends.
            /// It is inlined into the language's `matcher`, so one dispatch
            /// picks the pattern's function.
            #[inline(always)]
            pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
                match pattern {
                    #(#arms)*
                }
            }
        }

        #(#items)*

        #tables
    };
    let imports = imports(&body);
    let file = quote! {
        #![doc = " The matchers: for each pattern of the grammar, the function that decides"]
        #![doc = " whether it matches at a position and where."]
        #![doc = " Each is written from the pattern's parsed tree."]
        #![expect(
            clippy::inline_always,
            reason = "each test inlines into its matcher, and the dispatch into the grammar's"
        )]

        use super::engine::Lexer;
        use super::grammar::Pattern;
        #imports

        #body
    };
    let mut syntax = syn::parse2::<syn::File>(file).map_err(|error| {
        Error::Shape(format!("{name}: the written code does not parse: {error}"))
    })?;
    let long = long_functions(&prettyplease::unparse(&syntax));
    if !long.is_empty() {
        expect_long(&mut syntax, &long)?;
    }
    let mut text = header("codegen/src/matchers.rs", &source.path());
    text.push_str(&prettyplease::unparse(&syntax));
    Ok(File { path, text })
}

/// The ranges above ASCII that the file's class functions read, each table
/// written once.
#[derive(Default)]
struct Tables {
    /// Each table's ranges and its name, in the order first used.
    named: Vec<(Vec<(u32, u32)>, Ident)>,
}

impl Tables {
    /// The class function `class`, whose ranges above ASCII come from a table.
    fn function(&mut self, class: &ClassFunction) -> TokenStream {
        let ClassFunction {
            name,
            chars: Chars { ascii, wide },
        } = class;
        let mask = hex(*ascii, 32, "_u128");
        let wide = if wide.is_empty() {
            quote!(in_ranges(&[], c))
        } else {
            let table = self.table(name, wide);
            quote!(in_ranges(&#table, c))
        };
        quote! {
            #[inline]
            fn #name(c: char) -> bool {
                let code = u32::from(c);
                if code < 128 {
                    (#mask >> code) & 1 == 1
                } else {
                    #wide
                }
            }
        }
    }

    /// The name of the table of `wide`, which `function` reads.
    /// A new table is named for its class: `WORD` for `\w`, `SPACE` for `\s`,
    /// and otherwise for the first function that reads it.
    fn table(&mut self, function: &Ident, wide: &[(u32, u32)]) -> Ident {
        if let Some((_, name)) = self.named.iter().find(|(ranges, _)| ranges == wide) {
            return name.clone();
        }
        let class = if wide == Chars::word().wide {
            "WORD".to_owned()
        } else if wide == Chars::space().wide {
            "SPACE".to_owned()
        } else {
            function.to_string().to_uppercase()
        };
        let name = format_ident!("{class}_ABOVE_ASCII");
        self.named.push((wide.to_vec(), name.clone()));
        name
    }

    /// The tables as statics.
    fn statics(&self) -> TokenStream {
        self.named
            .iter()
            .map(|(ranges, name)| {
                let length = proc_macro2::Literal::usize_unsuffixed(ranges.len());
                let ranges = ranges.iter().map(|&(first, last)| {
                    let (first, last) =
                        (hex(u128::from(first), 1, ""), hex(u128::from(last), 1, ""));
                    quote!((#first, #last))
                });
                quote! {
                    /// Inclusive ranges of the code points above ASCII that a class takes.
                    static #name: [(u32, u32); #length] = [#(#ranges),*];
                }
            })
            .collect()
    }
}

/// The functions of `text` whose bodies hold more than [`FUNCTION_LINES`]
/// lines of code, as clippy's `too_many_lines` counts them.
///
/// It counts each line that is not blank, from the body's opening brace to
/// its closing one.
/// The written code holds no comments, so no line is a comment alone.
fn long_functions(text: &str) -> BTreeSet<String> {
    let mut long = BTreeSet::new();
    let lines = text.lines().collect::<Vec<_>>();
    for (index, line) in lines.iter().enumerate() {
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();
        let Some(signature) = trimmed
            .strip_prefix("fn ")
            .or_else(|| trimmed.strip_prefix("pub(super) fn "))
        else {
            continue;
        };
        let name = signature
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .next()
            .unwrap_or_default();
        let rest = lines.get(index..).unwrap_or_default();
        let Some(open) = rest.iter().position(|line| line.ends_with('{')) else {
            continue;
        };
        let close = format!("{}}}", " ".repeat(indent));
        let Some(end) = rest.iter().position(|line| **line == close) else {
            continue;
        };
        let body = rest.get(open..=end).unwrap_or_default();
        if body.iter().filter(|line| !line.trim().is_empty()).count() > FUNCTION_LINES {
            long.insert(name.to_owned());
        }
    }
    long
}

/// Adds to each function of `syntax` named in `long` the expectation of
/// clippy's `too_many_lines`.
fn expect_long(syntax: &mut syn::File, long: &BTreeSet<String>) -> Result<(), Error> {
    let expectation = || {
        syn::Attribute::parse_outer
            .parse2(quote! {
                #[expect(clippy::too_many_lines, reason = "one pattern's matcher, written whole")]
            })
            .map_err(|error| Error::Shape(format!("the expectation does not parse: {error}")))
    };
    for item in &mut syntax.items {
        if let syn::Item::Fn(function) = item
            && long.contains(&function.sig.ident.to_string())
        {
            function.attrs.extend(expectation()?);
        } else if let syn::Item::Impl(block) = item {
            for item in &mut block.items {
                if let syn::ImplItem::Fn(function) = item
                    && long.contains(&function.sig.ident.to_string())
                {
                    function.attrs.extend(expectation()?);
                }
            }
        }
    }
    Ok(())
}

/// The `use` items of the runtime names that `body` calls.
fn imports(body: &TokenStream) -> TokenStream {
    let mut names = BTreeSet::new();
    idents(body, &mut names);
    let runtime = RUNTIME
        .iter()
        .filter(|item| names.contains(**item))
        .map(|item| format_ident!("{item}"))
        .collect::<Vec<Ident>>();
    let chars = CHARS.iter().any(|method| names.contains(*method)).then(|| {
        quote!(
            use crate::rules::Chars;
        )
    });
    let runtime =
        (!runtime.is_empty()).then(|| quote!(use crate::rules::generated::{#(#runtime),*};));
    quote!(#chars #runtime)
}

/// Adds the name of each identifier of `stream` to `names`.
fn idents(stream: &TokenStream, names: &mut BTreeSet<String>) {
    for tree in stream.clone() {
        match tree {
            TokenTree::Ident(ident) => {
                names.insert(ident.to_string());
            }
            TokenTree::Group(group) => idents(&group.stream(), names),
            TokenTree::Punct(_) | TokenTree::Literal(_) => {}
        }
    }
}

/// `name` in snake case, such as `comments_at1_end` for `CommentsAt1End`.
fn snake(name: &str) -> String {
    let mut out = String::new();
    for (index, c) in name.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if index > 0 {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}
