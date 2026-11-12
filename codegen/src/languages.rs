//! Each rule-engine language's [`RuleTables`] value: what it needs beyond its
//! grammar.
//!
//! [`ALL`] lists the languages whose tables codegen builds so far.

mod go;
mod proto;
mod python;
mod sass;
mod scala;

use nishiki::unstable::codegen::tables::RuleTables;

/// The languages whose tables codegen builds, by module name.
pub(crate) const ALL: [RuleTables; 5] = [
    go::GO,
    proto::PROTO,
    python::PYTHON,
    sass::SASS,
    scala::SCALA,
];
