//! Documentation comment tags: `@name`, `\name` and their arguments.

use super::super::engine::Lexer;
use super::chars::is_space;

/// Tags that take no argument: the first list of the grammar.
const NO_ARGUMENT: &[&[u8]] = &[
    b"callergraph",
    b"callgraph",
    b"else",
    b"endif",
    b"f$",
    b"f[",
    b"f]",
    b"hidecallergraph",
    b"hidecallgraph",
    b"hiderefby",
    b"hiderefs",
    b"hideinitializer",
    b"htmlinclude",
    b"n",
    b"nosubgrouping",
    b"private",
    b"privatesection",
    b"protected",
    b"protectedsection",
    b"public",
    b"publicsection",
    b"pure",
    b"showinitializer",
    b"showrefby",
    b"showrefs",
    b"tableofcontents",
    b"\"",
    b"#",
    b"$",
    b"%",
    b".",
    b"<",
    b"=",
    b">",
    b"::",
    b"|",
    b"--",
    b"---",
];

/// Tags whose argument is a word.
const WORD_ARGUMENT: &[&[u8]] = &[
    b"a",
    b"anchor",
    b"b",
    b"c",
    b"cite",
    b"copybrief",
    b"copydetail",
    b"copydoc",
    b"def",
    b"dir",
    b"dontinclude",
    b"e",
    b"em",
    b"emoji",
    b"enum",
    b"example",
    b"extends",
    b"file",
    b"idlexcept",
    b"implements",
    b"include",
    b"includedoc",
    b"includelineno",
    b"latexinclude",
    b"link",
    b"memberof",
    b"namespace",
    b"p",
    b"package",
    b"ref",
    b"refitem",
    b"related",
    b"relates",
    b"relatedalso",
    b"relatesalso",
    b"verbinclude",
];

/// Tags whose argument is a name of a declaration.
const NAME_ARGUMENT: &[&[u8]] = &[
    b"addindex",
    b"addtogroup",
    b"category",
    b"class",
    b"defgroup",
    b"diafile",
    b"dotfile",
    b"elseif",
    b"fn",
    b"headerfile",
    b"if",
    b"ifnot",
    b"image",
    b"ingroup",
    b"interface",
    b"line",
    b"mainpage",
    b"mscfile",
    b"name",
    b"overload",
    b"page",
    b"property",
    b"protocol",
    b"section",
    b"skip",
    b"skipline",
    b"snippet",
    b"snippetdoc",
    b"snippetlineno",
    b"struct",
    b"subpage",
    b"subsection",
    b"subsubsection",
    b"typedef",
    b"union",
    b"until",
    b"vhdlflow",
    b"weakgroup",
];

/// Tags that start a text.
const TEXT: &[&[u8]] = &[
    b"arg",
    b"attention",
    b"author",
    b"authors",
    b"brief",
    b"bug",
    b"copyright",
    b"date",
    b"deprecated",
    b"details",
    b"exception",
    b"invariant",
    b"li",
    b"note",
    b"par",
    b"paragraph",
    b"param",
    b"post",
    b"pre",
    b"remark",
    b"remarks",
    b"result",
    b"return",
    b"returns",
    b"retval",
    b"sa",
    b"see",
    b"short",
    b"since",
    b"test",
    b"throw",
    b"throws",
    b"todo",
    b"tparam",
    b"version",
    b"warning",
    b"xrefitem",
];

/// Tags that open or close a block.
const BLOCK: &[&[u8]] = &[
    b"code",
    b"cond",
    b"docbookonly",
    b"dot",
    b"htmlonly",
    b"internal",
    b"latexonly",
    b"link",
    b"manonly",
    b"msc",
    b"parblock",
    b"rtfonly",
    b"secreflist",
    b"startuml",
    b"verbatim",
    b"xmlonly",
    b"endcode",
    b"endcond",
    b"enddocbookonly",
    b"enddot",
    b"endhtmlonly",
    b"endinternal",
    b"endlatexonly",
    b"endlink",
    b"endmanonly",
    b"endmsc",
    b"endparblock",
    b"endrtfonly",
    b"endsecreflist",
    b"enduml",
    b"endverbatim",
    b"endxmlonly",
];

impl Lexer<'_> {
    /// `(?<=[!*/\s])[@\\](?:…)\b(?:\{[^}]*})?`: a tag from `names`, in the
    /// order the expression tries them, then a boundary and an optional
    /// braced argument.
    fn doc_name(&self, p: usize, names: &[&[u8]]) -> Option<usize> {
        let before_ok = self
            .ch_before(p)
            .is_some_and(|c| matches!(c, '!' | '*' | '/') || is_space(c));
        if !before_ok || !matches!(self.at(p), b'@' | b'\\') || p >= self.lim() {
            return None;
        }
        for name in names {
            let Some(end) = self.lit(p + 1, name) else {
                continue;
            };
            if !self.boundary(end) {
                continue;
            }
            if self.at(end) == b'{'
                && end < self.lim()
                && let Some(close) = self.s[end + 1..].iter().position(|&b| b == b'}')
            {
                return Some(end + 1 + close + 1);
            }
            return Some(end);
        }
        None
    }

    /// The first list of tags, those with no argument.
    pub(super) fn doc_plain(&self, p: usize) -> Option<usize> {
        self.doc_name(p, NO_ARGUMENT)
    }

    /// The tags that take a word.
    pub(super) fn doc_word(&self, p: usize) -> Option<usize> {
        self.doc_name(p, WORD_ARGUMENT)
    }

    /// The tags that take a name.
    pub(super) fn doc_decl(&self, p: usize) -> Option<usize> {
        self.doc_name(p, NAME_ARGUMENT)
    }

    /// The tags that start a text.
    pub(super) fn doc_text(&self, p: usize) -> Option<usize> {
        self.doc_name(p, TEXT)
    }

    /// The tags that bracket a block.
    pub(super) fn doc_block(&self, p: usize) -> Option<usize> {
        self.doc_name(p, BLOCK)
    }

    /// `((?<=[!*/\s])[@\\]param)(?:\s*\[((?:,?\s+{0,1}(?:in|out)\s+{0,1})+)])?
    /// (\s+((?<!\w)ID(?!\w))(?:(,)\s+{0,1}((?<!\w)ID(?!\w)))*)`.
    ///
    /// The optional direction, `[in]` or `[in, out]`.
    pub(super) fn doc_param(&mut self, p: usize) -> Option<usize> {
        let before_ok = self
            .ch_before(p)
            .is_some_and(|c| matches!(c, '!' | '*' | '/') || is_space(c));
        if !before_ok || !matches!(self.at(p), b'@' | b'\\') || p >= self.lim() {
            return None;
        }
        let tag_end = self.lit(p + 1, b"param")?;
        let mut direction = None;
        let open = self.skip_space(tag_end);
        if self.at(open) == b'[' && open < self.lim() {
            let mut at = open + 1;
            let mut count = 0;
            loop {
                let mut q = at;
                if self.at(q) == b',' && q < self.lim() {
                    q += 1;
                }
                q = self.skip_space(q);
                let Some(word) = self.lit_any(q, &[b"in", b"out"]) else {
                    break;
                };
                at = self.skip_space(word);
                count += 1;
            }
            if count > 0 && self.at(at) == b']' && at < self.lim() {
                direction = Some((open + 1, at, at + 1));
            }
        }
        let after = direction.map_or(tag_end, |(_, _, end)| end);
        let names = self.skip_space(after);
        if names == after {
            return None;
        }
        let first = self.word_ident(names)?;
        let mut end = first;
        let mut last_comma = None;
        let mut last_name = None;
        loop {
            if self.at(end) != b',' || end >= self.lim() {
                break;
            }
            let name = self.skip_space(end + 1);
            let Some(stop) = self.word_ident(name) else {
                break;
            };
            last_comma = Some(end);
            last_name = Some((name, stop));
            end = stop;
        }
        self.cap(1, p, tag_end);
        if let Some((a, b, _)) = direction {
            self.cap(2, a, b);
        }
        self.cap(3, after, end);
        self.cap(4, names, first);
        if let (Some(comma), Some((a, b))) = (last_comma, last_name) {
            self.cap(5, comma, comma + 1);
            self.cap(6, a, b);
        }
        Some(end)
    }
}
