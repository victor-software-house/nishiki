//! Variables.
//!
//! Each variable pattern comes twice in the grammar.
//! The copy for strings takes no members after the name, such as `.Count`.
//! [`Members`] says which copy a matcher follows.

use super::super::engine::Lexer;
use super::is_name;
use crate::rules::Chars;

/// The variables named `variable.other.constant.powershell`, in the grammar's
/// order.
pub(super) const CONSTANTS: &[&[u8]] = &[
    b"Error",
    b"ExecutionContext",
    b"Host",
    b"Home",
    b"PID",
    b"PsHome",
    b"PsVersionTable",
    b"ShellID",
];

/// The automatic variables, in the grammar's order.
const AUTOMATIC: &[&[u8]] = &[
    b"_",
    b"Args",
    b"ConsoleFileName",
    b"Event",
    b"EventArgs",
    b"EventSubscriber",
    b"ForEach",
    b"Input",
    b"LastExitCode",
    b"Matches",
    b"MyInvocation",
    b"NestedPromptLevel",
    b"Profile",
    b"PSBoundParameters",
    b"PsCmdlet",
    b"PsCulture",
    b"PSDebugContext",
    b"PSItem",
    b"PSCommandPath",
    b"PSScriptRoot",
    b"PsUICulture",
    b"Pwd",
    b"Sender",
    b"SourceArgs",
    b"SourceEventArgs",
    b"StackTrace",
    b"Switch",
    b"This",
];

/// The preference variables, named `variable.language.powershell`, in the
/// grammar's order.
pub(super) const PREFERENCES: &[&[u8]] = &[
    b"ConfirmPreference",
    b"DebugPreference",
    b"ErrorActionPreference",
    b"ErrorView",
    b"FormatEnumerationLimit",
    b"InformationPreference",
    b"LogCommandHealthEvent",
    b"LogCommandLifecycleEvent",
    b"LogEngineHealthEvent",
    b"LogEngineLifecycleEvent",
    b"LogProviderHealthEvent",
    b"LogProviderLifecycleEvent",
    b"MaximumAliasCount",
    b"MaximumDriveCount",
    b"MaximumErrorCount",
    b"MaximumFunctionCount",
    b"MaximumHistoryCount",
    b"MaximumVariableCount",
    b"OFS",
    b"OutputEncoding",
    b"PSCulture",
    b"PSDebugContext",
    b"PSDefaultParameterValues",
    b"PSEmailServer",
    b"PSItem",
    b"PSModuleAutoLoadingPreference",
    b"PSModuleAutoloadingPreference",
    b"PSSenderInfo",
    b"PSSessionApplicationName",
    b"PSSessionConfigurationName",
    b"PSSessionOption",
    b"ProgressPreference",
    b"VerbosePreference",
    b"WarningPreference",
    b"WhatIfPreference",
];

/// The scopes a variable may name, in the grammar's order.
const SCOPES: &[&[u8]] = &[
    b"global",
    b"local",
    b"private",
    b"script",
    b"using",
    b"workflow",
];

/// Whether a variable pattern takes members after the name, such as `.Count`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Members {
    /// It does: `((?:\.[_\p{L}\d]+)*\b)?` follows the name.
    Kept,
    /// It does not, as in a string.
    Never,
}

/// Which sigils a variable pattern takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Sigils {
    /// `[$@]`: a variable or a splatted one.
    Both,
    /// `\$` only.
    Dollar,
}

impl Lexer<'_> {
    /// `(\$)(?i:(False|Null|True))\b`, group 1 around the `$`.
    pub(super) fn language_constant(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' {
            return None;
        }
        let end = self.first_folded(p + 1, &[b"False", b"Null", b"True"], |lexer, at| {
            lexer.word_at(at).is_none()
        })?;
        self.capture(1, p, p + 1);
        Some(end)
    }

    /// `((?:\.[_\p{L}\d]+)*\b)?` from `start`: where the members end, at the
    /// last place the chain reaches where `\b` holds.
    ///
    /// A name gives back no characters.
    /// `\b` cannot hold inside a name.
    /// The chain gives back whole members instead.
    fn members(&self, start: usize) -> Option<usize> {
        let mut found = self.boundary(start).then_some(start);
        let mut end = start;
        while self.at(end) == b'.' {
            let member = self.class_end(end + 1, is_name);
            if member == end + 1 {
                break;
            }
            end = member;
            if self.boundary(end) {
                found = Some(end);
            }
        }
        found
    }

    /// The end of a variable whose name ends at `name`, with its members when
    /// `members` keeps them, as group `group`.
    fn with_members(&mut self, name: usize, members: Members, group: usize) -> usize {
        match members {
            Members::Never => name,
            Members::Kept => match self.members(name) {
                Some(end) => {
                    self.capture(group, name, end);
                    end
                }
                None => name,
            },
        }
    }

    /// A variable of the list `words`, group 1 around the `$`:
    /// - `(\$)(?i:(Error|…))((?:\.[_\p{L}\d]+)*\b)?\b` when `members` keeps
    ///   them, group 3 around the members;
    /// - `(\$)(?i:(Error|…))\b` when it does not.
    ///
    /// Either way the name must end where `\b` holds, or at members that do.
    pub(super) fn listed_variable(
        &mut self,
        p: usize,
        words: &[&[u8]],
        members: Members,
    ) -> Option<usize> {
        if self.at(p) != b'$' {
            return None;
        }
        let (name, end) = words.iter().find_map(|word| {
            let name = self.folded(p + 1, word)?;
            match members {
                Members::Kept => self.members(name).map(|end| (name, end)),
                Members::Never => self.boundary(name).then_some((name, name)),
            }
        })?;
        self.capture(1, p, p + 1);
        self.capture(2, p + 1, name);
        if members == Members::Kept {
            self.capture(3, name, end);
        }
        Some(end)
    }

    /// `(\$)([$?^]|(?i:_|Args|…|This)\b)`, then `((?:\.[_\p{L}\d]+)*\b)?` when
    /// `members` keeps them: an automatic variable, group 1 around the `$`,
    /// group 2 around its name and group 3 around its members.
    pub(super) fn automatic(&mut self, p: usize, members: Members) -> Option<usize> {
        if self.at(p) != b'$' {
            return None;
        }
        let name = match self.at(p + 1) {
            b'$' | b'?' | b'^' => p + 2,
            _ => self.first_folded(p + 1, AUTOMATIC, Chars::boundary)?,
        };
        self.capture(1, p, p + 1);
        self.capture(2, p + 1, name);
        Some(self.with_members(name, members, 3))
    }

    /// `(?i:([$@])(global|…|workflow):([_\p{L}\d]+))`, with `\$` alone when
    /// `sigils` allows it, then `((?:\.[_\p{L}\d]+)*\b)?` when `members` keeps
    /// them: a variable with a scope.
    /// Group 1 is around the sigil, group 2 around the scope, group 3 around
    /// the name and group 4 around the members.
    pub(super) fn scoped(&mut self, p: usize, sigils: Sigils, members: Members) -> Option<usize> {
        if !self.sigil(p, sigils) {
            return None;
        }
        let scope = self.first_folded(p + 1, SCOPES, |lexer, at| lexer.at(at) == b':')?;
        let name = self.class_end(scope + 1, is_name);
        if name == scope + 1 {
            return None;
        }
        self.capture(1, p, p + 1);
        self.capture(2, p + 1, scope);
        self.capture(3, scope + 1, name);
        Some(self.with_members(name, members, 4))
    }

    /// Whether a sigil that `sigils` takes stands at `p`.
    fn sigil(&self, p: usize, sigils: Sigils) -> bool {
        matches!((self.at(p), sigils), (b'$', _) | (b'@', Sigils::Both))
    }

    /// Where the text of a braced variable, which starts at `from`, ends at
    /// the `}` that closes it: ``([^}]*[^`}])(})``.
    ///
    /// The `[^}]*` runs to the first `}`.
    /// It then gives back one character for ``[^`}]``, which a backtick fails.
    fn braced_text(&self, from: usize) -> Option<usize> {
        let close = from + memchr::memchr(b'}', self.s.get(from..)?)?;
        self.spend(close - from);
        (close > from && self.at(close - 1) != b'`').then_some(close)
    }

    /// ``(?i:(\$)(\{)(global|…|workflow):([^}]*[^`}])(}))``, then
    /// `((?:\.[_\p{L}\d]+)*\b)?` when `members` keeps them: a braced variable
    /// with a scope.
    /// Groups 1 to 5 are around the `$`, the `{`, the scope, the text and the
    /// `}`, and group 6 around the members.
    pub(super) fn braced_scoped(&mut self, p: usize, members: Members) -> Option<usize> {
        if !self.starts(p, b"${") {
            return None;
        }
        let scope = self.first_folded(p + 2, SCOPES, |lexer, at| lexer.at(at) == b':')?;
        let close = self.braced_text(scope + 1)?;
        self.capture(1, p, p + 1);
        self.capture(2, p + 1, p + 2);
        self.capture(3, p + 2, scope);
        self.capture(4, scope + 1, close);
        self.capture(5, close, close + 1);
        Some(self.with_members(close + 1, members, 6))
    }

    /// `(?i:([$@])([_\p{L}\d]+:)?([_\p{L}\d]+))`, with `\$` alone when
    /// `sigils` allows it, then `((?:\.[_\p{L}\d]+)*\b)?` when `members` keeps
    /// them: a variable.
    /// Group 1 is around the sigil, group 2 around its drive, group 3 around
    /// its name and group 4 around the members.
    ///
    /// A drive needs a name after it.
    /// Without one, the drive's letters are the name.
    pub(super) fn variable(&mut self, p: usize, sigils: Sigils, members: Members) -> Option<usize> {
        if !self.sigil(p, sigils) {
            return None;
        }
        let first = self.class_end(p + 1, is_name);
        if first == p + 1 {
            return None;
        }
        let drive = (self.at(first) == b':')
            .then(|| (first, self.class_end(first + 1, is_name)))
            .filter(|&(_, name)| name > first + 1);
        let (name_at, name) = match drive {
            Some((colon, name)) => {
                self.capture(2, p + 1, colon + 1);
                (colon + 1, name)
            }
            None => (p + 1, first),
        };
        self.capture(1, p, p + 1);
        self.capture(3, name_at, name);
        Some(self.with_members(name, members, 4))
    }

    /// ``(?i:(\$)(\{)([_\p{L}\d]+:)?([^}]*[^`}])(}))``, then
    /// `((?:\.[_\p{L}\d]+)*\b)?` when `members` keeps them: a braced
    /// variable.
    /// Groups 1 to 5 are around the `$`, the `{`, the drive, the text and the
    /// `}`, and group 6 around the members.
    ///
    /// A drive needs text after it.
    /// Without any, the drive's characters are the text.
    pub(super) fn braced(&mut self, p: usize, members: Members) -> Option<usize> {
        if !self.starts(p, b"${") {
            return None;
        }
        let first = self.class_end(p + 2, is_name);
        let drive = (first > p + 2 && self.at(first) == b':')
            .then(|| self.braced_text(first + 1).map(|close| (first + 1, close)))
            .flatten();
        let (text_at, close) = match drive {
            Some((text_at, close)) => {
                self.capture(3, p + 2, text_at);
                (text_at, close)
            }
            None => (p + 2, self.braced_text(p + 2)?),
        };
        self.capture(1, p, p + 1);
        self.capture(2, p + 1, p + 2);
        self.capture(4, text_at, close);
        self.capture(5, close, close + 1);
        Some(self.with_members(close + 1, members, 6))
    }
}
