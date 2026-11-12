//! The analyses a code generator runs on a grammar's patterns.
//!
//! [`starts`] finds the bytes and places where a match can start.
//! [`blank_runs`] decides whether a pattern treats a wide space as a space.
//! Both read each pattern through [`super::regex`], the parser the engine
//! compiles with.

pub mod blank_runs;
pub mod starts;
