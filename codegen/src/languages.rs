//! Each rule-engine language's [`RuleTables`] value: what it needs beyond its
//! grammar.
//!
//! [`ALL`] lists the languages whose tables codegen builds so far.

mod common_lisp;
mod dart;
mod gdscript;
mod go;
mod java;
mod jinja;
mod less;
mod powershell;
mod proto;
mod python;
mod sass;
mod scala;
mod vb;

use nishiki::unstable::codegen::tables::RuleTables;

/// The languages whose tables codegen builds, by module name.
pub(crate) const ALL: [RuleTables; 13] = [
    common_lisp::COMMON_LISP,
    dart::DART,
    gdscript::GDSCRIPT,
    go::GO,
    java::JAVA,
    jinja::JINJA,
    less::LESS,
    powershell::POWERSHELL,
    proto::PROTO,
    python::PYTHON,
    sass::SASS,
    scala::SCALA,
    vb::VB,
];
