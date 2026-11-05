// R for the Oniguruma harness (onig.ts): the texts it probes expressions
// with, besides the lines of the R fixtures.

/** For the Oniguruma harness. */
export const probe = {
  /** Texts inserted into fixture lines. */
  snippets: [
    "#'", "##'", "  #'", "#' ", "#'@", "#' @examples", "#' @examples  ", "#' @examplesIf", "#' @examplesIf x > 1",
    "#' @examplesIfx", "#' @param x,y", "#' @param", "@paramx", "@field a", "@slot b", "@@", "@@x", "@", "@x1",
    "```", "````r", "``", "``` ", "```{r}", "[f]", "[f()]", "[pkg::f]", "[pkg:::f()]", "[pkg::::f]", "[pkg:f]",
    "[f(]", "[]", "__b__", "__b__x", "__ b __", "___b___", "_i_", "_i_x", "__", "_\\_x_", "_a\\", "**b**", "**",
    "***", "*i*", "* i*", "*\\*x*", "*", "`c`", "`a\\`b`", "`\\", "<x>", "<>", "<x", "\\alpha", "\\", "\\1",
    "r\"(", ")\"", "r\"-(", ")-\"", "R'--[", "]--'", "r\"{", "}\"", "r'(", "r\"", "r\"x", "r\"--x", "'", "\"",
    "\\n", "\\\"", "\\'", "\\`", "\\a", "\\x", "\\x4", "\\x4G", "\\101", "\\1234", "\\٣", "\\u", "\\u00e9",
    "\\U{1F600}", "\\u{", "\\u{}", "\\U{12", "\\q", "\\é", "function", "function(", "function (", "function\t(",
    "functionx(", "if", "ifx", "xif", "else", "in", "inx", "break", "next", "repeat", "for", "while", "\\(x)",
    "\\x", "\\.", "return(", "return (", "tryCatch(", "try(", "stop(", "R6Class(", "setGroupGeneric(", "TRUE",
    "TRUE1", "NA", "NA_", "NA_real_", "NA_real_x", "NaN", "Inf", "NULL", "pkg::", "pkg:::", "pkg::::", "1pkg::",
    "a.b::", "_x::", "x.::", "f(", "f.g(", "f_g(", "é(", ".5(", "x", ".x", "..", "x1", "x_1", "é", "x́",
    "Ⅻ", "x‿", "xः", "x⃝", "x٣", "αβ", "1", "1L", "1i", "1.", ".5", "1e", "1e+", "1e-3", "1E5L",
    "٣", "0x", "0x1F", "0xFFp2", "0xFFp", "0xFFp-2L", "0x1i", "0X1", "%in%", "%", "%%", "%>%", "% %", ":", "::",
    ":::", "::::", ":=", "|>", "=>", ">=", "<=", "==", "!=", "<<-", "->", "->>", "<-", "||", "&&", "-", "+", "=",
    "*", "**", "***", "!", "$", "&", ",", "/", "<", ">", "?", "^", "|", "~", "#", "# x", "{", "}", "[", "]",
    "(", ")", "x=", "x =", "x ==", "x = =", "x\t=1", "x.y =", "\t", "  ", " ",
  ] as string[],
};
