//! Scope selectors: the expressions that say where a grammar's injections
//! apply.
//! An example: `L:(meta.script.svelte | meta.style.svelte) - (meta source)`.
//!
//! A selector holds these parts:
//!
//! - scope paths, such as `a b`;
//! - groups in parentheses;
//! - `|` or `,` between alternatives;
//! - `-` before an operand it excludes.
//!
//! The path `a b` matches a stack that holds a scope named `a`.
//! Further in, the stack must hold a scope named `b`.
//!
//! A selector name matches a scope name equal to it.
//! It also matches a scope name that starts with it followed by a dot.
//!
//! A `L:` or `R:` in front orders the injection against the grammar's own
//! rules when both match at the same place.
//! `L:` puts the injection first, and `R:` puts it after.

/// A condition on a stack of scope names.
#[derive(Debug)]
pub(crate) enum Expr {
    /// The names in order, anywhere in the stack.
    Path(Vec<String>),
    /// None of the expression.
    Not(Box<Expr>),
    /// All of the expressions.
    And(Vec<Expr>),
    /// Any of the expressions.
    Or(Vec<Expr>),
}

/// One selector of an injection, with its priority: -1 for `L:`, 1 for `R:`
/// and 0 for neither.
#[derive(Debug)]
pub(crate) struct Matcher {
    pub(crate) priority: i8,
    pub(crate) expr: Expr,
}

impl Expr {
    /// Whether `scopes`, outermost first, satisfy the expression.
    pub(crate) fn matches(&self, scopes: &[&str]) -> bool {
        match self {
            Expr::Path(names) => {
                let mut from = 0;
                names.iter().all(|name| {
                    let found = scopes
                        .iter()
                        .skip(from)
                        .position(|scope| names_match(scope, name));
                    found.is_some_and(|at| {
                        from += at + 1;
                        true
                    })
                })
            }
            Expr::Not(inner) => !inner.matches(scopes),
            Expr::And(all) => all.iter().all(|e| e.matches(scopes)),
            Expr::Or(any) => any.iter().any(|e| e.matches(scopes)),
        }
    }
}

/// Whether the scope `scope` is the name `name` or one inside it.
fn names_match(scope: &str, name: &str) -> bool {
    scope == name
        || scope
            .strip_prefix(name)
            .is_some_and(|rest| rest.starts_with('.'))
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Token {
    Name(String),
    Prefix(i8),
    Comma,
    Bar,
    Minus,
    Open,
    Close,
}

fn is_name_start(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '_' | '.' | ':' | '+' | '*')
}

fn tokens(source: &str) -> Vec<Token> {
    let mut out = Vec::new();
    let mut chars = source.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            ',' => out.push(Token::Comma),
            '|' => out.push(Token::Bar),
            '-' => out.push(Token::Minus),
            '(' => out.push(Token::Open),
            ')' => out.push(Token::Close),
            'L' | 'R' if chars.peek() == Some(&':') => {
                chars.next();
                out.push(Token::Prefix(if c == 'L' { -1 } else { 1 }));
            }
            c if is_name_start(c) => {
                let mut name = String::from(c);
                while let Some(&next) = chars.peek() {
                    if is_name_start(next) || next == '-' {
                        name.push(next);
                        chars.next();
                    } else {
                        break;
                    }
                }
                out.push(Token::Name(name));
            }
            _ => {}
        }
    }
    out
}

struct Parser {
    tokens: Vec<Token>,
    at: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.at)
    }

    fn operand(&mut self) -> Option<Expr> {
        match self.peek()? {
            Token::Minus => {
                self.at += 1;
                self.operand().map(|e| Expr::Not(Box::new(e)))
            }
            Token::Open => {
                self.at += 1;
                let inner = self.alternatives();
                if self.peek() == Some(&Token::Close) {
                    self.at += 1;
                }
                inner
            }
            Token::Name(_) => {
                let mut names = Vec::new();
                while let Some(Token::Name(name)) = self.peek() {
                    names.push(name.clone());
                    self.at += 1;
                }
                Some(Expr::Path(names))
            }
            Token::Prefix(_) | Token::Comma | Token::Bar | Token::Close => None,
        }
    }

    fn conjunction(&mut self) -> Option<Expr> {
        let mut all = Vec::new();
        while let Some(operand) = self.operand() {
            all.push(operand);
        }
        match all.len() {
            0 => None,
            1 => all.pop(),
            _ => Some(Expr::And(all)),
        }
    }

    /// The conjunctions between `|` or `,`, inside parentheses.
    fn alternatives(&mut self) -> Option<Expr> {
        let mut any = Vec::new();
        while let Some(conjunction) = self.conjunction() {
            any.push(conjunction);
            if matches!(self.peek(), Some(Token::Bar | Token::Comma)) {
                while matches!(self.peek(), Some(Token::Bar | Token::Comma)) {
                    self.at += 1;
                }
            } else {
                break;
            }
        }
        match any.len() {
            0 => None,
            1 => any.pop(),
            _ => Some(Expr::Or(any)),
        }
    }
}

/// The selectors of an expression, separated by commas at the top level.
pub(crate) fn parse(source: &str) -> Vec<Matcher> {
    let mut parser = Parser {
        tokens: tokens(source),
        at: 0,
    };
    let mut out = Vec::new();
    while parser.peek().is_some() {
        let priority = match parser.peek() {
            Some(Token::Prefix(priority)) => {
                let priority = *priority;
                parser.at += 1;
                priority
            }
            _ => 0,
        };
        if let Some(expr) = parser.conjunction() {
            out.push(Matcher { priority, expr });
        }
        if parser.peek() == Some(&Token::Comma) {
            parser.at += 1;
        } else {
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::parse;

    fn holds(selector: &str, scopes: &[&str]) -> Option<(i8, bool)> {
        parse(selector)
            .first()
            .map(|m| (m.priority, m.expr.matches(scopes)))
    }

    #[test]
    fn a_path_finds_its_names_in_order_and_by_prefix() {
        let stack = ["source.js", "meta.script.svelte", "meta.lang.js"];
        assert_eq!(holds("meta.script meta.lang", &stack), Some((0, true)));
        assert_eq!(holds("meta.lang meta.script", &stack), Some((0, false)));
        assert_eq!(holds("meta.scr", &stack), Some((0, false)));
    }

    #[test]
    fn groups_alternatives_and_exclusions() {
        let stack = ["text.html", "meta.script.svelte", "meta.lang.ts"];
        let selector = "L:(meta.script.svelte | meta.style.svelte) (meta.lang.js | meta.lang.ts) - (meta source)";
        assert_eq!(holds(selector, &stack), Some((-1, true)));
        let excluded = [
            "text.html",
            "meta.script.svelte",
            "meta.lang.ts",
            "meta.x",
            "source.ts",
        ];
        assert_eq!(holds(selector, &excluded), Some((-1, false)));
        let inside = [
            "meta.script.svelte",
            "meta.lang.ts",
            "meta.embedded",
            "source.ts",
        ];
        assert_eq!(
            holds("(meta.script.svelte) - (meta source)", &inside),
            Some((0, false))
        );
    }

    #[test]
    fn commas_at_the_top_level_make_separate_selectors() {
        let matchers = parse("R:text.html - comment, L:source.js");
        assert_eq!(matchers.len(), 2);
        assert_eq!(matchers[0].priority, 1);
        assert_eq!(matchers[1].priority, -1);
    }
}
