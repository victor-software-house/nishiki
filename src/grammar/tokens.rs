//! A grammar as Rust, for a build script to write to `OUT_DIR`.
//!
//! The tokens build the unlinked grammar in a `static`, from borrowed text and
//! lists, so the program does no work to load it.
//! They name only the items of `nishiki::__private`, which the program's own
//! nishiki has without the `codegen` feature.

use proc_macro2::{Literal, TokenStream};
use quote::{ToTokens, quote};

use super::Grammar;
use crate::textmate::compile::{
    List, Repository, RuleId, Text, UnlinkedCapture, UnlinkedGrammar, UnlinkedRule,
};

/// Writes the grammar as an expression of type [`Grammar`], for a `static`.
///
/// The expression is a block.
/// It names `nishiki::__private::FORMAT_1`, so a program built against a
/// nishiki with another token format fails to compile.
/// Its grammar lives in a `static` of the block, so the program does no work
/// to load it.
/// Linking still happens at the first
/// [`Highlighter::for_grammar`](crate::Highlighter::for_grammar).
///
/// A build script writes the tokens to `OUT_DIR`, and the program holds them
/// in a `static`:
///
/// ```
/// use nishiki::Grammar;
/// use quote::{ToTokens as _, quote};
///
/// let json = r#"{
///     "scopeName": "source.demo",
///     "patterns": [{ "match": "let", "name": "keyword.demo" }]
/// }"#;
/// let grammar = Grammar::from_json(json)?;
/// let file = quote! { pub static GRAMMAR: ::nishiki::Grammar = #grammar; };
/// assert!(file.to_string().contains("FORMAT_1"));
/// # Ok::<(), nishiki::GrammarError>(())
/// ```
impl ToTokens for Grammar {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let unlinked = grammar(self.0);
        tokens.extend(quote! {
            {
                use ::std::borrow::Cow::Borrowed;
                use ::nishiki::__private as model;
                const _: () = model::FORMAT_1;
                static UNLINKED: model::UnlinkedGrammar = #unlinked;
                model::static_grammar(&UNLINKED)
            }
        });
    }
}

#[cfg(feature = "format")]
impl Grammar {
    /// The grammar as a Rust file, for a build script to write to `OUT_DIR`.
    ///
    /// The file holds `pub static GRAMMAR: nishiki::Grammar = …;`, laid out by
    /// prettyplease.
    /// A program includes it in a module of its own and uses `GRAMMAR`.
    ///
    /// # Panics
    ///
    /// Panics if `syn` cannot read the tokens nishiki writes.
    /// That is a bug in nishiki, and the test of every engine grammar guards
    /// against it.
    ///
    /// ```
    /// use nishiki::Grammar;
    ///
    /// let json = r#"{
    ///     "scopeName": "source.demo",
    ///     "patterns": [{ "match": "let", "name": "keyword.demo" }]
    /// }"#;
    /// let rust = Grammar::from_json(json)?.to_rust();
    /// assert!(rust.starts_with("pub static GRAMMAR: ::nishiki::Grammar = {"));
    /// # Ok::<(), nishiki::GrammarError>(())
    /// ```
    #[must_use]
    #[expect(
        clippy::expect_used,
        reason = "nishiki writes the tokens, so a parse failure is its own bug"
    )]
    pub fn to_rust(self) -> String {
        let file = syn::parse2::<syn::File>(static_file(self))
            .expect("nishiki wrote grammar tokens that syn cannot read: a bug in nishiki");
        prettyplease::unparse(&file)
    }
}

/// The tokens of the file that [`Grammar::to_rust`] lays out.
#[cfg(feature = "format")]
pub(super) fn static_file(grammar: Grammar) -> TokenStream {
    quote! { pub static GRAMMAR: ::nishiki::Grammar = #grammar; }
}

/// The struct expression of an unlinked grammar.
fn grammar(unlinked: &UnlinkedGrammar) -> TokenStream {
    let UnlinkedGrammar {
        scope,
        rules,
        patterns,
        repository: names,
        injections,
        injection_selector,
        inject_to,
    } = unlinked;
    let scope = text(scope);
    let rules = list(rules, rule);
    let patterns = list(patterns, |&id| rule_id(id));
    let names = repository(names);
    let injections = list(injections, |(selector, id)| {
        let selector = text(selector);
        let id = rule_id(*id);
        quote! { (#selector, #id) }
    });
    let injection_selector = optional(injection_selector.as_ref(), text);
    let inject_to = list(inject_to, text);
    quote! {
        model::UnlinkedGrammar {
            scope: #scope,
            rules: #rules,
            patterns: #patterns,
            repository: #names,
            injections: #injections,
            injection_selector: #injection_selector,
            inject_to: #inject_to,
        }
    }
}

/// The struct expression of one rule, with every field written out.
fn rule(unlinked: &UnlinkedRule) -> TokenStream {
    let UnlinkedRule {
        include,
        name,
        content_name,
        matched,
        begin,
        end,
        until,
        captures,
        begin_captures,
        end_captures,
        while_captures,
        apply_end_last,
        patterns,
        has_patterns,
        repository: names,
    } = unlinked;
    let include = optional(include.as_ref(), text);
    let name = list(name, text);
    let content_name = list(content_name, text);
    let matched = optional(matched.as_ref(), text);
    let begin = optional(begin.as_ref(), text);
    let end = optional(end.as_ref(), text);
    let until = optional(until.as_ref(), text);
    let captures = optional(captures.as_ref(), capture_list);
    let begin_captures = optional(begin_captures.as_ref(), capture_list);
    let end_captures = optional(end_captures.as_ref(), capture_list);
    let while_captures = optional(while_captures.as_ref(), capture_list);
    let patterns = list(patterns, |&id| rule_id(id));
    let names = optional(names.as_ref(), repository);
    quote! {
        model::UnlinkedRule {
            include: #include,
            name: #name,
            content_name: #content_name,
            matched: #matched,
            begin: #begin,
            end: #end,
            until: #until,
            captures: #captures,
            begin_captures: #begin_captures,
            end_captures: #end_captures,
            while_captures: #while_captures,
            apply_end_last: #apply_end_last,
            patterns: #patterns,
            has_patterns: #has_patterns,
            repository: #names,
        }
    }
}

/// The captures of one rule, as a borrowed list.
fn capture_list(captures: &List<UnlinkedCapture>) -> TokenStream {
    list(captures, |UnlinkedCapture { index, rule }| {
        let index = optional(index.as_ref(), |&group| Literal::usize_unsuffixed(group));
        let rule = rule_id(*rule);
        quote! { model::UnlinkedCapture { index: #index, rule: #rule } }
    })
}

/// Named rules, as a borrowed list of pairs.
fn repository(names: &Repository) -> TokenStream {
    list(names, |(name, id)| {
        let name = text(name);
        let id = rule_id(*id);
        quote! { (#name, #id) }
    })
}

/// Borrowed text.
fn text(value: &Text) -> TokenStream {
    let value: &str = value;
    quote! { Borrowed(#value) }
}

/// A rule's index, as an unsuffixed literal the field's type reads as `usize`.
fn rule_id(id: RuleId) -> Literal {
    Literal::usize_unsuffixed(id)
}

/// A borrowed list, each item written by `item`.
fn list<T: Clone, I: ToTokens>(items: &List<T>, item: impl Fn(&T) -> I) -> TokenStream {
    let items = items.iter().map(item);
    quote! { Borrowed(&[#(#items),*]) }
}

/// `Some` with the value written by `write`, or `None`.
fn optional<T, I: ToTokens>(value: Option<&T>, write: impl Fn(&T) -> I) -> TokenStream {
    value.map_or_else(
        || quote! { None },
        |value| {
            let value = write(value);
            quote! { Some(#value) }
        },
    )
}
