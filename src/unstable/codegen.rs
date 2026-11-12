//! The analyses and the model a code generator builds from a grammar.
//!
//! [`starts`] finds the bytes and places where a match can start.
//! [`blank_runs`] decides whether a pattern treats a wide space as a space.
//! Both read each pattern through [`super::regex`], the parser the engine
//! compiles with.
//!
//! [`flatten`] walks a grammar's rules into [`tables::Tables`], as a
//! language's [`tables::RuleTables`] says.

pub mod blank_runs;
mod error;
#[cfg(feature = "textmate")]
pub mod flatten;
pub mod starts;
pub mod tables;

pub use error::GenerateError;
