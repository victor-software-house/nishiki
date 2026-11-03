//! The recursion guards of the C# matchers that no `Limits` setting reaches.

/// How many angle brackets or parentheses may nest inside one type for the
/// type matchers to read it: the matchers count nesting in the bits of one
/// `u64`, so a type nested deeper than this is read as no type.
///
/// Oniguruma bounds the depth of the grammar's recursive type pattern too, at
/// a larger number; no real type is nested a tenth as deep.
pub(super) const MAX_NESTING: u32 = 64;

/// How many dotted names, pointer marks and array suffixes one type may have
/// for the type matchers to read past them: the matchers recurse once for
/// each, so the bound keeps the stack and the work of a hostile line in
/// check. A longer type reads as its first parts, which no real type exceeds.
pub(super) const MAX_TYPE_PARTS: u32 = 64;

/// How many dotted names before the last one a base-class constructor call
/// may have for the matcher to try each shorter run: it keeps where each name
/// ends, in a fixed array. A call with more names reads its first ones.
pub(super) const MAX_CALL_NAMES: usize = 32;
