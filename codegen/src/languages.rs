//! Each rule-engine language's [`RuleTables`] value: what it needs beyond its
//! grammar.
//!
//! [`ALL`] lists the languages whose tables codegen builds so far.

mod dart;
mod go;
mod java;
mod jinja;
mod powershell;
mod proto;
mod python;
mod sass;
mod scala;
mod vb;

use nishiki::unstable::codegen::tables::RuleTables;

/// The languages whose tables codegen builds, by module name.
pub(crate) const ALL: [RuleTables; 10] = [
    dart::DART,
    go::GO,
    java::JAVA,
    jinja::JINJA,
    powershell::POWERSHELL,
    proto::PROTO,
    python::PYTHON,
    sass::SASS,
    scala::SCALA,
    vb::VB,
];
