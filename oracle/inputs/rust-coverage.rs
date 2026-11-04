// Rust that nishiki's other oracle inputs leave out, written to exercise the
// rest of Shiki's `rust` grammar.
extern crate alloc;
extern crate core as std_core;

/** A block documentation comment. */
/**/
/* A plain block comment. */

pub(crate) const MAX_LEN: usize = 0o777_usize;
const FLAGS: u8 = 0b1010_u8;
const MASK: u32 = 0xff_u32;
const SMALL: f64 = 1.5e-3f64;
static GREETING: &'static str = "hi";

macro_rules! Shout {
    ($x:expr) => {
        $x
    };
}

macro_rules! whisper {
    ($($t:tt)*) => {};
}

pub enum Shape {
    Dot,
    Line(u8),
}

pub trait Draw {
    type Output;
    fn draw(&self) -> Self::Output;
}

pub struct Canvas<'a> {
    cells: &'a [u8],
    boxed: Box<[u8]>,
    apply: fn(u8) -> u8,
}

const fn double(n: u8) -> u8 {
    n * 2
}

fn compare(a: u8, b: u8) -> bool {
    (a) < (b) || [a] > [b] || {a} < {b}
}

fn bind(n: u8) -> u8 {
    match n {
        x @ 1..=9 => x,
        _ => 0,
    }
}

fn first<'a>(s: &'a str) -> &'a str {
    s
}

fn texts() -> (&'static [u8], &'static str, &'static str) {
    (b"bytes\x7f", "tab\t\x41\u{00e9}", r#"raw "quoted""#)
}

fn calls() {
    Shout!(1);
    whisper!();
}

macro_rules! declare {
    ($vis:vis $name:ident, $T:ty) => {
        $vis struct $name($T);
        $vis enum $name {}
        $vis trait $name {}
        $vis type $name = $T;
        $vis fn $name() {}
    };
}

fn lengths<'a>(words: &[&'a str]) -> usize {
    let n = <[u8]>::len(&[1]);
    let m = <[&'a str]>::len(words);
    n + m
}
