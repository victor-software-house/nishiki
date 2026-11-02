//! The high byte of every frame kind, one constant per lexer module.
//!
//! A frame of the shared arena (`src/arena.rs`) carries a kind code, and the
//! tag in its high bits says which lexer's enum the low bits belong to.
//! Lexers are written in parallel, so the tags are allocated here and nowhere
//! else; a lexer module takes its constant and never a "next free byte".
//! A lexer that needs more than the low byte for its own bits (a flag next to
//! the kind) claims a wider range by giving its [`Tag`] a mask that covers
//! fewer bits of the high byte, and the check below counts the whole range.
//! Bytes `0x10` and up belong to the shared rule engine.

/// A claimed range of the code space: the codes `c` with `c & mask == value`.
struct Tag {
    value: u16,
    mask: u16,
}

impl Tag {
    /// A whole high byte.
    const fn byte(value: u16) -> Self {
        Self {
            value,
            mask: 0xff00,
        }
    }
}

/// Rust.
pub(crate) const RUST: u16 = 0x0100;
/// Markdown.
pub(crate) const MARKDOWN: u16 = 0x0200;
/// TypeScript, JavaScript, JSX and TSX.
pub(crate) const TYPESCRIPT: u16 = 0x0300;
/// Shell.
pub(crate) const SHELL: u16 = 0x0400;
/// JSON.
pub(crate) const JSON: u16 = 0x0500;
/// Python.
pub(crate) const PYTHON: u16 = 0x0600;
/// YAML.
pub(crate) const YAML: u16 = 0x0800;
/// TOML.
pub(crate) const TOML: u16 = 0x0900;
/// Diff.
pub(crate) const DIFF: u16 = 0x0e00;

/// Java.
pub(crate) const JAVA: u16 = 0x0a00;
/// Makefiles.
pub(crate) const MAKE: u16 = 0x0b00;
/// Dockerfiles.
pub(crate) const DOCKERFILE: u16 = 0x2000;
/// C, with the next byte for its end-of-line flag.
pub(crate) const C: u16 = 0x0c00;
/// SQL.
pub(crate) const SQL: u16 = 0x0f00;
/// INI files.
pub(crate) const INI: u16 = 0x2100;
/// GraphQL.
pub(crate) const GRAPHQL: u16 = 0x2200;

/// Every claimed range. A new lexer adds its constant and its entry here.
const CLAIMED: &[Tag] = &[
    Tag::byte(RUST),
    Tag::byte(MARKDOWN),
    Tag::byte(TYPESCRIPT),
    Tag::byte(SHELL),
    Tag::byte(JSON),
    Tag::byte(PYTHON),
    Tag::byte(YAML),
    Tag::byte(TOML),
    Tag::byte(DIFF),
    Tag::byte(JAVA),
    Tag::byte(MAKE),
    Tag::byte(DOCKERFILE),
    Tag {
        value: C,
        mask: 0xfe00,
    },
    Tag::byte(SQL),
    Tag::byte(INI),
    Tag::byte(GRAPHQL),
];

/// Two ranges overlap when some code lies in both: when their values agree on
/// every bit that both masks look at.
const fn all_distinct() -> bool {
    let mut i = 0;
    while i < CLAIMED.len() {
        let mut j = i + 1;
        while j < CLAIMED.len() {
            let both = CLAIMED[i].mask & CLAIMED[j].mask;
            if (CLAIMED[i].value ^ CLAIMED[j].value) & both == 0 {
                return false;
            }
            j += 1;
        }
        i += 1;
    }
    true
}

const _: () = assert!(all_distinct(), "two lexers claim the same frame tag");
