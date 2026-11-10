//! The engine against Shiki.
//!
//! Each grammar of the four hand-written languages runs over the fixtures
//! `oracle/generate.ts` wrote. It must give every non-whitespace byte the
//! scope stack Shiki gives it.

use std::collections::{BTreeSet, HashMap};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock, PoisonError};

use serde_json::Value;

use super::compile::link::link;
use super::compile::{UnlinkedGrammar, compile};
use super::grammar::{Grammar, Matching, NONE};
use super::regex::{self, Context, Programs, Regex, Sketch};
use super::tokenizer::{self, Scratch, Text};
use crate::arena::Arena;
use crate::highlighter::{Sealed, lex_line};
use crate::language::Language;
use crate::limits::{Limit, Limits, LineReport};
use crate::{Highlighter, Stack, State, Syntax, Token};

/// The tables the engine's tests run, one cell per table name.
///
/// The lock guards only the list of cells.
/// Each cell links its table once a process, so a test waits only for the
/// table it runs.
static LINKED_TABLES: Mutex<Vec<(&str, &OnceLock<&Grammar>)>> = Mutex::new(Vec::new());

/// The test table named `name`, as `tests/engine/grammars/tables.json` lists
/// it, linked on first use.
fn table(name: &'static str) -> &'static Grammar {
    let cell = {
        let mut cells = LINKED_TABLES.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(&(_, cell)) = cells.iter().find(|&&(held, _)| held == name) {
            cell
        } else {
            let cell: &OnceLock<&Grammar> = Box::leak(Box::new(OnceLock::new()));
            cells.push((name, cell));
            cell
        }
    };
    cell.get_or_init(|| link_table(name))
}

/// Links the test table named `name`.
///
/// The Rust compiler links it from the Shiki grammars in
/// `tests/engine/grammars/`, which `oracle/engine-grammars.ts` writes.
/// The table names the languages it hands regions to.
/// So an include of any other language's whole grammar matches nothing.
fn link_table(name: &str) -> &'static Grammar {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/engine/grammars");
    let read = |file: &str| fs::read_to_string(dir.join(file)).unwrap();
    let tables: Value = serde_json::from_str(&read("tables.json")).unwrap();
    let entry = tables
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["name"] == name)
        .unwrap();
    let grammars: Vec<UnlinkedGrammar> = entry["grammars"]
        .as_array()
        .unwrap()
        .iter()
        .map(|grammar| compile(&read(&format!("{}.json", grammar.as_str().unwrap()))).unwrap())
        .collect();
    let root = grammars
        .iter()
        .find(|grammar| entry["root"] == grammar.scope.as_str())
        .unwrap();
    let set: Vec<&UnlinkedGrammar> = grammars.iter().collect();
    let hand_offs: Vec<Language> = entry["handOffs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|id| Language::from_name(id.as_str().unwrap()).unwrap())
        .collect();
    link(root, &set, Some(&hand_offs)).unwrap()
}

/// Whether the engine finds where rules start with its scanners, or searches
/// every rule alone, to check that both give one output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scanners {
    On,
    Off,
}

/// A linked table as what a [`Highlighter`] lexes.
/// The tests run the engine through the highlighter, as a caller does.
#[derive(Clone, Copy, Debug)]
struct Table(&'static Grammar);

impl Syntax for Table {}

impl Sealed for Table {
    type Lexer = Table;

    fn start(lexer: Table, arena: &mut Arena, base: u32) -> u32 {
        tokenizer::start(arena, lexer.0, base)
    }

    fn line(
        highlighter: &mut Highlighter<Table>,
        line: &str,
        state: &mut State,
        out: &mut Vec<Token>,
    ) -> LineReport {
        lex_line(highlighter, line, state, out, |table, cx, text, from| {
            super::lex_linked(cx, table.0, text, from)
        })
    }
}

/// One source line of a fixture and Shiki's scope stack for each byte.
struct Line {
    text: String,
    stacks: Vec<String>,
}

fn fixtures(language: &str) -> Vec<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/oracle")
        .join(language);
    let mut paths: Vec<PathBuf> = fs::read_dir(&dir)
        .expect("the fixture directory exists")
        .map(|entry| entry.expect("a readable directory entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "txt"))
        .collect();
    paths.sort();
    assert!(!paths.is_empty(), "no fixtures in {}", dir.display());
    paths
}

fn parse(path: &Path) -> Vec<Line> {
    let content = fs::read_to_string(path).expect("a readable fixture");
    parse_text(&content, &path.display().to_string())
}

/// The lines of a fixture's text, each with the scope stack of every byte.
/// `source` names the fixture in a panic.
fn parse_text(content: &str, source: &str) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    let mut start = 0;
    for record in content.lines() {
        if let Some(text) = record.strip_prefix("L ") {
            lines.push(Line {
                text: text.to_owned(),
                stacks: Vec::new(),
            });
            start = 0;
        } else if let Some(token) = record.strip_prefix("T ") {
            let (end, scopes) = token.split_once(' ').expect("a byte end and scopes");
            let end: usize = end.parse().expect("a byte end");
            let line = lines.last_mut().expect("a token after its line");
            line.stacks.extend((start..end).map(|_| scopes.to_owned()));
            start = end;
        } else {
            panic!("{source}: unknown record {record:?}");
        }
    }
    lines
}

/// What gives an embedded block of a fixture its scopes.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Embedding {
    /// The tables' own rules, linked from the language's grammar.
    Tables,
    /// The language's hand lexer, which the tables hand the block to.
    #[cfg(all(feature = "json", feature = "lang-javascript", feature = "lang-css"))]
    Lexers,
}

/// Which rules the grammar's root reaches, through lists and captures.
fn reachable_rules(grammar: &Grammar) -> Vec<bool> {
    let mut reachable = vec![false; grammar.rules.len()];
    let mut todo = vec![grammar.root];
    while let Some(id) = todo.pop() {
        if std::mem::replace(&mut reachable[id], true) {
            continue;
        }
        let rule = &grammar.rules[id];
        if let Some(list) = grammar.lists.get(rule.list) {
            todo.extend(list.rules);
        }
        todo.extend(
            rule.captures
                .iter()
                .chain(rule.end_captures)
                .map(|cap| cap.retokenize)
                .filter(|&retokenize| retokenize != NONE),
        );
    }
    reachable
}

/// Which lines sit in an embedded block, a fence or front matter, of a
/// language the grammar's tables do not hold.
/// The tables hold the languages the oracle had when they were written.
/// A block of a language added since cannot match.
/// It is left out rather than failing every new language's pull request.
///
/// A block is of such a language in either of two cases:
/// - Shiki gives it a scope the tables never name;
/// - no list holds the rule that embeds the language, because the tables left
///   the rule out with the language.
///
/// Either of two signs shows the missing rule:
/// - a `meta.embedded.block.*` scope in the block that no rule reachable from
///   the root gives its contents;
/// - a fence line that Shiki names `fenced_code.block.language.markdown`, a
///   language's own fence rule. No reachable rule that names its language so
///   has a begin that matches the line. A body that ends at once holds no
///   embedded scope, as a Less body popped by its first character does. Only
///   this sign shows it.
///
/// With [`Embedding::Lexers`], the first case does not apply.
/// Hand lexers give the blocks their scopes, which the tables never name.
/// Then the rule that gives a block its `meta.embedded.block.*` scope must
/// also hand the block off or hold rules.
/// An include of a language that neither the set nor a lexer serves leaves it
/// with neither.
fn unknown_fences(grammar: &Grammar, lines: &[Line], embedding: Embedding) -> Vec<bool> {
    let inside = |line: &Line| {
        line.stacks.iter().any(|stack| {
            stack.contains("markup.fenced_code.block.markdown") || stack.contains(".frontmatter")
        })
    };
    let reachable = reachable_rules(grammar);
    let fills = |list: usize| {
        embedding == Embedding::Tables
            || grammar.hand_off(list).is_some()
            || grammar
                .lists
                .get(list)
                .is_some_and(|list| !list.rules.is_empty())
    };
    let embeds = |scope: &str| {
        grammar
            .rules
            .iter()
            .zip(&reachable)
            .any(|(rule, &reached)| {
                reached
                    && fills(rule.list)
                    && rule
                        .content_name
                        .iter()
                        .any(|&name| grammar.names.get(usize::from(name)) == Some(&scope))
            })
    };
    let fence_language = grammar
        .names
        .iter()
        .position(|&name| name == "fenced_code.block.language.markdown")
        .and_then(|index| u16::try_from(index).ok());
    let fence_rules: Vec<Regex> = grammar
        .rules
        .iter()
        .zip(&reachable)
        .filter(|&(rule, &reached)| {
            reached
                && rule
                    .captures
                    .iter()
                    .any(|cap| fence_language.is_some_and(|name| cap.name.contains(&name)))
        })
        .filter_map(|(rule, _)| {
            Regex::for_grammar(grammar.patterns[rule.begin].source, Programs::Full).ok()
        })
        .collect();
    let opens_known_fence = |line: &Line| {
        let text = format!("{}\n", line.text);
        let mut scratch = regex::Scratch::default();
        fence_rules.iter().any(|regex| {
            let mut steps = 10_000_000;
            let context = Context {
                search: Some(0),
                text_start: false,
            };
            regex.find(&mut scratch, text.as_bytes(), 0, context, &mut steps)
        })
    };
    let dropped_fence = |line: &Line| {
        line.stacks
            .iter()
            .any(|stack| stack.ends_with("fenced_code.block.language.markdown"))
            && !opens_known_fence(line)
    };
    let mut skip = vec![false; lines.len()];
    let mut at = 0;
    while at < lines.len() {
        if !inside(&lines[at]) {
            at += 1;
            continue;
        }
        let end = (at..lines.len())
            .find(|&next| !inside(&lines[next]))
            .unwrap_or(lines.len());
        let unknown = dropped_fence(&lines[at])
            || lines[at..end]
                .iter()
                .flat_map(|line| &line.stacks)
                .flat_map(|stack| stack.split(' '))
                .any(|scope| {
                    scope != grammar.scope
                        && ((embedding == Embedding::Tables && !grammar.names.contains(&scope))
                            || (scope.starts_with("meta.embedded.block.") && !embeds(scope)))
                });
        if unknown {
            skip[at..end].fill(true);
        }
        at = end;
    }
    skip
}

/// The bytes of `lines` whose stack differs from Shiki's, with a message for
/// each of the first few.
/// Blocks of languages the tables do not hold are left out, as
/// [`unknown_fences`] finds them.
fn run(grammar: &'static Grammar, lines: &[Line], scanners: Scanners) -> (usize, usize, String) {
    let mut highlighter = Highlighter::<Table>::with_lexer(Table(grammar), Limits::unlimited());
    highlighter.scratch_mut().textmate.no_scanner = scanners == Scanners::Off;
    let found = compare(
        highlighter,
        lines,
        &unknown_fences(grammar, lines, Embedding::Tables),
    );
    (found.checked, found.missed, found.message)
}

/// The first byte whose scope stack differs from Shiki's.
struct Difference {
    /// The line, from 1.
    line: usize,
    /// The byte in the line, from 1.
    column: usize,
    expected: String,
    actual: String,
}

/// What lexing a fixture's lines found against Shiki's stacks.
struct Comparison {
    /// The non-whitespace bytes compared.
    checked: usize,
    /// How many of them differ.
    missed: usize,
    /// A message for each of the first few lines that differ.
    message: String,
    first: Option<Difference>,
    /// Every limit a line reached.
    reached: Vec<Limit>,
}

/// Lexes `lines` with `highlighter` and compares each non-whitespace byte
/// with Shiki's stack.
/// A line that `skip` marks is lexed but not compared.
fn compare<S: Syntax>(
    mut highlighter: Highlighter<S>,
    lines: &[Line],
    skip: &[bool],
) -> Comparison {
    let mut state = State::default();
    let mut names: HashMap<u32, String> = HashMap::new();
    let (mut checked, mut missed) = (0, 0);
    let mut message = String::new();
    let mut shown = 0;
    let mut first = None;
    let mut reached: Vec<Limit> = Vec::new();
    for (number, line) in lines.iter().enumerate() {
        let mut out = Vec::new();
        let report = highlighter.line(&line.text, &mut state, &mut out);
        for limit in report.limits() {
            if !reached.contains(&limit) {
                reached.push(limit);
            }
        }
        let mut actual: Vec<u32> = Vec::new();
        let mut start = 0;
        for token in &out {
            actual.extend((start..token.end).map(|_| token.stack.0));
            start = token.end;
        }
        if skip.get(number).copied().unwrap_or(false) {
            continue;
        }
        let mut line_missed = 0;
        for (column, byte) in line.text.bytes().enumerate() {
            if byte.is_ascii_whitespace() {
                continue;
            }
            checked += 1;
            let expected = line.stacks.get(column).map_or("", String::as_str);
            let got = match actual.get(column) {
                Some(&stack) => names.entry(stack).or_insert_with(|| {
                    let mut scopes: Vec<&str> = highlighter.scopes(Stack(stack)).collect();
                    scopes.reverse();
                    scopes.join(" ")
                }),
                None => "",
            };
            if expected != got {
                missed += 1;
                line_missed += 1;
                if first.is_none() {
                    first = Some(Difference {
                        line: number + 1,
                        column: column + 1,
                        expected: expected.to_owned(),
                        actual: String::from(got),
                    });
                }
                if line_missed == 1 && shown < 8 {
                    shown += 1;
                    let _ = write!(
                        message,
                        "\n  line {} col {}: {:?}",
                        number + 1,
                        column + 1,
                        line.text
                    );
                    let _ = write!(message, "\n    expected {expected}");
                    let _ = write!(message, "\n    actual   {got}");
                }
            }
        }
    }
    Comparison {
        checked,
        missed,
        message,
        first,
        reached,
    }
}

fn check(grammar: &'static Grammar, language: &str, scanners: Scanners) {
    let mut failures = Vec::new();
    for path in fixtures(language) {
        let (checked, missed, message) = run(grammar, &parse(&path), scanners);
        if missed > 0 {
            failures.push(format!(
                "{}: {missed} of {checked} bytes differ{message}",
                path.display()
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

#[test]
fn rust_grammar_matches_shiki() {
    check(table("rust"), "rust", Scanners::On);
}

/// Elixir has a hand lexer.
/// The engine runs its grammar from a test table, as a second oracle.
/// The table hands its `~H` and `~L` sigils to HTML.
#[cfg(feature = "lang-html")]
#[test]
fn elixir_grammar_matches_shiki() {
    check(table("elixir"), "elixir", Scanners::On);
}

/// HTML has a hand lexer.
/// The engine runs its grammar from a test table, which hands `<script>` to
/// the JavaScript lexer and `<style>` to the CSS lexer.
/// The script's own `//` comment rule applies only where no JavaScript
/// context is open, as `mathjax-tex-mml.txt` checks.
#[cfg(all(feature = "lang-javascript", feature = "lang-css"))]
#[test]
fn html_grammar_matches_shiki() {
    check(table("html"), "html", Scanners::On);
}

/// Assembly has a hand lexer.
/// The engine runs its grammar from a test table, as a second oracle.
#[test]
fn asm_grammar_matches_shiki() {
    check(table("asm"), "asm", Scanners::On);
}

/// Common Lisp has a hand lexer.
/// The engine runs its grammar from a test table, as a second oracle.
#[test]
fn common_lisp_grammar_matches_shiki() {
    check(table("common-lisp"), "common-lisp", Scanners::On);
}

/// GDScript has a hand lexer.
/// The engine runs its grammar from a test table, as a second oracle.
#[test]
fn gdscript_grammar_matches_shiki() {
    check(table("gdscript"), "gdscript", Scanners::On);
}

/// MATLAB has a hand lexer.
/// The engine runs its grammar from a test table, as a second oracle.
#[test]
fn matlab_grammar_matches_shiki() {
    check(table("matlab"), "matlab", Scanners::On);
}

/// Visual Basic has a hand lexer.
/// The engine runs its grammar from a test table, as a second oracle.
#[test]
fn vb_grammar_matches_shiki() {
    check(table("vb"), "vb", Scanners::On);
}

/// Jinja has a hand lexer.
/// The engine runs its grammar from a test table, as a second oracle.
#[test]
fn jinja_grammar_matches_shiki() {
    check(table("jinja"), "jinja", Scanners::On);
}

/// TeX has a hand lexer.
/// The engine runs its grammar from a test table, as a second oracle.
/// The table hands R's regions to R's lexer.
#[cfg(feature = "lang-r")]
#[test]
fn tex_grammar_matches_shiki() {
    check(table("tex"), "tex", Scanners::On);
}

/// HAML has a hand lexer.
/// The engine runs its grammar from a test table, as a second oracle.
/// The table hands its filters and its Ruby to the languages they embed.
#[cfg(feature = "lang-haml")]
#[test]
fn haml_grammar_matches_shiki() {
    check(table("haml"), "haml", Scanners::On);
}

/// Objective-C has a hand lexer.
/// The engine runs its grammar from a test table, as a second oracle.
#[test]
fn objective_c_grammar_matches_shiki() {
    check(table("objective-c"), "objective-c", Scanners::On);
}

/// Objective-C++ has a hand lexer.
/// The engine runs its grammar from a test table, as a second oracle.
#[test]
fn objective_cpp_grammar_matches_shiki() {
    check(table("objective-cpp"), "objective-cpp", Scanners::On);
}

#[test]
fn shell_grammar_matches_shiki() {
    check(table("shellscript"), "shellscript", Scanners::On);
}

#[test]
fn typescript_grammar_matches_shiki() {
    check(table("typescript"), "typescript", Scanners::On);
}

#[test]
fn markdown_grammar_matches_shiki() {
    check(table("markdown"), "markdown", Scanners::On);
}

/// Shiki's lines for a `less` fence whose body `)` pops at once.
/// The body holds no `meta.embedded.block.less`.
const POPPED_LESS_FENCE: &str = "\
L ```less
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L ) a
T 3 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
";

/// Shiki's lines for a `css` fence that closes at once.
const EMPTY_CSS_FENCE: &str = "\
L ```css
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
";

/// The Markdown tables drop the `less` fence rule with Less.
/// A popped `less` fence shows it only by its fence line, and is left out.
#[test]
fn a_fence_of_a_dropped_language_is_left_out_without_an_embedded_scope() {
    let lines = parse_text(POPPED_LESS_FENCE, "a popped less fence");
    assert_eq!(
        unknown_fences(table("markdown"), &lines, Embedding::Tables),
        [true, true, true]
    );
}

/// The Markdown tables keep the `css` fence rule.
/// An empty `css` fence holds no embedded scope either, and is still compared.
#[test]
fn an_empty_fence_of_a_kept_language_is_compared() {
    let lines = parse_text(EMPTY_CSS_FENCE, "an empty css fence");
    assert_eq!(
        unknown_fences(table("markdown"), &lines, Embedding::Tables),
        [false, false]
    );
}

#[test]
fn python_grammar_matches_shiki() {
    check(table("python"), "python", Scanners::On);
}

#[test]
fn yaml_grammar_matches_shiki() {
    check(table("yaml"), "yaml", Scanners::On);
}

#[test]
fn make_grammar_matches_shiki() {
    check(table("make"), "make", Scanners::On);
}

#[test]
fn toml_grammar_matches_shiki() {
    check(table("toml"), "toml", Scanners::On);
}

#[test]
fn javascript_grammar_matches_shiki() {
    check(table("javascript"), "javascript", Scanners::On);
}

#[test]
fn every_grammar_matches_shiki_without_the_scanners() {
    check(table("rust"), "rust", Scanners::Off);
    check(table("shellscript"), "shellscript", Scanners::Off);
    check(table("typescript"), "typescript", Scanners::Off);
    check(table("markdown"), "markdown", Scanners::Off);
    check(table("python"), "python", Scanners::Off);
    check(table("yaml"), "yaml", Scanners::Off);
    check(table("make"), "make", Scanners::Off);
    check(table("toml"), "toml", Scanners::Off);
    check(table("javascript"), "javascript", Scanners::Off);
}

/// Where a line's frames come from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Frames {
    /// The frames the last line left on the scratch.
    Kept,
    /// The arena. A line of another grammar runs before each line on the
    /// same scratch, so no line finds its frames kept.
    Rebuilt,
}

/// The tokens of each of `lines` in `grammar`, as ends and scope names.
fn scoped_tokens(
    grammar: &'static Grammar,
    lines: &[Line],
    frames: Frames,
) -> Vec<Vec<(usize, String)>> {
    let other: &'static Grammar = if grammar.scope == table("toml").scope {
        table("yaml")
    } else {
        table("toml")
    };
    let mut arena = Arena::new(None);
    let limits = Limits::unlimited();
    let mut scratch = Scratch::default();
    let mut state = tokenizer::start(&mut arena, grammar, crate::arena::ROOT);
    let other_state = tokenizer::start(&mut arena, other, crate::arena::ROOT);
    let mut report = LineReport::default();
    let mut lexed = Vec::new();
    for line in lines {
        let mut out = Vec::new();
        if frames == Frames::Rebuilt {
            let text = Text {
                grammar: other,
                bytes: b"a = 1",
                start: 0,
                anchored: false,
            };
            tokenizer::line(
                &mut scratch,
                &mut arena,
                &limits,
                text,
                other_state,
                &mut out,
                &mut report,
            );
            out.clear();
        }
        let text = Text {
            grammar,
            bytes: line.text.as_bytes(),
            start: 0,
            anchored: false,
        };
        state = tokenizer::line(
            &mut scratch,
            &mut arena,
            &limits,
            text,
            state,
            &mut out,
            &mut report,
        );
        lexed.push(
            out.iter()
                .map(|token| {
                    let mut scopes: Vec<&str> = arena.scopes(token.stack.0).collect();
                    scopes.reverse();
                    (token.end, scopes.join(" "))
                })
                .collect(),
        );
    }
    lexed
}

/// A line that reuses the frames the last line left lexes as one that loads
/// them from the arena.
/// The test covers every fixture of every grammar with fixtures.
#[test]
fn kept_frames_lex_as_rebuilt_ones() {
    let tables: [(&'static Grammar, &str); 12] = [
        (table("rust"), "rust"),
        (table("matlab"), "matlab"),
        (table("objective-c"), "objective-c"),
        (table("objective-cpp"), "objective-cpp"),
        (table("shellscript"), "shellscript"),
        (table("typescript"), "typescript"),
        (table("markdown"), "markdown"),
        (table("python"), "python"),
        (table("yaml"), "yaml"),
        (table("make"), "make"),
        (table("toml"), "toml"),
        (table("javascript"), "javascript"),
    ];
    for (grammar, language) in tables.into_iter().chain(SHIPPED.iter().copied()) {
        for path in fixtures(language) {
            let lines = parse(&path);
            assert_eq!(
                scoped_tokens(grammar, &lines, Frames::Kept),
                scoped_tokens(grammar, &lines, Frames::Rebuilt),
                "{}",
                path.display()
            );
        }
    }
}

/// The engine grammars the crate ships, with the oracle fixtures of each,
/// for the features that are on.
static SHIPPED: &[(&Grammar, &str)] = &[];

/// Lines with text beyond ASCII:
///
/// - text that starts with every UTF-8 lead byte from `0xC2` to `0xF4`, alone,
///   between ASCII and all together;
/// - such text across and after the 64-byte blocks of the ASCII check;
/// - samples of CJK, Cyrillic, Greek, Arabic, emoji and a combining mark alone.
pub(super) fn lines_beyond_ascii() -> Vec<String> {
    let mut buffer = [0; 4];
    let firsts: Vec<char> = (0xc2..=0xf4_u8)
        .filter_map(|lead| {
            (0x80..=0x10_ffff)
                .filter_map(char::from_u32)
                .find(|c| c.encode_utf8(&mut buffer).as_bytes().first() == Some(&lead))
        })
        .collect();
    let mut lines: Vec<String> = firsts
        .iter()
        .flat_map(|c| [c.to_string(), format!("a{c}b ss {c}")])
        .collect();
    lines.push(firsts.iter().collect());
    let ascii = "x".repeat(63);
    lines.extend([
        format!("{ascii}ß"),
        format!("{ascii}xß{ascii}"),
        format!("{ascii}{ascii}xx漢字 ﬁ"),
        format!("{ascii}{ascii}xx漢字"),
    ]);
    lines.extend(
        [
            "let 漢字 = \"中文字符\"",
            "Привет, мир",
            "λόγος και ἀρχή",
            "مرحبا بالعالم",
            "party 🎉🎉 time",
            "\u{301}",
            "straße ﬁle",
            "x ∈ A → y ≤ z",
            "R = 5 Ω",
        ]
        .map(String::from),
    );
    lines
}

/// Every shipped engine grammar lexes the lines of [`lines_beyond_ascii`]
/// within the default work limit.
/// The line kind's check, and each step after it, makes progress on any text.
#[test]
fn lines_beyond_ascii_lex_within_the_work_limit() {
    let limits = Limits::default();
    let lines = lines_beyond_ascii();
    for &(grammar, language) in SHIPPED {
        let mut arena = Arena::new(None);
        let mut scratch = Scratch::default();
        let mut state = tokenizer::start(&mut arena, grammar, crate::arena::ROOT);
        for line in &lines {
            let mut out = Vec::new();
            let mut report = LineReport::default();
            state = tokenizer::line(
                &mut scratch,
                &mut arena,
                &limits,
                Text {
                    grammar,
                    bytes: line.as_bytes(),
                    start: 0,
                    anchored: false,
                },
                state,
                &mut out,
                &mut report,
            );
            assert!(report.is_complete(), "{language} on {line:?}: {report:?}");
        }
    }
}

/// The most fixture lines of one grammar the lean check reads.
const LEAN_LINES: usize = 3_000;

/// The letters the folds of more than one character are made of in ASCII, in
/// both cases, and the pairs they form.
const FOLD_LETTERS: &[u8] = b"sSfFtTiIlL";
const FOLD_PAIRS: &[&str] = &["ss", "SS", "st", "ST", "ff", "fi", "FI", "fl", "ffi", "ffl"];

/// A step of a xorshift generator, so the mutations are the same on every
/// run.
fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

/// Two copies of `line` mutated in ASCII, as `oracle/fuzz.ts` mutates a
/// corpus.
///
/// - One has a third of its letters turned into the letters of the folds.
/// - One has a pair of them put in at a random place.
fn fold_mutations(line: &str, seed: &mut u64) -> [String; 2] {
    let letters: String = line
        .chars()
        .map(|c| {
            let roll = next(seed);
            if c.is_ascii_alphabetic() && roll.is_multiple_of(3) {
                let pick = usize::try_from(roll / 3).unwrap_or(0) % FOLD_LETTERS.len();
                char::from(FOLD_LETTERS[pick])
            } else {
                c
            }
        })
        .collect();
    let boundaries: Vec<usize> = (0..=line.len())
        .filter(|&at| line.is_char_boundary(at))
        .collect();
    let roll = usize::try_from(next(seed)).unwrap_or(0);
    let at = boundaries[roll % boundaries.len()];
    let pair = FOLD_PAIRS[(roll / boundaries.len()) % FOLD_PAIRS.len()];
    [letters, format!("{}{pair}{}", &line[..at], &line[at..])]
}

/// Every match of `regex` in `text`, each with its groups, searching on
/// from the end of the last as the tokenizer does, with `\G` where each
/// search starts.
fn every_match(regex: &Regex, text: &[u8]) -> Vec<Vec<Option<(usize, usize)>>> {
    let mut scratch = regex::Scratch::default();
    let mut found = Vec::new();
    let mut from = 0;
    while from <= text.len() {
        let mut steps = 10_000_000;
        let context = Context {
            search: Some(from),
            text_start: true,
        };
        if !regex.find(&mut scratch, text, from, context, &mut steps) {
            break;
        }
        let groups: Vec<Option<(usize, usize)>> =
            (0..=regex.groups()).map(|g| scratch.group(g)).collect();
        let Some(Some((start, end))) = groups.first().copied() else {
            break;
        };
        found.push(groups);
        from = if end > start { end } else { start + 1 };
        while text.get(from).is_some_and(|&b| b & 0xc0 == 0x80) {
            from += 1;
        }
    }
    found
}

/// Lean and full programs find the same matches and groups on a line that
/// holds no wide fold character.
///
/// The test covers every pattern of every shipped engine grammar whose lean
/// program differs.
///
/// The lines are the grammar's fixtures that are lean, up to [`LEAN_LINES`].
/// Two copies of each are mutated in ASCII toward the folds' letters and
/// pairs. The lean program must still match them as the full one does.
#[test]
fn lean_programs_match_as_full_ones_on_lean_lines() {
    let mut failures = Vec::new();
    let mut seed = 0x9e37_79b9_7f4a_7c15;
    for &(grammar, language) in SHIPPED {
        let patterns: Vec<&str> = grammar
            .patterns
            .iter()
            .map(|pattern| pattern.source)
            .filter(|source| Regex::lean_differs(source))
            .collect();
        if patterns.is_empty() {
            continue;
        }
        let read: Vec<String> = fixtures(language)
            .iter()
            .flat_map(|path| {
                parse(path)
                    .into_iter()
                    .map(|line| line.text)
                    .collect::<Vec<_>>()
            })
            .filter(|text| Programs::of(text.as_bytes()) == Programs::Lean)
            .collect();
        let stride = read.len().div_ceil(LEAN_LINES).max(1);
        let mut lines = Vec::new();
        for text in read.iter().step_by(stride) {
            lines.extend(fold_mutations(text, &mut seed));
            lines.push(text.clone());
        }
        for source in patterns {
            let lean = Regex::for_grammar(source, Programs::Lean);
            let full = Regex::for_grammar(source, Programs::Full);
            let (Ok(lean), Ok(full)) = (lean, full) else {
                failures.push(format!("{language}: {source:?} compiles for one kind only"));
                continue;
            };
            for line in &lines {
                let text = format!("{line}\n");
                if every_match(&lean, text.as_bytes()) != every_match(&full, text.as_bytes()) {
                    failures.push(format!("{language}: {source:?} on {line:?}"));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Every grammar with tables. HTML's joins when the languages it hands
/// regions to are built.
fn every_table() -> Vec<&'static Grammar> {
    let mut grammars: Vec<&'static Grammar> = SHIPPED.iter().map(|&(grammar, _)| grammar).collect();
    grammars.extend([
        table("javascript"),
        table("make"),
        table("markdown"),
        table("python"),
        table("rust"),
        table("shellscript"),
        table("toml"),
        table("typescript"),
        table("yaml"),
    ]);
    #[cfg(all(feature = "lang-javascript", feature = "lang-css"))]
    grammars.push(table("html"));
    grammars
}

/// Each list that holds a hand-off, with the hand-off's index in
/// [`Grammar::hand_offs`].
fn hand_off_lists(grammar: &Grammar) -> Vec<(usize, usize)> {
    grammar
        .list_hand_offs
        .iter()
        .enumerate()
        .filter_map(|(list, index)| Some((list, usize::from((*index)?))))
        .collect()
}

/// A grammar that hands off names a hand-off or none for every list. Exactly
/// one list names each hand-off. It holds the rules of the hand-off's
/// `before` list, then the rules of its `after` list. Neither part is the
/// list of a hand-off.
#[test]
fn a_hand_off_splits_its_list_in_two() {
    let mut wrong = Vec::new();
    for grammar in every_table() {
        let handed = hand_off_lists(grammar);
        if !grammar.hand_offs.is_empty() && grammar.list_hand_offs.len() != grammar.lists.len() {
            wrong.push(format!("{}: one entry per list", grammar.scope));
        }
        for (index, hand_off) in grammar.hand_offs.iter().enumerate() {
            let lists: Vec<usize> = handed
                .iter()
                .filter(|&&(_, at)| at == index)
                .map(|&(list, _)| list)
                .collect();
            let before = grammar.lists[hand_off.before].rules;
            let after = grammar.lists[hand_off.after].rules;
            let split = match lists.as_slice() {
                &[list] => grammar.lists[list].rules == [before, after].concat(),
                _ => false,
            };
            let part_handed = handed
                .iter()
                .any(|&(list, _)| list == hand_off.before || list == hand_off.after);
            if !split || part_handed {
                wrong.push(format!(
                    "{} hand-off {index} ({:?})",
                    grammar.scope, hand_off.language
                ));
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "hand-offs that do not split their list: {}",
        wrong.join(", ")
    );
}

/// A capture runs the handing loop only when its rule can reach a hand-off.
/// `Grammar::handing_captures` lists exactly the capture rules that reach one,
/// in their own list or through a nested rule. A capture missing from it would
/// drop its hand-off.
#[test]
fn the_handing_captures_are_the_captures_that_reach_a_hand_off() {
    let mut wrong = Vec::new();
    for grammar in every_table() {
        let handed: Vec<usize> = hand_off_lists(grammar)
            .into_iter()
            .map(|(list, _)| list)
            .collect();
        let retokenized: BTreeSet<usize> = grammar
            .rules
            .iter()
            .flat_map(|rule| rule.captures.iter().chain(rule.end_captures))
            .map(|cap| cap.retokenize)
            .filter(|&rule| rule != NONE)
            .collect();
        let reaching: Vec<usize> = retokenized
            .into_iter()
            .filter(|&start| {
                let mut seen = BTreeSet::new();
                let mut stack = vec![start];
                while let Some(rule) = stack.pop() {
                    if !seen.insert(rule) {
                        continue;
                    }
                    let list = grammar.rules[rule].list;
                    if handed.contains(&list) {
                        return true;
                    }
                    stack.extend(grammar.lists[list].rules);
                }
                false
            })
            .collect();
        if reaching != grammar.handing_captures {
            wrong.push(format!(
                "{}: {:?}, not {:?}",
                grammar.scope, grammar.handing_captures, reaching
            ));
        }
    }
    assert!(
        wrong.is_empty(),
        "handing captures that differ from those that reach a hand-off: {}",
        wrong.join("; ")
    );
}

/// No hand-off halt uses `\G`. The rule applies to the `before` and `after`
/// rules and to the end of the rule that opens the region. The engine finds
/// where the halts start before the language runs. `\G` matches where the
/// last match ended, which those searches do not know.
#[test]
fn no_hand_off_halts_where_the_last_match_ended() {
    let uses_last_match_end = |source: &str| {
        let mut bytes = source.bytes();
        while let Some(byte) = bytes.next() {
            if byte == b'\\' && bytes.next() == Some(b'G') {
                return true;
            }
        }
        false
    };
    let mut wrong = Vec::new();
    for grammar in every_table() {
        for (list, index) in hand_off_lists(grammar) {
            let hand_off = &grammar.hand_offs[index];
            let parts = [hand_off.before, hand_off.after]
                .into_iter()
                .flat_map(|list| grammar.lists[list].rules)
                .map(|&rule| grammar.rules[rule].begin);
            let ends = grammar
                .rules
                .iter()
                .filter(|rule| rule.list == list && rule.matching == Matching::BeginEnd)
                .map(|rule| rule.end);
            for pattern in parts.chain(ends) {
                if let Some(pattern) = grammar.patterns.get(pattern)
                    && uses_last_match_end(pattern.source)
                {
                    wrong.push(format!("{}: {}", grammar.scope, pattern.source));
                }
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "hand-off halts that use \\G: {}",
        wrong.join(", ")
    );
}

/// HAML's filters hand their whole region to Ruby, JavaScript, CSS and
/// Markdown. No rule of HAML's stands around their include. A line of Ruby
/// code, `= expr`, keeps HAML's rules before the include and after it.
#[cfg(feature = "lang-haml")]
#[test]
fn haml_hands_its_filters_over_whole() {
    use crate::language::Language;
    let grammar = table("haml");
    let hand_offs: Vec<(Language, usize, usize)> = grammar
        .hand_offs
        .iter()
        .map(|hand_off| {
            (
                hand_off.language,
                grammar.lists[hand_off.before].rules.len(),
                grammar.lists[hand_off.after].rules.len(),
            )
        })
        .collect();
    for filter in [
        Language::Ruby,
        Language::JavaScript,
        Language::Css,
        Language::Markdown,
    ] {
        assert!(
            hand_offs.contains(&(filter, 0, 0)),
            "{filter:?}'s filter: {hand_offs:?}"
        );
    }
    assert!(
        hand_offs
            .iter()
            .any(|&(language, before, after)| language == Language::Ruby
                && before > 0
                && after > 0),
        "Ruby code: {hand_offs:?}"
    );
}

/// Times the engine on the fixtures of one language, in MiB/s.
/// It reports three figures:
///
/// - the best of five passes, each with a fresh scratch: a document of its own,
///   whose lazy automata start empty;
/// - the best of five passes that reuse one scratch;
/// - the slowest line's best time in the fresh passes.
///
/// The first pass is not timed.
/// It builds the scanners, which a program builds once.
/// Run with `cargo test --release --lib engine_speed -- --ignored --nocapture`.
#[expect(
    clippy::cast_precision_loss,
    clippy::as_conversions,
    reason = "a measurement that prints its table"
)]
fn speed(grammar: &'static Grammar, language: &str, limits: &Limits, scanners: Scanners) {
    use std::time::Instant;
    for path in fixtures(language) {
        let lines = parse(&path);
        let bytes: usize = lines.iter().map(|l| l.text.len() + 1).sum();
        let mut worst = vec![f64::MAX; lines.len()];
        let pass = |scratch: &mut Scratch, worst: Option<&mut Vec<f64>>| {
            let mut arena = Arena::new(limits.nodes);
            scratch.no_scanner = scanners == Scanners::Off;
            let mut state = tokenizer::start(&mut arena, grammar, crate::arena::ROOT);
            let mut out = Vec::new();
            let mut worst = worst;
            let begin = Instant::now();
            for (number, line) in lines.iter().enumerate() {
                out.clear();
                let mut report = LineReport::default();
                let t = Instant::now();
                state = tokenizer::line(
                    scratch,
                    &mut arena,
                    limits,
                    Text {
                        grammar,
                        bytes: line.text.as_bytes(),
                        start: 0,
                        anchored: false,
                    },
                    state,
                    &mut out,
                    &mut report,
                );
                if let Some(worst) = worst.as_deref_mut() {
                    worst[number] = worst[number].min(t.elapsed().as_secs_f64());
                }
            }
            begin.elapsed().as_secs_f64()
        };
        let first = pass(&mut Scratch::default(), None);
        eprintln!("  first pass (builds the scanners) {:.1} ms", first * 1e3);
        let cold = (0..5)
            .map(|_| pass(&mut Scratch::default(), Some(&mut worst)))
            .fold(f64::MAX, f64::min);
        let mut scratch = Scratch::default();
        let warm = (0..5)
            .map(|_| pass(&mut scratch, None))
            .fold(f64::MAX, f64::min);
        let mut sorted = worst.clone();
        sorted.sort_by(f64::total_cmp);
        let p99 = sorted[sorted.len() * 99 / 100];
        let max = sorted[sorted.len() - 1];
        let mib = |seconds: f64| bytes as f64 / seconds / 1_048_576.0;
        eprintln!(
            "{language:12} {:32} cold {:7.2}  warm {:7.2} MiB/s  p99 {:7.1} us  max {:7.1} us",
            path.file_stem().and_then(|s| s.to_str()).unwrap_or(""),
            mib(cold),
            mib(warm),
            p99 * 1e6,
            max * 1e6,
        );
    }
}

#[test]
#[ignore = "a measurement, run in release mode"]
fn engine_speed() {
    let limits = Limits::default();
    for scanners in [Scanners::Off, Scanners::On] {
        eprintln!(
            "scanners {}",
            if scanners == Scanners::Off {
                "off"
            } else {
                "on"
            }
        );
        speed(table("rust"), "rust", &limits, scanners);
        speed(table("shellscript"), "shellscript", &limits, scanners);
        speed(table("typescript"), "typescript", &limits, scanners);
        speed(table("markdown"), "markdown", &limits, scanners);
        speed(table("python"), "python", &limits, scanners);
        speed(table("yaml"), "yaml", &limits, scanners);
    }
}

#[test]
fn an_embedded_line_is_lexed_from_its_start_with_absolute_ends() {
    let grammar: &'static Grammar = table("rust");
    let mut arena = Arena::new(None);
    let mut scratch = Scratch::default();
    let state = tokenizer::start_embedded(&mut arena, grammar, crate::arena::ROOT);
    let mut out = Vec::new();
    let mut report = LineReport::default();
    let line = b"> let x = 1;";
    tokenizer::line(
        &mut scratch,
        &mut arena,
        &Limits::unlimited(),
        Text {
            grammar,
            bytes: line,
            start: 2,
            anchored: true,
        },
        state,
        &mut out,
        &mut report,
    );
    let ends: Vec<usize> = out.iter().map(|t| t.end).collect();
    assert!(ends.iter().all(|&end| end > 2), "{ends:?}");
    assert_eq!(ends.last(), Some(&line.len()));
    let first = arena.scopes(out[0].stack.0).collect::<Vec<_>>();
    assert_eq!(first.first(), Some(&"storage.type.rust"), "{first:?}");
    assert!(!first.contains(&"source.rust"), "{first:?}");
}

/// The most work any oracle line costs the machine, as a share of the
/// default budget.
/// That budget is 64 steps a byte plus 4,096, at 32 machine steps each.
#[test]
#[ignore = "a measurement"]
#[expect(
    clippy::cast_precision_loss,
    clippy::as_conversions,
    reason = "a measurement that prints its table"
)]
fn work_per_line() {
    let grammars: [(&str, &'static Grammar); 6] = [
        ("rust", table("rust")),
        ("shellscript", table("shellscript")),
        ("typescript", table("typescript")),
        ("markdown", table("markdown")),
        ("python", table("python")),
        ("yaml", table("yaml")),
    ];
    for (language, grammar) in grammars {
        let mut worst = 0.0_f64;
        let mut worst_line = String::new();
        for path in fixtures(language) {
            let lines = parse(&path);
            let mut arena = Arena::new(None);
            let mut scratch = Scratch::default();
            let mut state = tokenizer::start(&mut arena, grammar, crate::arena::ROOT);
            for line in &lines {
                let mut out = Vec::new();
                let mut report = LineReport::default();
                state = tokenizer::line(
                    &mut scratch,
                    &mut arena,
                    &Limits::unlimited(),
                    Text {
                        grammar,
                        bytes: line.text.as_bytes(),
                        start: 0,
                        anchored: false,
                    },
                    state,
                    &mut out,
                    &mut report,
                );
                let budget = (64 * line.text.len() + 4096) * 32;
                let ratio = scratch.spent as f64 / budget as f64;
                if ratio > worst {
                    worst = ratio;
                    worst_line = format!("{} steps: {:?}", scratch.spent, line.text);
                }
            }
        }
        eprintln!(
            "{language:12} worst line {worst:5.2} of the default budget  {}",
            worst_line.chars().take(90).collect::<String>()
        );
    }
}

#[cfg(feature = "json")]
mod fidelity;
#[cfg(all(feature = "json", feature = "lang-javascript", feature = "lang-css"))]
mod loaded;

/// A grammar of one rule that names its forty-second group, as C++'s
/// function definitions name a group far into their pattern.
mod many_groups {
    use std::sync::OnceLock;

    use super::super::grammar::{Capture, Folding, Grammar, List, Matching, Pattern, Rule};

    static PATTERNS: [Pattern; 1] = [Pattern::new(
        "()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()(x)",
    )];
    static LISTS: [List; 2] = [List::new(&[1]), List::new(&[])];
    static RULES: [Rule; 2] = [
        Rule {
            matching: Matching::Container,
            name: &[],
            content_name: &[],
            begin: usize::MAX,
            end: usize::MAX,
            until: usize::MAX,
            captures: &[],
            end_captures: &[],
            list: 0,
            apply_end_last: false,
        },
        Rule {
            matching: Matching::Match,
            name: &[],
            content_name: &[],
            begin: 0,
            end: usize::MAX,
            until: usize::MAX,
            captures: &[Capture {
                index: 41,
                name: &[0],
                content_name: &[],
                retokenize: usize::MAX,
            }],
            end_captures: &[],
            list: 1,
            apply_end_last: false,
        },
    ];
    pub(super) static GRAMMAR: Grammar = Grammar {
        scope: "source.test",
        names: &["keyword.test"],
        rules: &RULES,
        lists: &LISTS,
        injections: &[],
        patterns: &PATTERNS,
        root: 0,
        hand_offs: &[],
        list_hand_offs: &[],
        handing_captures: &[],
        dfa_cache: 1 << 22,
        folding: Folding::Same,
        cells: OnceLock::new(),
        full_cells: OnceLock::new(),
    };
}

#[test]
fn a_group_far_into_a_pattern_is_named() {
    let grammar: &'static Grammar = &many_groups::GRAMMAR;
    let mut arena = Arena::new(None);
    let mut scratch = Scratch::default();
    let state = tokenizer::start(&mut arena, grammar, crate::arena::ROOT);
    let mut out = Vec::new();
    tokenizer::line(
        &mut scratch,
        &mut arena,
        &Limits::unlimited(),
        Text {
            grammar,
            bytes: b"ax",
            start: 0,
            anchored: false,
        },
        state,
        &mut out,
        &mut LineReport::default(),
    );
    let scopes: Vec<Vec<&str>> = out
        .iter()
        .map(|t| arena.scopes(t.stack.0).collect())
        .collect();
    assert_eq!(scopes.len(), 2, "{scopes:?}");
    assert_eq!(scopes[1], ["keyword.test", "source.test"]);
}

/// A grammar of one rule, `(<)(.*)`, whose second group runs the root's
/// rules again.
/// A line of `<` nests one group per `<`, as Perl's heredoc openers do with
/// the rest of their line.
mod nested_captures {
    use std::sync::OnceLock;

    use super::super::grammar::{Capture, Folding, Grammar, List, Matching, NONE, Pattern, Rule};

    static PATTERNS: [Pattern; 1] = [Pattern::new("(<)(.*)")];
    static LISTS: [List; 2] = [List::new(&[1]), List::new(&[])];
    static RULES: [Rule; 2] = [
        Rule {
            matching: Matching::Container,
            name: &[],
            content_name: &[],
            begin: NONE,
            end: NONE,
            until: NONE,
            captures: &[],
            end_captures: &[],
            list: 0,
            apply_end_last: false,
        },
        Rule {
            matching: Matching::Match,
            name: &[],
            content_name: &[],
            begin: 0,
            end: NONE,
            until: NONE,
            captures: &[
                Capture {
                    index: 1,
                    name: &[0],
                    content_name: &[],
                    retokenize: NONE,
                },
                Capture {
                    index: 2,
                    name: &[],
                    content_name: &[],
                    retokenize: 0,
                },
            ],
            end_captures: &[],
            list: 1,
            apply_end_last: false,
        },
    ];
    pub(super) static GRAMMAR: Grammar = Grammar {
        scope: "source.test",
        names: &["punctuation.test"],
        rules: &RULES,
        lists: &LISTS,
        injections: &[],
        patterns: &PATTERNS,
        root: 0,
        hand_offs: &[],
        list_hand_offs: &[],
        handing_captures: &[],
        dfa_cache: 1 << 22,
        folding: Folding::Same,
        cells: OnceLock::new(),
        full_cells: OnceLock::new(),
    };
}

/// Nested capture groups stop at the engine's bound, below the depth limit.
/// Each level is a nested call.
/// A debug build on a test thread's 2 MiB stack overflowed on this line of
/// 1,000 levels.
///
/// A much longer line runs out of work before it nests that deep.
/// The line still ends with a token, and reports the depth limit.
#[test]
fn nested_captures_stop_at_their_bound() {
    let grammar: &'static Grammar = &nested_captures::GRAMMAR;
    let mut arena = Arena::new(None);
    let mut scratch = Scratch::default();
    let state = tokenizer::start(&mut arena, grammar, crate::arena::ROOT);
    let line = "<".repeat(1_000);
    let mut out = Vec::new();
    let mut report = LineReport::default();
    tokenizer::line(
        &mut scratch,
        &mut arena,
        &Limits::default(),
        Text {
            grammar,
            bytes: line.as_bytes(),
            start: 0,
            anchored: false,
        },
        state,
        &mut out,
        &mut report,
    );
    assert!(report.limits().any(|limit| limit == Limit::Depth));
    assert_eq!(out.last().map(|token| token.end), Some(line.len()));
}

/// A grammar whose block, `{` to `}`, holds `x`, and whose injection,
/// `L:meta.block.test`, tries its own `x` in any context under the block.
/// Outside the block, `x` is the grammar's own.
mod injected {
    use std::sync::OnceLock;

    use super::super::grammar::{Folding, Grammar, Injection, List, Matching, NONE, Pattern, Rule};

    /// A rule that matches as `matching`, with no captures.
    const fn rule(
        matching: Matching,
        name: &'static [u16],
        begin: usize,
        end: usize,
        list: usize,
    ) -> Rule {
        Rule {
            matching,
            name,
            content_name: &[],
            begin,
            end,
            until: NONE,
            captures: &[],
            end_captures: &[],
            list,
            apply_end_last: false,
        }
    }

    static PATTERNS: [Pattern; 3] = [Pattern::new(r"\{"), Pattern::new(r"\}"), Pattern::new("x")];
    static LISTS: [List; 4] = [
        List::new(&[1, 2]),
        List::new(&[2]),
        List::new(&[]),
        List::new(&[4]),
    ];
    static RULES: [Rule; 5] = [
        rule(Matching::Container, &[], NONE, NONE, 0),
        rule(Matching::BeginEnd, &[0], 0, 1, 1),
        rule(Matching::Match, &[1], 2, NONE, 2),
        rule(Matching::Container, &[], NONE, NONE, 3),
        rule(Matching::Match, &[2], 2, NONE, 2),
    ];
    static INJECTIONS: [Injection; 1] = [Injection {
        selector: "L:meta.block.test",
        rule: 3,
    }];
    pub(super) static GRAMMAR: Grammar = Grammar {
        scope: "source.test",
        names: &["meta.block.test", "plain.test", "injected.test"],
        rules: &RULES,
        lists: &LISTS,
        injections: &INJECTIONS,
        patterns: &PATTERNS,
        root: 0,
        hand_offs: &[],
        list_hand_offs: &[],
        handing_captures: &[],
        dfa_cache: 1 << 22,
        folding: Folding::Same,
        cells: OnceLock::new(),
        full_cells: OnceLock::new(),
    };
}

/// An `L:` injection applies where its selector matches the scopes, and wins
/// a tie with the context's own patterns.
/// No grammar the crate ships has an injection, so this test grammar keeps
/// the path tested.
#[test]
fn an_injection_wins_a_tie_where_its_selector_matches() {
    let grammar: &'static Grammar = &injected::GRAMMAR;
    let mut arena = Arena::new(None);
    let mut scratch = Scratch::default();
    let state = tokenizer::start(&mut arena, grammar, crate::arena::ROOT);
    let line = "x {x}";
    let mut out = Vec::new();
    tokenizer::line(
        &mut scratch,
        &mut arena,
        &Limits::default(),
        Text {
            grammar,
            bytes: line.as_bytes(),
            start: 0,
            anchored: false,
        },
        state,
        &mut out,
        &mut LineReport::default(),
    );
    let scopes = |end: usize| -> Vec<&str> {
        out.iter()
            .find(|t| t.end == end)
            .map(|t| arena.scopes(t.stack.0).collect())
            .unwrap_or_default()
    };
    assert_eq!(scopes(1), ["plain.test", "source.test"]);
    assert_eq!(
        scopes(4),
        ["injected.test", "meta.block.test", "source.test"]
    );
}

/// A tab that a capture brings into a filled scope name stays inside that
/// scope, as vscode-textmate splits a name at spaces only.
/// The shell grammar's quoted `<<-` heredoc end names its tab-indented
/// delimiter.
#[test]
fn a_tab_in_a_filled_name_stays_in_its_scope() {
    let lines: Vec<Line> = ["cat <<-\"EOF\"", "\ttext", "\tEOF"]
        .into_iter()
        .map(|text| Line {
            text: text.to_owned(),
            stacks: Vec::new(),
        })
        .collect();
    let lexed = scoped_tokens(table("shellscript"), &lines, Frames::Kept);
    let end = lexed.last().expect("the delimiter's line");
    assert!(
        end.iter()
            .any(|(_, scopes)| scopes
                .ends_with(" punctuation.definition.string.heredoc.\tEOF.shell")),
        "{end:?}"
    );
}

/// The halts the tests below pass, as an embedding language's would be.
mod halts {
    use super::super::tokenizer::{self, Embedded, Scratch, Text};
    use super::table;
    use crate::arena::{Arena, ROOT};
    use crate::language::{Context, Halt, Line};
    use crate::limits::{Limit, Limits, LineReport, Steps, Work};

    /// Whether `s` holds `%>` at `p`, as ERB's end would.
    fn at_erb_close(s: &[u8], p: usize) -> bool {
        s.get(p..).is_some_and(|rest| rest.starts_with(b"%>"))
    }

    /// Whether `s` holds a backtick at `p`, which no Rust pattern matches.
    fn at_backtick(s: &[u8], p: usize) -> bool {
        s.get(p) == Some(&b'`')
    }

    /// Whether `s` holds `=` at `p`, which a Rust pattern matches.
    fn at_equals(s: &[u8], p: usize) -> bool {
        s.get(p) == Some(&b'=')
    }

    /// A halt that never holds.
    fn never(_: &[u8], _: usize) -> bool {
        false
    }

    /// Each line of `lines`, lexed in turn by the Rust test grammar as an
    /// embedded language.
    ///
    /// The language halts where `end` or `after` returns true.
    /// The result holds where it halted, and the ends of its tokens.
    fn halting(
        lines: &[&str],
        end: &dyn Halt,
        after: Option<&dyn Halt>,
    ) -> Vec<(Option<usize>, Vec<usize>)> {
        let grammar = table("rust");
        let mut arena = Arena::new(None);
        let mut scratch = Scratch::default();
        let mut state = tokenizer::start_embedded(&mut arena, grammar, ROOT);
        lines
            .iter()
            .map(|line| {
                let mut out = Vec::new();
                let mut report = LineReport::default();
                let (next, halted) = tokenizer::line_until(
                    &mut scratch,
                    &mut arena,
                    &Limits::unlimited(),
                    Text {
                        grammar,
                        bytes: line.as_bytes(),
                        start: 0,
                        anchored: false,
                    },
                    state,
                    &mut out,
                    &mut report,
                    Embedded { end, after },
                );
                state = next;
                (halted, out.iter().map(|token| token.end).collect())
            })
            .collect()
    }

    /// One line, lexed as [`halting`] lexes each.
    fn halting_one(
        line: &str,
        end: &dyn Halt,
        after: Option<&dyn Halt>,
    ) -> (Option<usize>, Vec<usize>) {
        halting(&[line], end, after)
            .into_iter()
            .next()
            .unwrap_or_default()
    }

    #[test]
    fn an_end_halt_takes_the_line_back_at_the_root() {
        let (halted, ends) = halting_one("let x = 1 %> rest", &at_erb_close, None);
        assert_eq!(halted, Some(10));
        assert_eq!(ends.last(), Some(&10), "{ends:?}");
    }

    /// The end is tried before the root's patterns.
    /// Rust's `%` operator, which matches at the same place, loses to it.
    /// The name that starts earlier keeps its token.
    #[test]
    fn an_end_halt_beats_a_match_at_its_place_but_not_an_earlier_one() {
        let (halted, ends) = halting_one("x%>", &at_erb_close, None);
        assert_eq!(halted, Some(1));
        assert_eq!(ends, [1]);
    }

    #[test]
    fn an_end_halt_waits_while_a_rule_is_open() {
        let line = r#"let s = "a %> b"; %>"#;
        let (halted, _) = halting_one(line, &at_erb_close, None);
        assert_eq!(halted, line.rfind("%>"));
    }

    #[test]
    fn a_halt_holds_at_the_lines_start() {
        let (halted, ends) = halting_one("%> x", &at_erb_close, None);
        assert_eq!(halted, Some(0));
        assert!(ends.is_empty(), "{ends:?}");
    }

    #[test]
    fn an_after_halt_holds_only_where_no_pattern_matches() {
        let line = "let a = 1 ` b";
        let (halted, _) = halting_one(line, &never, Some(&at_backtick));
        assert_eq!(halted, line.find('`'));
        let (halted, _) = halting_one("a = b", &never, Some(&at_equals));
        assert_eq!(halted, None);
    }

    /// A comment left open on one line keeps the root closed on the next.
    /// The state after a halt is the root's, so the next line starts there.
    #[test]
    fn the_state_carries_across_lines() {
        let lines = ["/* open %>", "still %> */ x %>", "let y = 2;"];
        let halts: Vec<Option<usize>> = halting(&lines, &at_erb_close, None)
            .into_iter()
            .map(|(halted, _)| halted)
            .collect();
        assert_eq!(halts, [None, lines[1].rfind("%>"), None]);
    }

    /// `line`, lexed by the Rust test grammar as a language that a host
    /// line under `host` embeds and lends its work to.
    /// It returns three values:
    ///
    /// 1. what the host has spent once it takes the engine's count back;
    /// 2. what the engine spent, in steps of the work limit;
    /// 3. whether the line ran out of work.
    fn counted(line: &str, host: &Limits) -> (u64, u64, bool) {
        let grammar = table("rust");
        let mut arena = Arena::new(None);
        let mut scratch = crate::scratch::Scratch::default();
        let state = tokenizer::start_embedded(&mut arena, grammar, ROOT);
        let steps = Steps::new(host, line.len());
        let lent = steps.lend(host);
        let mut out = Vec::new();
        let mut report = LineReport::default();
        scratch.start_line();
        let mut cx = Context {
            scratch: &mut scratch,
            arena: &mut arena,
            limits: &lent,
            out: &mut out,
            report: &mut report,
            first_token: 0,
        };
        let text = Line {
            text: line,
            start: 0,
            embedded: false,
        };
        let (finished, _) = crate::textmate::lex_until(&mut cx, grammar, text, state, &never, None);
        steps.take_back(finished);
        (
            steps.into_spent(),
            scratch.textmate.work_spent(),
            report.reached(Limit::Work),
        )
    }

    #[test]
    fn the_host_line_counts_what_the_engine_spent() {
        let (host, engine, ran_out) = counted("let x = a + b * c;", &Limits::default());
        assert!(engine > 0, "the engine spent nothing");
        assert_eq!(host, engine);
        assert!(!ran_out);
    }

    /// With two steps left on the host line, the engine stops within them,
    /// where the same line under the default limits spends more.
    #[test]
    fn the_engine_spends_only_what_the_host_line_has_left() {
        let line = "let a = b + c * d - e / f; let g = h % i; let j = k && l || m;";
        let tight = Limits {
            work: Some(Work {
                per_byte: 0,
                base: 2,
            }),
            ..Limits::default()
        };
        let (_, free, _) = counted(line, &Limits::default());
        assert!(free > 2, "the line spends {free} steps");
        let (host, engine, ran_out) = counted(line, &tight);
        assert!(ran_out);
        assert!(engine <= 2, "the engine spent {engine} steps");
        assert_eq!(host, engine);
    }
}

/// Every begin pattern a shipped grammar's lists hold compiles if it parses,
/// for both kinds of line.
/// A scanner sketches a list's begin patterns from their parse alone.
/// A pattern that parsed but failed to compile would sit in the automaton.
/// Its candidates would then change the work a line spends.
///
/// An `end` or `while` pattern can refer to its begin's groups.
/// It compiles only once the tokenizer fills those in, and no list holds it.
#[test]
fn every_listed_pattern_that_parses_compiles() {
    let mut failures = Vec::new();
    for &(grammar, language) in SHIPPED {
        let begins: BTreeSet<usize> = grammar
            .lists
            .iter()
            .flat_map(|list| list.rules)
            .filter_map(|&rule| grammar.rules.get(rule).map(|rule| rule.begin))
            .collect();
        for pattern in begins
            .iter()
            .filter_map(|&begin| grammar.patterns.get(begin))
        {
            if Sketch::for_grammar(pattern.source).is_err() {
                continue;
            }
            for programs in [Programs::Lean, Programs::Full] {
                if let Err(error) = Regex::for_grammar(pattern.source, programs) {
                    failures.push(format!(
                        "{language} {programs:?} {:?}: {error:?}",
                        pattern.source
                    ));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// The engine hands regions to languages lexed by hand, on HAML's table and
/// HTML's test table. Design section 14.4 of
/// `openspec/changes/engine-handoff` describes the steps.
#[cfg(all(feature = "lang-haml", feature = "lang-objective-c"))]
mod hand_offs {
    use indoc::indoc;

    use super::super::grammar::Grammar;
    use super::super::tokenizer;
    use super::table;
    use crate::Token;
    use crate::arena::{Arena, ROOT};
    use crate::language::{Context, Line};
    use crate::limits::{Limit, Limits, LineReport, Steps, Work};
    use crate::scratch::Scratch;

    /// One token: its text and its scopes, outermost first.
    type Scoped = (String, Vec<String>);

    /// A document of a grammar that hands off. It is lexed a line at a time
    /// with one scratch and one arena, as a highlighter lexes it.
    struct Document {
        grammar: &'static Grammar,
        arena: Arena,
        scratch: Scratch,
        tokens: Vec<Token>,
    }

    impl Document {
        fn new(grammar: &'static Grammar) -> Document {
            Document {
                grammar,
                arena: Arena::new(None),
                scratch: Scratch::default(),
                tokens: Vec::new(),
            }
        }

        /// Lexes `text` from the start. It calls `each` with each line, the
        /// line's tokens and the arena.
        fn run(&mut self, text: &str, mut each: impl FnMut(&str, &[Token], &Arena)) {
            let limits = Limits::default();
            let mut state = tokenizer::start(&mut self.arena, self.grammar, ROOT);
            for line in text.lines() {
                self.tokens.clear();
                let mut report = LineReport::default();
                self.scratch.start_line();
                let mut cx = Context {
                    scratch: &mut self.scratch,
                    arena: &mut self.arena,
                    limits: &limits,
                    out: &mut self.tokens,
                    report: &mut report,
                    first_token: 0,
                };
                let text = Line {
                    text: line,
                    start: 0,
                    embedded: false,
                };
                state = crate::textmate::lex_handing(&mut cx, self.grammar, text, state);
                each(line, &self.tokens, &self.arena);
            }
        }

        /// Lexes `text` from the start, keeping each line's tokens.
        fn lex(&mut self, text: &str) -> Vec<Vec<Scoped>> {
            let mut rows = Vec::new();
            self.run(text, |line, tokens, arena| {
                let mut from = 0;
                rows.push(
                    tokens
                        .iter()
                        .map(|token| {
                            let mut stack: Vec<String> =
                                arena.scopes(token.stack().0).map(str::to_owned).collect();
                            stack.reverse();
                            let scoped = (line[from..token.end()].to_owned(), stack);
                            from = token.end();
                            scoped
                        })
                        .collect(),
                );
            });
            rows
        }
    }

    /// Whether `scope` is of `language`, the last part of its name.
    fn in_language(scope: &str, language: &str) -> bool {
        scope.rsplit('.').next() == Some(language)
    }

    /// The innermost scope of the first token whose text is `text`.
    fn scope_of<'a>(row: &'a [Scoped], text: &str) -> &'a str {
        row.iter()
            .find(|(token, _)| token.trim() == text)
            .and_then(|(_, scopes)| scopes.last())
            .map_or("", String::as_str)
    }

    #[test]
    fn a_ruby_filter_hands_its_lines_to_ruby_and_takes_the_line_back_after() {
        let rows = Document::new(table("haml")).lex(indoc! {"
            :ruby
              total = 1 + 2
            %p done
        "});
        assert_eq!(
            scope_of(&rows[1], "="),
            "keyword.operator.assignment.ruby",
            "{:?}",
            rows[1]
        );
        assert!(
            in_language(scope_of(&rows[1], "1"), "ruby"),
            "{:?}",
            rows[1]
        );
        assert!(
            rows[2]
                .iter()
                .all(|(_, scopes)| scopes.iter().all(|scope| !in_language(scope, "ruby"))),
            "{:?}",
            rows[2]
        );
    }

    /// As in Shiki, a `:sass` filter scopes only its own line, and its body
    /// lexes as HAML. A `:coffee` filter hands its lines to the CoffeeScript
    /// lexer. Its backticks hand on to JavaScript, three levels down.
    #[test]
    fn a_coffee_filter_lexes_coffeescript_and_a_sass_filter_ends_at_its_line() {
        let rows = Document::new(table("haml")).lex(indoc! {"
            :sass
              .a
                color: red
            :coffee
              x = `1`
        "});
        assert_eq!(
            scope_of(&rows[0], ":sass"),
            "source.sass.embedded.filter.haml",
            "{:?}",
            rows[0]
        );
        assert!(
            rows[1..3]
                .iter()
                .flatten()
                .all(|(_, stack)| stack.iter().all(|scope| !in_language(scope, "sass"))),
            "{:?}",
            &rows[1..3]
        );
        assert_eq!(
            scope_of(&rows[4], "1"),
            "constant.numeric.decimal.js",
            "{:?}",
            rows[4]
        );
    }

    #[test]
    fn a_markdown_filter_lexes_an_objective_c_fence_with_the_engine() {
        let rows = Document::new(table("haml")).lex(indoc! {"
            :markdown
              ```objc
              a = 1
              ```
        "});
        assert!(
            in_language(scope_of(&rows[2], "1"), "objc"),
            "{:?}",
            rows[2]
        );
    }

    /// The fence runs the engine inside the hand-off, on the spare scratch.
    /// Lexing the document again allocates nothing.
    #[test]
    fn lexing_a_nested_engine_fence_again_allocates_nothing() {
        let text = indoc! {"
            :markdown
              ```objc
              a = 1
              ```
            %p done
        "};
        let mut document = Document::new(table("haml"));
        let first = document.lex(text);
        let mut again = Vec::with_capacity(first.len());
        let allocations = nishiki_alloc_count::count(|| {
            document.run(text, |_, tokens, _| again.push(tokens.len()));
        });
        assert_eq!(again, first.iter().map(Vec::len).collect::<Vec<_>>());
        assert_eq!(allocations, 0);
    }

    /// Three engine levels nest here, each on its own spare scratch. A
    /// `:ruby` filter hands its lines to Ruby. Ruby's `<<~HAML` hands its
    /// lines to HAML. That HAML's `:markdown` filter hands its lines to
    /// Markdown, whose fence runs Objective-C on the engine. Lexing the
    /// document again allocates nothing.
    #[test]
    fn lexing_three_nested_engine_levels_again_allocates_nothing() {
        let text = indoc! {"
            :ruby
              page = <<~HAML
                :markdown
                  ```objc
                  a = 1
                  ```
              HAML
            %p done
        "};
        let mut document = Document::new(table("haml"));
        let first = document.lex(text);
        assert!(
            in_language(scope_of(&first[4], "1"), "objc"),
            "{:?}",
            first[4]
        );
        let mut again = Vec::with_capacity(first.len());
        let allocations = nishiki_alloc_count::count(|| {
            document.run(text, |_, tokens, _| again.push(tokens.len()));
        });
        assert_eq!(again, first.iter().map(Vec::len).collect::<Vec<_>>());
        assert_eq!(allocations, 0);
    }

    #[test]
    fn a_script_hands_javascript_its_text_up_to_the_end_tag() {
        let rows = Document::new(table("html")).lex("<script>let a = 1;</script><b>x</b>\n");
        assert!(
            in_language(scope_of(&rows[0], "let"), "js"),
            "{:?}",
            rows[0]
        );
        assert!(
            scope_of(&rows[0], "script").contains("html"),
            "{:?}",
            rows[0]
        );
        let after: Vec<&Scoped> = rows[0]
            .iter()
            .skip_while(|(text, _)| text != "</")
            .collect();
        assert!(
            after
                .iter()
                .all(|(_, scopes)| scopes.iter().all(|scope| !in_language(scope, "js"))),
            "{after:?}"
        );
    }

    #[test]
    fn a_style_hands_css_its_lines_across_the_document() {
        let rows = Document::new(table("html")).lex(indoc! {"
            <style>
            a { color: red; }
            </style>
        "});
        assert!(
            in_language(scope_of(&rows[1], "color"), "css"),
            "{:?}",
            rows[1]
        );
        assert!(
            rows[2]
                .iter()
                .all(|(_, scopes)| scopes.iter().all(|scope| !in_language(scope, "css"))),
            "{:?}",
            rows[2]
        );
    }

    /// An event attribute's value is a capture whose rule includes the whole
    /// JavaScript grammar. Its text goes to the JavaScript lexer. The line
    /// that lexer sees ends at the capture's end, so the tag closes in HTML.
    #[test]
    fn an_event_attribute_hands_its_value_to_javascript() {
        let rows = Document::new(table("html")).lex("<div onclick=\"var a = 1\">\n");
        assert_eq!(
            scope_of(&rows[0], "var"),
            "storage.type.js",
            "{:?}",
            rows[0]
        );
        assert_eq!(
            scope_of(&rows[0], "1"),
            "constant.numeric.decimal.js",
            "{:?}",
            rows[0]
        );
        assert_eq!(
            scope_of(&rows[0], ">"),
            "punctuation.definition.tag.end.html",
            "{:?}",
            rows[0]
        );
    }

    /// What the last of `lines` costs, lexed under `last` after the others
    /// under the default limits. HAML is a language that a host line embeds
    /// and lends its work to. Returns what the host spent once it takes
    /// HAML's count back, whether the line ran out of work, and whether its
    /// tokens cover it.
    fn budgeted(lines: &[&str], last: &Limits) -> (u64, bool, bool) {
        let mut arena = Arena::new(None);
        let mut scratch = Scratch::default();
        let mut state = tokenizer::start_embedded(&mut arena, table("haml"), ROOT);
        let mut out = Vec::new();
        let mut result = (0, false, false);
        for (index, &line) in lines.iter().enumerate() {
            let host = if index + 1 == lines.len() {
                *last
            } else {
                Limits::default()
            };
            let steps = Steps::new(&host, line.len());
            let lent = steps.lend(&host);
            out.clear();
            let mut report = LineReport::default();
            scratch.start_line();
            let mut cx = Context {
                scratch: &mut scratch,
                arena: &mut arena,
                limits: &lent,
                out: &mut out,
                report: &mut report,
                first_token: 0,
            };
            let text = Line {
                text: line,
                start: 0,
                embedded: false,
            };
            let finished =
                crate::textmate::lex_handing_embedded(&mut cx, table("haml"), text, state);
            state = steps.take_back(finished);
            result = (
                steps.into_spent(),
                report.reached(Limit::Work),
                out.last().is_some_and(|token| token.end() == line.len()),
            );
        }
        result
    }

    /// With two steps left on the host line, a HAML line stops within them.
    /// The rule holds whichever language lexes the costly part:
    ///
    /// - Ruby on a line of code;
    /// - JavaScript in a filter;
    /// - Objective-C in a Markdown fence in a filter.
    ///
    /// Under the default limits each line spends more.
    #[test]
    fn a_hand_off_spends_only_what_the_host_line_has_left() {
        let tight = Limits {
            work: Some(Work {
                per_byte: 0,
                base: 2,
            }),
            ..Limits::default()
        };
        let ruby = format!("= {}", "a + b * c - d / e % f && g || h; ".repeat(40));
        let javascript = format!("  {}", "let a = b + c * d - e / f; ".repeat(40));
        let objective_c = format!("  {}", "a = b + c * d - e / f; ".repeat(40));
        let documents = [
            vec![ruby.as_str()],
            vec![":javascript", javascript.as_str()],
            vec![":markdown", "  ```objc", objective_c.as_str()],
        ];
        for lines in documents {
            let (free, _, _) = budgeted(&lines, &Limits::default());
            assert!(free > 2, "{:?} spends {free} steps", lines[0]);
            let (spent, ran_out, covered) = budgeted(&lines, &tight);
            assert!(ran_out, "{:?} never ran out", lines[0]);
            assert!(spent <= 2, "{:?} spent {spent} steps", lines[0]);
            assert!(covered, "{:?} left its line uncovered", lines[0]);
        }
    }
}
