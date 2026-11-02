//! A prototype TextMate engine, written from scratch: an Oniguruma-dialect
//! backtracking regular-expression engine ([`regex`]), a grammar compiler
//! ([`grammar`]) and the line tokenizer ([`tokenizer`]).
//!
//! It exists to answer a question (see the research note beside it) and to
//! be rerun. The nishiki crate takes none of its code until a decision says so.

pub mod grammar;
pub mod regex;
pub mod tokenizer;
