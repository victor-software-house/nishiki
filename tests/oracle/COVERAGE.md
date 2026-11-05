# Oracle coverage

How much of each grammar that a hand-written lexer follows the oracle fixtures exercise.
`mise run oracle:coverage` writes this file from [`oracle/coverage.ts`](../../oracle/coverage.ts); do not edit it.
The [oracle-coverage design](../../openspec/changes/oracle-coverage/design.md) defines the units.

- An expression is exercised when Shiki takes its match on a fixture.
- A capture is exercised when its group takes part in such a match and holds a non-whitespace byte.
- Fuzz only counts the units that only the fixed fuzz corpus exercises: 100 mutated windows per language, and 100 generated documents where `oracle/fuzz/<lang>.ts` has pieces, at seed 1.
  No CI job compares that corpus with nishiki.
- Waived counts the units that [`oracle/coverage-waivers.ts`](../../oracle/coverage-waivers.ts) waives, each for a reason given there.
- Complete reads `yes` for a grammar listed complete there, whose every unit a fixture or a waiver covers, which the coverage check holds; and `ready` for one that could be listed.

| Lexer | Grammar | Expressions | Share | Captures | Share | Waived | Fuzz only | Complete |
|:--|:--|--:|--:|--:|--:|--:|--:|:--|
| Batch | `bat` | 79 of 79 | 100.0% | 53 of 54 | 98.1% | 1 | 0 | yes |
| C | `c` | 321 of 342 | 93.9% | 318 of 341 | 93.3% | 44 | 0 | yes |
| CMake | `cmake` | 29 of 29 | 100.0% | 0 of 0 | - | 0 | 0 | yes |
| CoffeeScript | `coffee` | 136 of 143 | 95.1% | 132 of 134 | 98.5% | 9 | 0 | yes |
| Common Lisp | `common-lisp` | 60 of 62 | 96.8% | 51 of 56 | 91.1% | 7 | 0 | yes |
| C++ | `cpp` | 877 of 912 | 96.2% | 2480 of 2679 | 92.6% | 234 | 0 | yes |
| C++ | `cpp-macro` | 593 of 611 | 97.1% | 1532 of 1645 | 93.1% | 131 | 0 | yes |
| C# | `csharp` | 544 of 547 | 99.5% | 515 of 525 | 98.1% | 13 | 0 | yes |
| CSS | `css` | 219 of 222 | 98.6% | 181 of 184 | 98.4% | 6 | 0 | yes |
| Dart | `dart` | 92 of 93 | 98.9% | 18 of 18 | 100.0% | 1 | 0 | yes |
| diff | `diff` | 16 of 16 | 100.0% | 22 of 23 | 95.7% | 1 | 0 | yes |
| Dockerfile | `docker` | 8 of 9 | 88.9% | 6 of 13 | 46.2% | 8 | 0 | yes |
| Elixir | `elixir` | 130 of 130 | 100.0% | 80 of 84 | 95.2% | 4 | 0 | yes |
| ERB | `erb` | 12 of 12 | 100.0% | 10 of 14 | 71.4% | 4 | 0 | yes |
| GDScript | `gdscript` | 102 of 105 | 97.1% | 94 of 99 | 94.9% | 8 | 0 | yes |
| Go | `go` | 317 of 336 | 94.3% | 240 of 250 | 96.0% | 29 | 0 | yes |
| GraphQL | `graphql` | 84 of 88 | 95.5% | 90 of 113 | 79.6% | 27 | 0 | yes |
| Haskell | `haskell` | 191 of 192 | 99.5% | 280 of 287 | 97.6% | 8 | 0 | yes |
| HTML | `html` | 144 of 227 | 63.4% | 183 of 368 | 49.7% | 0 | 3 |  |
| HTML | `html-derivative` | 2 of 3 | 66.7% | 3 of 3 | 100.0% | 0 | 1 |  |
| INI | `ini` | 15 of 15 | 100.0% | 10 of 12 | 83.3% | 2 | 0 | yes |
| Java | `java` | 214 of 214 | 100.0% | 133 of 134 | 99.3% | 1 | 0 | yes |
| TypeScript | `javascript` | 532 of 534 | 99.6% | 600 of 605 | 99.2% | 7 | 0 | yes |
| JSON | `json` | 23 of 23 | 100.0% | 15 of 15 | 100.0% | 0 | 0 | yes |
| JSON | `jsonc` | 23 of 23 | 100.0% | 15 of 15 | 100.0% | 0 | 0 | yes |
| JSON | `jsonl` | 23 of 23 | 100.0% | 15 of 15 | 100.0% | 0 | 0 | yes |
| TypeScript | `jsx` | 532 of 534 | 99.6% | 600 of 605 | 99.2% | 7 | 0 | yes |
| Julia | `julia` | 105 of 106 | 99.1% | 95 of 95 | 100.0% | 1 | 0 | yes |
| Kotlin | `kotlin` | 63 of 65 | 96.9% | 35 of 35 | 100.0% | 2 | 0 | yes |
| Less | `less` | 581 of 590 | 98.5% | 417 of 437 | 95.4% | 29 | 0 | yes |
| Lua | `lua` | 147 of 152 | 96.7% | 72 of 74 | 97.3% | 7 | 0 | yes |
| Make | `make` | 75 of 75 | 100.0% | 28 of 32 | 87.5% | 4 | 0 | yes |
| Markdown | `markdown` | 229 of 311 | 73.6% | 290 of 373 | 77.7% | 165 | 0 | yes |
| Markdown | `markdown-vue` | 2 of 2 | 100.0% | 3 of 4 | 75.0% | 0 | 0 |  |
| MATLAB | `matlab` | 127 of 132 | 96.2% | 77 of 91 | 84.6% | 19 | 0 | yes |
| Mermaid | `mermaid` | 282 of 287 | 98.3% | 699 of 715 | 97.8% | 21 | 0 | yes |
| Nix | `nix` | 155 of 157 | 98.7% | 47 of 49 | 95.9% | 4 | 0 | yes |
| Objective-C | `objective-c` | 371 of 377 | 98.4% | 346 of 377 | 91.8% | 37 | 0 | yes |
| Objective-C++ | `objective-cpp` | 395 of 427 | 92.5% | 380 of 433 | 87.8% | 85 | 0 | yes |
| Pascal | `pascal` | 24 of 25 | 96.0% | 13 of 14 | 92.9% | 2 | 0 | yes |
| Perl | `perl` | 269 of 276 | 97.5% | 316 of 383 | 82.5% | 74 | 1 | yes |
| PHP | `php` | 476 of 476 | 100.0% | 411 of 433 | 94.9% | 22 | 0 | yes |
| PostCSS | `postcss` | 53 of 53 | 100.0% | 2 of 4 | 50.0% | 2 | 0 | yes |
| PowerShell | `powershell` | 96 of 96 | 100.0% | 130 of 136 | 95.6% | 6 | 0 | yes |
| Protobuf | `proto` | 40 of 40 | 100.0% | 58 of 58 | 100.0% | 0 | 0 | yes |
| Python | `python` | 349 of 350 | 99.7% | 311 of 372 | 83.6% | 62 | 0 | yes |
| R | `r` | 70 of 71 | 98.6% | 25 of 25 | 100.0% | 1 | 0 | yes |
| Ruby | `ruby` | 328 of 336 | 97.6% | 284 of 293 | 96.9% | 17 | 0 | yes |
| Rust | `rust` | 94 of 95 | 98.9% | 97 of 99 | 98.0% | 3 | 0 | yes |
| Sass | `sass` | 78 of 78 | 100.0% | 11 of 20 | 55.0% | 9 | 0 | yes |
| Scala | `scala` | 153 of 155 | 98.7% | 140 of 147 | 95.2% | 9 | 0 | yes |
| SCSS | `scss` | 161 of 168 | 95.8% | 136 of 179 | 76.0% | 50 | 0 | yes |
| shell | `shellscript` | 172 of 174 | 98.9% | 188 of 199 | 94.5% | 13 | 0 | yes |
| SQL | `sql` | 72 of 75 | 96.0% | 75 of 79 | 94.9% | 7 | 0 | yes |
| Svelte | `svelte` | 162 of 166 | 97.6% | 70 of 70 | 100.0% | 4 | 0 | yes |
| Swift | `swift` | 425 of 448 | 94.9% | 441 of 448 | 98.4% | 30 | 0 | yes |
| TOML | `toml` | 54 of 54 | 100.0% | 39 of 53 | 73.6% | 14 | 0 | yes |
| TypeScript | `tsx` | 532 of 534 | 99.6% | 600 of 605 | 99.2% | 7 | 0 | yes |
| TypeScript | `typescript` | 513 of 515 | 99.6% | 571 of 576 | 99.1% | 7 | 0 | yes |
| Visual Basic | `vb` | 35 of 36 | 97.2% | 15 of 20 | 75.0% | 6 | 0 | yes |
| Vue | `vue` | 130 of 197 | 66.0% | 88 of 128 | 68.8% | 0 | 0 |  |
| XML | `xml` | 32 of 32 | 100.0% | 55 of 66 | 83.3% | 11 | 0 | yes |
| YAML | `yaml` | 60 of 62 | 96.8% | 51 of 55 | 92.7% | 6 | 0 | yes |
| Zig | `zig` | 54 of 56 | 96.4% | 5 of 5 | 100.0% | 2 | 0 | yes |

## Units no fixture exercises

Each line is an expression's path in its grammar, then the start of its regex, `fuzz` when the fuzz corpus exercises it, and the reason when a waiver covers it.
An expression the fixtures do exercise is listed with the groups they do not, each marked `fuzz` and the waiver's reason likewise.
The groups of an unexercised expression count as unexercised and are not listed.

### `bat`

```text
#strings.patterns[0].end  groups 2 (waived: the group matches only the line feed at the end of the line, and the oracle compares no blank)
```

### `c`

```text
#anon_pattern_10.match  \b((?:int8|int16|int32|int64|uint8|uint16|uint32|uint64|int…  waived: every word of it is in `#storage_types.patterns[1]`, which matches it at the same place and comes earlier in the only list that includes it
#anon_pattern_8.match  \b(u_char|u_short|u_int|u_long|ushort|uint|u_quad_t|quad_t|…  waived: every word of it is in `#storage_types.patterns[1]`, which matches it at the same place and comes earlier in the only list that includes it
#anon_pattern_9.match  \b(pthread_(?:attr_|cond_|condattr_|mutex_|mutexattr_|once_…  waived: every word of it is in `#storage_types.patterns[1]`, which matches it at the same place and comes earlier in the only list that includes it
#anon_pattern_range_1.begin  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#anon_pattern_range_1.beginCaptures[4].patterns[0].match  \*/  waived: it scans only a gap that opens its owner's expression, and that gap never holds a comment
#anon_pattern_range_1.beginCaptures[4].patterns[1].match  \*  waived: it scans only a gap that opens its owner's expression, and that gap never holds a comment
#block_comment.patterns[1].begin  \s*+(/\*)  waived: it repeats `patterns[0]`, which comes first in the same list
#block_comment.patterns[1].end  \*/  waived: its begin never matches
#comments.patterns[0].patterns[0].patterns[6].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#inline_comment.patterns[0].patterns[1].match  (/\*)((?:[^*]|\*++[^/])*+(\*++/))  waived: it is `#inline_comment.patterns[0].patterns[0]` with possessive quantifiers in place of atomic groups, so it matches the same text, and that pattern comes first
#inline_comment.patterns[0].patterns[1].captures[3].patterns[0].match  \*/  waived: it scans a group of an expression that never matches
#inline_comment.patterns[0].patterns[1].captures[3].patterns[1].match  \*  waived: it scans a group of an expression that never matches
#inline_comment.patterns[1].match  (/\*)((?:[^*]|\*++[^/])*+(\*++/))  waived: it is `#inline_comment.patterns[0].patterns[0]` with possessive quantifiers in place of atomic groups, so it matches the same text, and that pattern comes first
#inline_comment.patterns[1].captures[3].patterns[0].match  \*/  waived: it scans a group of an expression that never matches
#inline_comment.patterns[1].captures[3].patterns[1].match  \*  waived: it scans a group of an expression that never matches
#line_comment.patterns[1].begin  \s*+(//)  waived: it repeats `patterns[0]`, which comes first in the same list
#line_comment.patterns[1].end  (?<=\n)(?<!\\\n)  waived: its begin never matches
#preprocessor-rule-enabled.patterns[0].begin  groups 3 (waived: the grammar names the group, but the expression has two groups)
#storage_types.patterns[3].patterns[2].begin  groups 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 5 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#storage_types.patterns[3].patterns[2].beginCaptures[5].patterns[0].match  \*/  waived: it scans only a gap that opens its owner's expression, and that gap never holds a comment
#storage_types.patterns[3].patterns[2].beginCaptures[5].patterns[1].match  \*  waived: it scans only a gap that opens its owner's expression, and that gap never holds a comment
#switch_statement.begin  groups 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 5 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#switch_statement.patterns[2].begin  (?<=}|%>|\?\?>)[\n\s]*  waived: the rule's own end, `(?<=}|%>|\?\?>)`, matches at the same place wherever this begin's lookbehind holds, and an end is tried before the patterns
#switch_statement.patterns[2].end  [\n\s]*(?=;)  waived: its begin never matches
#switch_statement.beginCaptures[5].patterns[0].match  \*/  waived: it scans only a gap that opens its owner's expression, and that gap never holds a comment
#switch_statement.beginCaptures[5].patterns[1].match  \*  waived: it scans only a gap that opens its owner's expression, and that gap never holds a comment
```

### `coffee`

```text
patterns[8].begin  (?<![$\w])(/)(?=(?![*+/?])(.+)(/)[gimuy]*(?!\s*[$(/\w]))  waived: the regular expression rule includes only `source.js.regexp`, which no grammar provides, so vscode-textmate skips it
patterns[8].end  (/)[gimuy]*(?!\s*[$(/\w])  waived: the regular expression rule includes only `source.js.regexp`, which no grammar provides, so vscode-textmate skips it
patterns[27].match  ((?<=console\.)(debug|warn|info|log|error|time|timeEnd|asse…  waived: the method must follow `console.`, `Array.`, `Object.`, `Math.` or `Number.`, and a property or call match takes that `.` first, since no match ends at it
patterns[29].match  ((?<=Array\.)(isArray))\b  waived: the method must follow `console.`, `Array.`, `Object.`, `Math.` or `Number.`, and a property or call match takes that `.` first, since no match ends at it
patterns[30].match  ((?<=Object\.)(create|definePropert(ies|y)|freeze|getOwnPro…  waived: the method must follow `console.`, `Array.`, `Object.`, `Math.` or `Number.`, and a property or call match takes that `.` first, since no match ends at it
patterns[31].match  ((?<=Math\.)(abs|acosh??|asinh??|atan2??|atanh|ceil|cosh??|…  waived: the method must follow `console.`, `Array.`, `Object.`, `Math.` or `Number.`, and a property or call match takes that `.` first, since no match ends at it
patterns[32].match  ((?<=Number\.)(is(Finite|Integer|NaN)|toInteger))\b  waived: the method must follow `console.`, `Array.`, `Object.`, `Math.` or `Number.`, and a property or call match takes that `.` first, since no match ends at it
```

### `common-lisp`

```text
#comment.begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#punctuation.patterns[10].match  (?i)(?<=^|[(\s]|,@|,\.?)(#)(:)(?=\S)  waived: `#package`, before it in the root, matches the same `#` before a `:` after the same lookbehind
#punctuation.patterns[11].match  groups 2 (waived: `#punctuation`'s first pattern, before this one, matches every backquote before a `\S`, after a lookbehind that holds wherever this one's does), 3 (waived: `#punctuation`'s first pattern, before this one, matches every backquote before a `\S`, after a lookbehind that holds wherever this one's does), 4 (waived: `#punctuation`'s first pattern, before this one, matches every backquote before a `\S`, after a lookbehind that holds wherever this one's does)
#variable.patterns[0].match  (?i)(?<=^|[(\s]|,@|,\.?)\*(?:trace-output|terminal-io|stand…  waived: `#style-guide`'s `(\*[^*\s]+\*)`, before it in the root, matches every `*name*` it does, after a lookbehind that holds wherever its own does
```

### `cpp`

```text
#access_control_keywords.match  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#access_control_keywords.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+  waived: it scans only a gap that opens its owner's expression, and that gap never holds a comment
#assembly.patterns[2].begin  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#class_block.begin  groups 16 (waived: the gap just before it, with nothing between them, takes every comment first), 17 (waived: the gap just before it, with nothing between them, takes every comment first), 18 (waived: the gap just before it, with nothing between them, takes every comment first), 19 (waived: the gap just before it, with nothing between them, takes every comment first)
#constructor_inline.patterns[2].begin  (?<=}|%>|\?\?>)\s*  waived: the rule's own end, `(?<=}|%>|\?\?>)`, matches at the same place wherever this begin's lookbehind holds, and an end is tried before the patterns
#constructor_inline.patterns[2].end  \s*(?=;)  waived: its begin never matches
#constructor_root.patterns[2].begin  (?<=}|%>|\?\?>)\s*  waived: the rule's own end, `(?<=}|%>|\?\?>)`, matches at the same place wherever this begin's lookbehind holds, and an end is tried before the patterns
#constructor_root.patterns[2].end  \s*(?=;)  waived: its begin never matches
#control_flow_keywords.match  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#control_flow_keywords.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+  waived: it scans only a gap that opens its owner's expression, and that gap never holds a comment
#destructor_inline.patterns[2].begin  (?<=}|%>|\?\?>)\s*  waived: the rule's own end, `(?<=}|%>|\?\?>)`, matches at the same place wherever this begin's lookbehind holds, and an end is tried before the patterns
#destructor_inline.patterns[2].end  \s*(?=;)  waived: its begin never matches
#destructor_root.begin  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#destructor_root.patterns[2].begin  (?<=}|%>|\?\?>)\s*  waived: the rule's own end, `(?<=}|%>|\?\?>)`, matches at the same place wherever this begin's lookbehind holds, and an end is tried before the patterns
#destructor_root.patterns[2].end  \s*(?=;)  waived: its begin never matches
#exception_keywords.match  (((?:\s*+/\*(?:[^*]++|\*+(?!/))*+\*/\s*+)+)|\s++|(?<=\W)|(?…  waived: `#control_flow_keywords` has the same gap, lists `throw`, `catch` and `try` too, and comes before it in every list that holds it
#exception_keywords.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+  waived: it scans only a gap that opens its owner's expression, and that gap never holds a comment
#extern_block.begin  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#function_definition.patterns[2].begin  (?<=}|%>|\?\?>)\s*  waived: the rule's own end, `(?<=}|%>|\?\?>)`, matches at the same place wherever this begin's lookbehind holds, and an end is tried before the patterns
#function_definition.patterns[2].end  \s*(?=;)  waived: its begin never matches
#function_pointer.end  groups 11 (waived: the lookahead before it allows only blanks up to `;`, `=`, `{` or the line end, so the gap never holds a comment), 12 (waived: the lookahead before it allows only blanks up to `;`, `=`, `{` or the line end, so the gap never holds a comment), 13 (waived: the lookahead before it allows only blanks up to `;`, `=`, `{` or the line end, so the gap never holds a comment), 14 (waived: the lookahead before it allows only blanks up to `;`, `=`, `{` or the line end, so the gap never holds a comment)
#function_pointer_parameter.end  groups 11 (waived: the lookahead before it allows only blanks up to `;`, `=`, `{` or the line end, so the gap never holds a comment), 12 (waived: the lookahead before it allows only blanks up to `;`, `=`, `{` or the line end, so the gap never holds a comment), 13 (waived: the lookahead before it allows only blanks up to `;`, `=`, `{` or the line end, so the gap never holds a comment), 14 (waived: the lookahead before it allows only blanks up to `;`, `=`, `{` or the line end, so the gap never holds a comment)
#goto_statement.match  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#goto_statement.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+  waived: it scans only a gap that opens its owner's expression, and that gap never holds a comment
#label.match  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#label.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+  waived: it scans only a gap that opens its owner's expression, and that gap never holds a comment
#lambdas.begin  groups 5 (waived: the group sits inside a negative lookahead, so it never holds text), 6 (waived: the group sits inside a negative lookahead, so it never holds text), 7 (waived: the group sits inside a negative lookahead, so it never holds text), 8 (waived: the group sits inside a negative lookahead, so it never holds text)
#member_access.match  groups 1 (waived: the gap opens the expression, and a comment rule comes before it in every list but the chain captures of `#member_access` and `#method_access`, whose text holds no comment), 2 (waived: the gap opens the expression, and a comment rule comes before it in every list but the chain captures of `#member_access` and `#method_access`, whose text holds no comment)
#member_access.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+  waived: it scans only a gap that opens its owner's expression, and that gap never holds a comment
#member_access.captures[11].patterns[0].match  groups 1 (waived: the chain group it scans holds only names, dots, arrows and blanks, never a comment), 2 (waived: the chain group it scans holds only names, dots, arrows and blanks, never a comment), 3 (waived: the chain group it scans holds only names, dots, arrows and blanks, never a comment), 4 (waived: the chain group it scans holds only names, dots, arrows and blanks, never a comment)
#member_access.captures[11].patterns[1].match  (?:((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<…  waived: `patterns[0]` is the same expression behind a lookbehind for `.` or `->`, which holds at the start of every element of the chain, so it starts at the same place and comes first
#memory_operators.match  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#memory_operators.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+  waived: it scans only a gap that opens its owner's expression, and that gap never holds a comment
#method_access.begin  groups 1 (waived: the gap opens the expression, and a comment rule comes before it in every list but the chain captures of `#member_access` and `#method_access`, whose text holds no comment), 2 (waived: the gap opens the expression, and a comment rule comes before it in every list but the chain captures of `#member_access` and `#method_access`, whose text holds no comment), 3 (waived: the gap opens the expression, and a comment rule comes before it in every list but the chain captures of `#member_access` and `#method_access`, whose text holds no comment), 4 (waived: the gap opens the expression, and a comment rule comes before it in every list but the chain captures of `#member_access` and `#method_access`, whose text holds no comment)
#method_access.beginCaptures[13].patterns[0].match  groups 1 (waived: the chain group it scans holds only names, dots, arrows and blanks, never a comment), 2 (waived: the chain group it scans holds only names, dots, arrows and blanks, never a comment), 3 (waived: the chain group it scans holds only names, dots, arrows and blanks, never a comment), 4 (waived: the chain group it scans holds only names, dots, arrows and blanks, never a comment)
#method_access.beginCaptures[13].patterns[1].match  (?:((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<…  waived: `patterns[0]` is the same expression behind a lookbehind for `.` or `->`, which holds at the start of every element of the chain, so it starts at the same place and comes first
#misc_keywords.match  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#misc_keywords.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+  waived: it scans only a gap that opens its owner's expression, and that gap never holds a comment
#namespace_block.patterns[2].begin  (?<=}|%>|\?\?>)\s*  waived: the rule's own end, `(?<=}|%>|\?\?>)`, matches at the same place wherever this begin's lookbehind holds, and an end is tried before the patterns
#namespace_block.patterns[2].end  \s*(?=;)  waived: its begin never matches
#operator_overload.begin  groups 41 (waived: the gap just before it, with nothing between them, takes every comment first), 42 (waived: the gap just before it, with nothing between them, takes every comment first), 43 (waived: the gap just before it, with nothing between them, takes every comment first), 44 (waived: the gap just before it, with nothing between them, takes every comment first)
#operator_overload.patterns[2].begin  (?<=}|%>|\?\?>)\s*  waived: the rule's own end, `(?<=}|%>|\?\?>)`, matches at the same place wherever this begin's lookbehind holds, and an end is tried before the patterns
#operator_overload.patterns[2].end  \s*(?=;)  waived: its begin never matches
#operators.patterns[12].match  groups 2 (waived: `patterns[8]` (`<<|>>`) and `patterns[11]` (`[&^|~]`) match the first characters of each of its alternatives at the same place, earlier in the same list)
#over_qualified_types.patterns[0].match  groups 8 (waived: the gap just before it, with nothing between them, takes every comment first), 9 (waived: the gap just before it, with nothing between them, takes every comment first)
#over_qualified_types.patterns[0].captures[9].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+  waived: it scans only group 9, a gap that the gap just before it leaves without a comment
#over_qualified_types.patterns[1].match  groups 8 (waived: the gap just before it, with nothing between them, takes every comment first), 9 (waived: the gap just before it, with nothing between them, takes every comment first)
#over_qualified_types.patterns[1].captures[9].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+  waived: it scans only group 9, a gap that the gap just before it leaves without a comment
#over_qualified_types.patterns[2].match  groups 8 (waived: the gap just before it, with nothing between them, takes every comment first), 9 (waived: the gap just before it, with nothing between them, takes every comment first)
#over_qualified_types.patterns[2].captures[9].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+  waived: it scans only group 9, a gap that the gap just before it leaves without a comment
#over_qualified_types.patterns[3].match  groups 8 (waived: the gap just before it, with nothing between them, takes every comment first), 9 (waived: the gap just before it, with nothing between them, takes every comment first)
#over_qualified_types.patterns[3].captures[9].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+  waived: it scans only group 9, a gap that the gap just before it leaves without a comment
#parameter.begin  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#parameter.patterns[5].match  groups 7 (waived: the gap just before it, with nothing between them, takes every comment first), 8 (waived: the gap just before it, with nothing between them, takes every comment first), 9 (waived: the gap just before it, with nothing between them, takes every comment first), 10 (waived: the gap just before it, with nothing between them, takes every comment first)
#parameter.patterns[11].match  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#parameter.patterns[16].match  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#parameter_or_maybe_value.begin  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#parameter_or_maybe_value.patterns[7].match  groups 7 (waived: the gap just before it, with nothing between them, takes every comment first), 8 (waived: the gap just before it, with nothing between them, takes every comment first), 9 (waived: the gap just before it, with nothing between them, takes every comment first), 10 (waived: the gap just before it, with nothing between them, takes every comment first)
#parameter_or_maybe_value.patterns[13].match  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#parameter_or_maybe_value.patterns[18].match  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#simple_constructor_call.captures[16].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+  waived: it scans group 16, which sits in a lookahead past the end of the match, and vscode-textmate applies no capture that starts after the match ends
#string_escaped_char.match  \\(["'?\\abefnprtv]|[0-3][0-7]{0,2}|[4-7]\d?|x\h{0,2}|u\h{0…  waived: it is the same expression as `#backslash_escapes`, which comes before it in its only list
#struct_block.begin  groups 16 (waived: the gap just before it, with nothing between them, takes every comment first), 17 (waived: the gap just before it, with nothing between them, takes every comment first), 18 (waived: the gap just before it, with nothing between them, takes every comment first), 19 (waived: the gap just before it, with nothing between them, takes every comment first)
#switch_statement.begin  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#switch_statement.patterns[2].begin  (?<=}|%>|\?\?>)\s*  waived: the rule's own end, `(?<=}|%>|\?\?>)`, matches at the same place wherever this begin's lookbehind holds, and an end is tried before the patterns
#switch_statement.patterns[2].end  \s*(?=;)  waived: its begin never matches
#type_casting_operators.match  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#type_casting_operators.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+  waived: it scans only a gap that opens its owner's expression, and that gap never holds a comment
#typedef_class.patterns[0].begin  groups 16 (waived: the gap just before it, with nothing between them, takes every comment first), 17 (waived: the gap just before it, with nothing between them, takes every comment first), 18 (waived: the gap just before it, with nothing between them, takes every comment first), 19 (waived: the gap just before it, with nothing between them, takes every comment first)
#typedef_struct.patterns[0].begin  groups 16 (waived: the gap just before it, with nothing between them, takes every comment first), 17 (waived: the gap just before it, with nothing between them, takes every comment first), 18 (waived: the gap just before it, with nothing between them, takes every comment first), 19 (waived: the gap just before it, with nothing between them, takes every comment first)
#typedef_union.patterns[0].begin  groups 16 (waived: the gap just before it, with nothing between them, takes every comment first), 17 (waived: the gap just before it, with nothing between them, takes every comment first), 18 (waived: the gap just before it, with nothing between them, takes every comment first), 19 (waived: the gap just before it, with nothing between them, takes every comment first)
#typename.match  groups 2 (waived: the gap opens the expression; a comment rule comes before it in every list but those of type captures, where names and scopes exclude `typename` and template arguments go to `#template_call_range`, which starts at the `<` and lists a comment rule first), 3 (waived: the gap opens the expression; a comment rule comes before it in every list but those of type captures, where names and scopes exclude `typename` and template arguments go to `#template_call_range`, which starts at the `<` and lists a comment rule first)
#typename.captures[3].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+  waived: it scans only a gap that opens its owner's expression, and that gap never holds a comment
#union_block.begin  groups 16 (waived: the gap just before it, with nothing between them, takes every comment first), 17 (waived: the gap just before it, with nothing between them, takes every comment first), 18 (waived: the gap just before it, with nothing between them, takes every comment first), 19 (waived: the gap just before it, with nothing between them, takes every comment first)
```

### `cpp-macro`

```text
#assembly.patterns[0].match  ^((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\…  waived: in a macro body a `\` or the macro's end follows `asm`, so the begin never reaches the line end, the next line starts without `\G`, and `(?!\G)` ends the rule before this `^` pattern
#assembly.patterns[2].begin  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#class_block.begin  groups 16 (waived: the gap just before it, with nothing between them, takes every comment first), 17 (waived: the gap just before it, with nothing between them, takes every comment first), 18 (waived: the gap just before it, with nothing between them, takes every comment first), 19 (waived: the gap just before it, with nothing between them, takes every comment first)
#constructor_inline.patterns[2].begin  (?<=}|%>|\?\?>)\s*  waived: the rule's own end, `(?<=}|%>|\?\?>)`, matches at the same place wherever this begin's lookbehind holds, and an end is tried before the patterns
#constructor_inline.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)  waived: its begin never matches
#constructor_root.patterns[2].begin  (?<=}|%>|\?\?>)\s*  waived: the rule's own end, `(?<=}|%>|\?\?>)`, matches at the same place wherever this begin's lookbehind holds, and an end is tried before the patterns
#constructor_root.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)  waived: its begin never matches
#destructor_inline.patterns[2].begin  (?<=}|%>|\?\?>)\s*  waived: the rule's own end, `(?<=}|%>|\?\?>)`, matches at the same place wherever this begin's lookbehind holds, and an end is tried before the patterns
#destructor_inline.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)  waived: its begin never matches
#destructor_root.begin  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#destructor_root.patterns[2].begin  (?<=}|%>|\?\?>)\s*  waived: the rule's own end, `(?<=}|%>|\?\?>)`, matches at the same place wherever this begin's lookbehind holds, and an end is tried before the patterns
#destructor_root.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)  waived: its begin never matches
#extern_block.begin  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#function_definition.patterns[2].begin  (?<=}|%>|\?\?>)\s*  waived: the rule's own end, `(?<=}|%>|\?\?>)`, matches at the same place wherever this begin's lookbehind holds, and an end is tried before the patterns
#function_definition.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)  waived: its begin never matches
#function_pointer.end  groups 11 (waived: the lookahead before it allows only blanks up to `;`, `=`, `{` or the line end, so the gap never holds a comment), 12 (waived: the lookahead before it allows only blanks up to `;`, `=`, `{` or the line end, so the gap never holds a comment), 13 (waived: the lookahead before it allows only blanks up to `;`, `=`, `{` or the line end, so the gap never holds a comment), 14 (waived: the lookahead before it allows only blanks up to `;`, `=`, `{` or the line end, so the gap never holds a comment)
#function_pointer_parameter.end  groups 11 (waived: the lookahead before it allows only blanks up to `;`, `=`, `{` or the line end, so the gap never holds a comment), 12 (waived: the lookahead before it allows only blanks up to `;`, `=`, `{` or the line end, so the gap never holds a comment), 13 (waived: the lookahead before it allows only blanks up to `;`, `=`, `{` or the line end, so the gap never holds a comment), 14 (waived: the lookahead before it allows only blanks up to `;`, `=`, `{` or the line end, so the gap never holds a comment)
#lambdas.begin  groups 5 (waived: the group sits inside a negative lookahead, so it never holds text), 6 (waived: the group sits inside a negative lookahead, so it never holds text), 7 (waived: the group sits inside a negative lookahead, so it never holds text), 8 (waived: the group sits inside a negative lookahead, so it never holds text)
#method_access.begin  groups 1 (waived: the gap opens the expression, and a comment rule comes before it in every list but the chain captures of `#member_access` and `#method_access`, whose text holds no comment), 2 (waived: the gap opens the expression, and a comment rule comes before it in every list but the chain captures of `#member_access` and `#method_access`, whose text holds no comment), 3 (waived: the gap opens the expression, and a comment rule comes before it in every list but the chain captures of `#member_access` and `#method_access`, whose text holds no comment), 4 (waived: the gap opens the expression, and a comment rule comes before it in every list but the chain captures of `#member_access` and `#method_access`, whose text holds no comment)
#method_access.beginCaptures[13].patterns[0].match  groups 1 (waived: the chain group it scans holds only names, dots, arrows and blanks, never a comment), 2 (waived: the chain group it scans holds only names, dots, arrows and blanks, never a comment), 3 (waived: the chain group it scans holds only names, dots, arrows and blanks, never a comment), 4 (waived: the chain group it scans holds only names, dots, arrows and blanks, never a comment)
#method_access.beginCaptures[13].patterns[1].match  (?:((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<…  waived: `patterns[0]` is the same expression behind a lookbehind for `.` or `->`, which holds at the start of every element of the chain, so it starts at the same place and comes first
#namespace_block.patterns[2].begin  (?<=}|%>|\?\?>)\s*  waived: the rule's own end, `(?<=}|%>|\?\?>)`, matches at the same place wherever this begin's lookbehind holds, and an end is tried before the patterns
#namespace_block.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)  waived: its begin never matches
#operator_overload.begin  groups 41 (waived: the gap just before it, with nothing between them, takes every comment first), 42 (waived: the gap just before it, with nothing between them, takes every comment first), 43 (waived: the gap just before it, with nothing between them, takes every comment first), 44 (waived: the gap just before it, with nothing between them, takes every comment first)
#operator_overload.patterns[2].begin  (?<=}|%>|\?\?>)\s*  waived: the rule's own end, `(?<=}|%>|\?\?>)`, matches at the same place wherever this begin's lookbehind holds, and an end is tried before the patterns
#operator_overload.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)  waived: its begin never matches
#operators.patterns[12].match  groups 2 (waived: `patterns[8]` (`<<|>>`) and `patterns[11]` (`[&^|~]`) match the first characters of each of its alternatives at the same place, earlier in the same list)
#parameter.begin  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#parameter.patterns[5].match  groups 7 (waived: the gap just before it, with nothing between them, takes every comment first), 8 (waived: the gap just before it, with nothing between them, takes every comment first), 9 (waived: the gap just before it, with nothing between them, takes every comment first), 10 (waived: the gap just before it, with nothing between them, takes every comment first)
#parameter.patterns[11].match  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#parameter.patterns[16].match  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#parameter_or_maybe_value.begin  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#parameter_or_maybe_value.patterns[7].match  groups 7 (waived: the gap just before it, with nothing between them, takes every comment first), 8 (waived: the gap just before it, with nothing between them, takes every comment first), 9 (waived: the gap just before it, with nothing between them, takes every comment first), 10 (waived: the gap just before it, with nothing between them, takes every comment first)
#parameter_or_maybe_value.patterns[13].match  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#parameter_or_maybe_value.patterns[18].match  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#struct_block.begin  groups 16 (waived: the gap just before it, with nothing between them, takes every comment first), 17 (waived: the gap just before it, with nothing between them, takes every comment first), 18 (waived: the gap just before it, with nothing between them, takes every comment first), 19 (waived: the gap just before it, with nothing between them, takes every comment first)
#switch_statement.begin  groups 1 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 2 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 3 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it), 4 (waived: the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it)
#switch_statement.patterns[2].begin  (?<=}|%>|\?\?>)\s*  waived: the rule's own end, `(?<=}|%>|\?\?>)`, matches at the same place wherever this begin's lookbehind holds, and an end is tried before the patterns
#switch_statement.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)  waived: its begin never matches
#typedef_class.patterns[0].begin  groups 16 (waived: the gap just before it, with nothing between them, takes every comment first), 17 (waived: the gap just before it, with nothing between them, takes every comment first), 18 (waived: the gap just before it, with nothing between them, takes every comment first), 19 (waived: the gap just before it, with nothing between them, takes every comment first)
#typedef_struct.patterns[0].begin  groups 16 (waived: the gap just before it, with nothing between them, takes every comment first), 17 (waived: the gap just before it, with nothing between them, takes every comment first), 18 (waived: the gap just before it, with nothing between them, takes every comment first), 19 (waived: the gap just before it, with nothing between them, takes every comment first)
#typedef_union.patterns[0].begin  groups 16 (waived: the gap just before it, with nothing between them, takes every comment first), 17 (waived: the gap just before it, with nothing between them, takes every comment first), 18 (waived: the gap just before it, with nothing between them, takes every comment first), 19 (waived: the gap just before it, with nothing between them, takes every comment first)
#union_block.begin  groups 16 (waived: the gap just before it, with nothing between them, takes every comment first), 17 (waived: the gap just before it, with nothing between them, takes every comment first), 18 (waived: the gap just before it, with nothing between them, takes every comment first), 19 (waived: the gap just before it, with nothing between them, takes every comment first)
```

### `csharp`

```text
#comment.patterns[0].begin  groups 1 (waived: the group matches only the blanks that lead the line, and the oracle compares no blank)
#comment.patterns[0].while  groups 1 (waived: the group matches only the blanks that lead the line, and the oracle compares no blank)
#comment.patterns[1].begin  groups 1 (waived: the group matches only the blanks that lead the line, and the oracle compares no blank)
#comment.patterns[1].end  groups 1 (waived: the group matches only the blanks that lead the line, and the oracle compares no blank)
#comment.patterns[1].patterns[0].while  groups 1 (waived: the group matches only the blanks that lead the line, and the oracle compares no blank)
#comment.patterns[2].begin  groups 1 (waived: the group matches only the blanks that lead the line, and the oracle compares no blank)
#comment.patterns[2].while  groups 1 (waived: the group matches only the blanks that lead the line, and the oracle compares no blank)
#numeric-literal.captures[0].patterns[0].patterns[1].match  groups 3 (waived: `[01_]`, the first alternative of the repetition, takes every `_`, so the separator group never takes part)
#preprocessor-line.patterns[0].match  groups 2 (waived: the regex `\\b(default|hidden)` has one group, so group 2 never exists)
#relational-pattern.begin  <=?|>=?  waived: `#constant-pattern`, earlier in `#pattern`, includes `#expression-operators`, whose `<=|>=|[<>]` and `\\.\\.` match first at every position this does
#relational-pattern.end  (?=[]\&),:;=?^|}]|!=|\b(and|or|when)\b)  waived: its begin never matches
#slice-pattern.match  \.\.  waived: `#constant-pattern`, earlier in `#pattern`, includes `#expression-operators`, whose `<=|>=|[<>]` and `\\.\\.` match first at every position this does
```

### `css`

```text
#at-rules.patterns[0].patterns[0].match  groups 2 (waived: the group matches only blanks, and the oracle compares no blank), 4 (waived: the group matches only blanks, and the oracle compares no blank)
#at-rules.patterns[0].patterns[2].patterns[0].begin  (?:\G|^)(?=[^"]+$)  waived: group 6 of `#at-rules.patterns[0].patterns[0]`, earlier in the `@charset` rule, takes an unclosed string from the same `"`, and a closed one fails the lookahead
#at-rules.patterns[0].patterns[2].patterns[0].end  $  waived: its begin never matches
#at-rules.patterns[3].end  groups 0 (waived: the expression is a lookahead, so the match holds no text)
#escapes.patterns[1].end  ^(?<!\G)  waived: the begin, `\\$\s*`, takes the line feed, so `\G` stands at the start of the next line and `^(?<!\G)` never holds
```

### `dart`

```text
#operators.patterns[3].match  (([\&^|]|<<|>>>?)=)  waived: `#operators.patterns[2]`, `(<<|>>>?|[\&^|~])`, before it, matches at the start of every one of its operators
```

### `diff`

```text
patterns[11].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
```

### `docker`

```text
patterns[2].match  ^\s*(?i:(ONBUILD)\s+)?(?i:(CMD|ENTRYPOINT))\s  waived: `patterns[1]`, before it, lists `CMD` and `ENTRYPOINT` after the same optional `ONBUILD`, so it matches wherever this does
patterns[4].begin  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[4].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[5].begin  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[5].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[6].match  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
```

### `elixir`

```text
patterns[7].begin  groups 1 (waived: the group is a lookahead, so it never holds text)
#core_syntax.patterns[21].match  groups 1 (waived: the grammar names the group, but the expression has no groups)
#core_syntax.patterns[47].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#core_syntax.patterns[48].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
```

### `erb`

```text
injections[text.html.erb - (meta.embedded.block.erb | meta.embedded.line.erb | comment)].patterns[0].begin  groups 0 (waived: the match is only the blanks that lead the line, before the lookahead, and the oracle compares no blank)
injections[text.html.erb - (meta.embedded.block.erb | meta.embedded.line.erb | comment)].patterns[0].end  groups 0 (waived: the match is only blanks and the line feed that end the line, and the oracle compares no blank)
injections[text.html.erb - (meta.embedded.block.erb | meta.embedded.line.erb | comment)].patterns[1].begin  groups 0 (waived: the match is only the blanks that lead the line, before the lookahead, and the oracle compares no blank)
injections[text.html.erb - (meta.embedded.block.erb | meta.embedded.line.erb | comment)].patterns[1].end  groups 0 (waived: the match is only blanks and the line feed that end the line, and the oracle compares no blank)
```

### `gdscript`

```text
#annotated_parameter.patterns[1].match  =(?!=)  waived: `#expression`, before it in its only list, holds `#assignment_operator`, whose `=` matches at every `=` this does
#any_method.match  \b([A-Z_a-z]\w*)\b(?=\s*\()  waived: `#function_call` has the same test, `(?=\b[A-Z_a-z]\w*\b\s*\()`, and `#base_expression` holds it before it in `#expression`, its only list
#builtin_get_node_shorthand_bare.match  groups 4 (waived: the grammar names the group, but the expression has three groups)
#class_declaration.match  groups 2 (waived: the grammar names the group, but the expression has one group)
#class_new.match  groups 3 (waived: the grammar names the group, but the expression has two groups)
#in_keyword.patterns[0].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
#lambda_declaration.begin  groups 2 (waived: the grammar names the group, but the expression has one group)
#statement_keyword.patterns[1].match  \b(?<!\.)(class)\b  waived: `#keywords`, before it in `#base_expression`, its only list, matches `class` with the same `\b` on each side
```

### `go`

```text
#generic_param_types.patterns[5].match  groups 3 (waived: the grammar names the group, but the expression has two groups)
#generic_param_types.patterns[5].captures[3].patterns[1].match  \w+  waived: it scans group 3, which the expression lacks
#map_types.patterns[3].match  \[  waived: `#parameter-variable-types`, before it in its only list, opens `([*.\w]+)?(\[)` at every `[`
#map_types.patterns[4].match  ]  waived: the rule's end, `(])`, matches at every `]` the rules inside it leave, and an end is tried before the patterns
#map_types.patterns[5].match  \{  waived: `#parameter-variable-types`, before it in its only list, has the same expression
#map_types.patterns[6].match  }  waived: `#parameter-variable-types`, before it in its only list, has the same expression
#map_types.patterns[7].match  \(  waived: `#parameter-variable-types`, before it in its only list, has the same expression
#numeric_literals.captures[0].patterns[0].patterns[0].match  groups 18 (waived: the alternative starts at a `.` before a digit, where `#delimiters`, before `#numeric_literals` in every list that holds it, matches `\.(?!\.\.)` first, and the ellipsis rule of `#operators`, also earlier, takes a `.` that two more follow), 19 (waived: the alternative starts at a `.` before a digit, where `#delimiters`, before `#numeric_literals` in every list that holds it, matches `\.(?!\.\.)` first, and the ellipsis rule of `#operators`, also earlier, takes a `.` that two more follow), 20 (waived: the alternative starts at a `.` before a digit, where `#delimiters`, before `#numeric_literals` in every list that holds it, matches `\.(?!\.\.)` first, and the ellipsis rule of `#operators`, also earlier, takes a `.` that two more follow), 21 (waived: the alternative starts at a `.` before a digit, where `#delimiters`, before `#numeric_literals` in every list that holds it, matches `\.(?!\.\.)` first, and the ellipsis rule of `#operators`, also earlier, takes a `.` that two more follow), 22 (waived: the alternative starts at a `.` before a digit, where `#delimiters`, before `#numeric_literals` in every list that holds it, matches `\.(?!\.\.)` first, and the ellipsis rule of `#operators`, also earlier, takes a `.` that two more follow), 23 (waived: the alternative starts at a `.` before a digit, where `#delimiters`, before `#numeric_literals` in every list that holds it, matches `\.(?!\.\.)` first, and the ellipsis rule of `#operators`, also earlier, takes a `.` that two more follow), 24 (waived: the alternative starts at a `.` before a digit, where `#delimiters`, before `#numeric_literals` in every list that holds it, matches `\.(?!\.\.)` first, and the ellipsis rule of `#operators`, also earlier, takes a `.` that two more follow), 25 (waived: the alternative starts at a `.` before a digit, where `#delimiters`, before `#numeric_literals` in every list that holds it, matches `\.(?!\.\.)` first, and the ellipsis rule of `#operators`, also earlier, takes a `.` that two more follow)
#numeric_literals.captures[0].patterns[0].patterns[0].captures[19].patterns[0].match  (?<=\h)_(?=\h)  waived: the alternative starts at a `.` before a digit, where `#delimiters`, before `#numeric_literals` in every list that holds it, matches `\.(?!\.\.)` first, and the ellipsis rule of `#operators`, also earlier, takes a `.` that two more follow
#numeric_literals.captures[0].patterns[0].patterns[0].captures[24].patterns[0].match  (?<=\h)_(?=\h)  waived: the alternative starts at a `.` before a digit, where `#delimiters`, before `#numeric_literals` in every list that holds it, matches `\.(?!\.\.)` first, and the ellipsis rule of `#operators`, also earlier, takes a `.` that two more follow
#single_type.patterns[1].patterns[3].match  \[  waived: the begin stops before the `[` its lookahead requires, where `#parameter-variable-types`, earlier in the list, opens `([*.\w]+)?(\[)` and runs to the matching `]`; right after it the end, `(?<=])` with an optional group, matches, and an end is tried before the patterns
#single_type.patterns[1].patterns[4].match  ]  waived: the begin stops before the `[` its lookahead requires, where `#parameter-variable-types`, earlier in the list, opens `([*.\w]+)?(\[)` and runs to the matching `]`; right after it the end, `(?<=])` with an optional group, matches, and an end is tried before the patterns
#single_type.patterns[1].patterns[5].match  \{  waived: the begin stops before the `[` its lookahead requires, where `#parameter-variable-types`, earlier in the list, opens `([*.\w]+)?(\[)` and runs to the matching `]`; right after it the end, `(?<=])` with an optional group, matches, and an end is tried before the patterns
#single_type.patterns[1].patterns[6].match  }  waived: the begin stops before the `[` its lookahead requires, where `#parameter-variable-types`, earlier in the list, opens `([*.\w]+)?(\[)` and runs to the matching `]`; right after it the end, `(?<=])` with an optional group, matches, and an end is tried before the patterns
#single_type.patterns[1].patterns[7].match  \(  waived: the begin stops before the `[` its lookahead requires, where `#parameter-variable-types`, earlier in the list, opens `([*.\w]+)?(\[)` and runs to the matching `]`; right after it the end, `(?<=])` with an optional group, matches, and an end is tried before the patterns
#single_type.patterns[1].patterns[8].match  \)  waived: the begin stops before the `[` its lookahead requires, where `#parameter-variable-types`, earlier in the list, opens `([*.\w]+)?(\[)` and runs to the matching `]`; right after it the end, `(?<=])` with an optional group, matches, and an end is tried before the patterns
#single_type.patterns[1].patterns[9].match  \w+  waived: the begin stops before the `[` its lookahead requires, where `#parameter-variable-types`, earlier in the list, opens `([*.\w]+)?(\[)` and runs to the matching `]`; right after it the end, `(?<=])` with an optional group, matches, and an end is tried before the patterns
#support_functions.captures[2].patterns[1].match  \d\w*  waived: `#type-declarations`, before it in the list, holds `#numeric_literals`, whose `(?<!\w)\.?\d` matches at every digit that starts the group
#switch_types.beginCaptures[3].patterns[2].match  \btype\b  waived: group 3 is `.(type)`, and `#brackets`, before it in the list, opens `\(` at its `(` and closes after `type`
#syntax_errors.patterns[0].match  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#type_assertion_inline.captures[2].patterns[1].match  \(  waived: group 2 holds only blanks, brackets, `*`, `.`, `<-`, braces, commas and words, never `(` or `)`
#type_assertion_inline.captures[2].patterns[2].match  \)  waived: group 2 holds only blanks, brackets, `*`, `.`, `<-`, braces, commas and words, never `(` or `)`
```

### `graphql`

```text
#graphql-comment.patterns[0].match  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#graphql-description-docstring.begin  """  waived: `#graphql-comment`, before it in every list that holds it, opens `(""")` at every `"""`
#graphql-description-docstring.end  """  waived: its begin never matches
#graphql-description-singleline.match  #(?=([^"]*"[^"]*")*[^"]*$).*$  waived: `#graphql-comment`, before it in every list that holds it, matches `(\s*)(#).*` at or before every `#`
#graphql-directive-definition.begin  groups 3 (waived: `beginCaptures` name the groups of the `on` rule inside it, and the grammar names the group, but the expression has two groups), 4 (waived: `beginCaptures` name the groups of the `on` rule inside it, and the grammar names the group, but the expression has two groups)
#graphql-fragment-definition.end  groups 1 (waived: the rule's `captures` name the begin's groups for its end too, and the grammar names the group, but the expression has no groups), 2 (waived: the rule's `captures` name the begin's groups for its end too, and the grammar names the group, but the expression has no groups), 3 (waived: the rule's `captures` name the begin's groups for its end too, and the grammar names the group, but the expression has no groups), 4 (waived: the rule's `captures` name the begin's groups for its end too, and the grammar names the group, but the expression has no groups)
#graphql-fragment-spread.end  groups 1 (waived: the rule's `captures` name the begin's groups for its end too, and the grammar names the group, but the expression has no groups), 2 (waived: the rule's `captures` name the begin's groups for its end too, and the grammar names the group, but the expression has no groups)
#graphql-inline-fragment.end  groups 1 (waived: the rule's `captures` name the begin's groups for its end too, and the grammar names the group, but the expression has no groups), 2 (waived: the rule's `captures` name the begin's groups for its end too, and the grammar names the group, but the expression has no groups), 3 (waived: the rule's `captures` name the begin's groups for its end too, and the grammar names the group, but the expression has no groups)
#graphql-input-types.patterns[2].begin  groups 2 (waived: the rule's `captures` name its end's `!` group for its begin too, and the grammar names the group, but the expression has one group)
#graphql-string-value.end  groups 3 (waived: the possessive `\s*+` before it takes the line feed, so `(\n)` never matches)
#graphql-type-interface.end  (?=.)  waived: the rule's last pattern, `#graphql-ignore-spaces`, matches `\s*` at every place, and the rule tries its end after its patterns
#graphql-union.end  groups 1 (waived: the rule's `captures` name the begin's groups for its end too, and the grammar names the group, but the expression has no groups), 2 (waived: the rule's `captures` name the begin's groups for its end too, and the grammar names the group, but the expression has no groups)
#graphql-union.patterns[0].end  groups 1 (waived: the rule's `captures` name the begin's groups for its end too, and the grammar names the group, but the expression has no groups), 2 (waived: the rule's `captures` name the begin's groups for its end too, and the grammar names the group, but the expression has no groups)
```

### `haskell`

```text
patterns[17].end  groups 1 (waived: the rule's `captures` name the begin's group for its end too, and the end's expression has no group)
#comments.patterns[1].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#comments.patterns[3].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#type_signature.patterns[4].match  (\()(#)\s*(#)(\))  waived: it repeats `patterns[2]`, which comes earlier in the same list
```

### `html`

```text
#attribute.patterns[1].patterns[0].patterns[0].patterns[2].begin  '
#attribute.patterns[1].patterns[0].patterns[0].patterns[2].end  (')
#attribute.patterns[1].patterns[0].patterns[1].match  =
#attribute.patterns[2].patterns[0].patterns[0].patterns[0].match  (([^"'/<=>`\s]|/(?!>))+)
#attribute.patterns[2].patterns[0].patterns[0].patterns[1].patterns[1].begin  //
#attribute.patterns[2].patterns[0].patterns[0].patterns[1].patterns[1].end  (?=")|\n
#attribute.patterns[2].patterns[0].patterns[0].patterns[1].patterns[2].begin  /\*
#attribute.patterns[2].patterns[0].patterns[0].patterns[1].patterns[2].end  (?=")|\*/
#attribute.patterns[2].patterns[0].patterns[0].patterns[2].begin  '
#attribute.patterns[2].patterns[0].patterns[0].patterns[2].end  (')
#attribute.patterns[2].patterns[0].patterns[0].patterns[2].patterns[0].match  ([^\n'/]|/(?![*/]))+
#attribute.patterns[2].patterns[0].patterns[0].patterns[2].patterns[1].begin  //
#attribute.patterns[2].patterns[0].patterns[0].patterns[2].patterns[1].end  (?=')|\n
#attribute.patterns[2].patterns[0].patterns[0].patterns[2].patterns[2].begin  /\*
#attribute.patterns[2].patterns[0].patterns[0].patterns[2].patterns[2].end  (?=')|\*/
#attribute.patterns[2].patterns[0].patterns[1].match  =
#cdata.begin  <!\[CDATA\[
#cdata.end  ]]>
#comment.patterns[0].match  \G-?>
#comment.patterns[1].match  <!-(?:-(?!>)|(?=-->))
#comment.patterns[2].match  --!>
#doctype.patterns[1].begin  "
#doctype.patterns[1].end  "
#entities.patterns[1].match  (&)#[0-9]+(;)
#entities.patterns[2].match  groups 3
#math.patterns[0].patterns[0].begin  (?<!>)\G
#math.patterns[0].patterns[0].end  >
#math.repository.#attribute.patterns[2].match  [^>\s]+  fuzz
#math.repository.#tags.patterns[2].match  (?i)(<)(annotation|annotation-xml|semantics|menclose|merror…
#math.repository.#tags.patterns[3].end  groups 4
#math.repository.#tags.patterns[3].patterns[0].begin  (?<!>)\G
#math.repository.#tags.patterns[3].patterns[0].end  (?=/>)|>
#math.repository.#tags.patterns[4].match  (?i)(<)(m(?:[inos]|space|text|aligngroup|alignmark))(?=\s|/…
#math.repository.#tags.patterns[5].end  groups 4
#math.repository.#tags.patterns[5].patterns[0].begin  (?<!>)\G
#math.repository.#tags.patterns[5].patterns[0].end  (?=/>)|>
#math.repository.#tags.patterns[6].match  (?i)(<)(mglyph)(?=\s|/?>)(([^"'>]|"[^"]*"|'[^']*')*)(/>)
#math.repository.#tags.patterns[7].begin  (?i)(<)(mglyph)(?=\s|/?>)(?:(([^"'>]|"[^"]*"|'[^']*')*)(>))?
#math.repository.#tags.patterns[7].end  (?i)(</)(\2)\s*(>)|(/>)|(?=</\w+)
#math.repository.#tags.patterns[7].patterns[0].begin  (?<!>)\G
#math.repository.#tags.patterns[7].patterns[0].end  (?=/>)|>
#math.repository.#tags.patterns[8].match  (?i)(<)(([:\w]+))(?=\s|/?>)(([^"'>]|"[^"]*"|'[^']*')*)(/>)
#math.repository.#tags.patterns[9].begin  (?i)(<)((\w[^>\s]*))(?=\s|/?>)(?:(([^"'>]|"[^"]*"|'[^']*')*…
#math.repository.#tags.patterns[9].end  (?i)(</)((\2))\s*(>)|(/>)|(?=</\w+)
#math.repository.#tags.patterns[9].patterns[0].begin  (?<!>)\G
#math.repository.#tags.patterns[9].patterns[0].end  (?=/>)|>
#svg.repository.#tags.patterns[2].match  (?i)(<)(color-profile|desc|metadata|script|style|title)(?=\…
#svg.repository.#tags.patterns[3].begin  (?i)(<)(color-profile|desc|metadata|script|style|title)(?=\…
#svg.repository.#tags.patterns[3].end  (?i)(</)(\2)\s*(>)|(/>)|(?=</\w+)
#svg.repository.#tags.patterns[3].patterns[0].begin  (?<!>)\G
#svg.repository.#tags.patterns[3].patterns[0].end  (?=/>)|>
#svg.repository.#tags.patterns[4].match  (?i)(<)(animateMotion|clipPath|defs|feComponentTransfer|feD…
#svg.repository.#tags.patterns[5].end  groups 4
#svg.repository.#tags.patterns[6].match  (?i)(<)(a|animate|discard|feBlend|feColorMatrix|feComposite…
#svg.repository.#tags.patterns[7].begin  (?i)(<)(a|animate|discard|feBlend|feColorMatrix|feComposite…
#svg.repository.#tags.patterns[7].end  (?i)(</)(\2)\s*(>)|(/>)|(?=</\w+)
#svg.repository.#tags.patterns[7].patterns[0].begin  (?<!>)\G
#svg.repository.#tags.patterns[7].patterns[0].end  (?=/>)|>
#svg.repository.#tags.patterns[9].begin  groups 3, 5
#svg.repository.#tags.patterns[9].end  groups 1, 2, 3
#svg.repository.#tags.patterns[9].patterns[0].end  groups 0 (fuzz)
#svg.repository.#tags.patterns[10].match  (?i)(<)((altGlyph|altGlyphDef|altGlyphItem|animateColor|ani…
#svg.repository.#tags.patterns[11].begin  (?i)(<)((altGlyph|altGlyphDef|altGlyphItem|animateColor|ani…
#svg.repository.#tags.patterns[11].end  (?i)(</)((\2))\s*(>)|(/>)|(?=</\w+)
#svg.repository.#tags.patterns[11].patterns[0].begin  (?<!>)\G
#svg.repository.#tags.patterns[11].patterns[0].end  (?=/>)|>
#svg.repository.#tags.patterns[12].match  (?i)(<)(([:\w]+))(?=\s|/?>)(([^"'>]|"[^"]*"|'[^']*')*)(/>)
#svg.repository.#tags.patterns[13].end  groups 5
#svg.repository.#tags.patterns[13].patterns[0].begin  (?<!>)\G
#svg.repository.#tags.patterns[13].patterns[0].end  (?=/>)|>
#tags-valid.patterns[0].begin  groups 1
#tags-valid.patterns[0].end  groups 1
#tags-valid.patterns[0].patterns[0].patterns[0].begin  groups 1
#tags-valid.patterns[1].begin  groups 1
#tags-valid.patterns[1].end  groups 1
#tags-valid.patterns[1].patterns[0].patterns[0].patterns[0].patterns[0].patterns[0].begin  groups 1
#tags-valid.patterns[1].patterns[0].patterns[0].patterns[0].patterns[0].patterns[1].begin  /\*
#tags-valid.patterns[1].patterns[0].patterns[0].patterns[0].patterns[0].patterns[1].end  \*/|(?=</script)
#tags-valid.patterns[14].begin  (?i)(<)((basefont|isindex))(?=\s|/?>)
#tags-valid.patterns[14].end  /?>
#tags-valid.patterns[15].begin  (?i)(<)((center|frameset|noembed|noframes))(?=\s|/?>)
#tags-valid.patterns[15].end  >
#tags-valid.patterns[16].begin  (?i)(</)((center|frameset|noembed|noframes))(?=\s|/?>)
#tags-valid.patterns[16].end  >
#tags-valid.patterns[17].begin  (?i)(<)((acronym|big|blink|font|strike|tt|xmp))(?=\s|/?>)
#tags-valid.patterns[17].end  >
#tags-valid.patterns[18].begin  (?i)(</)((acronym|big|blink|font|strike|tt|xmp))(?=\s|/?>)
#tags-valid.patterns[18].end  >
#tags-valid.patterns[19].begin  (?i)(<)((frame))(?=\s|/?>)
#tags-valid.patterns[19].end  /?>
#tags-valid.patterns[20].begin  (?i)(<)((applet))(?=\s|/?>)
#tags-valid.patterns[20].end  >
#tags-valid.patterns[21].begin  (?i)(</)((applet))(?=\s|/?>)
#tags-valid.patterns[21].end  >
#xml-processing.begin  (<\?)(xml)
#xml-processing.end  (\?>)
injections[R:text.html - (comment.block, text.html meta.embedded, meta.tag.*.*.html, meta.tag.*.*.*.html, meta.tag.*.*.*.*.html)].patterns[0].match  <  fuzz
```

### `html-derivative`

```text
injections[R:text.html - (comment.block, text.html meta.embedded, meta.tag.*.*.html, meta.tag.*.*.*.html, meta.tag.*.*.*.*.html)].patterns[0].match  <  fuzz
```

### `ini`

```text
patterns[0].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
patterns[1].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
```

### `java`

```text
#comments-inline.patterns[1].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
```

### `javascript`

```text
#comment.patterns[2].begin  groups 1 (waived: the group matches only the blanks that lead the line, and the oracle compares no blank)
#expression-operators.patterns[6].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))instanceof(?![$_[:…  waived: `#instanceof-expr` begins with the same expression and comes before `#expression-operators` in `#expressionWithoutIdentifiers`, the only rule that includes it
#expression-operators.patterns[7].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))new(?![$_[:alnum:]…  waived: `#new-expr` begins with the same expression and comes before `#expression-operators` in `#expressionWithoutIdentifiers`, the only rule that includes it
#single-line-comment-consuming-line-ending.begin  groups 1 (waived: the group matches only the blanks that lead the line, and the oracle compares no blank)
#support-objects.patterns[4].match  groups 4 (waived: the group sits inside the negative lookahead after the property names, so it never holds text; the constant names are group 53)
#template.patterns[1].begin  groups 1 (waived: `#template-call`, the pattern before it, starts at the same name wherever this group would match one)
#template-type.patterns[1].begin  groups 1 (waived: `#template-call`, the pattern before it, starts at the same name wherever this group would match one)
```

### `jsx`

```text
#comment.patterns[2].begin  groups 1 (waived: the group matches only the blanks that lead the line, and the oracle compares no blank)
#expression-operators.patterns[6].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))instanceof(?![$_[:…  waived: `#instanceof-expr` begins with the same expression and comes before `#expression-operators` in `#expressionWithoutIdentifiers`, the only rule that includes it
#expression-operators.patterns[7].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))new(?![$_[:alnum:]…  waived: `#new-expr` begins with the same expression and comes before `#expression-operators` in `#expressionWithoutIdentifiers`, the only rule that includes it
#single-line-comment-consuming-line-ending.begin  groups 1 (waived: the group matches only the blanks that lead the line, and the oracle compares no blank)
#support-objects.patterns[4].match  groups 4 (waived: the group sits inside the negative lookahead after the property names, so it never holds text; the constant names are group 53)
#template.patterns[1].begin  groups 1 (waived: `#template-call`, the pattern before it, starts at the same name wherever this group would match one)
#template-type.patterns[1].begin  groups 1 (waived: `#template-call`, the pattern before it, starts at the same name wherever this group would match one)
```

### `julia`

```text
#operator.patterns[12].match  ∘  waived: `#operator.patterns[11]`, before it, holds `∘` in its class `[…∗-∜…]`
```

### `kotlin`

```text
#operators.patterns[5].match  (--|\+\+)  waived: `#operators.patterns[3]`, `([-%*+/])`, before it, takes the first character of every `--` and `++`
#type-parameter.patterns[1].match  \b(in|out)\b  waived: `#type-parameter.patterns[0]`, `\b\w+\b`, before it, matches every word this does
```

### `less`

```text
#at-container.patterns[0].begin  groups 3 (waived: the grammar names the group, but the expression has two groups)
#at-container.patterns[0].patterns[0].patterns[1].patterns[5].match  /  waived: `(([<>])=?)|[/=]`, earlier in the same list, matches every `/`
#at-custom-media.patterns[0].match  \s*;  waived: the rule's end, `\s*(?=;)`, matches at the same position first
#at-custom-media.patterns[1].match  groups 3 (waived: the grammar names the group, but the expression has two groups)
#at-keyframes.begin  groups 4 (waived: the grammar names the group, but the expression has two groups)
#at-media.patterns[0].begin  groups 3 (waived: the grammar names the group, but the expression has two groups)
#at-supports.patterns[0].begin  groups 3 (waived: the grammar names the group, but the expression has two groups)
#less-namespace-accessors.patterns[0].patterns[0].match  groups 2 (waived: the grammar names the group, but the expression has one group)
#less-namespace-accessors.patterns[0].patterns[3].begin  \s*(\{)  waived: the rule's end, `\s*(?:(,)|(?=[;{]))`, matches at the same position first
#less-namespace-accessors.patterns[0].patterns[3].end  (?=})  waived: its begin never matches
#less-namespace-accessors.patterns[1].end  groups 2 (waived: the grammar names the group, but the expression has one group)
#less-strings.patterns[0].end  groups 2 (waived: the group matches only the line feed at the end of the line, and the oracle compares no blank)
#less-variable-assignment.patterns[0].patterns[0].match  groups 4 (waived: the group matches only blanks, and the oracle compares no blank)
#less-variable-comparison.patterns[0].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
#less-variable-comparison.patterns[0].patterns[5].match  ,  waived: `#unquoted-string`, earlier in the same list, matches every `,`
#literal-string.patterns[0].end  groups 2 (waived: the group matches only the line feed at the end of the line, and the oracle compares no blank)
#literal-string.patterns[1].end  groups 2 (waived: the group matches only the line feed at the end of the line, and the oracle compares no blank)
#media-query.patterns[2].patterns[1].match  \b(portrait|landscape|progressive|interlace)  waived: the media-feature name, the begin before it in the same list, matches every word this does
#rule-list-body.patterns[5].patterns[5].patterns[0].match  groups 4 (waived: the group matches only blanks, and the oracle compares no blank)
#rule-list-body.patterns[5].patterns[5].patterns[3].match  ,  waived: `#property-values`, earlier in the same list, takes every `,` through `#comma-delimiter`
#rule-list-body.patterns[5].patterns[9].patterns[0].match  groups 4 (waived: the group matches only blanks, and the oracle compares no blank)
#rule-list-body.patterns[5].patterns[10].patterns[0].match  groups 4 (waived: the group matches only blanks, and the oracle compares no blank)
#rule-list-body.patterns[5].patterns[11].patterns[0].match  groups 4 (waived: the group matches only blanks, and the oracle compares no blank)
#rule-list-body.patterns[5].patterns[11].patterns[3].match  unset|initial|inherit|auto  waived: the name pattern, earlier in the same list, matches every word this does
#rule-list-body.patterns[6].begin  groups 2 (waived: the group matches only blanks, and the oracle compares no blank)
#selectors.patterns[10].patterns[3].patterns[3].match  groups 1 (waived: the enclosing rule's end, `(?=([]\s]))`, matches at the blank before the flag first)
#selectors.patterns[10].patterns[3].patterns[4].match  ]  waived: the enclosing rule's end, `(?=([]\s]))`, matches at every `]` first
```

### `lua`

```text
patterns[3].match  (?<![.\w\d])0[Xx]\h+(?![.0-9EPep])  waived: `patterns[1]`, the hexadecimal float before it, matches every hexadecimal integer this does
patterns[6].match  (?<![.\w\d])\d+(?![.0-9EPep])  waived: `patterns[4]`, the decimal float before it, matches every integer this does
patterns[17].match  \b(function)\b(?![,:])  waived: `patterns[0]`, the function rule before it, begins with `(function)` under the same lookahead, so it matches wherever this does
#comment.patterns[0].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#comment.patterns[0].end  groups 1 (waived: the group holds only blanks and the line feed, and the oracle compares no blank)
#emmydoc.patterns[18].patterns[1].match  #  waived: the `@see` rule's end, `(?=[\n#@])`, matches at every `#` and is tried first
#emmydoc.type.patterns[4].patterns[1].match  (\.\.\.)[\t ]*  waived: `[-\]*,.0-9<>A-\[_a-z]+`, the pattern before it, takes every `.` of a `...`
```

### `make`

```text
#comment.begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#directives.patterns[2].end  groups 2 (waived: the grammar names the group, but the expression has one group), 3 (waived: the grammar names the group, but the expression has one group), 4 (waived: the grammar names the group, but the expression has one group)
```

### `markdown`

```text
#fenced_code_block_abap.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(abap)((\s+|[,:?{])[^`]*)?$)  waived: the fence's only pattern includes `source.abap`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_abap.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `source.abap`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_abap.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `source.abap`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_abap.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `source.abap`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_bibtex.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(bibtex)((\s+|[,:?{])[^`]*)?…  waived: the fence's only pattern includes `text.bibtex`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_bibtex.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `text.bibtex`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_bibtex.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `text.bibtex`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_bibtex.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `text.bibtex`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_clojure.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(cl(?:js??|ojure))((\s+|[,:?…  waived: the fence's only pattern includes `source.clojure`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_clojure.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `source.clojure`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_clojure.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `source.clojure`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_clojure.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `source.clojure`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_erlang.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(erlang)((\s+|[,:?{])[^`]*)?…  waived: the fence's only pattern includes `source.erlang`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_erlang.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `source.erlang`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_erlang.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `source.erlang`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_erlang.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `source.erlang`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_fsharp.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(f(?:s|sharp|#))((\s+|[,:?{]…  waived: the fence's only pattern includes `source.fsharp`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_fsharp.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `source.fsharp`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_fsharp.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `source.fsharp`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_fsharp.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `source.fsharp`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_git_commit.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:((?:COMMIT_EDIT|MERGE_)MSG)(…  waived: the fence's only pattern includes `text.git-commit`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_git_commit.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `text.git-commit`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_git_commit.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `text.git-commit`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_git_commit.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `text.git-commit`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_git_rebase.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(git-rebase-todo)((\s+|[,:?{…  waived: the fence's only pattern includes `text.git-rebase`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_git_rebase.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `text.git-rebase`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_git_rebase.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `text.git-rebase`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_git_rebase.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `text.git-rebase`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_groovy.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(g(?:roovy|vy))((\s+|[,:?{])…  waived: the fence's only pattern includes `source.groovy`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_groovy.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `source.groovy`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_groovy.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `source.groovy`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_groovy.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `source.groovy`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_handlebars.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(h(?:andlebars|bs))((\s+|[,:…  waived: the fence's only pattern includes `text.html.handlebars`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_handlebars.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `text.html.handlebars`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_handlebars.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `text.html.handlebars`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_handlebars.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `text.html.handlebars`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_ignore.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:((?:git|)ignore)((\s+|[,:?{]…  waived: the fence's only pattern includes `source.ignore`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_ignore.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `source.ignore`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_ignore.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `source.ignore`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_ignore.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `source.ignore`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_js_regexp.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(regexp)((\s+|[,:?{])[^`]*)?…  waived: the fence's only pattern includes `source.js.regexp`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_js_regexp.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `source.js.regexp`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_js_regexp.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `source.js.regexp`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_js_regexp.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `source.js.regexp`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_latex.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:((?:la|)tex)((\s+|[,:?{])[^`…  waived: the fence's only pattern includes `text.tex.latex`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_latex.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `text.tex.latex`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_latex.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `text.tex.latex`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_latex.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `text.tex.latex`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_log.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(log)((\s+|[,:?{])[^`]*)?$)  waived: the fence's only pattern includes `text.log`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_log.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `text.log`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_log.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `text.log`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_log.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `text.log`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_perl6.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(perl6|p6|pl6|pm6|nqp)((\s+|…  waived: the fence's only pattern includes `source.perl.6`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_perl6.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `source.perl.6`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_perl6.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `source.perl.6`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_perl6.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `source.perl.6`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_pug.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(jade|pug)((\s+|[,:?{])[^`]*…  waived: the fence's only pattern includes `text.pug`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_pug.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `text.pug`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_pug.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `text.pug`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_pug.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `text.pug`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_regexp_python.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(re)((\s+|[,:?{])[^`]*)?$)  waived: the fence's only pattern includes `source.regexp.python`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_regexp_python.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `source.regexp.python`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_regexp_python.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `source.regexp.python`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_regexp_python.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `source.regexp.python`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_restructuredtext.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(r(?:estructuredtext|st))((\…  waived: the fence's only pattern includes `source.rst`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_restructuredtext.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `source.rst`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_restructuredtext.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `source.rst`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_restructuredtext.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `source.rst`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_twig.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(twig)((\s+|[,:?{])[^`]*)?$)  waived: the fence's only pattern includes `source.twig`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_twig.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `source.twig`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_twig.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `source.twig`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_twig.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `source.twig`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_xsl.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(xslt??)((\s+|[,:?{])[^`]*)?…  waived: the fence's only pattern includes `text.xml.xsl`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_xsl.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `text.xml.xsl`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_xsl.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `text.xml.xsl`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_xsl.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `text.xml.xsl`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_yang.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(yang)((\s+|[,:?{])[^`]*)?$)  waived: the fence's only pattern includes `source.yang`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_yang.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `source.yang`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_yang.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `source.yang`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_yang.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `source.yang`, which the oracle does not load, so vscode-textmate drops the rule
#html.patterns[0].begin  groups 1 (waived: the group matches only the empty start of a line, so it never holds text)
#html.patterns[0].end  groups 2 (waived: the grammar names the group, but the expression has one group)
#link-email.match  groups 4 (waived: the grammar names the group, but the expression has three groups)
#list_paragraph.begin  (^|\G)(?=\S)(?![*->]\s|[0-9]+\.\s)  waived: `#paragraph`, which `#block` includes before it in both list rules, begins at every position this does
#list_paragraph.while  (^|\G)(?!\s*$|#| {0,3}([-*>_] {2,}){3,}[\t ]*$\n?| {0,3}[*-…  waived: its begin never matches
```

### `markdown-vue`

```text
#vue-code-block.begin  groups 5
```

### `matlab`

```text
#blocks.patterns[1].end  groups 2 (waived: the grammar names the group, but the expression has one group)
#blocks.patterns[1].patterns[0].end  ^  waived: the rule has a `match`, so vscode-textmate never reads its `end`
#blocks.patterns[1].patterns[1].end  ^  waived: the rule has a `match`, so vscode-textmate never reads its `end`
#blocks.patterns[4].patterns[0].end  ^  waived: the rule has a `match`, so vscode-textmate never reads its `end`
#blocks.patterns[4].patterns[1].end  ^  waived: the rule has a `match`, so vscode-textmate never reads its `end`
#blocks.patterns[5].patterns[0].end  ^  waived: the rule has a `match`, so vscode-textmate never reads its `end`
#comment_block.begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#comments.patterns[0].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#comments.patterns[2].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
```

### `mermaid`

```text
#mermaid.patterns[1].patterns[9].match  (?i)^\s*((?:|Enterprise_|System_)Boundary)\b  waived: the boundary's long form, the pattern before it, makes every part after the word optional, so it matches wherever this one does
#mermaid.patterns[1].patterns[15].match  (?i)^\s*(Component(?:|Db|Queue|_Ext|Db_Ext|Queue_Ext))\b(\(…  waived: it is the same expression as `patterns[14]`, the pattern before it, which matches first
#mermaid.patterns[2].patterns[11].match  (?i)^\s*((?:|Enterprise_|System_|Container_)Boundary)\b  waived: the boundary's long form, the pattern before it, makes every part after the word optional, so it matches wherever this one does
#mermaid.patterns[3].patterns[9].match  (?i)^\s*((?:|Enterprise_|System_)Boundary)\b  waived: the boundary's long form, the pattern before it, makes every part after the word optional, so it matches wherever this one does
#mermaid.patterns[4].patterns[11].match  (?i)^\s*((?:|Enterprise_|System_)Boundary)\b  waived: the boundary's long form, the pattern before it, makes every part after the word optional, so it matches wherever this one does
#mermaid.patterns[9].patterns[5].begin  groups 3 (waived: the grammar names the group, but the expression has two groups)
#mermaid.patterns[9].patterns[5].patterns[0].patterns[0].patterns[0].match  groups 1 (waived: the rule's begin `([^"]*)` takes the text up to the quote, where its end `(?=")` holds, or the rest of the line; Shiki lexes the next line outside the rule, so this match only matches empty)
#mermaid.patterns[9].patterns[6].patterns[0].patterns[0].patterns[0].match  groups 1 (waived: the rule's begin `([^"]*)` takes the text up to the quote, where its end `(?=")` holds, or the rest of the line; Shiki lexes the next line outside the rule, so this match only matches empty)
```

### `nix`

```text
#attrset-or-function.patterns[3].patterns[3].patterns[1].begin  ,  waived: the enclosing rule's end, `(?=([]),;}]|\b(else|then)\b))`, matches before every `,`, and an end is tried before the patterns
#attrset-or-function.patterns[3].patterns[3].patterns[1].end  (?=([]),;}]|\b(else|then)\b))  waived: the enclosing rule's end, `(?=([]),;}]|\b(else|then)\b))`, matches before every `,`, and an end is tried before the patterns
#if.patterns[2].end  groups 0 (waived: the group is a lookahead, so it never holds text)
```

### `objective-c`

```text
#anonymous_pattern_1.begin  groups 8 (waived: the group matches only blanks, and the oracle compares no blank), 9 (waived: the grammar names the group, but the expression has eight groups)
#anonymous_pattern_1.end  groups 4 (waived: the rule's `captures` name the begin's groups for its end too, and the end's expression has two groups), 6 (waived: the rule's `captures` name the begin's groups for its end too, and the end's expression has two groups), 7 (waived: the rule's `captures` name the begin's groups for its end too, and the end's expression has two groups), 8 (waived: the rule's `captures` name the begin's groups for its end too, and the end's expression has two groups), 9 (waived: the rule's `captures` name the begin's groups for its end too, and the end's expression has two groups)
#anonymous_pattern_14.patterns[0].match  groups 1 (waived: the grammar names the group, but the expression has no groups)
#anonymous_pattern_18.match  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#anonymous_pattern_19.match  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#anonymous_pattern_2.end  groups 4 (waived: the rule's `captures` name the begin's groups for its end too, and the end's expression has two groups), 5 (waived: the rule's `captures` name the begin's groups for its end too, and the end's expression has two groups)
#bracketed_content.patterns[0].begin  groups 2 (waived: the grammar names the group, but the expression has one group)
#c_functions.patterns[0].match  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#c_functions.patterns[1].match  groups 1 (waived: the group matches only blanks, and the oracle compares no blank), 3 (waived: the group sits inside a negative lookahead, so it never holds text)
#c_lang.patterns[25].match  \b(u_char|u_short|u_int|u_long|ushort|uint|u_quad_t|quad_t|…  waived: `#storage_types`, earlier in the same list, takes every word of this alternation first
#c_lang.patterns[26].match  \b(pthread_(?:attr_|cond_|condattr_|mutex_|mutexattr_|once_…  waived: `#storage_types`, earlier in the same list, takes every word of this alternation first
#c_lang.patterns[27].match  \b((?:int8|int16|int32|int64|uint8|uint16|uint32|uint64|int…  waived: `#storage_types`, earlier in the same list, takes every word of this alternation first
#c_lang.patterns[36].match  \[\s*]  waived: the root tries `#bracketed_content`, whose begin is `\[`, before `#c_lang`, so it takes every `[` first
#c_lang.repository.#comments.patterns[3].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#c_lang.repository.#preprocessor-rule-enabled.patterns[0].begin  groups 3 (waived: the grammar names the group, but the expression has two groups)
#c_lang.repository.#switch_statement.patterns[2].begin  (?<=})[\n\s]*  waived: the switch's end `(?<=})` holds where this begin could, and a rule's end comes before its patterns
#c_lang.repository.#switch_statement.patterns[2].end  [\n\s]*(?=;)  waived: the switch's end `(?<=})` holds where this begin could, and a rule's end comes before its patterns
#comment.patterns[1].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#preprocessor-rule-disabled-implementation.end  groups 3 (waived: the grammar names the group, but the expression has two groups)
#preprocessor-rule-disabled-implementation.patterns[0].end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups)
#preprocessor-rule-disabled-interface.end  groups 3 (waived: the grammar names the group, but the expression has two groups)
#preprocessor-rule-disabled-interface.patterns[0].end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups)
#preprocessor-rule-enabled-implementation.end  groups 3 (waived: the grammar names the group, but the expression has two groups)
#preprocessor-rule-enabled-implementation.patterns[0].end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups)
#preprocessor-rule-enabled-interface.end  groups 3 (waived: the grammar names the group, but the expression has two groups)
#preprocessor-rule-enabled-interface.patterns[0].end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups)
```

### `objective-cpp`

```text
#anonymous_pattern_1.begin  groups 8 (waived: the group matches only blanks, and the oracle compares no blank), 9 (waived: the grammar names the group, but the expression has eight groups)
#anonymous_pattern_1.end  groups 4 (waived: the rule's `captures` name the begin's groups for its end too, and the end's expression has two groups), 6 (waived: the rule's `captures` name the begin's groups for its end too, and the end's expression has two groups), 7 (waived: the rule's `captures` name the begin's groups for its end too, and the end's expression has two groups), 8 (waived: the rule's `captures` name the begin's groups for its end too, and the end's expression has two groups), 9 (waived: the rule's `captures` name the begin's groups for its end too, and the end's expression has two groups)
#anonymous_pattern_14.patterns[0].match  groups 1 (waived: the grammar names the group, but the expression has no groups)
#anonymous_pattern_18.match  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#anonymous_pattern_19.match  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#anonymous_pattern_2.end  groups 4 (waived: the rule's `captures` name the begin's groups for its end too, and the end's expression has two groups), 5 (waived: the rule's `captures` name the begin's groups for its end too, and the end's expression has two groups)
#apple_foundation_functional_macros.begin  \b(API_AVAILABLE|API_DEPRECATED|API_UNAVAILABLE|NS_AVAILABL…  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its function rule `patterns[33]` takes a name before `(`, as in `sqrt(4)` and `API_AVAILABLE(ios(13.0))`
#apple_foundation_functional_macros.end  \)  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its function rule `patterns[33]` takes a name before `(`, as in `sqrt(4)` and `API_AVAILABLE(ios(13.0))`
#bracketed_content.begin  \[  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its `patterns[35]` and `patterns[36]` take every `[` before `#bracketed_content`
#bracketed_content.end  ]  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its `patterns[35]` and `patterns[36]` take every `[` before `#bracketed_content`
#bracketed_content.patterns[0].begin  (?=predicateWithFormat:)(?<=NSPredicate )(predicateWithForm…  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its `patterns[35]` and `patterns[36]` take every `[` before `#bracketed_content`
#bracketed_content.patterns[0].end  (?=])  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its `patterns[35]` and `patterns[36]` take every `[` before `#bracketed_content`
#bracketed_content.patterns[0].patterns[0].match  \bargument(Array|s)(:)  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its `patterns[35]` and `patterns[36]` take every `[` before `#bracketed_content`
#bracketed_content.patterns[0].patterns[1].match  \b\w+(:)  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its `patterns[35]` and `patterns[36]` take every `[` before `#bracketed_content`
#bracketed_content.patterns[0].patterns[2].begin  @"  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its `patterns[35]` and `patterns[36]` take every `[` before `#bracketed_content`
#bracketed_content.patterns[0].patterns[2].end  "  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its `patterns[35]` and `patterns[36]` take every `[` before `#bracketed_content`
#bracketed_content.patterns[0].patterns[2].patterns[0].match  \b(AND|OR|NOT|IN)\b  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its `patterns[35]` and `patterns[36]` take every `[` before `#bracketed_content`
#bracketed_content.patterns[0].patterns[2].patterns[1].match  \b(ALL|ANY|SOME|NONE)\b  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its `patterns[35]` and `patterns[36]` take every `[` before `#bracketed_content`
#bracketed_content.patterns[0].patterns[2].patterns[2].match  \b(NULL|NIL|SELF|TRUE|YES|FALSE|NO|FIRST|LAST|SIZE)\b  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its `patterns[35]` and `patterns[36]` take every `[` before `#bracketed_content`
#bracketed_content.patterns[0].patterns[2].patterns[3].match  \b(MATCHES|CONTAINS|BEGINSWITH|ENDSWITH|BETWEEN)\b  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its `patterns[35]` and `patterns[36]` take every `[` before `#bracketed_content`
#bracketed_content.patterns[0].patterns[2].patterns[4].match  \bC(ASEINSENSITIVE|I)\b  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its `patterns[35]` and `patterns[36]` take every `[` before `#bracketed_content`
#bracketed_content.patterns[0].patterns[2].patterns[5].match  \b(ANYKEY|SUBQUERY|CAST|TRUEPREDICATE|FALSEPREDICATE)\b  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its `patterns[35]` and `patterns[36]` take every `[` before `#bracketed_content`
#bracketed_content.patterns[0].patterns[2].patterns[6].match  \\(["'?\\abefnrtv]|[0-3]\d{0,2}|[4-7]\d?|x[0-9A-Za-z]+)  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its `patterns[35]` and `patterns[36]` take every `[` before `#bracketed_content`
#bracketed_content.patterns[0].patterns[2].patterns[7].match  \\.  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its `patterns[35]` and `patterns[36]` take every `[` before `#bracketed_content`
#bracketed_content.patterns[1].begin  (?=\w)(?<=[]")\w] )(\w+(?:(:)|(?=])))  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its `patterns[35]` and `patterns[36]` take every `[` before `#bracketed_content`
#bracketed_content.patterns[1].end  (?=])  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its `patterns[35]` and `patterns[36]` take every `[` before `#bracketed_content`
#bracketed_content.patterns[1].patterns[0].match  \b\w+(:)  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its `patterns[35]` and `patterns[36]` take every `[` before `#bracketed_content`
#c_functions.patterns[0].match  (\s*)\b(hypot([fl])?|s(scanf|ystem|nprintf|ca(nf|lb(n([fl])…  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its function rule `patterns[33]` takes a name before `(`, as in `sqrt(4)` and `API_AVAILABLE(ios(13.0))`
#c_functions.patterns[1].match  (?:(?=\s)(?:(?<=else|new|return)|(?<!\w))(\s+))?\b((?!(whil…  waived: the root reaches `#c_lang` first, through `#cpp_lang`, and its function rule `patterns[33]` takes a name before `(`, as in `sqrt(4)` and `API_AVAILABLE(ios(13.0))`
#c_lang.patterns[25].match  \b(u_char|u_short|u_int|u_long|ushort|uint|u_quad_t|quad_t|…  waived: `#storage_types`, earlier in the same list, takes every word of this alternation first
#c_lang.patterns[26].match  \b(pthread_(?:attr_|cond_|condattr_|mutex_|mutexattr_|once_…  waived: `#storage_types`, earlier in the same list, takes every word of this alternation first
#c_lang.patterns[27].match  \b((?:int8|int16|int32|int64|uint8|uint16|uint32|uint64|int…  waived: `#storage_types`, earlier in the same list, takes every word of this alternation first
#c_lang.repository.#comments.patterns[3].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#c_lang.repository.#preprocessor-rule-enabled.patterns[0].begin  groups 3 (waived: the grammar names the group, but the expression has two groups)
#c_lang.repository.#switch_statement.patterns[2].begin  (?<=})[\n\s]*  waived: the switch's end `(?<=})` holds where this begin could, and a rule's end comes before its patterns
#c_lang.repository.#switch_statement.patterns[2].end  [\n\s]*(?=;)  waived: the switch's end `(?<=})` holds where this begin could, and a rule's end comes before its patterns
#comment.patterns[1].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#cpp_lang.patterns[17].begin  (?:^|(?<!else|new|=))((?:[A-Z_a-z][0-9A-Z_a-z]*::)*+~[A-Z_a…  waived: it repeats `patterns[16]`, which comes earlier in the same list
#cpp_lang.patterns[17].end  \)  waived: it repeats `patterns[16]`, which comes earlier in the same list
#cpp_lang.repository.#constructor.patterns[0].begin  ^\s*((?!while|for|do|if|else|switch|catch|enumerate|r?itera…  waived: vscode-textmate drops the rule: its only patterns include `probably_a_parameter` and `function-innards`, which no repository in its reach defines
#cpp_lang.repository.#constructor.patterns[0].end  \)  waived: vscode-textmate drops the rule: its only patterns include `probably_a_parameter` and `function-innards`, which no repository in its reach defines
#cpp_lang.repository.#special_block.patterns[2].patterns[1].end  groups 2 (waived: the group matches only blanks and the line feed that ends the line, and the oracle compares no blank)
#preprocessor-rule-disabled-implementation.end  groups 3 (waived: the grammar names the group, but the expression has two groups)
#preprocessor-rule-disabled-implementation.patterns[0].end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups)
#preprocessor-rule-disabled-interface.end  groups 3 (waived: the grammar names the group, but the expression has two groups)
#preprocessor-rule-disabled-interface.patterns[0].end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups)
#preprocessor-rule-enabled-implementation.end  groups 3 (waived: the grammar names the group, but the expression has two groups)
#preprocessor-rule-enabled-implementation.patterns[0].end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups)
#preprocessor-rule-enabled-interface.end  groups 3 (waived: the grammar names the group, but the expression has two groups)
#preprocessor-rule-enabled-interface.patterns[0].end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups)
```

### `pascal`

```text
patterns[12].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
patterns[15].patterns[0].match  ''  waived: the string's end `'` is tried before its patterns at every quote, so `''` ends the string and the next quote starts another
```

### `perl`

```text
patterns[3].patterns[0].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[3].patterns[1].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[3].patterns[2].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[3].patterns[3].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[3].patterns[4].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[3].patterns[5].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[4].patterns[0].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[4].patterns[1].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[4].patterns[2].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[4].patterns[3].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[4].patterns[4].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[4].patterns[5].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[4].patterns[5].patterns[3].begin  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[4].patterns[5].patterns[3].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[5].patterns[0].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[5].patterns[1].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[5].patterns[2].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[5].patterns[3].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[6].patterns[0].end  groups 0 (waived: the end is a lookahead, which holds no text), 1 (waived: the grammar names the group, but the expression has no groups)
patterns[7].patterns[0].end  groups 0 (waived: the end is a lookahead, which holds no text), 1 (waived: the grammar names the group, but the expression has no groups)
patterns[12].match  groups 3 (waived: the grammar names the group, but the expression has two groups)
patterns[13].match  groups 2 (waived: the grammar names the group, but the expression has one group), 3 (waived: the grammar names the group, but the expression has one group)
patterns[14].patterns[0].match  groups 1 (waived: the group matches only blanks, and the oracle compares no blank), 2 (waived: the group matches only blanks, and the oracle compares no blank)
patterns[17].end  \z  fuzz  waived: vscode-textmate rewrites `\z` so that it never matches a line, so the data section runs to the document's end
patterns[43].end  groups 0 (waived: the end is `$`, which holds no text)
#heredoc.patterns[5].begin  ((((<<(~)?) *')(POSTSCRIPT)(')))(.*)\n?  waived: the rule's only pattern includes `source.postscript`, which Shiki lacks, so vscode-textmate drops it, and the quoted heredoc after it takes the label
#heredoc.patterns[5].end  ^((?!\5)\s+)?((\6))$  waived: the rule's only pattern includes `source.postscript`, which Shiki lacks, so vscode-textmate drops it, and the quoted heredoc after it takes the label
#heredoc.patterns[5].patterns[0].begin  ^  waived: the rule's only pattern includes `source.postscript`, which Shiki lacks, so vscode-textmate drops it, and the quoted heredoc after it takes the label
#heredoc.patterns[5].patterns[0].end  \n  waived: the rule's only pattern includes `source.postscript`, which Shiki lacks, so vscode-textmate drops it, and the quoted heredoc after it takes the label
#heredoc.patterns[7].begin  groups 7 (waived: the group is `()`, the closing quote of a label that has none, so it never holds text)
#heredoc.patterns[15].begin  groups 7 (waived: the group is `()`, the closing quote of a label that has none, so it never holds text)
#heredoc.patterns[16].begin  groups 7 (waived: the group is `()`, the closing quote of a label that has none, so it never holds text)
#heredoc.patterns[17].begin  groups 7 (waived: the group is `()`, the closing quote of a label that has none, so it never holds text)
#heredoc.patterns[18].begin  groups 7 (waived: the group is `()`, the closing quote of a label that has none, so it never holds text)
#heredoc.patterns[19].begin  groups 7 (waived: the group is `()`, the closing quote of a label that has none, so it never holds text)
#heredoc.patterns[20].begin  groups 7 (waived: the group is `()`, the closing quote of a label that has none, so it never holds text)
#heredoc.patterns[21].begin  groups 7 (waived: the group is `()`, the closing quote of a label that has none, so it never holds text)
#heredoc.patterns[22].begin  ((((<<(~)?) *`)([^`]*)(`)))(.*)\n?  waived: the bare heredoc before it matches first at `<<`, with an empty label, since a bare label stops at the backtick
#heredoc.patterns[22].end  ^((?!\5)\s+)?((\6))$  waived: the bare heredoc before it matches first at `<<`, with an empty label, since a bare label stops at the backtick
#line_comment.patterns[0].begin  groups 1 (waived: the grammar names the group, but the expression has no groups)
#nested_braces.begin  groups 1 (waived: the grammar names the group, but the expression has no groups)
#nested_braces.end  groups 1 (waived: the grammar names the group, but the expression has no groups)
#nested_braces_interpolated.begin  groups 1 (waived: the grammar names the group, but the expression has no groups)
#nested_braces_interpolated.end  groups 1 (waived: the grammar names the group, but the expression has no groups)
#nested_brackets.begin  groups 1 (waived: the grammar names the group, but the expression has no groups)
#nested_brackets.end  groups 1 (waived: the grammar names the group, but the expression has no groups)
#nested_brackets_interpolated.begin  groups 1 (waived: the grammar names the group, but the expression has no groups)
#nested_brackets_interpolated.end  groups 1 (waived: the grammar names the group, but the expression has no groups)
#nested_ltgt.begin  groups 1 (waived: the grammar names the group, but the expression has no groups)
#nested_ltgt.end  groups 1 (waived: the grammar names the group, but the expression has no groups)
#nested_ltgt_interpolated.begin  groups 1 (waived: the grammar names the group, but the expression has no groups)
#nested_ltgt_interpolated.end  groups 1 (waived: the grammar names the group, but the expression has no groups)
#nested_parens.begin  groups 1 (waived: the grammar names the group, but the expression has no groups)
#nested_parens.end  groups 1 (waived: the grammar names the group, but the expression has no groups)
#nested_parens_interpolated.begin  groups 1 (waived: the grammar names the group, but the expression has no groups)
#nested_parens_interpolated.end  groups 1 (waived: the grammar names the group, but the expression has no groups)
```

### `php`

```text
#comments.patterns[2].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#comments.patterns[3].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#heredoc_interior.patterns[0].begin  groups 5 (waived: the group matches only blanks, and the oracle compares no blank)
#heredoc_interior.patterns[1].begin  groups 5 (waived: the group matches only blanks, and the oracle compares no blank)
#heredoc_interior.patterns[2].begin  groups 5 (waived: the group matches only blanks, and the oracle compares no blank)
#heredoc_interior.patterns[3].begin  groups 5 (waived: the group matches only blanks, and the oracle compares no blank)
#heredoc_interior.patterns[4].begin  groups 5 (waived: the group matches only blanks, and the oracle compares no blank)
#heredoc_interior.patterns[5].begin  groups 5 (waived: the group matches only blanks, and the oracle compares no blank)
#heredoc_interior.patterns[6].begin  groups 5 (waived: the group matches only blanks, and the oracle compares no blank)
#heredoc_interior.patterns[6].patterns[5].end  groups 0 (waived: the end is `$`, which holds no text)
#heredoc_interior.patterns[7].begin  groups 5 (waived: the group matches only blanks, and the oracle compares no blank)
#heredoc_interior.patterns[8].begin  groups 5 (waived: the group matches only blanks, and the oracle compares no blank)
#nowdoc_interior.patterns[0].begin  groups 3 (waived: the group matches only blanks, and the oracle compares no blank)
#nowdoc_interior.patterns[1].begin  groups 3 (waived: the group matches only blanks, and the oracle compares no blank)
#nowdoc_interior.patterns[2].begin  groups 3 (waived: the group matches only blanks, and the oracle compares no blank)
#nowdoc_interior.patterns[3].begin  groups 3 (waived: the group matches only blanks, and the oracle compares no blank)
#nowdoc_interior.patterns[4].begin  groups 3 (waived: the group matches only blanks, and the oracle compares no blank)
#nowdoc_interior.patterns[5].begin  groups 3 (waived: the group matches only blanks, and the oracle compares no blank)
#nowdoc_interior.patterns[6].begin  groups 3 (waived: the group matches only blanks, and the oracle compares no blank)
#nowdoc_interior.patterns[6].patterns[4].end  groups 0 (waived: the end is `$`, which holds no text)
#nowdoc_interior.patterns[7].begin  groups 3 (waived: the group matches only blanks, and the oracle compares no blank)
#nowdoc_interior.patterns[8].begin  groups 3 (waived: the group matches only blanks, and the oracle compares no blank)
```

### `postcss`

```text
patterns[12].end  groups 0 (waived: the group is a lookahead, so it never holds text)
patterns[13].end  groups 0 (waived: the end matches only the line feed or a lookahead, and the oracle compares no blank)
```

### `powershell`

```text
#commentLine.end  groups 1 (waived: the grammar names the group, but the expression has no groups)
#variableNoProperty.patterns[1].match  groups 3 (waived: the grammar names the group, but the expression has two groups)
#variableNoProperty.patterns[2].match  groups 3 (waived: the grammar names the group, but the expression has two groups)
#variableNoProperty.patterns[3].match  groups 3 (waived: the grammar names the group, but the expression has two groups)
#variableNoProperty.patterns[4].match  groups 4 (waived: the grammar names the group, but the expression has three groups)
#variableNoProperty.patterns[6].match  groups 4 (waived: the grammar names the group, but the expression has three groups)
```

### `python`

```text
#docstring.patterns[2].end  groups 2 (waived: the group matches only the line feed at the end of the line, and the oracle compares no blank)
#docstring.patterns[3].end  groups 2 (waived: the group matches only the line feed at the end of the line, and the oracle compares no blank)
#double-one-regexp-character-set.patterns[1].end  groups 2 (waived: the group is a lookahead, so it never holds text)
#double-one-regexp-comments.end  groups 2 (waived: the group is a lookahead, so it never holds text)
#double-one-regexp-conditional.end  groups 2 (waived: the group is a lookahead, so it never holds text)
#double-one-regexp-lookahead.end  groups 2 (waived: the group is a lookahead, so it never holds text)
#double-one-regexp-lookahead-negative.end  groups 2 (waived: the group is a lookahead, so it never holds text)
#double-one-regexp-lookbehind.end  groups 2 (waived: the group is a lookahead, so it never holds text)
#double-one-regexp-lookbehind-negative.end  groups 2 (waived: the group is a lookahead, so it never holds text)
#double-one-regexp-named-group.end  groups 2 (waived: the group is a lookahead, so it never holds text)
#double-one-regexp-parentheses.end  groups 2 (waived: the group is a lookahead, so it never holds text)
#double-one-regexp-parentheses-non-capturing.end  groups 2 (waived: the group is a lookahead, so it never holds text)
#double-three-regexp-character-set.patterns[1].end  groups 2 (waived: the grammar names the group, but the expression has one group)
#double-three-regexp-comments.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#double-three-regexp-conditional.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#double-three-regexp-lookahead.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#double-three-regexp-lookahead-negative.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#double-three-regexp-lookbehind.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#double-three-regexp-lookbehind-negative.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#double-three-regexp-named-group.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#double-three-regexp-parentheses.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#double-three-regexp-parentheses-non-capturing.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#fstring-fnorm-quoted-multi-line.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#fstring-fnorm-quoted-single-line.end  groups 2 (waived: the group matches only the line feed at the end of the line, and the oracle compares no blank)
#fstring-formatting-braces.patterns[0].match  groups 2 (waived: the group matches only blanks, and the oracle compares no blank)
#fstring-normf-quoted-multi-line.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#fstring-normf-quoted-single-line.end  groups 2 (waived: the group matches only the line feed at the end of the line, and the oracle compares no blank)
#fstring-raw-quoted-multi-line.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#fstring-raw-quoted-single-line.end  groups 2 (waived: the group matches only the line feed at the end of the line, and the oracle compares no blank)
#impossible.match  $.^  waived: `$.^` needs a character between the end of a line and the start of one, and `.` takes no line feed
#regexp-double-one-line.end  groups 2 (waived: the group matches only the line feed at the end of the line, and the oracle compares no blank)
#regexp-double-three-line.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#regexp-single-one-line.end  groups 2 (waived: the group matches only the line feed at the end of the line, and the oracle compares no blank)
#regexp-single-three-line.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#single-one-regexp-character-set.patterns[1].end  groups 2 (waived: the group is a lookahead, so it never holds text)
#single-one-regexp-comments.end  groups 2 (waived: the group is a lookahead, so it never holds text)
#single-one-regexp-conditional.end  groups 2 (waived: the group is a lookahead, so it never holds text)
#single-one-regexp-lookahead.end  groups 2 (waived: the group is a lookahead, so it never holds text)
#single-one-regexp-lookahead-negative.end  groups 2 (waived: the group is a lookahead, so it never holds text)
#single-one-regexp-lookbehind.end  groups 2 (waived: the group is a lookahead, so it never holds text)
#single-one-regexp-lookbehind-negative.end  groups 2 (waived: the group is a lookahead, so it never holds text)
#single-one-regexp-named-group.end  groups 2 (waived: the group is a lookahead, so it never holds text)
#single-one-regexp-parentheses.end  groups 2 (waived: the group is a lookahead, so it never holds text)
#single-one-regexp-parentheses-non-capturing.end  groups 2 (waived: the group is a lookahead, so it never holds text)
#single-three-regexp-character-set.patterns[1].end  groups 2 (waived: the grammar names the group, but the expression has one group)
#single-three-regexp-comments.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#single-three-regexp-conditional.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#single-three-regexp-lookahead.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#single-three-regexp-lookahead-negative.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#single-three-regexp-lookbehind.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#single-three-regexp-lookbehind-negative.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#single-three-regexp-named-group.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#single-three-regexp-parentheses.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#single-three-regexp-parentheses-non-capturing.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#string-bin-quoted-multi-line.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#string-bin-quoted-single-line.end  groups 2 (waived: the group matches only the line feed at the end of the line, and the oracle compares no blank)
#string-quoted-multi-line.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#string-quoted-single-line.end  groups 2 (waived: the group matches only the line feed at the end of the line, and the oracle compares no blank)
#string-raw-bin-quoted-multi-line.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#string-raw-bin-quoted-single-line.end  groups 2 (waived: the group matches only the line feed at the end of the line, and the oracle compares no blank)
#string-raw-quoted-multi-line.end  groups 2 (waived: the grammar names the group, but the expression has one group)
#string-raw-quoted-single-line.end  groups 2 (waived: the group matches only the line feed at the end of the line, and the oracle compares no blank)
```

### `r`

```text
#basic-roxygen-example.patterns[0].match  ^\s*#+'  waived: `#roxygen-example.patterns[0]`, the same expression, comes before it in the one list that holds it
```

### `ruby`

```text
patterns[21].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
patterns[86].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
patterns[88].begin  ^__END__\n  waived: `patterns[29]` (`\b[A-Z_]\w*\b`), earlier in the same list, takes `__END__` as a constant at the same place
patterns[88].end  (?=not)impossible  waived: its begin never matches
patterns[88].patterns[0].begin  (?=<?xml|<(?i:html\b)|!DOCTYPE (?i:html\b))  waived: the rule that holds it never opens
patterns[88].patterns[0].end  (?=not)impossible  waived: the rule that holds it never opens
#nest_brackets_r.begin  \[  waived: `#regex_sub`, before it in every list that holds it, opens a character class at every `[`
#nest_brackets_r.end  ]  waived: its begin never matches
#nest_parens_r.begin  \(  waived: `#regex_sub`, before it in every list that holds it, opens a group at every `(`
#nest_parens_r.end  \)  waived: its begin never matches
#regex_sub.patterns[6].end  groups 0 (waived: the group matches only the line feed at the end of the line, and the oracle compares no blank)
```

### `rust`

```text
#functions.patterns[1].match  \b(pub)(\()  waived: `\b(pub)\b`, the pattern before it, matches wherever `\b(pub)(\()` does
```

### `sass`

```text
patterns[2].end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups), 3 (waived: the grammar names the group, but the expression has no groups)
patterns[9].end  groups 0 (waived: the end matches only the line feed, or nothing before a lookahead, and the oracle compares no blank)
patterns[17].end  groups 0 (waived: the group is a lookahead, so it never holds text)
patterns[18].end  groups 0 (waived: the group is a lookahead, so it never holds text)
patterns[19].end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups)
patterns[20].end  groups 0 (waived: the end matches only the line feed, or nothing before a lookahead, and the oracle compares no blank)
```

### `scala`

```text
#comments.patterns[1].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#exports.patterns[5].patterns[2].match  groups 1 (waived: `\b(given)\b`, before it in the same list, matches at every `given` that would start the group)
#imports.patterns[6].patterns[2].match  groups 1 (waived: `\b(given)\b`, before it in the same list, matches at every `given` that would start the group)
#meta-bounds.match  <%|=:=|<:<|<%<|>:|<:  waived: `#keywords`, before it in `#code`, takes every run of two or more operator characters, and each bound is such a run
#meta-brackets.patterns[0].match  \{  waived: `#curly-braces`, before it in `#code`, opens at every `{`
#xml-literal.patterns[1].end  groups 2 (waived: the rule's `captures` name the begin's groups for its end too, and the end's expression has one group), 3 (waived: the rule's `captures` name the begin's groups for its end too, and the end's expression has one group), 4 (waived: the rule's `captures` name the begin's groups for its end too, and the end's expression has one group), 5 (waived: the rule's `captures` name the begin's groups for its end too, and the end's expression has one group)
```

### `scss`

```text
#at_rule_charset.end  groups 1 (waived: the group is a lookahead, so it never holds text), 2 (waived: the grammar names the group, but the expression has one group)
#at_rule_content.end  groups 1 (waived: the group is a lookahead, so it never holds text)
#at_rule_each.end  groups 1 (waived: the group is a lookahead, so it never holds text), 2 (waived: the grammar names the group, but the expression has one group)
#at_rule_else.end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups)
#at_rule_extend.end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups)
#at_rule_for.end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups)
#at_rule_forward.end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups)
#at_rule_function.patterns[0].begin  groups 3 (waived: the grammar names the group, but the expression has two groups)
#at_rule_function.patterns[0].end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups), 3 (waived: the grammar names the group, but the expression has no groups)
#at_rule_function.patterns[1].match  \s*((@)function)\b\s*  waived: `#at_rule_function.patterns[0]`, before it, begins with the same expression
#at_rule_if.end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups)
#at_rule_import.end  groups 1 (waived: the group is a lookahead, so it never holds text), 2 (waived: the grammar names the group, but the expression has one group)
#at_rule_namespace.patterns[1].end  groups 1 (waived: the grammar names the group, but the expression has no groups)
#at_rule_page.patterns[0].end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups), 3 (waived: the grammar names the group, but the expression has no groups)
#at_rule_return.end  groups 1 (waived: the group is a lookahead, so it never holds text), 2 (waived: the grammar names the group, but the expression has one group)
#at_rule_supports.end  groups 0 (waived: the expression is a lookahead or the end of the line, so the match holds no text), 1 (waived: the grammar names the group, but the expression has no groups)
#at_rule_use.end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups)
#at_rule_warn.end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups)
#at_rule_while.end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups)
#comment_docblock.begin  ///  waived: the rule's only pattern includes `source.sassdoc`, which Shiki does not load, so vscode-textmate drops the rule
#comment_docblock.end  (?=$)  waived: the rule's only pattern includes `source.sassdoc`, which Shiki does not load, so vscode-textmate drops the rule
#constant_sass_functions.begin  (headings|stylesheet-url|rgba?|hsla?|ie-hex-str|red|green|b…  waived: `#constant_functions`, before it in `#property_values`, the only rule that includes it, matches every `name(` this does
#constant_sass_functions.end  (\))  waived: its begin never matches
#parent_selector_suffix.match  groups 2 (waived: the grammar names the group, but the expression has one group)
#parent_selector_suffix.captures[2].patterns[1].match  \\(\h{1,6}|.)  waived: it applies to group 2, which the expression does not have
#parent_selector_suffix.captures[2].patterns[2].match  [$}]  waived: it applies to group 2, which the expression does not have
```

### `shellscript`

```text
#alias_statement.begin  groups 7 (waived: the alternative before it in the index, `\$?(?<!\w)[-0-9A-Z_a-z]+(?!\w)`, matches every text this group's `-?\d+` can)
#array_value.begin  groups 4 (waived: the alternative before it in the index, `\$?(?<!\w)[-0-9A-Z_a-z]+(?!\w)`, matches every text this group's `-?\d+` can)
#interpolation.patterns[2].patterns[1].begin  groups 1 (waived: the grammar names the group, but the expression has no groups)
#loop.patterns[0].begin  (?<=^|[\&;\s])(for)\s+(.+?)\s+(in)(?=[\&;\s]|$)  waived: `#for_statement`, before `#loop` in `#initial_context`, matches `\bfor\b` wherever this does
#loop.patterns[0].end  (?<=^|[\&;\s])done(?=[\&;\s]|$|\))  waived: its begin never matches, since `#for_statement`, before `#loop` in `#initial_context`, takes every `for`
#modified_assignment_statement.patterns[2].match  groups 4 (waived: the alternative before it in the index, `\$?(?<!\w)[-0-9A-Z_a-z]+(?!\w)`, matches every text this group's `-?\d+` can)
#normal_assignment_statement.begin  groups 4 (waived: the alternative before it in the index, `\$?(?<!\w)[-0-9A-Z_a-z]+(?!\w)`, matches every text this group's `-?\d+` can)
#normal_statement_seperator.match  groups 3 (waived: `#pipeline`, before it in every context that includes both, takes the first `|` of every `||` with `[!|]`)
#option.begin  groups 2 (waived: the group is a lookahead, so it never holds text)
```

### `sql`

```text
#comments.patterns[0].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#comments.patterns[1].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#regexps.patterns[0].begin  /(?=\S.*/)  waived: `patterns[21]`, `[-+/]`, before `#regexps` in the only list that holds it, matches every `/`
#regexps.patterns[0].end  /  waived: its begin never matches
#regexps.patterns[0].patterns[1].match  \\/  waived: its rule's begin never matches
```

### `svelte`

```text
injections[L:meta.style.svelte meta.lang.stylus - (meta source)].patterns[0].begin  (?<=>)(?!</)  waived: the rule's only pattern includes `source.stylus`, which Shiki does not load, so vscode-textmate drops the rule
injections[L:meta.style.svelte meta.lang.stylus - (meta source)].patterns[0].end  (?=</)  waived: the rule's only pattern includes `source.stylus`, which Shiki does not load, so vscode-textmate drops the rule
injections[L:meta.template.svelte meta.lang.pug - (meta source)].patterns[0].begin  (?<=>)(?!</)  waived: the rule's only pattern includes `text.pug`, which Shiki does not load, so vscode-textmate drops the rule
injections[L:meta.template.svelte meta.lang.pug - (meta source)].patterns[0].end  (?=</)  waived: the rule's only pattern includes `text.pug`, which Shiki does not load, so vscode-textmate drops the rule
```

### `swift`

```text
#comments.patterns[5].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#compiler-control.patterns[1].end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups)
#declarations-import.patterns[0].patterns[2].captures[1].patterns[0].match  \.  waived: group 1 is a run of operator heads and combining marks, none of them `.`, so the pattern never finds one in it
#keywords.patterns[5].match  groups 2 (waived: the group matches only blanks, and the oracle compares no blank)
#keywords.patterns[10].match  groups 2 (waived: the group matches only blanks, and the oracle compares no blank)
#literals-regular-expression-literal.patterns[2].match  groups 5 (waived: the group matches only the line feed at the end of the line, and the oracle compares no blank)
#literals-regular-expression-literal-quote.end  groups 1 (waived: the group matches only the line feed at the end of the line, and the oracle compares no blank)
#literals-string-raw-string-guts.patterns[2].patterns[1].begin  \(  waived: `$self`, before it in the same list, holds `#expressions-without-trailing-closures-or-member-references-parenthesized-expression`, whose begin `\(` matches every `(`
#literals-string-raw-string-guts.patterns[2].patterns[1].end  \)  waived: its begin never matches
#literals-string-string-guts.patterns[2].patterns[1].begin  \(  waived: `$self`, before it in the same list, holds `#expressions-without-trailing-closures-or-member-references-parenthesized-expression`, whose begin `\(` matches every `(`
#literals-string-string-guts.patterns[2].patterns[1].end  \)  waived: its begin never matches
#operators.patterns[1].patterns[0].captures[0].patterns[0].match  \G(\+\+|--)$  waived: vscode-textmate tokenizes a capture's patterns with no anchor, so `\G` never matches there and the operator falls to the `.+` pattern after it
#operators.patterns[1].patterns[0].captures[0].patterns[1].match  \G([-+])$  waived: vscode-textmate tokenizes a capture's patterns with no anchor, so `\G` never matches there and the operator falls to the `.+` pattern after it
#operators.patterns[1].patterns[0].captures[0].patterns[2].match  \G!$  waived: vscode-textmate tokenizes a capture's patterns with no anchor, so `\G` never matches there and the operator falls to the `.+` pattern after it
#operators.patterns[1].patterns[0].captures[0].patterns[3].match  \G~$  waived: vscode-textmate tokenizes a capture's patterns with no anchor, so `\G` never matches there and the operator falls to the `.+` pattern after it
#operators.patterns[1].patterns[1].captures[0].patterns[0].match  \G(\+\+|--)$  waived: vscode-textmate tokenizes a capture's patterns with no anchor, so `\G` never matches there and the operator falls to the `.+` pattern after it
#operators.patterns[1].patterns[1].captures[0].patterns[1].match  \G!$  waived: vscode-textmate tokenizes a capture's patterns with no anchor, so `\G` never matches there and the operator falls to the `.+` pattern after it
#operators.patterns[1].patterns[2].captures[0].patterns[0].match  \G=$  waived: vscode-textmate tokenizes a capture's patterns with no anchor, so `\G` never matches there and the operator falls to the `.+` pattern after it
#operators.patterns[1].patterns[2].captures[0].patterns[1].match  \G([-%*+/]|<<|>>|[\&^|]|&&|\|\|)=$  waived: vscode-textmate tokenizes a capture's patterns with no anchor, so `\G` never matches there and the operator falls to the `.+` pattern after it
#operators.patterns[1].patterns[2].captures[0].patterns[2].match  \G([-*+/])$  waived: vscode-textmate tokenizes a capture's patterns with no anchor, so `\G` never matches there and the operator falls to the `.+` pattern after it
#operators.patterns[1].patterns[2].captures[0].patterns[3].match  \G&([-*+])$  waived: vscode-textmate tokenizes a capture's patterns with no anchor, so `\G` never matches there and the operator falls to the `.+` pattern after it
#operators.patterns[1].patterns[2].captures[0].patterns[4].match  \G%$  waived: vscode-textmate tokenizes a capture's patterns with no anchor, so `\G` never matches there and the operator falls to the `.+` pattern after it
#operators.patterns[1].patterns[2].captures[0].patterns[5].match  \G(==|!=|[<>]|>=|<=|~=)$  waived: vscode-textmate tokenizes a capture's patterns with no anchor, so `\G` never matches there and the operator falls to the `.+` pattern after it
#operators.patterns[1].patterns[2].captures[0].patterns[6].match  \G\?\?$  waived: vscode-textmate tokenizes a capture's patterns with no anchor, so `\G` never matches there and the operator falls to the `.+` pattern after it
#operators.patterns[1].patterns[2].captures[0].patterns[7].match  \G(&&|\|\|)$  waived: vscode-textmate tokenizes a capture's patterns with no anchor, so `\G` never matches there and the operator falls to the `.+` pattern after it
#operators.patterns[1].patterns[2].captures[0].patterns[8].match  \G([\&^|]|<<|>>)$  waived: vscode-textmate tokenizes a capture's patterns with no anchor, so `\G` never matches there and the operator falls to the `.+` pattern after it
#operators.patterns[1].patterns[2].captures[0].patterns[9].match  \G([!=]==)$  waived: vscode-textmate tokenizes a capture's patterns with no anchor, so `\G` never matches there and the operator falls to the `.+` pattern after it
#operators.patterns[1].patterns[2].captures[0].patterns[10].match  \G\?$  waived: vscode-textmate tokenizes a capture's patterns with no anchor, so `\G` never matches there and the operator falls to the `.+` pattern after it
#operators.patterns[1].patterns[5].captures[0].patterns[0].match  \G\.\.[.<]$  waived: vscode-textmate tokenizes a capture's patterns with no anchor, so `\G` never matches there and the operator falls to the `.+` pattern after it
```

### `toml`

```text
#comments.begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#key_pair.patterns[0].end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups)
#key_pair.patterns[1].end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups), 3 (waived: the grammar names the group, but the expression has no groups), 4 (waived: the grammar names the group, but the expression has no groups), 5 (waived: the grammar names the group, but the expression has no groups)
#key_pair.patterns[2].end  groups 1 (waived: the grammar names the group, but the expression has no groups), 2 (waived: the grammar names the group, but the expression has no groups), 4 (waived: the grammar names the group, but the expression has no groups), 5 (waived: the grammar names the group, but the expression has no groups)
#key_pair.patterns[3].end  groups 1 (waived: the grammar names the group, but the expression has no groups), 3 (waived: the grammar names the group, but the expression has no groups)
```

### `tsx`

```text
#comment.patterns[2].begin  groups 1 (waived: the group matches only the blanks that lead the line, and the oracle compares no blank)
#expression-operators.patterns[6].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))instanceof(?![$_[:…  waived: `#instanceof-expr` begins with the same expression and comes before `#expression-operators` in `#expressionWithoutIdentifiers`, the only rule that includes it
#expression-operators.patterns[7].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))new(?![$_[:alnum:]…  waived: `#new-expr` begins with the same expression and comes before `#expression-operators` in `#expressionWithoutIdentifiers`, the only rule that includes it
#single-line-comment-consuming-line-ending.begin  groups 1 (waived: the group matches only the blanks that lead the line, and the oracle compares no blank)
#support-objects.patterns[4].match  groups 4 (waived: the group sits inside the negative lookahead after the property names, so it never holds text; the constant names are group 53)
#template.patterns[1].begin  groups 1 (waived: `#template-call`, the pattern before it, starts at the same name wherever this group would match one)
#template-type.patterns[1].begin  groups 1 (waived: `#template-call`, the pattern before it, starts at the same name wherever this group would match one)
```

### `typescript`

```text
#comment.patterns[2].begin  groups 1 (waived: the group matches only the blanks that lead the line, and the oracle compares no blank)
#expression-operators.patterns[6].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))instanceof(?![$_[:…  waived: `#instanceof-expr` begins with the same expression and comes before `#expression-operators` in `#expressionWithoutIdentifiers`, the only rule that includes it
#expression-operators.patterns[7].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))new(?![$_[:alnum:]…  waived: `#new-expr` begins with the same expression and comes before `#expression-operators` in `#expressionWithoutIdentifiers`, the only rule that includes it
#single-line-comment-consuming-line-ending.begin  groups 1 (waived: the group matches only the blanks that lead the line, and the oracle compares no blank)
#support-objects.patterns[4].match  groups 4 (waived: the group sits inside the negative lookahead after the property names, so it never holds text; the constant names are group 53)
#template.patterns[1].begin  groups 1 (waived: `#template-call`, the pattern before it, starts at the same name wherever this group would match one)
#template-type.patterns[1].begin  groups 1 (waived: `#template-call`, the pattern before it, starts at the same name wherever this group would match one)
```

### `vb`

```text
patterns[2].patterns[0].match  groups 1 (waived: the group matches only blanks, and the oracle compares no blank), 2 (waived: the group matches only blanks, and the oracle compares no blank)
patterns[3].patterns[0].match  groups 1 (waived: the group matches only blanks, and the oracle compares no blank), 2 (waived: the group matches only blanks, and the oracle compares no blank)
patterns[5].begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
patterns[12].patterns[0].match  ""  waived: the string's end `"` is tried before its patterns at every quote, so `""` ends the string and the next quote starts another
```

### `vue`

```text
patterns[2].patterns[0].begin  ([-0-:A-Za-z]+)\b(?=[^>]*\blang\s*=\s*(["']?)md\b\2)
patterns[2].patterns[0].end  (</)(\1)\s*(?=>)
patterns[2].patterns[0].patterns[1].begin  (?<=>)
patterns[2].patterns[0].patterns[1].end  (?=</)
patterns[2].patterns[2].patterns[1].begin  (?<=>)
patterns[2].patterns[2].patterns[1].end  (?=</)
patterns[2].patterns[3].patterns[1].begin  (?<=>)
patterns[2].patterns[3].patterns[1].end  (?=</)
patterns[2].patterns[4].patterns[1].begin  (?<=>)
patterns[2].patterns[4].patterns[1].end  (?=</)
patterns[2].patterns[12].end  (</)(\1)\s*(?=>)
patterns[2].patterns[12].patterns[1].end  (?=</script[>\s])
patterns[2].patterns[12].patterns[2].begin  (?<=>)
patterns[2].patterns[12].patterns[2].while  ^(?!\s*</script[>\s])
patterns[2].patterns[14].begin  ([-0-:A-Za-z]+)\b(?=[^>]*\blang\s*=\s*(["']?)json\b\2)
patterns[2].patterns[14].end  (</)(\1)\s*(?=>)
patterns[2].patterns[14].patterns[1].begin  (?<=>)
patterns[2].patterns[14].patterns[1].end  (?=</)
patterns[2].patterns[15].begin  ([-0-:A-Za-z]+)\b(?=[^>]*\blang\s*=\s*(["']?)jsonc\b\2)
patterns[2].patterns[15].end  (</)(\1)\s*(?=>)
patterns[2].patterns[15].patterns[1].begin  (?<=>)
patterns[2].patterns[15].patterns[1].end  (?=</)
patterns[2].patterns[16].patterns[1].begin  (?<=>)
patterns[2].patterns[16].patterns[1].end  (?=</)
patterns[2].patterns[18].begin  ([-0-:A-Za-z]+)\b(?=[^>]*\blang\s*=\s*(["']?)toml\b\2)
patterns[2].patterns[18].end  (</)(\1)\s*(?=>)
patterns[2].patterns[18].patterns[1].begin  (?<=>)
patterns[2].patterns[18].patterns[1].end  (?=</)
patterns[2].patterns[20].begin  ([-0-:A-Za-z]+)\b(?=[^>]*\blang\s*=\s*(["']?)vue\b\2)
patterns[2].patterns[20].end  (</)(\1)\s*(?=>)
patterns[2].patterns[20].patterns[1].begin  (?<=>)
patterns[2].patterns[20].patterns[1].end  (?=</)
#multi-line-script-tag-stuff.patterns[1].patterns[1].begin  (?<=>)(?=[^\n]*</script[>\s])
#multi-line-script-tag-stuff.patterns[1].patterns[1].end  (?=</script[>\s])
#multi-line-script-tag-stuff.patterns[2].patterns[1].begin  (?<=>)(?=[^\n]*</script[>\s])
#multi-line-script-tag-stuff.patterns[2].patterns[1].end  (?=</script[>\s])
#multi-line-style-tag-stuff.patterns[1].begin  (?=\blang\s*=\s*["']?scss\b)
#multi-line-style-tag-stuff.patterns[1].end  (?=</style[>\s])
#multi-line-style-tag-stuff.patterns[1].patterns[1].begin  (?<=>)
#multi-line-style-tag-stuff.patterns[1].patterns[1].end  (?=</style[>\s])
#multi-line-style-tag-stuff.patterns[2].patterns[1].begin  (?<=>)
#multi-line-style-tag-stuff.patterns[2].patterns[1].end  (?=</style[>\s])
#multi-line-style-tag-stuff.patterns[4].patterns[1].begin  (?<=>)
#multi-line-style-tag-stuff.patterns[4].patterns[1].end  (?=</style[>\s])
#template-tag-1.begin  (<)(template)\b(>)
#template-tag-1.end  (/?>)
#template-tag-1.patterns[0].begin  \G
#template-tag-1.patterns[0].end  (?=/>)|((</)(template)(?=[>\s]))
#vue-comments-key-value.begin  (<!--)\s*(@)([$\w]+)(?=\s)
#vue-comments-key-value.end  (-->)
#vue-directives-generic-attr.begin  \b(generic)\s*(=)
#vue-directives-generic-attr.end  (?<=["'])
#vue-directives-generic-attr.patterns[0].begin  (["'])
#vue-directives-generic-attr.patterns[0].end  (\1)
#vue-directives-generic-attr.patterns[0].patterns[1].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(extends|in|out)(?…
#vue-directives-generic-attr.patterns[0].patterns[4].match  (=)(?!>)
#vue-directives-original.begin  groups 6, 7, 8
#vue-directives-original.patterns[0].match  (\.)([-\w]*)
#vue-directives-style-attr.begin  \b(style)\s*(=)
#vue-directives-style-attr.end  (?<=["'])
#vue-directives-style-attr.patterns[0].begin  (["'])
#vue-directives-style-attr.patterns[0].end  (\1)
#vue-directives-style-attr.patterns[0].patterns[3].match  (?<![-\w])--[-A-Z_a-z[^\x00-\x7F]](?:[-0-9A-Z_a-z[^\x00-\x7…
#vue-directives-style-attr.patterns[0].patterns[4].begin  (?<![-A-Za-z])(?=[-A-Za-z])
#vue-directives-style-attr.patterns[0].patterns[4].end  $|(?![-A-Za-z])
#vue-directives-style-attr.patterns[0].patterns[5].begin  (:)\s*
#vue-directives-style-attr.patterns[0].patterns[5].end  \s*(;)|\s*(?=["'])
#vue-directives-style-attr.patterns[0].patterns[6].match  ;
```

### `xml`

```text
patterns[0].end  groups 2 (waived: the rule's `captures` name the group for its end too, and the end's expression has one group)
patterns[1].end  groups 2 (waived: the rule's `captures` name the group for its end too, and the end's expression has one group), 3 (waived: the rule's `captures` name the group for its end too, and the end's expression has one group)
patterns[4].end  groups 2 (waived: the rule's `captures` name the group for its end too, and the end's expression has one group), 3 (waived: the rule's `captures` name the group for its end too, and the end's expression has one group), 4 (waived: the rule's `captures` name the group for its end too, and the end's expression has one group), 5 (waived: the rule's `captures` name the group for its end too, and the end's expression has one group)
#EntityDecl.end  groups 2 (waived: the rule's `captures` name the group for its end too, and the end's expression has one group), 3 (waived: the rule's `captures` name the group for its end too, and the end's expression has one group), 4 (waived: the rule's `captures` name the group for its end too, and the end's expression has one group), 5 (waived: the rule's `captures` name the group for its end too, and the end's expression has one group)
```

### `yaml`

```text
#block-pair.patterns[0].begin  groups 1 (waived: the grammar names the group, but the expression has no groups)
#comment.begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#flow-pair.patterns[0].patterns[3].begin  :(?=\s|$|[],\[{}])  waived: `#flow-pair`, earlier in the same list, includes `#flow-pair.patterns[3]`, whose begin is the same expression
#flow-pair.patterns[0].patterns[3].end  (?=[],}])  waived: its begin never matches
#flow-pair.patterns[3].end  groups 0 (waived: the group is a lookahead, so it never holds text)
```

### `zig`

```text
#numbers.patterns[2].match  \b[0-9][0-9_]*\b  waived: `#numbers.patterns[1]`, before it, matches every one of its matches, since its fraction and exponent are optional
#operators.patterns[4].match  (==|\+\+|\*\*|->)  waived: the comparison and arithmetic rules before it match the first byte of `==`, `++`, `**` and `->`
```

## Units no root reaches

Neither an input language's grammar nor an injection reaches these, so they are left out of the shares.

### `c`

```text
#access-method.begin
#access-method.end
#access-method.beginCaptures[4].patterns[0].match
#access-method.beginCaptures[4].patterns[1].match
#access-method.beginCaptures[4].patterns[2].match
#access-method.beginCaptures[4].patterns[3].match
#static_assert.begin
#static_assert.end
#static_assert.patterns[0].begin
#static_assert.patterns[0].end
#static_assert.beginCaptures[4].patterns[0].match
#static_assert.beginCaptures[4].patterns[1].match
#static_assert.beginCaptures[9].patterns[0].match
#static_assert.beginCaptures[9].patterns[1].match
```

### `cpp`

```text
#alignas_operator.begin
#alignas_operator.end
#alignof_operator.begin
#alignof_operator.end
#assignment_operator.match
#class_declare.match
#class_declare.captures[3].patterns[0].match
#class_declare.captures[5].patterns[0].match
#class_declare.captures[5].patterns[1].match
#class_declare.captures[5].patterns[2].match
#class_declare.captures[7].patterns[0].match
#class_declare.captures[9].patterns[0].match
#class_declare.captures[11].patterns[0].match
#class_declare.captures[14].patterns[0].match
#d9bc4796b0b_module_import.match
#d9bc4796b0b_module_import.captures[2].patterns[0].match
#d9bc4796b0b_module_import.captures[9].patterns[0].match
#d9bc4796b0b_module_import.captures[14].patterns[0].match
#d9bc4796b0b_module_import.captures[17].patterns[0].match
#d9bc4796b0b_module_import.captures[19].patterns[0].match
#d9bc4796b0b_module_import.captures[21].patterns[0].match
#decltype_specifier.begin
#decltype_specifier.end
#empty_square_brackets.match
#enum_declare.match
#enum_declare.captures[3].patterns[0].match
#enum_declare.captures[5].patterns[0].match
#enum_declare.captures[5].patterns[1].match
#enum_declare.captures[5].patterns[2].match
#enum_declare.captures[7].patterns[0].match
#enum_declare.captures[9].patterns[0].match
#enum_declare.captures[11].patterns[0].match
#enum_declare.captures[14].patterns[0].match
#identifier.match
#noexcept_operator.begin
#noexcept_operator.end
#parameter_class.match
#parameter_class.captures[3].patterns[0].match
#parameter_class.captures[6].patterns[0].match
#parameter_class.captures[7].patterns[0].match
#parameter_class.captures[7].patterns[1].match
#parameter_class.captures[7].patterns[2].match
#parameter_class.captures[9].patterns[0].match
#parameter_class.captures[11].patterns[0].match
#parameter_class.captures[13].patterns[0].match
#parameter_class.captures[16].patterns[0].match
#parameter_class.captures[18].patterns[0].match
#parameter_class.captures[20].patterns[0].match
#parameter_enum.match
#parameter_enum.captures[3].patterns[0].match
#parameter_enum.captures[6].patterns[0].match
#parameter_enum.captures[7].patterns[0].match
#parameter_enum.captures[7].patterns[1].match
#parameter_enum.captures[7].patterns[2].match
#parameter_enum.captures[9].patterns[0].match
#parameter_enum.captures[11].patterns[0].match
#parameter_enum.captures[13].patterns[0].match
#parameter_enum.captures[16].patterns[0].match
#parameter_enum.captures[18].patterns[0].match
#parameter_enum.captures[20].patterns[0].match
#parameter_struct.match
#parameter_struct.captures[3].patterns[0].match
#parameter_struct.captures[6].patterns[0].match
#parameter_struct.captures[7].patterns[0].match
#parameter_struct.captures[7].patterns[1].match
#parameter_struct.captures[7].patterns[2].match
#parameter_struct.captures[9].patterns[0].match
#parameter_struct.captures[11].patterns[0].match
#parameter_struct.captures[13].patterns[0].match
#parameter_struct.captures[16].patterns[0].match
#parameter_struct.captures[18].patterns[0].match
#parameter_struct.captures[20].patterns[0].match
#parameter_union.match
#parameter_union.captures[3].patterns[0].match
#parameter_union.captures[6].patterns[0].match
#parameter_union.captures[7].patterns[0].match
#parameter_union.captures[7].patterns[1].match
#parameter_union.captures[7].patterns[2].match
#parameter_union.captures[9].patterns[0].match
#parameter_union.captures[11].patterns[0].match
#parameter_union.captures[13].patterns[0].match
#parameter_union.captures[16].patterns[0].match
#parameter_union.captures[18].patterns[0].match
#parameter_union.captures[20].patterns[0].match
#preprocessor_conditional_parentheses.begin
#preprocessor_conditional_parentheses.end
#qualifiers_and_specifiers_post_parameters_inline.match
#qualifiers_and_specifiers_post_parameters_inline.captures[1].patterns[0].match
#requires_keyword.begin
#requires_keyword.end
#scope_resolution.match
#scope_resolution_function_call.match
#scope_resolution_function_definition.match
#scope_resolution_function_definition_operator_overload.match
#scope_resolution_function_definition_operator_overload_inner_generated.match
#scope_resolution_namespace_alias.match
#scope_resolution_namespace_block.match
#scope_resolution_namespace_using.match
#scope_resolution_parameter.match
#scope_resolution_template_call.match
#scope_resolution_template_definition.match
#simple_type.match
#simple_type.captures[1].patterns[0].match
#simple_type.captures[1].patterns[1].match
#simple_type.captures[1].patterns[9].match
#simple_type.captures[4].patterns[0].match
#simple_type.captures[6].patterns[0].match
#simple_type.captures[7].patterns[0].match
#simple_type.captures[7].patterns[1].match
#simple_type.captures[11].patterns[0].match
#simple_type.captures[13].patterns[0].match
#simple_type.captures[13].patterns[1].match
#simple_type.captures[13].patterns[2].match
#simple_type.captures[15].patterns[0].match
#simple_type.captures[17].patterns[0].match
#sizeof_operator.begin
#sizeof_operator.end
#sizeof_variadic_operator.begin
#sizeof_variadic_operator.end
#std_space.match
#std_space.captures[1].patterns[0].match
#struct_declare.match
#struct_declare.captures[3].patterns[0].match
#struct_declare.captures[5].patterns[0].match
#struct_declare.captures[5].patterns[1].match
#struct_declare.captures[5].patterns[2].match
#struct_declare.captures[7].patterns[0].match
#struct_declare.captures[9].patterns[0].match
#struct_declare.captures[11].patterns[0].match
#struct_declare.captures[14].patterns[0].match
#typedef_function_pointer.begin
#typedef_function_pointer.end
#typedef_function_pointer.patterns[0].begin
#typedef_function_pointer.patterns[0].end
#typedef_function_pointer.patterns[0].beginCaptures[1].patterns[0].match
#typedef_function_pointer.patterns[0].beginCaptures[1].patterns[1].match
#typedef_function_pointer.patterns[0].beginCaptures[1].patterns[9].match
#typedef_function_pointer.patterns[0].beginCaptures[11].patterns[0].match
#typedef_function_pointer.patterns[0].beginCaptures[11].patterns[1].match
#typedef_function_pointer.patterns[0].beginCaptures[19].patterns[0].match
#typedef_function_pointer.patterns[0].beginCaptures[19].patterns[1].match
#typedef_function_pointer.patterns[0].beginCaptures[19].patterns[2].match
#typeid_operator.begin
#typeid_operator.end
#union_declare.match
#union_declare.captures[3].patterns[0].match
#union_declare.captures[5].patterns[0].match
#union_declare.captures[5].patterns[1].match
#union_declare.captures[5].patterns[2].match
#union_declare.captures[7].patterns[0].match
#union_declare.captures[9].patterns[0].match
#union_declare.captures[11].patterns[0].match
#union_declare.captures[14].patterns[0].match
#variable_declare.match
#variable_declare.captures[2].patterns[0].match
#variable_declare.captures[4].patterns[0].match
#variable_declare.captures[5].patterns[0].match
#variable_declare.captures[5].patterns[1].match
#variable_declare.captures[5].patterns[9].match
#variable_declare.captures[8].patterns[0].match
#variable_declare.captures[10].patterns[0].match
#variable_declare.captures[11].patterns[0].match
#variable_declare.captures[11].patterns[1].match
#variable_declare.captures[15].patterns[0].match
#variable_declare.captures[17].patterns[0].match
#variable_declare.captures[17].patterns[1].match
#variable_declare.captures[17].patterns[2].match
#variable_declare.captures[19].patterns[0].match
#variable_declare.captures[21].patterns[0].match
#variable_declare.captures[23].patterns[0].match
#variable_declare.captures[30].patterns[0].match
```

### `cpp-macro`

```text
#alignas_operator.begin
#alignas_operator.end
#alignof_operator.begin
#alignof_operator.end
#decltype_specifier.begin
#decltype_specifier.end
#noexcept_operator.begin
#noexcept_operator.end
#preprocessor_conditional_parentheses.begin
#preprocessor_conditional_parentheses.end
#requires_keyword.begin
#requires_keyword.end
#sizeof_operator.begin
#sizeof_operator.end
#sizeof_variadic_operator.begin
#sizeof_variadic_operator.end
#typedef_function_pointer.begin
#typedef_function_pointer.end
#typedef_function_pointer.patterns[0].begin
#typedef_function_pointer.patterns[0].end
#typedef_function_pointer.patterns[0].beginCaptures[1].patterns[0].match
#typedef_function_pointer.patterns[0].beginCaptures[1].patterns[1].match
#typedef_function_pointer.patterns[0].beginCaptures[1].patterns[9].match
#typedef_function_pointer.patterns[0].beginCaptures[11].patterns[0].match
#typedef_function_pointer.patterns[0].beginCaptures[11].patterns[1].match
#typedef_function_pointer.patterns[0].beginCaptures[19].patterns[0].match
#typedef_function_pointer.patterns[0].beginCaptures[19].patterns[1].match
#typedef_function_pointer.patterns[0].beginCaptures[19].patterns[2].match
#typeid_operator.begin
#typeid_operator.end
```

### `graphql`

```text
#graphql-union-mark.match
```

### `less`

```text
#color-adjuster-operators.match
```

### `nix`

```text
#function-body.begin
#function-body.end
```

### `objective-c`

```text
#c_lang.repository.#access-method.begin
#c_lang.repository.#access-method.end
#c_lang.repository.#access-method.beginCaptures[4].patterns[0].match
#c_lang.repository.#access-method.beginCaptures[4].patterns[1].match
#c_lang.repository.#access-method.beginCaptures[4].patterns[2].match
#c_lang.repository.#access-method.beginCaptures[4].patterns[3].match
#c_lang.repository.#static_assert.begin
#c_lang.repository.#static_assert.end
#c_lang.repository.#static_assert.patterns[0].begin
#c_lang.repository.#static_assert.patterns[0].end
```

### `objective-cpp`

```text
#c_lang.repository.#access-method.begin
#c_lang.repository.#access-method.end
#c_lang.repository.#access-method.beginCaptures[4].patterns[0].match
#c_lang.repository.#access-method.beginCaptures[4].patterns[1].match
#c_lang.repository.#access-method.beginCaptures[4].patterns[2].match
#c_lang.repository.#access-method.beginCaptures[4].patterns[3].match
#c_lang.repository.#static_assert.begin
#c_lang.repository.#static_assert.end
#c_lang.repository.#static_assert.patterns[0].begin
#c_lang.repository.#static_assert.patterns[0].end
#cpp_lang.repository.#block.begin
#cpp_lang.repository.#block.end
#cpp_lang.repository.#block.patterns[0].match
#cpp_lang_newish.patterns[1].match
#cpp_lang_newish.patterns[3].match
#cpp_lang_newish.patterns[4].match
#cpp_lang_newish.patterns[5].match
#cpp_lang_newish.patterns[6].match
#cpp_lang_newish.patterns[7].match
#cpp_lang_newish.patterns[8].match
#cpp_lang_newish.patterns[10].match
#cpp_lang_newish.patterns[13].match
#cpp_lang_newish.patterns[14].match
#cpp_lang_newish.patterns[16].match
#cpp_lang_newish.patterns[17].match
#cpp_lang_newish.patterns[18].begin
#cpp_lang_newish.patterns[18].end
#cpp_lang_newish.patterns[19].begin
#cpp_lang_newish.patterns[19].end
#cpp_lang_newish.patterns[24].match
#cpp_lang_newish.patterns[26].match
#cpp_lang_newish.patterns[31].begin
#cpp_lang_newish.patterns[31].end
#cpp_lang_newish.patterns[32].begin
#cpp_lang_newish.patterns[32].end
#cpp_lang_newish.patterns[32].patterns[0].begin
#cpp_lang_newish.patterns[32].patterns[0].end
#cpp_lang_newish.patterns[32].patterns[1].begin
#cpp_lang_newish.patterns[32].patterns[1].end
#cpp_lang_newish.patterns[32].patterns[2].begin
#cpp_lang_newish.patterns[32].patterns[2].end
#cpp_lang_newish.patterns[33].begin
#cpp_lang_newish.patterns[33].end
#cpp_lang_newish.patterns[33].patterns[1].begin
#cpp_lang_newish.patterns[33].patterns[1].end
#cpp_lang_newish.patterns[33].patterns[2].begin
#cpp_lang_newish.patterns[33].patterns[2].end
#cpp_lang_newish.patterns[35].begin
#cpp_lang_newish.patterns[35].end
#cpp_lang_newish.patterns[36].begin
#cpp_lang_newish.patterns[36].end
#cpp_lang_newish.patterns[36].patterns[0].match
#cpp_lang_newish.patterns[37].begin
#cpp_lang_newish.patterns[37].end
#cpp_lang_newish.patterns[37].patterns[1].match
#cpp_lang_newish.patterns[38].match
#cpp_lang_newish.patterns[39].match
#cpp_lang_newish.patterns[40].match
#cpp_lang_newish.patterns[41].match
#cpp_lang_newish.patterns[44].begin
#cpp_lang_newish.patterns[44].end
#cpp_lang_newish.patterns[46].begin
#cpp_lang_newish.patterns[46].end
#cpp_lang_newish.patterns[47].match
#cpp_lang_newish.patterns[48].match
#cpp_lang_newish.patterns[49].match
#cpp_lang_newish.repository.#access-member.match
#cpp_lang_newish.repository.#access-member.captures[4].patterns[0].match
#cpp_lang_newish.repository.#access-member.captures[4].patterns[1].match
#cpp_lang_newish.repository.#access-member.captures[4].patterns[2].match
#cpp_lang_newish.repository.#access-member.captures[4].patterns[3].match
#cpp_lang_newish.repository.#access-method.begin
#cpp_lang_newish.repository.#access-method.end
#cpp_lang_newish.repository.#access-method.beginCaptures[4].patterns[0].match
#cpp_lang_newish.repository.#access-method.beginCaptures[4].patterns[1].match
#cpp_lang_newish.repository.#access-method.beginCaptures[4].patterns[2].match
#cpp_lang_newish.repository.#access-method.beginCaptures[4].patterns[3].match
#cpp_lang_newish.repository.#angle_brackets.begin
#cpp_lang_newish.repository.#angle_brackets.end
#cpp_lang_newish.repository.#block.begin
#cpp_lang_newish.repository.#block.end
#cpp_lang_newish.repository.#block.patterns[0].match
#cpp_lang_newish.repository.#block-c.patterns[0].begin
#cpp_lang_newish.repository.#block-c.patterns[0].end
#cpp_lang_newish.repository.#block_innards-c.patterns[6].begin
#cpp_lang_newish.repository.#block_innards-c.patterns[6].end
#cpp_lang_newish.repository.#block_innards-c.patterns[7].begin
#cpp_lang_newish.repository.#block_innards-c.patterns[7].end
#cpp_lang_newish.repository.#c_function_call.begin
#cpp_lang_newish.repository.#c_function_call.end
#cpp_lang_newish.repository.#comments-c.patterns[0].match
#cpp_lang_newish.repository.#comments-c.patterns[1].begin
#cpp_lang_newish.repository.#comments-c.patterns[1].end
#cpp_lang_newish.repository.#comments-c.patterns[2].match
#cpp_lang_newish.repository.#comments-c.patterns[3].begin
#cpp_lang_newish.repository.#comments-c.patterns[3].end
#cpp_lang_newish.repository.#comments-c.patterns[3].patterns[0].begin
#cpp_lang_newish.repository.#comments-c.patterns[3].patterns[0].end
#cpp_lang_newish.repository.#constants.match
#cpp_lang_newish.repository.#constructor.patterns[0].begin
#cpp_lang_newish.repository.#constructor.patterns[0].end
#cpp_lang_newish.repository.#constructor.patterns[1].begin
#cpp_lang_newish.repository.#constructor.patterns[1].end
#cpp_lang_newish.repository.#disabled.begin
#cpp_lang_newish.repository.#disabled.end
#cpp_lang_newish.repository.#function-call-innards-c.patterns[5].begin
#cpp_lang_newish.repository.#function-call-innards-c.patterns[5].end
#cpp_lang_newish.repository.#function-call-innards-c.patterns[6].begin
#cpp_lang_newish.repository.#function-call-innards-c.patterns[6].end
#cpp_lang_newish.repository.#function-call-innards-c.patterns[7].begin
#cpp_lang_newish.repository.#function-call-innards-c.patterns[7].end
#cpp_lang_newish.repository.#function-innards-c.patterns[4].begin
#cpp_lang_newish.repository.#function-innards-c.patterns[4].end
#cpp_lang_newish.repository.#function-innards-c.patterns[5].begin
#cpp_lang_newish.repository.#function-innards-c.patterns[5].end
#cpp_lang_newish.repository.#line_continuation_character.patterns[0].match
#cpp_lang_newish.repository.#literal_numeric_seperator.match
#cpp_lang_newish.repository.#memory_operators.match
#cpp_lang_newish.repository.#number_literal.match
#cpp_lang_newish.repository.#operator_overload.begin
#cpp_lang_newish.repository.#operator_overload.end
#cpp_lang_newish.repository.#operators.patterns[0].match
#cpp_lang_newish.repository.#operators.patterns[1].match
#cpp_lang_newish.repository.#operators.patterns[2].match
#cpp_lang_newish.repository.#operators.patterns[3].match
#cpp_lang_newish.repository.#operators.patterns[4].match
#cpp_lang_newish.repository.#operators.patterns[5].match
#cpp_lang_newish.repository.#operators.patterns[6].match
#cpp_lang_newish.repository.#operators.patterns[7].match
#cpp_lang_newish.repository.#operators.patterns[8].match
#cpp_lang_newish.repository.#operators.patterns[9].match
#cpp_lang_newish.repository.#operators.patterns[10].match
#cpp_lang_newish.repository.#operators.patterns[11].begin
#cpp_lang_newish.repository.#operators.patterns[11].end
#cpp_lang_newish.repository.#parens-block-c.begin
#cpp_lang_newish.repository.#parens-block-c.end
#cpp_lang_newish.repository.#parens-block-c.patterns[1].match
#cpp_lang_newish.repository.#parens-c.begin
#cpp_lang_newish.repository.#parens-c.end
#cpp_lang_newish.repository.#pragma-mark.match
#cpp_lang_newish.repository.#preprocessor-rule-conditional.patterns[0].begin
#cpp_lang_newish.repository.#preprocessor-rule-conditional.patterns[0].end
#cpp_lang_newish.repository.#preprocessor-rule-conditional.patterns[0].patterns[0].begin
#cpp_lang_newish.repository.#preprocessor-rule-conditional.patterns[0].patterns[0].end
#cpp_lang_newish.repository.#preprocessor-rule-conditional.patterns[0].patterns[4].begin
#cpp_lang_newish.repository.#preprocessor-rule-conditional.patterns[0].patterns[4].end
#cpp_lang_newish.repository.#preprocessor-rule-conditional.patterns[1].match
#cpp_lang_newish.repository.#preprocessor-rule-conditional-block.patterns[0].begin
#cpp_lang_newish.repository.#preprocessor-rule-conditional-block.patterns[0].end
#cpp_lang_newish.repository.#preprocessor-rule-conditional-block.patterns[0].patterns[0].begin
#cpp_lang_newish.repository.#preprocessor-rule-conditional-block.patterns[0].patterns[0].end
#cpp_lang_newish.repository.#preprocessor-rule-conditional-block.patterns[0].patterns[4].begin
#cpp_lang_newish.repository.#preprocessor-rule-conditional-block.patterns[0].patterns[4].end
#cpp_lang_newish.repository.#preprocessor-rule-conditional-block.patterns[1].match
#cpp_lang_newish.repository.#preprocessor-rule-conditional-line.patterns[0].match
#cpp_lang_newish.repository.#preprocessor-rule-conditional-line.patterns[1].match
#cpp_lang_newish.repository.#preprocessor-rule-conditional-line.patterns[5].begin
#cpp_lang_newish.repository.#preprocessor-rule-conditional-line.patterns[5].end
#cpp_lang_newish.repository.#preprocessor-rule-conditional-line.patterns[8].match
#cpp_lang_newish.repository.#preprocessor-rule-conditional-line.patterns[10].begin
#cpp_lang_newish.repository.#preprocessor-rule-conditional-line.patterns[10].end
#cpp_lang_newish.repository.#preprocessor-rule-define-line-blocks.patterns[0].begin
#cpp_lang_newish.repository.#preprocessor-rule-define-line-blocks.patterns[0].end
#cpp_lang_newish.repository.#preprocessor-rule-define-line-contents.patterns[1].match
#cpp_lang_newish.repository.#preprocessor-rule-define-line-contents.patterns[2].begin
#cpp_lang_newish.repository.#preprocessor-rule-define-line-contents.patterns[2].end
#cpp_lang_newish.repository.#preprocessor-rule-define-line-contents.patterns[3].match
#cpp_lang_newish.repository.#preprocessor-rule-define-line-contents.patterns[4].match
#cpp_lang_newish.repository.#preprocessor-rule-define-line-contents.patterns[5].begin
#cpp_lang_newish.repository.#preprocessor-rule-define-line-contents.patterns[5].end
#cpp_lang_newish.repository.#preprocessor-rule-define-line-contents.patterns[6].begin
#cpp_lang_newish.repository.#preprocessor-rule-define-line-contents.patterns[6].end
#cpp_lang_newish.repository.#preprocessor-rule-define-line-contents.patterns[7].begin
#cpp_lang_newish.repository.#preprocessor-rule-define-line-contents.patterns[7].end
#cpp_lang_newish.repository.#preprocessor-rule-define-line-functions.patterns[6].begin
#cpp_lang_newish.repository.#preprocessor-rule-define-line-functions.patterns[6].end
#cpp_lang_newish.repository.#preprocessor-rule-define-line-functions.patterns[7].begin
#cpp_lang_newish.repository.#preprocessor-rule-define-line-functions.patterns[7].end
#cpp_lang_newish.repository.#preprocessor-rule-disabled.patterns[0].begin
#cpp_lang_newish.repository.#preprocessor-rule-disabled.patterns[0].end
#cpp_lang_newish.repository.#preprocessor-rule-disabled.patterns[0].patterns[0].begin
#cpp_lang_newish.repository.#preprocessor-rule-disabled.patterns[0].patterns[0].end
#cpp_lang_newish.repository.#preprocessor-rule-disabled.patterns[0].patterns[5].begin
#cpp_lang_newish.repository.#preprocessor-rule-disabled.patterns[0].patterns[5].end
#cpp_lang_newish.repository.#preprocessor-rule-disabled.patterns[0].patterns[5].patterns[0].begin
#cpp_lang_newish.repository.#preprocessor-rule-disabled.patterns[0].patterns[5].patterns[0].end
#cpp_lang_newish.repository.#preprocessor-rule-disabled.patterns[0].patterns[6].begin
#cpp_lang_newish.repository.#preprocessor-rule-disabled.patterns[0].patterns[6].end
#cpp_lang_newish.repository.#preprocessor-rule-disabled-block.patterns[0].begin
#cpp_lang_newish.repository.#preprocessor-rule-disabled-block.patterns[0].end
#cpp_lang_newish.repository.#preprocessor-rule-disabled-block.patterns[0].patterns[0].begin
#cpp_lang_newish.repository.#preprocessor-rule-disabled-block.patterns[0].patterns[0].end
#cpp_lang_newish.repository.#preprocessor-rule-disabled-block.patterns[0].patterns[5].begin
#cpp_lang_newish.repository.#preprocessor-rule-disabled-block.patterns[0].patterns[5].end
#cpp_lang_newish.repository.#preprocessor-rule-disabled-block.patterns[0].patterns[5].patterns[0].begin
#cpp_lang_newish.repository.#preprocessor-rule-disabled-block.patterns[0].patterns[5].patterns[0].end
#cpp_lang_newish.repository.#preprocessor-rule-disabled-block.patterns[0].patterns[6].begin
#cpp_lang_newish.repository.#preprocessor-rule-disabled-block.patterns[0].patterns[6].end
#cpp_lang_newish.repository.#preprocessor-rule-disabled-elif.begin
#cpp_lang_newish.repository.#preprocessor-rule-disabled-elif.end
#cpp_lang_newish.repository.#preprocessor-rule-disabled-elif.patterns[0].begin
#cpp_lang_newish.repository.#preprocessor-rule-disabled-elif.patterns[0].end
#cpp_lang_newish.repository.#preprocessor-rule-disabled-elif.patterns[2].begin
#cpp_lang_newish.repository.#preprocessor-rule-disabled-elif.patterns[2].end
#cpp_lang_newish.repository.#preprocessor-rule-enabled.patterns[0].begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled.patterns[0].end
#cpp_lang_newish.repository.#preprocessor-rule-enabled.patterns[0].patterns[0].begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled.patterns[0].patterns[0].end
#cpp_lang_newish.repository.#preprocessor-rule-enabled.patterns[0].patterns[2].begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled.patterns[0].patterns[2].end
#cpp_lang_newish.repository.#preprocessor-rule-enabled.patterns[0].patterns[3].begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled.patterns[0].patterns[3].end
#cpp_lang_newish.repository.#preprocessor-rule-enabled.patterns[0].patterns[4].begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled.patterns[0].patterns[4].end
#cpp_lang_newish.repository.#preprocessor-rule-enabled-block.patterns[0].begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled-block.patterns[0].end
#cpp_lang_newish.repository.#preprocessor-rule-enabled-block.patterns[0].patterns[0].begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled-block.patterns[0].patterns[0].end
#cpp_lang_newish.repository.#preprocessor-rule-enabled-block.patterns[0].patterns[2].begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled-block.patterns[0].patterns[2].end
#cpp_lang_newish.repository.#preprocessor-rule-enabled-block.patterns[0].patterns[3].begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled-block.patterns[0].patterns[3].end
#cpp_lang_newish.repository.#preprocessor-rule-enabled-block.patterns[0].patterns[4].begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled-block.patterns[0].patterns[4].end
#cpp_lang_newish.repository.#preprocessor-rule-enabled-elif.begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled-elif.end
#cpp_lang_newish.repository.#preprocessor-rule-enabled-elif.patterns[0].begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled-elif.patterns[0].end
#cpp_lang_newish.repository.#preprocessor-rule-enabled-elif.patterns[2].begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled-elif.patterns[2].end
#cpp_lang_newish.repository.#preprocessor-rule-enabled-elif.patterns[2].patterns[0].begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled-elif.patterns[2].patterns[0].end
#cpp_lang_newish.repository.#preprocessor-rule-enabled-elif.patterns[2].patterns[1].begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled-elif.patterns[2].patterns[1].end
#cpp_lang_newish.repository.#preprocessor-rule-enabled-elif-block.begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled-elif-block.end
#cpp_lang_newish.repository.#preprocessor-rule-enabled-elif-block.patterns[0].begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled-elif-block.patterns[0].end
#cpp_lang_newish.repository.#preprocessor-rule-enabled-elif-block.patterns[2].begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled-elif-block.patterns[2].end
#cpp_lang_newish.repository.#preprocessor-rule-enabled-elif-block.patterns[2].patterns[0].begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled-elif-block.patterns[2].patterns[0].end
#cpp_lang_newish.repository.#preprocessor-rule-enabled-elif-block.patterns[2].patterns[1].begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled-elif-block.patterns[2].patterns[1].end
#cpp_lang_newish.repository.#preprocessor-rule-enabled-else.begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled-else.end
#cpp_lang_newish.repository.#preprocessor-rule-enabled-else-block.begin
#cpp_lang_newish.repository.#preprocessor-rule-enabled-else-block.end
#cpp_lang_newish.repository.#probably_a_parameter.match
#cpp_lang_newish.repository.#scope_resolution.match
#cpp_lang_newish.repository.#special_block.patterns[0].begin
#cpp_lang_newish.repository.#special_block.patterns[0].end
#cpp_lang_newish.repository.#special_block.patterns[1].begin
#cpp_lang_newish.repository.#special_block.patterns[1].end
#cpp_lang_newish.repository.#special_block.patterns[1].patterns[0].begin
#cpp_lang_newish.repository.#special_block.patterns[1].patterns[0].end
#cpp_lang_newish.repository.#special_block.patterns[1].beginCaptures[2].patterns[0].match
#cpp_lang_newish.repository.#special_block.patterns[1].beginCaptures[2].patterns[1].match
#cpp_lang_newish.repository.#special_block.patterns[2].begin
#cpp_lang_newish.repository.#special_block.patterns[2].end
#cpp_lang_newish.repository.#special_block.patterns[2].patterns[1].begin
#cpp_lang_newish.repository.#special_block.patterns[2].patterns[1].end
#cpp_lang_newish.repository.#special_block.patterns[2].beginCaptures[7].patterns[0].match
#cpp_lang_newish.repository.#special_block.patterns[2].beginCaptures[7].patterns[1].match
#cpp_lang_newish.repository.#special_block.patterns[3].begin
#cpp_lang_newish.repository.#special_block.patterns[3].end
#cpp_lang_newish.repository.#special_block.patterns[3].patterns[0].begin
#cpp_lang_newish.repository.#special_block.patterns[3].patterns[0].end
#cpp_lang_newish.repository.#storage_types_c.patterns[0].match
#cpp_lang_newish.repository.#storage_types_c.patterns[1].match
#cpp_lang_newish.repository.#storage_types_c.patterns[2].match
#cpp_lang_newish.repository.#string_escaped_char-c.patterns[0].match
#cpp_lang_newish.repository.#string_escaped_char-c.patterns[1].match
#cpp_lang_newish.repository.#string_placeholder-c.patterns[0].match
#cpp_lang_newish.repository.#strings.patterns[0].begin
#cpp_lang_newish.repository.#strings.patterns[0].end
#cpp_lang_newish.repository.#strings.patterns[0].patterns[0].match
#cpp_lang_newish.repository.#strings.patterns[0].patterns[1].match
#cpp_lang_newish.repository.#strings.patterns[0].patterns[2].match
#cpp_lang_newish.repository.#strings.patterns[0].patterns[3].match
#cpp_lang_newish.repository.#strings.patterns[1].begin
#cpp_lang_newish.repository.#strings.patterns[1].end
#cpp_lang_newish.repository.#strings-c.patterns[0].begin
#cpp_lang_newish.repository.#strings-c.patterns[0].end
#cpp_lang_newish.repository.#strings-c.patterns[1].begin
#cpp_lang_newish.repository.#strings-c.patterns[1].end
#cpp_lang_newish.repository.#template_call_innards.match
#cpp_lang_newish.repository.#template_call_innards.captures[0].patterns[3].match
#cpp_lang_newish.repository.#template_call_innards.captures[0].patterns[7].match
#cpp_lang_newish.repository.#template_definition.begin
#cpp_lang_newish.repository.#template_definition.end
#cpp_lang_newish.repository.#template_definition_argument.match
#cpp_lang_newish.repository.#vararg_ellipses-c.match
```

### `proto`

```text
#storagetypes.match
```

### `python`

```text
#fregexp-base-expression.patterns[2].match
#fregexp-quantifier.match
```

### `shellscript`

```text
#basic_command_name.match
#basic_command_name.captures[2].patterns[0].match
#basic_command_name.captures[2].patterns[1].match
#block_comment.begin
#block_comment.end
#compound-command.patterns[0].begin
#compound-command.patterns[0].end
#compound-command.patterns[1].begin
#compound-command.patterns[1].end
#inline_comment.match
#inline_comment.captures[3].patterns[0].match
#inline_comment.captures[3].patterns[1].match
#keyword.patterns[0].match
#keyword.patterns[1].match
#line_comment.begin
#line_comment.end
#modifiers.match
#regexp.patterns[0].match
#simple_options.match
#simple_options.captures[0].patterns[0].match
#start_of_command.match
```

### `vue`

```text
#vue-interpolations.patterns[0].begin
#vue-interpolations.patterns[0].end
#vue-interpolations.patterns[0].patterns[0].begin
#vue-interpolations.patterns[0].patterns[0].end
```
