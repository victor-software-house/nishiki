//! The analyses and the model a code generator builds from a grammar.
//!
//! [`starts`] finds the bytes and places where a match can start.
//! [`blank_runs`] decides how a pattern reads a run of wide spaces.
//! Both read each pattern through [`super::regex`], the parser the engine
//! compiles with.
//!
//! [`flatten`] walks a grammar's rules into [`tables::Tables`], as a
//! language's [`tables::RuleTables`] says.

pub mod blank_runs;
mod error;
#[cfg(feature = "textmate")]
pub mod flatten;
#[cfg(feature = "textmate")]
mod split;
pub mod starts;
pub mod tables;

pub use error::GenerateError;
