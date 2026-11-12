//! Each rule-engine language's [`RuleTables`] value: what it needs beyond its
//! grammar.
//!
//! [`ALL`] lists the languages whose tables codegen builds so far.

mod common_lisp;
mod dart;
mod elixir;
mod gdscript;
mod go;
mod haml;
mod java;
mod jinja;
mod julia;
mod less;
mod lua;
mod powershell;
mod proto;
mod python;
mod sass;
mod scala;
mod vb;

use nishiki::unstable::codegen::tables::RuleTables;

/// The languages whose tables codegen builds, by module name.
pub(crate) const ALL: [RuleTables; 17] = [
    common_lisp::COMMON_LISP,
    dart::DART,
    elixir::ELIXIR,
    gdscript::GDSCRIPT,
    go::GO,
    haml::HAML,
    java::JAVA,
    jinja::JINJA,
    julia::JULIA,
    less::LESS,
    lua::LUA,
    powershell::POWERSHELL,
    proto::PROTO,
    python::PYTHON,
    sass::SASS,
    scala::SCALA,
    vb::VB,
];
