//! The scope names the lexer emits, in Shiki's naming.
//!
//! Names the grammar builds from text of the line (`support.function.$1`) are
//! listed whole for the words the grammar can capture; the flags of a
//! recipe line, which are any run of `+`, `-` and `@`, are made at run time.

// The grammar spells the comma's scope with a misspelled word; the escape in
// it keeps the spelling checker from reading it as a typo.
crate::scope::scope_table! {
    MAKE;
    Source "source.makefile"
    Continuation "constant.character.escape.continuation.makefile"
    Comma "punctuation.separator.delim\x65ter.comma.makefile"
    LeadingSpace "punctuation.whitespace.comment.leading.makefile"
    PunctComment "punctuation.definition.comment.makefile"
    CommentLine "comment.line.number-sign.makefile"
    KwInclude "keyword.control.include.makefile"
    KwVpath "keyword.control.vpath.makefile"
    KwOverride "keyword.control.override.makefile"
    KwDefine "keyword.control.define.makefile"
    KwExport "keyword.control.export.makefile"
    KwPrivate "keyword.control.private.makefile"
    KwUnexport "keyword.control.unexport.makefile"
    KwUndefine "keyword.control.undefine.makefile"
    KwIfeq "keyword.control.ifeq.makefile"
    KwIfneq "keyword.control.ifneq.makefile"
    KwIfdef "keyword.control.ifdef.makefile"
    KwIfndef "keyword.control.ifndef.makefile"
    KwEndif "keyword.control.endif.makefile"
    KwElse "keyword.control.else.makefile"
    VarOther "variable.other.makefile"
    KeyValue "punctuation.separator.key-value.makefile"
    MetaConditional "meta.scope.conditional.makefile"
    MetaCondition "meta.scope.condition.makefile"
    Placeholder "constant.other.placeholder.makefile"
    MetaTarget "meta.scope.target.makefile"
    MetaPrereq "meta.scope.prerequisites.makefile"
    TargetName "entity.name.function.target.makefile"
    MetaRecipe "meta.scope.recipe.makefile"
    VarLanguage "variable.language.makefile"
    StringInterp "string.interpolated.makefile"
    PunctVar "punctuation.definition.variable.makefile"
    MetaFunc "meta.scope.function-call.makefile"
    TgPhony "support.function.target.PHONY.makefile"
    TgSuffixes "support.function.target.SUFFIXES.makefile"
    TgDefault "support.function.target.DEFAULT.makefile"
    TgPrecious "support.function.target.PRECIOUS.makefile"
    TgIntermediate "support.function.target.INTERMEDIATE.makefile"
    TgSecondary "support.function.target.SECONDARY.makefile"
    TgSecondExpansion "support.function.target.SECONDEXPANSION.makefile"
    TgDeleteOnError "support.function.target.DELETE_ON_ERROR.makefile"
    TgIgnore "support.function.target.IGNORE.makefile"
    TgLowResolution "support.function.target.LOW_RESOLUTION_TIME.makefile"
    TgSilent "support.function.target.SILENT.makefile"
    TgExportAll "support.function.target.EXPORT_ALL_VARIABLES.makefile"
    TgNotParallel "support.function.target.NOTPARALLEL.makefile"
    TgOneShell "support.function.target.ONESHELL.makefile"
    TgPosix "support.function.target.POSIX.makefile"
    FnSubst "support.function.subst.makefile"
    FnPatsubst "support.function.patsubst.makefile"
    FnStrip "support.function.strip.makefile"
    FnFindstring "support.function.findstring.makefile"
    FnFilter "support.function.filter.makefile"
    FnFilterOut "support.function.filter-out.makefile"
    FnSort "support.function.sort.makefile"
    FnWord "support.function.word.makefile"
    FnWordlist "support.function.wordlist.makefile"
    FnFirstword "support.function.firstword.makefile"
    FnLastword "support.function.lastword.makefile"
    FnDir "support.function.dir.makefile"
    FnNotdir "support.function.notdir.makefile"
    FnSuffix "support.function.suffix.makefile"
    FnBasename "support.function.basename.makefile"
    FnAddsuffix "support.function.addsuffix.makefile"
    FnAddprefix "support.function.addprefix.makefile"
    FnJoin "support.function.join.makefile"
    FnWildcard "support.function.wildcard.makefile"
    FnRealpath "support.function.realpath.makefile"
    FnAbspath "support.function.abspath.makefile"
    FnInfo "support.function.info.makefile"
    FnError "support.function.error.makefile"
    FnWarning "support.function.warning.makefile"
    FnShell "support.function.shell.makefile"
    FnForeach "support.function.foreach.makefile"
    FnIf "support.function.if.makefile"
    FnOr "support.function.or.makefile"
    FnAnd "support.function.and.makefile"
    FnCall "support.function.call.makefile"
    FnEval "support.function.eval.makefile"
    FnValue "support.function.value.makefile"
    FnFile "support.function.file.makefile"
    FnGuile "support.function.guile.makefile"
    FnOrigin "support.function.origin.makefile"
    FnFlavor "support.function.flavor.makefile"
}

/// The functions `$(name ...)` calls, with the scope of each name.
pub(crate) const FUNCTIONS: &[(&str, S)] = &[
    ("subst", S::FnSubst),
    ("patsubst", S::FnPatsubst),
    ("strip", S::FnStrip),
    ("findstring", S::FnFindstring),
    ("filter", S::FnFilter),
    ("filter-out", S::FnFilterOut),
    ("sort", S::FnSort),
    ("word", S::FnWord),
    ("wordlist", S::FnWordlist),
    ("firstword", S::FnFirstword),
    ("lastword", S::FnLastword),
    ("dir", S::FnDir),
    ("notdir", S::FnNotdir),
    ("suffix", S::FnSuffix),
    ("basename", S::FnBasename),
    ("addsuffix", S::FnAddsuffix),
    ("addprefix", S::FnAddprefix),
    ("join", S::FnJoin),
    ("wildcard", S::FnWildcard),
    ("realpath", S::FnRealpath),
    ("abspath", S::FnAbspath),
    ("info", S::FnInfo),
    ("error", S::FnError),
    ("warning", S::FnWarning),
    ("shell", S::FnShell),
    ("foreach", S::FnForeach),
    ("if", S::FnIf),
    ("or", S::FnOr),
    ("and", S::FnAnd),
    ("call", S::FnCall),
    ("eval", S::FnEval),
    ("value", S::FnValue),
    ("file", S::FnFile),
    ("guile", S::FnGuile),
];

/// The special targets, with the scope of each.
pub(crate) const SPECIAL_TARGETS: &[(&str, S)] = &[
    (".PHONY", S::TgPhony),
    (".SUFFIXES", S::TgSuffixes),
    (".DEFAULT", S::TgDefault),
    (".PRECIOUS", S::TgPrecious),
    (".INTERMEDIATE", S::TgIntermediate),
    (".SECONDARY", S::TgSecondary),
    (".SECONDEXPANSION", S::TgSecondExpansion),
    (".DELETE_ON_ERROR", S::TgDeleteOnError),
    (".IGNORE", S::TgIgnore),
    (".LOW_RESOLUTION_TIME", S::TgLowResolution),
    (".SILENT", S::TgSilent),
    (".EXPORT_ALL_VARIABLES", S::TgExportAll),
    (".NOTPARALLEL", S::TgNotParallel),
    (".ONESHELL", S::TgOneShell),
    (".POSIX", S::TgPosix),
];

/// The built-in variables `$(NAME)` reads.
pub(crate) const BUILTINS: &[&str] = &[
    "MAKEFILES",
    "VPATH",
    "SHELL",
    "MAKESHELL",
    "MAKE",
    "MAKELEVEL",
    "MAKEFLAGS",
    "MAKECMDGOALS",
    "CURDIR",
    "SUFFIXES",
    ".LIBPATTERNS",
];
