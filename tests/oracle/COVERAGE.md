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
| C | `c` | 159 of 342 | 46.5% | 140 of 341 | 41.1% | 0 | 3 |  |
| C++ | `cpp` | 312 of 912 | 34.2% | 373 of 2679 | 13.9% | 0 | 288 |  |
| C++ | `cpp-macro` | 22 of 611 | 3.6% | 22 of 1645 | 1.3% | 0 | 68 |  |
| C# | `csharp` | 544 of 547 | 99.5% | 515 of 525 | 98.1% | 13 | 0 | yes |
| CSS | `css` | 102 of 222 | 45.9% | 76 of 184 | 41.3% | 0 | 17 |  |
| diff | `diff` | 10 of 16 | 62.5% | 10 of 23 | 43.5% | 0 | 1 |  |
| Dockerfile | `docker` | 8 of 9 | 88.9% | 5 of 13 | 38.5% | 0 | 0 |  |
| ERB | `erb` | 6 of 12 | 50.0% | 6 of 14 | 42.9% | 0 | 8 |  |
| Go | `go` | 201 of 336 | 59.8% | 145 of 250 | 58.0% | 0 | 2 |  |
| GraphQL | `graphql` | 76 of 88 | 86.4% | 83 of 113 | 73.5% | 0 | 7 |  |
| HTML | `html` | 131 of 227 | 57.7% | 162 of 368 | 44.0% | 0 | 2 |  |
| HTML | `html-derivative` | 2 of 3 | 66.7% | 3 of 3 | 100.0% | 0 | 1 |  |
| INI | `ini` | 14 of 15 | 93.3% | 10 of 12 | 83.3% | 0 | 0 |  |
| Java | `java` | 157 of 214 | 73.4% | 102 of 134 | 76.1% | 0 | 19 |  |
| TypeScript | `javascript` | 260 of 534 | 48.7% | 192 of 605 | 31.7% | 0 | 28 |  |
| JSON | `json` | 14 of 23 | 60.9% | 10 of 15 | 66.7% | 0 | 10 |  |
| JSON | `jsonc` | 19 of 23 | 82.6% | 13 of 15 | 86.7% | 0 | 0 |  |
| JSON | `jsonl` | 13 of 23 | 56.5% | 11 of 15 | 73.3% | 0 | 6 |  |
| TypeScript | `jsx` | 201 of 534 | 37.6% | 132 of 605 | 21.8% | 0 | 51 |  |
| Kotlin | `kotlin` | 57 of 65 | 87.7% | 25 of 35 | 71.4% | 0 | 0 |  |
| Less | `less` | 167 of 590 | 28.3% | 121 of 437 | 27.7% | 0 | 8 |  |
| Lua | `lua` | 81 of 152 | 53.3% | 32 of 74 | 43.2% | 0 | 3 |  |
| Make | `make` | 58 of 75 | 77.3% | 23 of 32 | 71.9% | 0 | 0 |  |
| Markdown | `markdown` | 145 of 311 | 46.6% | 158 of 373 | 42.4% | 0 | 8 |  |
| Python | `python` | 163 of 350 | 46.6% | 125 of 372 | 33.6% | 0 | 27 |  |
| Ruby | `ruby` | 168 of 336 | 50.0% | 139 of 293 | 47.4% | 0 | 16 |  |
| Rust | `rust` | 94 of 95 | 98.9% | 97 of 99 | 98.0% | 3 | 0 | yes |
| SCSS | `scss` | 90 of 168 | 53.6% | 81 of 179 | 45.3% | 0 | 10 |  |
| shell | `shellscript` | 119 of 174 | 68.4% | 106 of 199 | 53.3% | 0 | 10 |  |
| SQL | `sql` | 47 of 75 | 62.7% | 42 of 79 | 53.2% | 0 | 4 |  |
| Swift | `swift` | 267 of 448 | 59.6% | 183 of 448 | 40.8% | 0 | 11 |  |
| TOML | `toml` | 40 of 54 | 74.1% | 30 of 53 | 56.6% | 0 | 2 |  |
| TypeScript | `tsx` | 199 of 534 | 37.3% | 142 of 605 | 23.5% | 0 | 64 |  |
| TypeScript | `typescript` | 315 of 515 | 61.2% | 255 of 576 | 44.3% | 0 | 28 |  |
| XML | `xml` | 23 of 32 | 71.9% | 46 of 66 | 69.7% | 0 | 1 |  |
| YAML | `yaml` | 33 of 62 | 53.2% | 22 of 55 | 40.0% | 0 | 14 |  |

## Units no fixture exercises

Each line is an expression's path in its grammar, then the start of its regex, `fuzz` when the fuzz corpus exercises it, and the reason when a waiver covers it.
An expression the fixtures do exercise is listed with the groups they do not, each marked `fuzz` and the waiver's reason likewise.
The groups of an unexercised expression count as unexercised and are not listed.

### `c`

```text
#anon_pattern_10.match  \b((?:int8|int16|int32|int64|uint8|uint16|uint32|uint64|int…
#anon_pattern_11.match  \b(noErr|kNilOptions|kInvalidID|kVariableLengthArray)\b
#anon_pattern_12.match  \b(AbsoluteTime|Boolean|Byte|ByteCount|ByteOffset|BytePtr|C…
#anon_pattern_4.match  \bk[A-Z]\w*\b
#anon_pattern_5.match  \bg[A-Z]\w*\b
#anon_pattern_6.match  \bs[A-Z]\w*\b
#anon_pattern_8.match  \b(u_char|u_short|u_int|u_long|ushort|uint|u_quad_t|quad_t|…
#anon_pattern_9.match  \b(pthread_(?:attr_|cond_|condattr_|mutex_|mutexattr_|once_…
#anon_pattern_range_1.begin  groups 1, 2, 3, 4
#anon_pattern_range_1.beginCaptures[4].patterns[0].match  \*/
#anon_pattern_range_1.beginCaptures[4].patterns[1].match  \*
#anon_pattern_range_2.patterns[0].begin  "
#anon_pattern_range_2.patterns[0].end  "|(?<!\\)(?=\s*\n)
#anon_pattern_range_2.patterns[1].begin  '
#anon_pattern_range_2.patterns[1].end  '|(?<!\\)(?=\s*\n)
#anon_pattern_range_4.begin  ^\s*((#)\s*line)\b
#anon_pattern_range_4.end  (?=/[*/])|(?<!\\)(?=\n)
#backslash_escapes.match  \\(["'?\\abefnprtv]|[0-3][0-7]{0,2}|[4-7]\d?|x\h{0,2}|u\h{0…
#block_comment.patterns[1].begin  \s*+(/\*)
#block_comment.patterns[1].end  \*/
#block_innards.patterns[6].begin  (?=\s)(?<!else|new|return)(?<=\w)\s+(and|and_eq|bitand|bito…
#block_innards.patterns[6].end  \)
#case_statement.begin  groups 1, 2, 3, 4
#case_statement.beginCaptures[4].patterns[0].match  \*/
#case_statement.beginCaptures[4].patterns[1].match  \*
#comments.patterns[0].patterns[0].patterns[0].begin  ^(?>\s*)(//[!/]+)
#comments.patterns[0].patterns[0].patterns[0].end  (?<=\n)(?<!\\\n)
#comments.patterns[0].patterns[0].patterns[0].patterns[1].match  (?<=[!*/\s])[@\\](?:callergraph|callgraph|else|endif|f\$|f\…
#comments.patterns[0].patterns[0].patterns[0].patterns[2].match  ((?<=[!*/\s])[@\\](?:a|em?))\s+(\S+)
#comments.patterns[0].patterns[0].patterns[0].patterns[3].match  ((?<=[!*/\s])[@\\]b)\s+(\S+)
#comments.patterns[0].patterns[0].patterns[0].patterns[4].match  ((?<=[!*/\s])[@\\][cp])\s+(\S+)
#comments.patterns[0].patterns[0].patterns[0].patterns[5].match  (?<=[!*/\s])[@\\](?:a|anchor|[bc]|cite|copybrief|copydetail…
#comments.patterns[0].patterns[0].patterns[0].patterns[6].match  (?<=[!*/\s])[@\\](?:addindex|addtogroup|category|class|defg…
#comments.patterns[0].patterns[0].patterns[0].patterns[7].match  ((?<=[!*/\s])[@\\]param)(?:\s*\[((?:,?\s*(?:in|out)\s*)+)])…
#comments.patterns[0].patterns[0].patterns[0].patterns[7].captures[2].patterns[0].match  in|out
#comments.patterns[0].patterns[0].patterns[0].patterns[8].match  (?<=[!*/\s])[@\\](?:arg|attention|authors??|brief|bug|copyr…
#comments.patterns[0].patterns[0].patterns[0].patterns[9].match  (?<=[!*/\s])[@\\](?:code|cond|docbookonly|dot|htmlonly|inte…
#comments.patterns[0].patterns[0].patterns[0].patterns[10].match  \b[A-Z]+:|@[_a-z]+:
#comments.patterns[0].patterns[0].patterns[1].match  (/\*[!*]+(?=\s))(.+)([!*]*\*/)
#comments.patterns[0].patterns[0].patterns[1].captures[2].patterns[0].match  (?<=[!*/\s])[@\\](?:callergraph|callgraph|else|endif|f\$|f\…
#comments.patterns[0].patterns[0].patterns[1].captures[2].patterns[1].match  ((?<=[!*/\s])[@\\](?:a|em?))\s+(\S+)
#comments.patterns[0].patterns[0].patterns[1].captures[2].patterns[2].match  ((?<=[!*/\s])[@\\]b)\s+(\S+)
#comments.patterns[0].patterns[0].patterns[1].captures[2].patterns[3].match  ((?<=[!*/\s])[@\\][cp])\s+(\S+)
#comments.patterns[0].patterns[0].patterns[1].captures[2].patterns[4].match  (?<=[!*/\s])[@\\](?:a|anchor|[bc]|cite|copybrief|copydetail…
#comments.patterns[0].patterns[0].patterns[1].captures[2].patterns[5].match  (?<=[!*/\s])[@\\](?:addindex|addtogroup|category|class|defg…
#comments.patterns[0].patterns[0].patterns[1].captures[2].patterns[6].match  ((?<=[!*/\s])[@\\]param)(?:\s*\[((?:,?\s*(?:in|out)\s*)+)])…
#comments.patterns[0].patterns[0].patterns[1].captures[2].patterns[6].captures[2].patterns[0].match  in|out
#comments.patterns[0].patterns[0].patterns[1].captures[2].patterns[7].match  (?<=[!*/\s])[@\\](?:arg|attention|authors??|brief|bug|copyr…
#comments.patterns[0].patterns[0].patterns[1].captures[2].patterns[8].match  (?<=[!*/\s])[@\\](?:code|cond|docbookonly|dot|htmlonly|inte…
#comments.patterns[0].patterns[0].patterns[1].captures[2].patterns[9].match  \b[A-Z]+:|@[_a-z]+:
#comments.patterns[0].patterns[0].patterns[2].patterns[0].match  (?<=[!*/\s])[@\\](?:callergraph|callgraph|else|endif|f\$|f\…
#comments.patterns[0].patterns[0].patterns[2].patterns[1].match  ((?<=[!*/\s])[@\\](?:a|em?))\s+(\S+)
#comments.patterns[0].patterns[0].patterns[2].patterns[2].match  ((?<=[!*/\s])[@\\]b)\s+(\S+)
#comments.patterns[0].patterns[0].patterns[2].patterns[3].match  ((?<=[!*/\s])[@\\][cp])\s+(\S+)
#comments.patterns[0].patterns[0].patterns[2].patterns[4].match  (?<=[!*/\s])[@\\](?:a|anchor|[bc]|cite|copybrief|copydetail…
#comments.patterns[0].patterns[0].patterns[2].patterns[5].match  (?<=[!*/\s])[@\\](?:addindex|addtogroup|category|class|defg…
#comments.patterns[0].patterns[0].patterns[2].patterns[6].match  ((?<=[!*/\s])[@\\]param)(?:\s*\[((?:,?\s*(?:in|out)\s*)+)])…
#comments.patterns[0].patterns[0].patterns[2].patterns[6].captures[2].patterns[0].match  in|out
#comments.patterns[0].patterns[0].patterns[2].patterns[7].match  (?<=[!*/\s])[@\\](?:arg|attention|authors??|brief|bug|copyr…
#comments.patterns[0].patterns[0].patterns[2].patterns[8].match  (?<=[!*/\s])[@\\](?:code|cond|docbookonly|dot|htmlonly|inte…
#comments.patterns[0].patterns[0].patterns[2].patterns[9].match  \b[A-Z]+:|@[_a-z]+:
#comments.patterns[0].patterns[0].patterns[5].match  ^// =(\s*.*?)\s*=$\n?
#comments.patterns[0].patterns[0].patterns[6].begin  groups 1
#default_statement.begin  groups 1, 2, 3, 4
#default_statement.beginCaptures[4].patterns[0].match  \*/
#default_statement.beginCaptures[4].patterns[1].match  \*
#disabled.begin  ^\s*#\s*if(n?def)?\b.*$
#disabled.end  ^\s*#\s*endif\b
#inline_comment.patterns[0].patterns[0].match  (/\*)((?>(?:[^*]|(?>\*+)[^/])*)((?>\*+)/))
#inline_comment.patterns[0].patterns[0].captures[3].patterns[0].match  \*/
#inline_comment.patterns[0].patterns[0].captures[3].patterns[1].match  \*
#inline_comment.patterns[0].patterns[1].match  (/\*)((?:[^*]|\*++[^/])*+(\*++/))
#inline_comment.patterns[0].patterns[1].captures[3].patterns[0].match  \*/
#inline_comment.patterns[0].patterns[1].captures[3].patterns[1].match  \*
#inline_comment.patterns[1].match  (/\*)((?:[^*]|\*++[^/])*+(\*++/))
#inline_comment.patterns[1].captures[3].patterns[0].match  \*/
#inline_comment.patterns[1].captures[3].patterns[1].match  \*
#line_comment.patterns[1].begin  \s*+(//)
#line_comment.patterns[1].end  (?<=\n)(?<!\\\n)
#method_access.beginCaptures[4].patterns[2].match  groups 3
#numbers.captures[0].patterns[0].patterns[0].match  \G(0[Xx])(\h(?:\h|((?<=\h)'(?=\h)))*)?((?<=\h)\.|\.(?=\h))(…
#numbers.captures[0].patterns[0].patterns[0].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#numbers.captures[0].patterns[0].patterns[0].captures[5].patterns[0].match  (?<=\h)'(?=\h)
#numbers.captures[0].patterns[0].patterns[0].captures[11].patterns[0].match  (?<=\h)'(?=\h)
#numbers.captures[0].patterns[0].patterns[1].match  groups 3, 6, 8, 9, 10, 11, 12
#numbers.captures[0].patterns[0].patterns[1].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#numbers.captures[0].patterns[0].patterns[1].captures[5].patterns[0].match  (?<=\h)'(?=\h)
#numbers.captures[0].patterns[0].patterns[1].captures[11].patterns[0].match  (?<=\h)'(?=\h)
#numbers.captures[0].patterns[0].patterns[2].match  \G(0[Bb])([01](?:[01]|((?<=\h)'(?=\h)))*)((?:(?:(?:(?:(?:[U…
#numbers.captures[0].patterns[0].patterns[2].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#numbers.captures[0].patterns[0].patterns[3].match  \G(0)((?:[0-7]|((?<=\h)'(?=\h)))+)((?:(?:(?:(?:(?:[Uu]|[Uu]…
#numbers.captures[0].patterns[0].patterns[3].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#numbers.captures[0].patterns[0].patterns[4].match  groups 3, 5, 6, 7, 8, 9
#numbers.captures[0].patterns[0].patterns[4].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#numbers.captures[0].patterns[0].patterns[4].captures[8].patterns[0].match  (?<=\h)'(?=\h)
#numbers.captures[0].patterns[0].patterns[5].match  groups 3, 5, 6, 7, 8
#numbers.captures[0].patterns[0].patterns[5].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#numbers.captures[0].patterns[0].patterns[5].captures[8].patterns[0].match  (?<=\h)'(?=\h)
#numbers.captures[0].patterns[0].patterns[6].match  (?:['.0-9A-Z_a-z]|(?<=[EPep])[-+])+  fuzz
#parens-block.patterns[1].match  (?-im:(?<!:):(?!:))
#pragma-mark.match  ^\s*(((#)\s*pragma\s+mark)\s+(.*))
#predefined_macros.patterns[0].match  \b(__cplusplus|__DATE__|__FILE__|__LINE__|__STDC__|__STDC_H…
#predefined_macros.patterns[1].match  \b__([A-Z_]+)__\b  fuzz
#preprocessor-rule-conditional-block.patterns[1].match  ^\s*#\s*(e(?:lse|lif|ndif))\b
#preprocessor-rule-conditional-line.patterns[1].match  \bdefined\b
#preprocessor-rule-conditional-line.patterns[5].begin  \?
#preprocessor-rule-conditional-line.patterns[5].end  :
#preprocessor-rule-define-line-contents.patterns[6].begin  '
#preprocessor-rule-define-line-contents.patterns[6].end  '|(?<!\\)(?=\s*\n)
#preprocessor-rule-disabled.patterns[0].patterns[5].begin  ^\s*((#)\s*elif)\b
#preprocessor-rule-disabled.patterns[0].patterns[5].end  (?=^\s*((#)\s*e(?:lif|lse|ndif))\b)
#preprocessor-rule-disabled.patterns[0].patterns[5].patterns[0].begin  \G(?=.)(?!/(?:/|\*(?!.*\\\s*\n)))
#preprocessor-rule-disabled.patterns[0].patterns[5].patterns[0].end  (?=//)|(?=/\*(?!.*\\\s*\n))|(?<!\\)(?=\n)
#preprocessor-rule-disabled-block.patterns[0].patterns[5].begin  ^\s*((#)\s*elif)\b
#preprocessor-rule-disabled-block.patterns[0].patterns[5].end  (?=^\s*((#)\s*e(?:lif|lse|ndif))\b)
#preprocessor-rule-disabled-block.patterns[0].patterns[5].patterns[0].begin  \G(?=.)(?!/(?:/|\*(?!.*\\\s*\n)))
#preprocessor-rule-disabled-block.patterns[0].patterns[5].patterns[0].end  (?=//)|(?=/\*(?!.*\\\s*\n))|(?<!\\)(?=\n)
#preprocessor-rule-disabled-elif.begin  ^\s*((#)\s*elif)\b(?=\s*\(*\b0+\b\)*\s*(?:$|//|/\*))
#preprocessor-rule-disabled-elif.end  (?=^\s*((#)\s*e(?:lif|lse|ndif))\b)
#preprocessor-rule-disabled-elif.patterns[0].begin  \G(?=.)(?!/(?:/|\*(?!.*\\\s*\n)))
#preprocessor-rule-disabled-elif.patterns[0].end  (?=//)|(?=/\*(?!.*\\\s*\n))|(?<!\\)(?=\n)
#preprocessor-rule-disabled-elif.patterns[2].begin  \n
#preprocessor-rule-disabled-elif.patterns[2].end  (?=^\s*((#)\s*e(?:lse|lif|ndif))\b)
#preprocessor-rule-enabled.patterns[0].begin  ^\s*((#)\s*if)\b(?=\s*\(*\b0*1\b\)*\s*(?:$|//|/\*))
#preprocessor-rule-enabled.patterns[0].end  ^\s*((#)\s*endif)\b
#preprocessor-rule-enabled.patterns[0].patterns[0].begin  \G(?=.)(?!/(?:/|\*(?!.*\\\s*\n)))
#preprocessor-rule-enabled.patterns[0].patterns[0].end  (?=//)|(?=/\*(?!.*\\\s*\n))|(?=\n)
#preprocessor-rule-enabled.patterns[0].patterns[2].begin  ^\s*((#)\s*else)\b
#preprocessor-rule-enabled.patterns[0].patterns[2].end  (?=^\s*((#)\s*endif)\b)
#preprocessor-rule-enabled.patterns[0].patterns[3].begin  ^\s*((#)\s*elif)\b
#preprocessor-rule-enabled.patterns[0].patterns[3].end  (?=^\s*((#)\s*e(?:lse|lif|ndif))\b)
#preprocessor-rule-enabled.patterns[0].patterns[4].begin  \n
#preprocessor-rule-enabled.patterns[0].patterns[4].end  (?=^\s*((#)\s*e(?:lse|lif|ndif))\b)
#preprocessor-rule-enabled-block.patterns[0].begin  ^\s*((#)\s*if)\b(?=\s*\(*\b0*1\b\)*\s*(?:$|//|/\*))
#preprocessor-rule-enabled-block.patterns[0].end  ^\s*((#)\s*endif)\b
#preprocessor-rule-enabled-block.patterns[0].patterns[0].begin  \G(?=.)(?!/(?:/|\*(?!.*\\\s*\n)))
#preprocessor-rule-enabled-block.patterns[0].patterns[0].end  (?=//)|(?=/\*(?!.*\\\s*\n))|(?=\n)
#preprocessor-rule-enabled-block.patterns[0].patterns[2].begin  ^\s*((#)\s*else)\b
#preprocessor-rule-enabled-block.patterns[0].patterns[2].end  (?=^\s*((#)\s*endif)\b)
#preprocessor-rule-enabled-block.patterns[0].patterns[3].begin  ^\s*((#)\s*elif)\b
#preprocessor-rule-enabled-block.patterns[0].patterns[3].end  (?=^\s*((#)\s*e(?:lse|lif|ndif))\b)
#preprocessor-rule-enabled-block.patterns[0].patterns[4].begin  \n
#preprocessor-rule-enabled-block.patterns[0].patterns[4].end  (?=^\s*((#)\s*e(?:lse|lif|ndif))\b)
#preprocessor-rule-enabled-elif.begin  ^\s*((#)\s*elif)\b(?=\s*\(*\b0*1\b\)*\s*(?:$|//|/\*))
#preprocessor-rule-enabled-elif.end  (?=^\s*((#)\s*endif)\b)
#preprocessor-rule-enabled-elif.patterns[0].begin  \G(?=.)(?!/(?:/|\*(?!.*\\\s*\n)))
#preprocessor-rule-enabled-elif.patterns[0].end  (?=//)|(?=/\*(?!.*\\\s*\n))|(?<!\\)(?=\n)
#preprocessor-rule-enabled-elif.patterns[2].begin  \n
#preprocessor-rule-enabled-elif.patterns[2].end  (?=^\s*((#)\s*endif)\b)
#preprocessor-rule-enabled-elif.patterns[2].patterns[0].begin  ^\s*((#)\s*(else))\b
#preprocessor-rule-enabled-elif.patterns[2].patterns[0].end  (?=^\s*((#)\s*endif)\b)
#preprocessor-rule-enabled-elif.patterns[2].patterns[1].begin  ^\s*((#)\s*(elif))\b
#preprocessor-rule-enabled-elif.patterns[2].patterns[1].end  (?=^\s*((#)\s*e(?:lse|lif|ndif))\b)
#preprocessor-rule-enabled-elif-block.begin  ^\s*((#)\s*elif)\b(?=\s*\(*\b0*1\b\)*\s*(?:$|//|/\*))
#preprocessor-rule-enabled-elif-block.end  (?=^\s*((#)\s*endif)\b)
#preprocessor-rule-enabled-elif-block.patterns[0].begin  \G(?=.)(?!/(?:/|\*(?!.*\\\s*\n)))
#preprocessor-rule-enabled-elif-block.patterns[0].end  (?=//)|(?=/\*(?!.*\\\s*\n))|(?<!\\)(?=\n)
#preprocessor-rule-enabled-elif-block.patterns[2].begin  \n
#preprocessor-rule-enabled-elif-block.patterns[2].end  (?=^\s*((#)\s*endif)\b)
#preprocessor-rule-enabled-elif-block.patterns[2].patterns[0].begin  ^\s*((#)\s*(else))\b
#preprocessor-rule-enabled-elif-block.patterns[2].patterns[0].end  (?=^\s*((#)\s*endif)\b)
#preprocessor-rule-enabled-elif-block.patterns[2].patterns[1].begin  ^\s*((#)\s*(elif))\b
#preprocessor-rule-enabled-elif-block.patterns[2].patterns[1].end  (?=^\s*((#)\s*e(?:lse|lif|ndif))\b)
#storage_types.patterns[3].begin  \b(__asm__|asm)\b\s*((?:volatile)?)
#storage_types.patterns[3].end  (?!\G)
#storage_types.patterns[3].patterns[0].match  ^((?:(?>\s+)|(/\*)((?>(?:[^*]|(?>\*+)[^/])*)((?>\*+)/)))+?|…
#storage_types.patterns[3].patterns[0].captures[4].patterns[0].match  \*/
#storage_types.patterns[3].patterns[0].captures[4].patterns[1].match  \*
#storage_types.patterns[3].patterns[2].begin  (((?:(?>\s+)|(/\*)((?>(?:[^*]|(?>\*+)[^/])*)((?>\*+)/)))+?|…
#storage_types.patterns[3].patterns[2].end  (\))
#storage_types.patterns[3].patterns[2].patterns[0].begin  (R?)(")
#storage_types.patterns[3].patterns[2].patterns[0].end  (")
#storage_types.patterns[3].patterns[2].patterns[1].begin  (\()
#storage_types.patterns[3].patterns[2].patterns[1].end  (\))
#storage_types.patterns[3].patterns[2].patterns[2].match  \[((?:(?>\s+)|(/\*)((?>(?:[^*]|(?>\*+)[^/])*)((?>\*+)/)))+?…
#storage_types.patterns[3].patterns[2].patterns[2].captures[4].patterns[0].match  \*/
#storage_types.patterns[3].patterns[2].patterns[2].captures[4].patterns[1].match  \*
#storage_types.patterns[3].patterns[2].patterns[2].captures[9].patterns[0].match  \*/
#storage_types.patterns[3].patterns[2].patterns[2].captures[9].patterns[1].match  \*
#storage_types.patterns[3].patterns[2].patterns[3].match  :
#storage_types.patterns[3].patterns[2].beginCaptures[5].patterns[0].match  \*/
#storage_types.patterns[3].patterns[2].beginCaptures[5].patterns[1].match  \*
#string_escaped_char.patterns[1].match  \\.  fuzz
#string_placeholder.patterns[1].match  (%)(?!"\s*(PRI|SCN))
#switch_conditional_parentheses.begin  groups 1, 2, 3, 4
#switch_conditional_parentheses.beginCaptures[4].patterns[0].match  \*/
#switch_conditional_parentheses.beginCaptures[4].patterns[1].match  \*
#switch_statement.begin  groups 2, 3, 4, 5
#switch_statement.patterns[2].begin  (?<=}|%>|\?\?>)[\n\s]*
#switch_statement.patterns[2].end  [\n\s]*(?=;)
#switch_statement.beginCaptures[5].patterns[0].match  \*/
#switch_statement.beginCaptures[5].patterns[1].match  \*
#vararg_ellipses.match  (?<!\.)\.\.\.(?!\.)
```

### `cpp`

```text
#access_control_keywords.match  groups 1, 2
#access_control_keywords.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#alignas_attribute.begin  alignas\(
#alignas_attribute.end  \)
#alignas_attribute.patterns[1].begin  \(
#alignas_attribute.patterns[1].end  \)
#alignas_attribute.patterns[2].match  (using)\s+((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A…
#alignas_attribute.patterns[3].match  ,
#alignas_attribute.patterns[4].match  :
#alignas_attribute.patterns[5].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#alignas_attribute.patterns[6].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#assembly.begin  \b(__asm__|asm)\b\s+{0,1}((?:volatile)?)  fuzz
#assembly.end  (?!\G)  fuzz
#assembly.patterns[0].match  ^((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\…
#assembly.patterns[2].begin  ((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\W…  fuzz
#assembly.patterns[2].end  \)  fuzz
#assembly.patterns[2].patterns[0].begin  (R?)(")  fuzz
#assembly.patterns[2].patterns[0].end  "  fuzz
#assembly.patterns[2].patterns[1].begin  \(  fuzz
#assembly.patterns[2].patterns[1].end  \)  fuzz
#assembly.patterns[2].patterns[2].match  \[((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=…
#assembly.patterns[2].patterns[3].match  :  fuzz
#backslash_escapes.match  \\(["'?\\abefnprtv]|[0-3][0-7]{0,2}|[4-7]\d?|x\h{0,2}|u\h{0…
#builtin_storage_type_initilizer.begin  \s*+(?<!\w)(?:(?:(?:(unsigned|wchar_t|double|signed|short|f…  fuzz
#builtin_storage_type_initilizer.end  \)  fuzz
#case_statement.begin  groups 1, 2, 3, 4
#class_block.begin  groups 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 16, 17, 18, 19, 20 (fuzz)
#class_block.patterns[2].end  \s*(?=;)
#class_block.beginCaptures[11].patterns[0].match  ((?<!\w)final(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/…
#class_block.beginCaptures[11].patterns[1].match  groups 2, 3, 4, 5, 6, 7, 8, 9, 10
#class_block.beginCaptures[11].patterns[2].match  DLLEXPORT
#comments.patterns[0].patterns[1].match  (?<=[!*/\s])[@\\](?:callergraph|callgraph|else|endif|f\$|f\…  fuzz
#comments.patterns[0].patterns[2].match  ((?<=[!*/\s])[@\\](?:a|em?))\s+(\S+)
#comments.patterns[0].patterns[3].match  ((?<=[!*/\s])[@\\]b)\s+(\S+)
#comments.patterns[0].patterns[4].match  ((?<=[!*/\s])[@\\][cp])\s+(\S+)  fuzz
#comments.patterns[0].patterns[6].match  (?<=[!*/\s])[@\\](?:addindex|addtogroup|category|class|defg…
#comments.patterns[0].patterns[7].match  ((?<=[!*/\s])[@\\]param)(?:\s*\[((?:,?\s+{0,1}(?:in|out)\s+…
#comments.patterns[0].patterns[7].captures[2].patterns[0].match  in|out
#comments.patterns[0].patterns[7].captures[3].patterns[0].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#comments.patterns[0].patterns[7].captures[3].patterns[1].match  ,
#comments.patterns[0].patterns[8].match  (?<=[!*/\s])[@\\](?:arg|attention|authors??|brief|bug|copyr…
#comments.patterns[0].patterns[10].match  \b[A-Z]+:|@[_a-z]+:
#comments.patterns[1].captures[2].patterns[0].match  (?<=[!*/\s])[@\\](?:callergraph|callgraph|else|endif|f\$|f\…
#comments.patterns[1].captures[2].patterns[1].match  ((?<=[!*/\s])[@\\](?:a|em?))\s+(\S+)
#comments.patterns[1].captures[2].patterns[2].match  ((?<=[!*/\s])[@\\]b)\s+(\S+)
#comments.patterns[1].captures[2].patterns[3].match  ((?<=[!*/\s])[@\\][cp])\s+(\S+)
#comments.patterns[1].captures[2].patterns[5].match  (?<=[!*/\s])[@\\](?:addindex|addtogroup|category|class|defg…
#comments.patterns[1].captures[2].patterns[6].match  ((?<=[!*/\s])[@\\]param)(?:\s*\[((?:,?\s+{0,1}(?:in|out)\s+…
#comments.patterns[1].captures[2].patterns[6].captures[2].patterns[0].match  in|out
#comments.patterns[1].captures[2].patterns[6].captures[3].patterns[0].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#comments.patterns[1].captures[2].patterns[6].captures[3].patterns[1].match  ,
#comments.patterns[1].captures[2].patterns[7].match  (?<=[!*/\s])[@\\](?:arg|attention|authors??|brief|bug|copyr…  fuzz
#comments.patterns[1].captures[2].patterns[8].match  (?<=[!*/\s])[@\\](?:code|cond|docbookonly|dot|htmlonly|inte…
#comments.patterns[1].captures[2].patterns[9].match  \b[A-Z]+:|@[_a-z]+:
#comments.patterns[2].patterns[1].match  ((?<=[!*/\s])[@\\](?:a|em?))\s+(\S+)  fuzz
#comments.patterns[2].patterns[2].match  ((?<=[!*/\s])[@\\]b)\s+(\S+)  fuzz
#comments.patterns[2].patterns[6].match  groups 2 (fuzz), 5, 6
#comments.patterns[2].patterns[6].captures[2].patterns[0].match  in|out  fuzz
#comments.patterns[2].patterns[6].captures[3].patterns[1].match  ,
#comments.patterns[2].patterns[9].match  \b[A-Z]+:|@[_a-z]+:  fuzz
#constructor_bracket_call.match  groups 2, 3, 4, 5, 6, 7 (fuzz), 8, 10, 11, 13, 14, 16, 17, 18 (fuzz), 20, 21
#constructor_bracket_call.captures[1].patterns[0].match  ::  fuzz
#constructor_bracket_call.captures[1].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)  fuzz
#constructor_bracket_call.captures[4].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#constructor_bracket_call.captures[6].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#constructor_bracket_call.captures[7].patterns[0].match  ::  fuzz
#constructor_bracket_call.captures[7].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…  fuzz
#constructor_bracket_call.captures[11].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#constructor_bracket_call.captures[14].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#constructor_bracket_call.captures[21].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#constructor_inline.begin  groups 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14
#constructor_inline.patterns[0].patterns[1].match  (=)((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<…
#constructor_inline.patterns[0].patterns[3].patterns[0].begin  groups 2, 4, 5, 6
#constructor_inline.patterns[0].patterns[3].patterns[1].begin  ((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\…
#constructor_inline.patterns[0].patterns[3].patterns[1].end  }
#constructor_inline.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#constructor_inline.patterns[2].end  \s*(?=;)
#constructor_root.begin  \s*+((?:__(?:cdec|clrcal|stdcal|fastcal|thiscal|vectorcal)l…
#constructor_root.end  (?<=}|%>|\?\?>)|(?=[];=>\[])
#constructor_root.patterns[0].begin  \G ?
#constructor_root.patterns[0].end  \{|<%|\?\?<|(?=;)
#constructor_root.patterns[0].patterns[1].match  (=)((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<…
#constructor_root.patterns[0].patterns[3].begin  :
#constructor_root.patterns[0].patterns[3].end  (?=\{)
#constructor_root.patterns[0].patterns[3].patterns[0].begin  ((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\…
#constructor_root.patterns[0].patterns[3].patterns[0].end  \)
#constructor_root.patterns[0].patterns[3].patterns[1].begin  ((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\…
#constructor_root.patterns[0].patterns[3].patterns[1].end  }
#constructor_root.patterns[0].patterns[3].patterns[2].match  ,
#constructor_root.patterns[0].patterns[4].begin  \(
#constructor_root.patterns[0].patterns[4].end  \)
#constructor_root.patterns[1].begin  (?<=\{|<%|\?\?<)
#constructor_root.patterns[1].end  }|%>|\?\?>
#constructor_root.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#constructor_root.patterns[2].end  \s*(?=;)
#constructor_root.beginCaptures[6].patterns[0].match  ::
#constructor_root.beginCaptures[6].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#constructor_root.beginCaptures[9].patterns[0].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#constructor_root.beginCaptures[9].patterns[1].match  (?<=:)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?…
#constructor_root.beginCaptures[9].patterns[2].match  ::
#control_flow_keywords.match  groups 1, 2
#control_flow_keywords.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#cpp_attributes.begin  \[\[  fuzz
#cpp_attributes.end  ]]  fuzz
#cpp_attributes.patterns[1].begin  \(
#cpp_attributes.patterns[1].end  \)
#cpp_attributes.patterns[2].match  (using)\s+((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A…
#cpp_attributes.patterns[3].match  ,
#cpp_attributes.patterns[4].match  :
#cpp_attributes.patterns[5].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#cpp_attributes.patterns[6].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…  fuzz
#curly_initializer.begin  (\s*+((?:(?:(?:\[\[.*?]]|__attribute(?:__)?\s*\(\s*\(.*?\)\…  fuzz
#curly_initializer.end  }  fuzz
#curly_initializer.patterns[0].begin  (\.)((?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:…  fuzz
#curly_initializer.patterns[0].end  (,)|(?=})  fuzz
#curly_initializer.beginCaptures[1].patterns[0].match  ::
#curly_initializer.beginCaptures[1].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#curly_initializer.beginCaptures[1].patterns[9].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…  fuzz
#curly_initializer.beginCaptures[11].patterns[0].match  ::
#curly_initializer.beginCaptures[11].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#decltype.begin  groups 2, 3, 4, 5
#default_statement.begin  groups 1, 2, 3, 4
#destructor_inline.begin  groups 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 13, 14
#destructor_inline.patterns[0].patterns[1].match  groups 2, 3, 4, 5, 7
#destructor_inline.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#destructor_inline.patterns[2].end  \s*(?=;)
#destructor_root.begin  ((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\W…
#destructor_root.end  (?<=}|%>|\?\?>)|(?=[];=>\[])
#destructor_root.patterns[0].begin  \G ?
#destructor_root.patterns[0].end  \{|<%|\?\?<|(?=;)
#destructor_root.patterns[0].patterns[1].match  (=)((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<…
#destructor_root.patterns[0].patterns[2].begin  \(
#destructor_root.patterns[0].patterns[2].end  \)
#destructor_root.patterns[1].begin  (?<=\{|<%|\?\?<)
#destructor_root.patterns[1].end  }|%>|\?\?>
#destructor_root.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#destructor_root.patterns[2].end  \s*(?=;)
#destructor_root.beginCaptures[10].patterns[0].match  ::
#destructor_root.beginCaptures[10].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#destructor_root.beginCaptures[13].patterns[0].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#destructor_root.beginCaptures[13].patterns[1].match  (?<=:)~(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#destructor_root.beginCaptures[13].patterns[2].match  ::
#diagnostic.begin  groups 2 (fuzz), 3 (fuzz), 4 (fuzz), 5 (fuzz)
#diagnostic.patterns[1].begin  "  fuzz
#diagnostic.patterns[1].end  (")|(?<!\\)(?:(?=\n)|(?<=(?:^|[^\\])\n)(?=$))  fuzz
#diagnostic.patterns[2].begin  '
#diagnostic.patterns[2].end  (')|(?<!\\)(?:(?=\n)|(?<=(?:^|[^\\])\n)(?=$))
#emacs_file_banner.match  ^(?:(\s+{0,1}((//)\s+{0,1}([#*/;=C~]++(?![#*/;=C~]))\s+{0,1…  fuzz
#enum_block.begin  groups 3, 5 (fuzz), 6, 7, 8, 10, 11, 13, 14, 15, 16, 17 (fuzz)
#enum_block.end  groups 2 (fuzz)
#enum_block.patterns[2].begin  (?<=}|%>|\?\?>)\s*  fuzz
#enum_block.patterns[2].end  \s*(?=;)  fuzz
#enumerator_list.match  groups 2
#exception_keywords.match  (((?:\s*+/\*(?:[^*]++|\*+(?!/))*+\*/\s*+)+)|\s++|(?<=\W)|(?…
#exception_keywords.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#extern_block.begin  groups 1, 2, 3, 4
#extern_block.end  groups 1
#extern_block.patterns[0].end  groups 0 (fuzz)
#extern_block.patterns[1].begin  (?<=\{|<%|\?\?<)  fuzz
#extern_block.patterns[1].end  }|%>|\?\?>
#extern_block.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#extern_block.patterns[2].end  \s*(?=;)
#function_call.patterns[0].begin  groups 1 (fuzz), 2, 3, 6, 7, 8, 9, 10, 12, 13, 14
#function_call.patterns[1].begin  groups 6, 7, 8, 9, 12, 13, 14
#function_definition.begin  groups 1, 2, 3, 4, 5, 6, 9, 10, 11, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22, 24, 26, 27, 28, 29, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 50, 51, 54, 55, 56, 57
#function_definition.patterns[0].patterns[2].match  groups 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17, 19, 20, 21, 22
#function_definition.patterns[0].patterns[2].captures[6].patterns[0].match  ::
#function_definition.patterns[0].patterns[2].captures[6].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#function_definition.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#function_definition.patterns[2].end  \s*(?=;)
#function_definition.beginCaptures[7].patterns[0].match  groups 2, 3, 4, 5
#function_definition.beginCaptures[13].patterns[0].match  ::
#function_definition.beginCaptures[13].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#function_definition.beginCaptures[31].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#function_pointer.begin  groups 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14, 15, 16, 17, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 35, 36, 37
#function_pointer.end  groups 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14
#function_pointer.beginCaptures[1].patterns[0].match  ::
#function_pointer.beginCaptures[1].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#function_pointer.beginCaptures[1].patterns[9].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#function_pointer.beginCaptures[11].patterns[0].match  ::
#function_pointer.beginCaptures[11].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#function_pointer.beginCaptures[19].patterns[0].match  \*
#function_pointer.beginCaptures[19].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#function_pointer.beginCaptures[19].patterns[2].match  &
#function_pointer_parameter.begin  groups 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14, 15, 16, 17, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 35, 36, 37
#function_pointer_parameter.end  groups 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14
#function_pointer_parameter.beginCaptures[1].patterns[0].match  ::
#function_pointer_parameter.beginCaptures[1].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#function_pointer_parameter.beginCaptures[1].patterns[9].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#function_pointer_parameter.beginCaptures[11].patterns[0].match  ::
#function_pointer_parameter.beginCaptures[11].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#function_pointer_parameter.beginCaptures[19].patterns[0].match  \*
#function_pointer_parameter.beginCaptures[19].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#function_pointer_parameter.beginCaptures[19].patterns[2].match  &
#gcc_attributes.begin  __attribute(?:__)?\s*\(\s*\(  fuzz
#gcc_attributes.end  \)\s*\)  fuzz
#gcc_attributes.patterns[1].begin  \(
#gcc_attributes.patterns[1].end  \)
#gcc_attributes.patterns[2].match  (using)\s+((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A…
#gcc_attributes.patterns[3].match  ,
#gcc_attributes.patterns[4].match  :
#gcc_attributes.patterns[5].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#gcc_attributes.patterns[6].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…  fuzz
#goto_statement.match  (((?:\s*+/\*(?:[^*]++|\*+(?!/))*+\*/\s*+)+)|\s++|(?<=\W)|(?…  fuzz
#goto_statement.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#goto_statement.captures[5].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#include.match  groups 1 (fuzz), 2 (fuzz), 9, 10, 14, 15, 16, 17, 18, 19, 20, 21, 22
#include.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+  fuzz
#include.captures[10].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#include.captures[15].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#include.captures[18].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#include.captures[20].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#include.captures[22].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#inheritance_context.patterns[1].match  ,  fuzz
#inheritance_context.patterns[3].match  (?<!\w)virtual(?!\w)
#inheritance_context.patterns[4].match  groups 2, 3, 4, 5, 6, 8, 10, 11
#inheritance_context.patterns[4].captures[1].patterns[0].match  ::
#inheritance_context.patterns[4].captures[1].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#inheritance_context.patterns[4].captures[4].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#inheritance_context.patterns[4].captures[6].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#inheritance_context.patterns[4].captures[11].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#inline_builtin_storage_type.match  groups 3 (fuzz)
#inline_comment.match  (/\*)((?:[^*]++|\*+(?!/))*+(\*/))  fuzz
#invalid_comment_end.match  \*/  fuzz
#label.match  (((?:\s*+/\*(?:[^*]++|\*+(?!/))*+\*/\s*+)+)|\s++|(?<=\W)|(?…  fuzz
#label.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#label.captures[5].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#lambdas.begin  groups 5, 6, 7, 8
#lambdas.patterns[1].match  (?<!\w)(?:constexpr|consteval|mutable)(?!\w)  fuzz
#lambdas.beginCaptures[2].patterns[1].match  groups 2, 3, 4, 5, 6, 7
#line.begin  ^((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\…  fuzz
#line.end  (?<!\\)(?:(?=\n)|(?<=(?:^|[^\\])\n)(?=$))  fuzz
#macro.begin  groups 2, 3, 4, 5
#macro.patterns[0].captures[2].patterns[2].match  \.\.\.
#macro_argument.match  ##?(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\…  fuzz
#member_access.match  groups 1, 2, 6 (fuzz), 7
#member_access.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#member_access.captures[11].patterns[0].match  groups 1, 2, 3, 4, 5, 8, 9, 10
#member_access.captures[11].patterns[1].match  (?:((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<…
#memory_operators.match  groups 1, 2
#memory_operators.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#method_access.begin  groups 1, 2, 3, 4, 9
#method_access.beginCaptures[13].patterns[0].match  groups 1, 2, 3, 4, 5, 8, 9
#method_access.beginCaptures[13].patterns[1].match  (?:((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<…
#misc_keywords.match  groups 1, 2
#misc_keywords.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#ms_attributes.begin  __declspec\(  fuzz
#ms_attributes.end  \)  fuzz
#ms_attributes.patterns[1].begin  \(
#ms_attributes.patterns[1].end  \)
#ms_attributes.patterns[2].match  (using)\s+((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A…
#ms_attributes.patterns[3].match  ,
#ms_attributes.patterns[4].match  :
#ms_attributes.patterns[5].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#ms_attributes.patterns[6].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…  fuzz
#namespace_alias.match  (?<!\w)(namespace)\s+((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8…  fuzz
#namespace_block.patterns[0].patterns[4].match  groups 1 (fuzz), 2, 3, 6, 7
#namespace_block.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#namespace_block.patterns[2].end  \s*(?=;)
#normal_variable_assignment.begin  groups 1, 2, 3, 4, 7, 8, 9, 10, 11, 12, 13, 15, 16, 17, 18, 19, 20, 21, 22, 23, 27, 28, 29, 30, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 48, 50, 51, 52, 53, 54, 55
#normal_variable_assignment.beginCaptures[14].patterns[0].match  ::
#normal_variable_assignment.beginCaptures[14].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#normal_variable_assignment.beginCaptures[32].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#normal_variable_declaration.begin  groups 1, 2, 3, 4, 7, 8, 9, 10, 11, 12, 13, 15, 16, 17, 18, 19, 20, 21, 22, 23, 25, 27, 28, 29, 30, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 48, 50, 51, 52, 53
#normal_variable_declaration.beginCaptures[14].patterns[0].match  ::
#normal_variable_declaration.beginCaptures[32].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#number_literal.captures[0].patterns[0].patterns[0].match  \G(0[Xx])(\h(?:\h|((?<=\h)'(?=\h)))*)?((?<=\h)\.|\.(?=\h))(…
#number_literal.captures[0].patterns[0].patterns[0].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#number_literal.captures[0].patterns[0].patterns[0].captures[5].patterns[0].match  (?<=\h)'(?=\h)
#number_literal.captures[0].patterns[0].patterns[0].captures[10].patterns[0].match  (?<=\h)'(?=\h)
#number_literal.captures[0].patterns[0].patterns[1].match  groups 2, 5 (fuzz), 6, 7, 8, 9, 10 (fuzz), 11 (fuzz)
#number_literal.captures[0].patterns[0].patterns[1].captures[1].patterns[0].match  (?<=\h)'(?=\h)
#number_literal.captures[0].patterns[0].patterns[1].captures[4].patterns[0].match  (?<=\h)'(?=\h)  fuzz
#number_literal.captures[0].patterns[0].patterns[1].captures[9].patterns[0].match  (?<=\h)'(?=\h)
#number_literal.captures[0].patterns[0].patterns[2].match  \G(0[Bb])([01](?:[01]|((?<=\h)'(?=\h)))*)((?:[Uu]|[Uu]ll?|[…  fuzz
#number_literal.captures[0].patterns[0].patterns[2].captures[2].patterns[0].match  (?<=\h)'(?=\h)  fuzz
#number_literal.captures[0].patterns[0].patterns[3].match  \G(0)((?:[0-7]|((?<=\h)'(?=\h)))+)((?:[Uu]|[Uu]ll?|[Uu]LL?|…  fuzz
#number_literal.captures[0].patterns[0].patterns[3].captures[2].patterns[0].match  (?<=\h)'(?=\h)  fuzz
#number_literal.captures[0].patterns[0].patterns[4].match  groups 3, 4 (fuzz), 5, 6 (fuzz), 7 (fuzz), 8 (fuzz), 9 (fuzz)
#number_literal.captures[0].patterns[0].patterns[4].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#number_literal.captures[0].patterns[0].patterns[4].captures[7].patterns[0].match  (?<=\h)'(?=\h)
#number_literal.captures[0].patterns[0].patterns[5].match  groups 2 (fuzz), 3 (fuzz), 4, 5, 6 (fuzz), 8 (fuzz)
#number_literal.captures[0].patterns[0].patterns[5].captures[1].patterns[0].match  (?<=\h)'(?=\h)  fuzz
#number_literal.captures[0].patterns[0].patterns[5].captures[6].patterns[0].match  (?<=\h)'(?=\h)
#number_literal.captures[0].patterns[0].patterns[6].match  (?:['.0-9A-Z_a-z]|(?<=[EPep])[-+])+  fuzz
#operator_overload.begin  groups 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 18, 19, 20, 21, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45 (fuzz), 46, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 62 (fuzz), 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77 (fuzz), 78, 79, 80, 81, 82 (fuzz), 83, 84, 85, 86
#operator_overload.patterns[0].patterns[4].match  (=)((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<…
#operator_overload.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#operator_overload.patterns[2].end  \s*(?=;)
#operator_overload.beginCaptures[5].patterns[0].match  ::
#operator_overload.beginCaptures[5].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#operator_overload.beginCaptures[15].patterns[0].match  ::
#operator_overload.beginCaptures[15].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#operator_overload.beginCaptures[23].patterns[0].match  \*
#operator_overload.beginCaptures[23].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#operator_overload.beginCaptures[45].patterns[0].match  ::  fuzz
#operator_overload.beginCaptures[45].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…  fuzz
#operator_overload.beginCaptures[58].patterns[0].match  ::
#operator_overload.beginCaptures[58].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#operator_overload.beginCaptures[63].patterns[0].match  \*
#operator_overload.beginCaptures[63].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#operator_overload.beginCaptures[63].patterns[2].match  &
#operators.patterns[0].begin  groups 2, 3, 4, 5
#operators.patterns[1].begin  ((?<!\w)alignof(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\…  fuzz
#operators.patterns[1].end  \)  fuzz
#operators.patterns[2].begin  ((?<!\w)alignas(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\…  fuzz
#operators.patterns[2].end  \)  fuzz
#operators.patterns[3].begin  ((?<!\w)typeid(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*…  fuzz
#operators.patterns[3].end  \)  fuzz
#operators.patterns[4].begin  ((?<!\w)noexcept(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(…  fuzz
#operators.patterns[4].end  \)  fuzz
#operators.patterns[5].begin  groups 2, 3, 4, 5
#operators.patterns[12].match  groups 2
#over_qualified_types.patterns[0].match  \b(struct)(((?:\s*+/\*(?:[^*]++|\*+(?!/))*+\*/\s*+)+)|\s++|…
#over_qualified_types.patterns[0].captures[3].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[0].captures[6].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[0].captures[7].patterns[0].match  \*
#over_qualified_types.patterns[0].captures[7].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#over_qualified_types.patterns[0].captures[7].patterns[2].match  &
#over_qualified_types.patterns[0].captures[9].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[0].captures[11].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[0].captures[13].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[0].captures[16].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[0].captures[18].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[0].captures[20].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[1].match  \b(enum)(((?:\s*+/\*(?:[^*]++|\*+(?!/))*+\*/\s*+)+)|\s++|(?…
#over_qualified_types.patterns[1].captures[3].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[1].captures[6].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[1].captures[7].patterns[0].match  \*
#over_qualified_types.patterns[1].captures[7].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#over_qualified_types.patterns[1].captures[7].patterns[2].match  &
#over_qualified_types.patterns[1].captures[9].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[1].captures[11].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[1].captures[13].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[1].captures[16].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[1].captures[18].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[1].captures[20].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[2].match  \b(union)(((?:\s*+/\*(?:[^*]++|\*+(?!/))*+\*/\s*+)+)|\s++|(…
#over_qualified_types.patterns[2].captures[3].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[2].captures[6].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[2].captures[7].patterns[0].match  \*
#over_qualified_types.patterns[2].captures[7].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#over_qualified_types.patterns[2].captures[7].patterns[2].match  &
#over_qualified_types.patterns[2].captures[9].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[2].captures[11].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[2].captures[13].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[2].captures[16].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[2].captures[18].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[2].captures[20].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[3].match  \b(class)(((?:\s*+/\*(?:[^*]++|\*+(?!/))*+\*/\s*+)+)|\s++|(…  fuzz
#over_qualified_types.patterns[3].captures[3].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[3].captures[6].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[3].captures[7].patterns[0].match  \*
#over_qualified_types.patterns[3].captures[7].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#over_qualified_types.patterns[3].captures[7].patterns[2].match  &
#over_qualified_types.patterns[3].captures[9].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[3].captures[11].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[3].captures[13].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[3].captures[16].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[3].captures[18].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#over_qualified_types.patterns[3].captures[20].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#parameter.begin  groups 1, 2, 3, 4
#parameter.patterns[5].match  ((?:(thread_local|volatile|register|restrict|static|extern|…
#parameter.patterns[8].match  (?<!\w)(?:struct|class|union|enum)(?!\w)  fuzz
#parameter.patterns[11].match  groups 1, 2, 3, 4, 6, 7, 8, 9
#parameter.patterns[13].begin  \[  fuzz
#parameter.patterns[13].end  ]  fuzz
#parameter.patterns[16].match  groups 1, 2, 3, 4, 5, 6, 7, 8
#parameter.patterns[16].captures[0].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#parameter_or_maybe_value.begin  groups 1, 2, 3, 4
#parameter_or_maybe_value.patterns[7].match  ((?:(thread_local|volatile|register|restrict|static|extern|…
#parameter_or_maybe_value.patterns[11].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#parameter_or_maybe_value.patterns[12].end  groups 1
#parameter_or_maybe_value.patterns[13].match  groups 1, 2, 3, 4, 6, 7, 8, 9
#parameter_or_maybe_value.patterns[18].match  groups 1, 2, 3, 4, 5, 6, 7, 8
#parameter_or_maybe_value.patterns[18].captures[0].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#parentheses.patterns[2].match  (?<!:):(?!:)  fuzz
#pragma.begin  groups 1, 2, 3, 4
#pragma_mark.match  ^((((?:\s*+/\*(?:[^*]++|\*+(?!/))*+\*/\s*+)+)|\s++|(?<=\W)|…  fuzz
#pragma_mark.captures[3].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#preprocessor_conditional_range.begin  groups 1, 2, 3, 4
#preprocessor_conditional_standalone.match  groups 1, 2
#preprocessor_conditional_standalone.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#preprocessor_number_literal.captures[0].patterns[0].patterns[0].match  \G(0[Xx])(\h(?:\h|((?<=\h)'(?=\h)))*)?((?<=\h)\.|\.(?=\h))(…
#preprocessor_number_literal.captures[0].patterns[0].patterns[0].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#preprocessor_number_literal.captures[0].patterns[0].patterns[0].captures[5].patterns[0].match  (?<=\h)'(?=\h)
#preprocessor_number_literal.captures[0].patterns[0].patterns[0].captures[10].patterns[0].match  (?<=\h)'(?=\h)
#preprocessor_number_literal.captures[0].patterns[0].patterns[1].match  \G(?=[.0-9])(?!0[BXbx])([0-9](?:[0-9]|((?<=\h)'(?=\h)))*)?(…  fuzz
#preprocessor_number_literal.captures[0].patterns[0].patterns[1].captures[1].patterns[0].match  (?<=\h)'(?=\h)
#preprocessor_number_literal.captures[0].patterns[0].patterns[1].captures[4].patterns[0].match  (?<=\h)'(?=\h)
#preprocessor_number_literal.captures[0].patterns[0].patterns[1].captures[9].patterns[0].match  (?<=\h)'(?=\h)
#preprocessor_number_literal.captures[0].patterns[0].patterns[2].match  \G(0[Bb])([01](?:[01]|((?<=\h)'(?=\h)))*)((?:[Uu]|[Uu]ll?|[…
#preprocessor_number_literal.captures[0].patterns[0].patterns[2].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#preprocessor_number_literal.captures[0].patterns[0].patterns[3].match  \G(0)((?:[0-7]|((?<=\h)'(?=\h)))+)((?:[Uu]|[Uu]ll?|[Uu]LL?|…  fuzz
#preprocessor_number_literal.captures[0].patterns[0].patterns[3].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#preprocessor_number_literal.captures[0].patterns[0].patterns[4].match  \G(0[Xx])(\h(?:\h|((?<=\h)'(?=\h)))*)(?:(?<!')([Pp])(\+?)(-…  fuzz
#preprocessor_number_literal.captures[0].patterns[0].patterns[4].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#preprocessor_number_literal.captures[0].patterns[0].patterns[4].captures[7].patterns[0].match  (?<=\h)'(?=\h)
#preprocessor_number_literal.captures[0].patterns[0].patterns[5].match  groups 2, 3 (fuzz), 4, 5, 6 (fuzz)
#preprocessor_number_literal.captures[0].patterns[0].patterns[5].captures[1].patterns[0].match  (?<=\h)'(?=\h)
#preprocessor_number_literal.captures[0].patterns[0].patterns[5].captures[6].patterns[0].match  (?<=\h)'(?=\h)
#preprocessor_number_literal.captures[0].patterns[0].patterns[6].match  (?:['.0-9A-Z_a-z]|(?<=[EPep])[-+])+  fuzz
#qualified_type.match  groups 1 (fuzz), 2, 3, 4, 5, 7, 9, 10
#qualified_type.captures[0].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)  fuzz
#qualified_type.captures[3].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#qualified_type.captures[5].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#qualified_type.captures[6].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#qualified_type.captures[10].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#qualifiers_and_specifiers_post_parameters.patterns[0].begin  ((?<!\w)requires(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(…  fuzz
#qualifiers_and_specifiers_post_parameters.patterns[0].end  \)  fuzz
#qualifiers_and_specifiers_post_parameters.patterns[1].captures[1].patterns[0].match  groups 1, 2, 3, 4
#range_for_inner.patterns[0].match  groups 3, 4, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 21, 23, 24, 25, 26, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 42, 43, 44, 45
#range_for_inner.patterns[0].captures[10].patterns[0].match  ::
#range_for_inner.patterns[0].captures[10].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#range_for_inner.patterns[0].captures[28].patterns[0].match  \*
#range_for_inner.patterns[0].captures[28].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#range_for_inner.patterns[1].match  ((?:((?:(?:(?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\…  fuzz
#range_for_inner.patterns[1].captures[10].patterns[0].match  ::
#range_for_inner.patterns[1].captures[10].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#range_for_inner.patterns[1].captures[10].patterns[9].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#range_for_inner.patterns[1].captures[20].patterns[0].match  ::
#range_for_inner.patterns[1].captures[20].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#range_for_inner.patterns[1].captures[28].patterns[0].match  \*
#range_for_inner.patterns[1].captures[28].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#range_for_inner.patterns[1].captures[28].patterns[2].match  &  fuzz
#scope_resolution_function_call_inner_generated.match  groups 2, 3, 8, 9, 10
#scope_resolution_function_definition_inner_generated.match  groups 1, 2, 3, 6, 8, 9, 10
#scope_resolution_inner_generated.match  groups 2, 3, 8, 9, 10
#scope_resolution_namespace_alias_inner_generated.match  ((::)?(?:(?!\b(?:__has_cpp_attribute|reinterpret_cast|atomi…  fuzz
#scope_resolution_namespace_block_inner_generated.match  ((::)?(?:(?!\b(?:__has_cpp_attribute|reinterpret_cast|atomi…  fuzz
#scope_resolution_namespace_using_inner_generated.match  ((::)?(?:(?!\b(?:__has_cpp_attribute|reinterpret_cast|atomi…
#scope_resolution_parameter_inner_generated.match  groups 2, 3, 6, 8, 9, 10
#scope_resolution_template_call_inner_generated.match  groups 2, 3, 8, 9, 10
#scope_resolution_template_definition_inner_generated.match  groups 1, 2, 3, 8, 9, 10
#simple_array_assignment.match  groups 2, 3, 4, 5, 6, 7, 8, 10, 11, 14, 15, 16, 17, 18, 19, 23, 24, 27, 28, 29, 30
#simple_array_assignment.captures[1].patterns[0].match  ::
#simple_array_assignment.captures[1].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#simple_array_assignment.captures[1].patterns[9].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#simple_array_assignment.captures[4].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#simple_array_assignment.captures[6].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#simple_array_assignment.captures[7].patterns[0].match  ::
#simple_array_assignment.captures[7].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#simple_array_assignment.captures[11].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#simple_array_assignment.captures[13].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#simple_array_assignment.captures[13].patterns[2].match  &
#simple_array_assignment.captures[15].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#simple_array_assignment.captures[17].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#simple_array_assignment.captures[19].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#simple_array_assignment.captures[28].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#simple_constructor_call.match  groups 2 (fuzz), 3, 4, 5, 6, 7, 8, 10, 11, 13, 14, 15, 16
#simple_constructor_call.captures[1].patterns[0].match  ::
#simple_constructor_call.captures[1].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#simple_constructor_call.captures[4].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#simple_constructor_call.captures[6].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#simple_constructor_call.captures[7].patterns[0].match  ::
#simple_constructor_call.captures[7].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#simple_constructor_call.captures[11].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#simple_constructor_call.captures[14].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#simple_constructor_call.captures[16].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#standard_declares.patterns[0].match  ((?<!\w)struct(?!\w))(((?:\s*+/\*(?:[^*]++|\*+(?!/))*+\*/\s…
#standard_declares.patterns[0].captures[3].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#standard_declares.patterns[0].captures[5].patterns[0].match  \*
#standard_declares.patterns[0].captures[5].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#standard_declares.patterns[0].captures[5].patterns[2].match  &
#standard_declares.patterns[0].captures[7].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#standard_declares.patterns[0].captures[9].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#standard_declares.patterns[0].captures[11].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#standard_declares.patterns[0].captures[14].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#standard_declares.patterns[1].match  ((?<!\w)union(?!\w))(((?:\s*+/\*(?:[^*]++|\*+(?!/))*+\*/\s*…
#standard_declares.patterns[1].captures[3].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#standard_declares.patterns[1].captures[5].patterns[0].match  \*
#standard_declares.patterns[1].captures[5].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#standard_declares.patterns[1].captures[5].patterns[2].match  &
#standard_declares.patterns[1].captures[7].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#standard_declares.patterns[1].captures[9].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#standard_declares.patterns[1].captures[11].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#standard_declares.patterns[1].captures[14].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#standard_declares.patterns[2].match  ((?<!\w)enum(?!\w))(((?:\s*+/\*(?:[^*]++|\*+(?!/))*+\*/\s*+…
#standard_declares.patterns[2].captures[3].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#standard_declares.patterns[2].captures[5].patterns[0].match  \*
#standard_declares.patterns[2].captures[5].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#standard_declares.patterns[2].captures[5].patterns[2].match  &
#standard_declares.patterns[2].captures[7].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#standard_declares.patterns[2].captures[9].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#standard_declares.patterns[2].captures[11].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#standard_declares.patterns[2].captures[14].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#standard_declares.patterns[3].match  ((?<!\w)class(?!\w))(((?:\s*+/\*(?:[^*]++|\*+(?!/))*+\*/\s*…
#standard_declares.patterns[3].captures[3].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#standard_declares.patterns[3].captures[5].patterns[0].match  \*
#standard_declares.patterns[3].captures[5].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#standard_declares.patterns[3].captures[5].patterns[2].match  &
#standard_declares.patterns[3].captures[7].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#standard_declares.patterns[3].captures[9].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#standard_declares.patterns[3].captures[11].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#standard_declares.patterns[3].captures[14].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#static_assert.begin  groups 1, 2, 3, 4, 6, 7, 8, 9
#storage_specifiers.match  groups 1, 2
#storage_specifiers.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#string_context.patterns[0].begin  groups 1
#string_context.patterns[0].end  groups 2 (fuzz), 3 (fuzz)
#string_context.patterns[0].patterns[0].match  \\(?:u\h{4}|U\h{8})  fuzz
#string_context.patterns[0].patterns[2].match  \\[0-7]{1,3}  fuzz
#string_context.patterns[0].patterns[3].match  (\\x0*\h{2}(?!\h))|(\\x\h*)  fuzz
#string_context.patterns[1].begin  groups 1 (fuzz)
#string_context.patterns[1].end  groups 2 (fuzz), 3 (fuzz)
#string_context.patterns[1].patterns[0].match  (\\x0*\h{2}(?!\h))|(\\x\h*)  fuzz
#string_context.patterns[2].begin  ((?:[LUu]8?)?R)"(?:(?:_r|re)|regex)\(
#string_context.patterns[2].end  \)(?:(?:_r|re)|regex)"
#string_context.patterns[3].begin  ((?:[LUu]8?)?R)"(?:glsl|GLSL)\(
#string_context.patterns[3].end  \)(?:glsl|GLSL)"
#string_context.patterns[5].begin  ((?:u8??|[LU])?R)"(?:([^\t ()\\]{0,16})|([^\t ()\\]*))\(  fuzz
#string_context.patterns[5].end  (\)(\2)(\3)")(?:((?:[A-Za-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z…  fuzz
#string_escaped_char.match  \\(["'?\\abefnprtv]|[0-3][0-7]{0,2}|[4-7]\d?|x\h{0,2}|u\h{0…
#string_escapes_context_c.patterns[1].match  \\.  fuzz
#struct_block.begin  groups 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 16, 17, 18, 19
#struct_block.patterns[2].begin  (?<=}|%>|\?\?>)\s*  fuzz
#struct_block.patterns[2].end  \s*(?=;)
#struct_block.beginCaptures[11].patterns[0].match  ((?<!\w)final(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/…
#struct_block.beginCaptures[11].patterns[1].match  groups 2, 3, 4, 5, 6 (fuzz), 7, 8, 9, 10
#struct_block.beginCaptures[11].patterns[2].match  DLLEXPORT
#struct_block.beginCaptures[11].patterns[3].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#switch_conditional_parentheses.begin  groups 1, 2, 3, 4
#switch_statement.begin  groups 1, 2, 3, 4
#switch_statement.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#switch_statement.patterns[2].end  \s*(?=;)
#template_call_innards.match  groups 2, 3, 4
#template_call_range_helper.patterns[0].match  \b((?<!\w)\.?\d(?:['.0-9A-Z_a-z]|(?<=[EPep])[-+])*)(((?:\s*…  fuzz
#template_call_range_helper.patterns[0].captures[1].patterns[0].begin  (?=.)  fuzz
#template_call_range_helper.patterns[0].captures[1].patterns[0].end  $  fuzz
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[0].match  \G(0[Xx])(\h(?:\h|((?<=\h)'(?=\h)))*)?((?<=\h)\.|\.(?=\h))(…
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[0].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[0].captures[5].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[0].captures[10].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[1].match  \G(?=[.0-9])(?!0[BXbx])([0-9](?:[0-9]|((?<=\h)'(?=\h)))*)?(…
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[1].captures[1].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[1].captures[4].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[1].captures[9].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[2].match  \G(0[Bb])([01](?:[01]|((?<=\h)'(?=\h)))*)((?:[Uu]|[Uu]ll?|[…
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[2].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[3].match  \G(0)((?:[0-7]|((?<=\h)'(?=\h)))+)((?:[Uu]|[Uu]ll?|[Uu]LL?|…  fuzz
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[3].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[4].match  \G(0[Xx])(\h(?:\h|((?<=\h)'(?=\h)))*)(?:(?<!')([Pp])(\+?)(-…
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[4].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[4].captures[7].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[5].match  \G(?=[.0-9])(?!0[BXbx])([0-9](?:[0-9]|((?<=\h)'(?=\h)))*)(?…
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[5].captures[1].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[5].captures[6].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[6].match  (?:['.0-9A-Z_a-z]|(?<=[EPep])[-+])+
#template_call_range_helper.patterns[0].captures[3].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#template_call_range_helper.patterns[1].match  \b((?<!\w)\.?\d(?:['.0-9A-Z_a-z]|(?<=[EPep])[-+])*)(((?:\s*…  fuzz
#template_call_range_helper.patterns[1].captures[1].patterns[0].begin  (?=.)  fuzz
#template_call_range_helper.patterns[1].captures[1].patterns[0].end  $  fuzz
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[0].match  \G(0[Xx])(\h(?:\h|((?<=\h)'(?=\h)))*)?((?<=\h)\.|\.(?=\h))(…
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[0].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[0].captures[5].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[0].captures[10].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[1].match  \G(?=[.0-9])(?!0[BXbx])([0-9](?:[0-9]|((?<=\h)'(?=\h)))*)?(…  fuzz
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[1].captures[1].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[1].captures[4].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[1].captures[9].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[2].match  \G(0[Bb])([01](?:[01]|((?<=\h)'(?=\h)))*)((?:[Uu]|[Uu]ll?|[…
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[2].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[3].match  \G(0)((?:[0-7]|((?<=\h)'(?=\h)))+)((?:[Uu]|[Uu]ll?|[Uu]LL?|…
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[3].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[4].match  \G(0[Xx])(\h(?:\h|((?<=\h)'(?=\h)))*)(?:(?<!')([Pp])(\+?)(-…
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[4].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[4].captures[7].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[5].match  \G(?=[.0-9])(?!0[BXbx])([0-9](?:[0-9]|((?<=\h)'(?=\h)))*)(?…
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[5].captures[1].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[5].captures[6].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[6].match  (?:['.0-9A-Z_a-z]|(?<=[EPep])[-+])+
#template_call_range_helper.patterns[1].captures[3].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#template_definition_argument.match  groups 1, 2, 9, 10, 11, 12, 13, 14, 15
#template_definition_argument.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#template_explicit_instantiation.match  (?<!\w)(?:(extern)\s+)?(template)\s+  fuzz
#the_this_keyword.match  groups 1, 2
#the_this_keyword.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#type_alias.match  groups 3, 6, 7, 9, 10, 11, 12, 13, 17, 18, 22, 23, 24, 25, 26, 27, 28, 29, 30
#type_alias.captures[7].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#type_alias.captures[8].patterns[0].match  ::
#type_alias.captures[8].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#type_alias.captures[11].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#type_alias.captures[13].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#type_alias.captures[18].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#type_alias.captures[21].patterns[0].match  \*
#type_alias.captures[21].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#type_alias.captures[23].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#type_alias.captures[25].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#type_alias.captures[27].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#type_casting_operators.match  groups 1, 2
#type_casting_operators.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#typedef_class.begin  ((?<!\w)typedef(?!\w))\s+{0,1}(?=(?<!\w)class(?!\w))
#typedef_class.end  (?<=;)
#typedef_class.patterns[0].begin  ((?<!\w)class(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/…
#typedef_class.patterns[0].end  (?:(?<=}|%>|\?\?>)\s+{0,1}(;)|(;))|(?=[];=>\[])
#typedef_class.patterns[0].patterns[0].begin  \G ?
#typedef_class.patterns[0].patterns[0].end  \{|<%|\?\?<|(?=;)
#typedef_class.patterns[0].patterns[1].begin  (?<=\{|<%|\?\?<)
#typedef_class.patterns[0].patterns[1].end  }|%>|\?\?>
#typedef_class.patterns[0].patterns[2].begin  (?<=}|%>|\?\?>)\s*
#typedef_class.patterns[0].patterns[2].end  \s*(?=;)
#typedef_class.patterns[0].patterns[2].patterns[0].match  (((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\…
#typedef_class.patterns[0].patterns[2].patterns[0].captures[1].patterns[0].match  \*
#typedef_class.patterns[0].patterns[2].patterns[0].captures[1].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#typedef_class.patterns[0].patterns[2].patterns[0].captures[1].patterns[2].match  &
#typedef_class.patterns[0].patterns[2].patterns[1].match  ,
#typedef_class.patterns[0].beginCaptures[11].patterns[0].match  ((?<!\w)final(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/…
#typedef_class.patterns[0].beginCaptures[11].patterns[1].match  ((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\…
#typedef_class.patterns[0].beginCaptures[11].patterns[2].match  DLLEXPORT
#typedef_class.patterns[0].beginCaptures[11].patterns[3].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#typedef_struct.begin  ((?<!\w)typedef(?!\w))\s+{0,1}(?=(?<!\w)struct(?!\w))
#typedef_struct.end  (?<=;)
#typedef_struct.patterns[0].begin  ((?<!\w)struct(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*…
#typedef_struct.patterns[0].end  (?:(?<=}|%>|\?\?>)\s+{0,1}(;)|(;))|(?=[];=>\[])
#typedef_struct.patterns[0].patterns[0].begin  \G ?
#typedef_struct.patterns[0].patterns[0].end  \{|<%|\?\?<|(?=;)
#typedef_struct.patterns[0].patterns[1].begin  (?<=\{|<%|\?\?<)
#typedef_struct.patterns[0].patterns[1].end  }|%>|\?\?>
#typedef_struct.patterns[0].patterns[2].begin  (?<=}|%>|\?\?>)\s*
#typedef_struct.patterns[0].patterns[2].end  \s*(?=;)
#typedef_struct.patterns[0].patterns[2].patterns[0].match  (((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\…
#typedef_struct.patterns[0].patterns[2].patterns[0].captures[1].patterns[0].match  \*
#typedef_struct.patterns[0].patterns[2].patterns[0].captures[1].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#typedef_struct.patterns[0].patterns[2].patterns[0].captures[1].patterns[2].match  &
#typedef_struct.patterns[0].patterns[2].patterns[1].match  ,
#typedef_struct.patterns[0].beginCaptures[11].patterns[0].match  ((?<!\w)final(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/…
#typedef_struct.patterns[0].beginCaptures[11].patterns[1].match  ((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\…
#typedef_struct.patterns[0].beginCaptures[11].patterns[2].match  DLLEXPORT
#typedef_struct.patterns[0].beginCaptures[11].patterns[3].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#typedef_union.begin  ((?<!\w)typedef(?!\w))\s+{0,1}(?=(?<!\w)union(?!\w))
#typedef_union.end  (?<=;)
#typedef_union.patterns[0].begin  ((?<!\w)union(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/…
#typedef_union.patterns[0].end  (?:(?<=}|%>|\?\?>)\s+{0,1}(;)|(;))|(?=[];=>\[])
#typedef_union.patterns[0].patterns[0].begin  \G ?
#typedef_union.patterns[0].patterns[0].end  \{|<%|\?\?<|(?=;)
#typedef_union.patterns[0].patterns[1].begin  (?<=\{|<%|\?\?<)
#typedef_union.patterns[0].patterns[1].end  }|%>|\?\?>
#typedef_union.patterns[0].patterns[2].begin  (?<=}|%>|\?\?>)\s*
#typedef_union.patterns[0].patterns[2].end  \s*(?=;)
#typedef_union.patterns[0].patterns[2].patterns[0].match  (((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\…
#typedef_union.patterns[0].patterns[2].patterns[0].captures[1].patterns[0].match  \*
#typedef_union.patterns[0].patterns[2].patterns[0].captures[1].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#typedef_union.patterns[0].patterns[2].patterns[0].captures[1].patterns[2].match  &
#typedef_union.patterns[0].patterns[2].patterns[1].match  ,
#typedef_union.patterns[0].beginCaptures[11].patterns[0].match  ((?<!\w)final(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/…
#typedef_union.patterns[0].beginCaptures[11].patterns[1].match  ((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\…
#typedef_union.patterns[0].beginCaptures[11].patterns[2].match  DLLEXPORT
#typedef_union.patterns[0].beginCaptures[11].patterns[3].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#typename.match  groups 2, 3, 4, 5, 7, 8, 9, 10, 11, 15, 16
#typename.captures[3].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#typename.captures[5].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#typename.captures[6].patterns[0].match  ::
#typename.captures[6].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#typename.captures[9].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#typename.captures[11].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#typename.captures[16].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#undef.match  groups 2, 3, 5, 6
#undef.captures[3].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#undef.captures[6].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#union_block.begin  ((?<!\w)union(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/…  fuzz
#union_block.end  (?:(?<=}|%>|\?\?>)\s+{0,1}(;)|(;))|(?=[];=>\[])  fuzz
#union_block.patterns[0].begin  \G ?  fuzz
#union_block.patterns[0].end  \{|<%|\?\?<|(?=;)  fuzz
#union_block.patterns[1].begin  (?<=\{|<%|\?\?<)  fuzz
#union_block.patterns[1].end  }|%>|\?\?>  fuzz
#union_block.patterns[2].begin  (?<=}|%>|\?\?>)\s*  fuzz
#union_block.patterns[2].end  \s*(?=;)  fuzz
#union_block.beginCaptures[11].patterns[0].match  ((?<!\w)final(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/…
#union_block.beginCaptures[11].patterns[1].match  ((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\…
#union_block.beginCaptures[11].patterns[2].match  DLLEXPORT
#union_block.beginCaptures[11].patterns[3].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#using_namespace.begin  groups 3, 4, 5
#variable_assignment.match  (?:((?:(?:((?:\s*+/\*(?:[^*]++|\*+(?!/))*+\*/\s*+)+)|\s++|(…
#variable_assignment.captures[2].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#variable_assignment.captures[4].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#variable_assignment.captures[5].patterns[0].match  ::
#variable_assignment.captures[5].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#variable_assignment.captures[5].patterns[9].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#variable_assignment.captures[8].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#variable_assignment.captures[10].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#variable_assignment.captures[11].patterns[0].match  ::
#variable_assignment.captures[11].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#variable_assignment.captures[15].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#variable_assignment.captures[17].patterns[0].match  \*
#variable_assignment.captures[17].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#variable_assignment.captures[17].patterns[2].match  &
#variable_assignment.captures[19].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#variable_assignment.captures[21].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#variable_assignment.captures[23].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#variable_assignment.captures[30].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#wordlike_operators.patterns[0].match  (?<!\w)(?:noexcept|xor_eq|and_eq|delete|not_eq|bitand|bitor…  fuzz
```

### `cpp-macro`

```text
#alignas_attribute.begin  alignas\(
#alignas_attribute.end  \)|(?=(?<!\\)\n)
#alignas_attribute.patterns[1].begin  \(
#alignas_attribute.patterns[1].end  \)|(?=(?<!\\)\n)
#alignas_attribute.patterns[2].match  (using)\s+((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A…
#alignas_attribute.patterns[3].match  ,
#alignas_attribute.patterns[4].match  :
#alignas_attribute.patterns[5].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#alignas_attribute.patterns[6].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#assembly.begin  \b(__asm__|asm)\b\s+{0,1}((?:volatile)?)
#assembly.end  (?!\G)|(?=(?<!\\)\n)
#assembly.patterns[0].match  ^((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\…
#assembly.patterns[2].begin  ((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\W…
#assembly.patterns[2].end  \)|(?=(?<!\\)\n)
#assembly.patterns[2].patterns[0].begin  (R?)(")
#assembly.patterns[2].patterns[0].end  "|(?=(?<!\\)\n)
#assembly.patterns[2].patterns[1].begin  \(
#assembly.patterns[2].patterns[1].end  \)|(?=(?<!\\)\n)
#assembly.patterns[2].patterns[2].match  \[((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=…
#assembly.patterns[2].patterns[3].match  :
#block_comment.patterns[0].match  [^*]*\n  fuzz
#builtin_storage_type_initilizer.begin  \s*+(?<!\w)(?:(?:(?:(unsigned|wchar_t|double|signed|short|f…
#builtin_storage_type_initilizer.end  \)|(?=(?<!\\)\n)
#case_statement.begin  ((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\W…
#case_statement.end  :|(?=(?<!\\)\n)
#class_block.begin  ((?<!\w)class(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/…  fuzz
#class_block.end  (?:(?:(?<=}|%>|\?\?>)\s+{0,1}(;)|(;))|(?=[];=>\[]))|(?=(?<!…  fuzz
#class_block.patterns[0].begin  \G ?  fuzz
#class_block.patterns[0].end  (?:\{|<%|\?\?<|(?=;))|(?=(?<!\\)\n)  fuzz
#class_block.patterns[1].begin  (?<=\{|<%|\?\?<)  fuzz
#class_block.patterns[1].end  }|%>|\?\?>|(?=(?<!\\)\n)  fuzz
#class_block.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#class_block.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)
#class_block.beginCaptures[11].patterns[0].match  ((?<!\w)final(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/…
#class_block.beginCaptures[11].patterns[1].match  ((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\…  fuzz
#class_block.beginCaptures[11].patterns[2].match  DLLEXPORT
#class_block.beginCaptures[11].patterns[3].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#comments.patterns[0].begin  ^\s+{1,0}(//[!/]+)  fuzz
#comments.patterns[0].end  (?<=\n)(?<!\\\n)|(?=(?<!\\)\n)  fuzz
#comments.patterns[0].patterns[1].match  (?<=[!*/\s])[@\\](?:callergraph|callgraph|else|endif|f\$|f\…
#comments.patterns[0].patterns[2].match  ((?<=[!*/\s])[@\\](?:a|em?))\s+(\S+)
#comments.patterns[0].patterns[3].match  ((?<=[!*/\s])[@\\]b)\s+(\S+)
#comments.patterns[0].patterns[4].match  ((?<=[!*/\s])[@\\][cp])\s+(\S+)
#comments.patterns[0].patterns[5].match  (?<=[!*/\s])[@\\](?:a|anchor|[bc]|cite|copybrief|copydetail…
#comments.patterns[0].patterns[6].match  (?<=[!*/\s])[@\\](?:addindex|addtogroup|category|class|defg…
#comments.patterns[0].patterns[7].match  ((?<=[!*/\s])[@\\]param)(?:\s*\[((?:,?\s+{0,1}(?:in|out)\s+…
#comments.patterns[0].patterns[7].captures[2].patterns[0].match  in|out
#comments.patterns[0].patterns[7].captures[3].patterns[0].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#comments.patterns[0].patterns[7].captures[3].patterns[1].match  ,
#comments.patterns[0].patterns[8].match  (?<=[!*/\s])[@\\](?:arg|attention|authors??|brief|bug|copyr…
#comments.patterns[0].patterns[9].match  (?<=[!*/\s])[@\\](?:code|cond|docbookonly|dot|htmlonly|inte…  fuzz
#comments.patterns[0].patterns[10].match  \b[A-Z]+:|@[_a-z]+:
#comments.patterns[1].match  (/\*[!*]+(?=\s))(.+)([!*]*\*/)
#comments.patterns[1].captures[2].patterns[0].match  (?<=[!*/\s])[@\\](?:callergraph|callgraph|else|endif|f\$|f\…
#comments.patterns[1].captures[2].patterns[1].match  ((?<=[!*/\s])[@\\](?:a|em?))\s+(\S+)
#comments.patterns[1].captures[2].patterns[2].match  ((?<=[!*/\s])[@\\]b)\s+(\S+)
#comments.patterns[1].captures[2].patterns[3].match  ((?<=[!*/\s])[@\\][cp])\s+(\S+)
#comments.patterns[1].captures[2].patterns[4].match  (?<=[!*/\s])[@\\](?:a|anchor|[bc]|cite|copybrief|copydetail…
#comments.patterns[1].captures[2].patterns[5].match  (?<=[!*/\s])[@\\](?:addindex|addtogroup|category|class|defg…
#comments.patterns[1].captures[2].patterns[6].match  ((?<=[!*/\s])[@\\]param)(?:\s*\[((?:,?\s+{0,1}(?:in|out)\s+…
#comments.patterns[1].captures[2].patterns[6].captures[2].patterns[0].match  in|out
#comments.patterns[1].captures[2].patterns[6].captures[3].patterns[0].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#comments.patterns[1].captures[2].patterns[6].captures[3].patterns[1].match  ,
#comments.patterns[1].captures[2].patterns[7].match  (?<=[!*/\s])[@\\](?:arg|attention|authors??|brief|bug|copyr…
#comments.patterns[1].captures[2].patterns[8].match  (?<=[!*/\s])[@\\](?:code|cond|docbookonly|dot|htmlonly|inte…
#comments.patterns[1].captures[2].patterns[9].match  \b[A-Z]+:|@[_a-z]+:
#comments.patterns[2].begin  \s+{1,0}/\*[!*]+(?:(?:\n|$)|(?=\s))
#comments.patterns[2].end  [!*]*\*/|(?=(?<!\\)\n)
#comments.patterns[2].patterns[0].match  (?<=[!*/\s])[@\\](?:callergraph|callgraph|else|endif|f\$|f\…
#comments.patterns[2].patterns[1].match  ((?<=[!*/\s])[@\\](?:a|em?))\s+(\S+)
#comments.patterns[2].patterns[2].match  ((?<=[!*/\s])[@\\]b)\s+(\S+)
#comments.patterns[2].patterns[3].match  ((?<=[!*/\s])[@\\][cp])\s+(\S+)
#comments.patterns[2].patterns[4].match  (?<=[!*/\s])[@\\](?:a|anchor|[bc]|cite|copybrief|copydetail…
#comments.patterns[2].patterns[5].match  (?<=[!*/\s])[@\\](?:addindex|addtogroup|category|class|defg…
#comments.patterns[2].patterns[6].match  ((?<=[!*/\s])[@\\]param)(?:\s*\[((?:,?\s+{0,1}(?:in|out)\s+…
#comments.patterns[2].patterns[6].captures[2].patterns[0].match  in|out
#comments.patterns[2].patterns[6].captures[3].patterns[0].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#comments.patterns[2].patterns[6].captures[3].patterns[1].match  ,
#comments.patterns[2].patterns[7].match  (?<=[!*/\s])[@\\](?:arg|attention|authors??|brief|bug|copyr…
#comments.patterns[2].patterns[8].match  (?<=[!*/\s])[@\\](?:code|cond|docbookonly|dot|htmlonly|inte…
#comments.patterns[2].patterns[9].match  \b[A-Z]+:|@[_a-z]+:
#constructor_inline.begin  ^((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\…
#constructor_inline.end  (?:(?<=}|%>|\?\?>)|(?=[];=>\[]))|(?=(?<!\\)\n)
#constructor_inline.patterns[0].begin  \G ?
#constructor_inline.patterns[0].end  (?:\{|<%|\?\?<|(?=;))|(?=(?<!\\)\n)
#constructor_inline.patterns[0].patterns[1].match  (=)((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<…
#constructor_inline.patterns[0].patterns[3].begin  :
#constructor_inline.patterns[0].patterns[3].end  (?=\{)|(?=(?<!\\)\n)
#constructor_inline.patterns[0].patterns[3].patterns[0].begin  ((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\…
#constructor_inline.patterns[0].patterns[3].patterns[0].end  \)|(?=(?<!\\)\n)
#constructor_inline.patterns[0].patterns[3].patterns[1].begin  ((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\…
#constructor_inline.patterns[0].patterns[3].patterns[1].end  }|(?=(?<!\\)\n)
#constructor_inline.patterns[0].patterns[3].patterns[2].match  ,
#constructor_inline.patterns[0].patterns[4].begin  \(
#constructor_inline.patterns[0].patterns[4].end  \)|(?=(?<!\\)\n)
#constructor_inline.patterns[1].begin  (?<=\{|<%|\?\?<)
#constructor_inline.patterns[1].end  }|%>|\?\?>|(?=(?<!\\)\n)
#constructor_inline.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#constructor_inline.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)
#constructor_root.begin  \s*+((?:__(?:cdec|clrcal|stdcal|fastcal|thiscal|vectorcal)l…
#constructor_root.end  (?:(?<=}|%>|\?\?>)|(?=[];=>\[]))|(?=(?<!\\)\n)
#constructor_root.patterns[0].begin  \G ?
#constructor_root.patterns[0].end  (?:\{|<%|\?\?<|(?=;))|(?=(?<!\\)\n)
#constructor_root.patterns[0].patterns[1].match  (=)((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<…
#constructor_root.patterns[0].patterns[3].begin  :
#constructor_root.patterns[0].patterns[3].end  (?=\{)|(?=(?<!\\)\n)
#constructor_root.patterns[0].patterns[3].patterns[0].begin  ((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\…
#constructor_root.patterns[0].patterns[3].patterns[0].end  \)|(?=(?<!\\)\n)
#constructor_root.patterns[0].patterns[3].patterns[1].begin  ((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\…
#constructor_root.patterns[0].patterns[3].patterns[1].end  }|(?=(?<!\\)\n)
#constructor_root.patterns[0].patterns[3].patterns[2].match  ,
#constructor_root.patterns[0].patterns[4].begin  \(
#constructor_root.patterns[0].patterns[4].end  \)|(?=(?<!\\)\n)
#constructor_root.patterns[1].begin  (?<=\{|<%|\?\?<)
#constructor_root.patterns[1].end  }|%>|\?\?>|(?=(?<!\\)\n)
#constructor_root.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#constructor_root.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)
#constructor_root.beginCaptures[6].patterns[0].match  ::
#constructor_root.beginCaptures[6].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#constructor_root.beginCaptures[9].patterns[0].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#constructor_root.beginCaptures[9].patterns[1].match  (?<=:)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?…
#constructor_root.beginCaptures[9].patterns[2].match  ::
#cpp_attributes.patterns[1].begin  \(
#cpp_attributes.patterns[1].end  \)|(?=(?<!\\)\n)
#cpp_attributes.patterns[2].match  (using)\s+((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A…
#cpp_attributes.patterns[3].match  ,
#curly_initializer.begin  (\s*+((?:(?:(?:\[\[.*?]]|__attribute(?:__)?\s*\(\s*\(.*?\)\…
#curly_initializer.end  }|(?=(?<!\\)\n)
#curly_initializer.patterns[0].begin  (\.)((?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:…
#curly_initializer.patterns[0].end  (?:(,)|(?=}))|(?=(?<!\\)\n)
#curly_initializer.beginCaptures[1].patterns[0].match  ::
#curly_initializer.beginCaptures[1].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#curly_initializer.beginCaptures[1].patterns[9].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#curly_initializer.beginCaptures[11].patterns[0].match  ::
#curly_initializer.beginCaptures[11].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#decltype.begin  ((?<!\w)decltype(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(…
#decltype.end  \)|(?=(?<!\\)\n)
#default_statement.begin  ((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\W…
#default_statement.end  :|(?=(?<!\\)\n)
#destructor_inline.begin  ^((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\…
#destructor_inline.end  (?:(?<=}|%>|\?\?>)|(?=[];=>\[]))|(?=(?<!\\)\n)
#destructor_inline.patterns[0].begin  \G ?
#destructor_inline.patterns[0].end  (?:\{|<%|\?\?<|(?=;))|(?=(?<!\\)\n)
#destructor_inline.patterns[0].patterns[1].match  (=)((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<…
#destructor_inline.patterns[0].patterns[2].begin  \(
#destructor_inline.patterns[0].patterns[2].end  \)|(?=(?<!\\)\n)
#destructor_inline.patterns[1].begin  (?<=\{|<%|\?\?<)
#destructor_inline.patterns[1].end  }|%>|\?\?>|(?=(?<!\\)\n)
#destructor_inline.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#destructor_inline.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)
#destructor_root.begin  ((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\W…
#destructor_root.end  (?:(?<=}|%>|\?\?>)|(?=[];=>\[]))|(?=(?<!\\)\n)
#destructor_root.patterns[0].begin  \G ?
#destructor_root.patterns[0].end  (?:\{|<%|\?\?<|(?=;))|(?=(?<!\\)\n)
#destructor_root.patterns[0].patterns[1].match  (=)((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<…
#destructor_root.patterns[0].patterns[2].begin  \(
#destructor_root.patterns[0].patterns[2].end  \)|(?=(?<!\\)\n)
#destructor_root.patterns[1].begin  (?<=\{|<%|\?\?<)
#destructor_root.patterns[1].end  }|%>|\?\?>|(?=(?<!\\)\n)
#destructor_root.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#destructor_root.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)
#destructor_root.beginCaptures[10].patterns[0].match  ::
#destructor_root.beginCaptures[10].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#destructor_root.beginCaptures[13].patterns[0].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#destructor_root.beginCaptures[13].patterns[1].match  (?<=:)~(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#destructor_root.beginCaptures[13].patterns[2].match  ::
#diagnostic.begin  ^(((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=…
#diagnostic.end  (?<!\\)(?:(?=\n)|(?<=(?:^|[^\\])\n)(?=$))|(?=(?<!\\)\n)
#diagnostic.patterns[1].begin  "
#diagnostic.patterns[1].end  (?:(")|(?<!\\)(?:(?=\n)|(?<=(?:^|[^\\])\n)(?=$)))|(?=(?<!\\…
#diagnostic.patterns[2].begin  '
#diagnostic.patterns[2].end  (?:(')|(?<!\\)(?:(?=\n)|(?<=(?:^|[^\\])\n)(?=$)))|(?=(?<!\\…
#diagnostic.patterns[3].begin  [^"']
#diagnostic.patterns[3].end  (?<!\\)(?:(?=\n)|(?<=(?:^|[^\\])\n)(?=$))|(?=(?<!\\)\n)
#enum_block.begin  ((?<!\w)enum(?!\w))(?:\s+(class|struct))?(?:(?:\s+|((?:(?:(…
#enum_block.end  (?:(?:(?<=}|%>|\?\?>)\s+{0,1}(;)|(;))|(?=[];=>\[]))|(?=(?<!…
#enum_block.patterns[0].begin  \G ?
#enum_block.patterns[0].end  (?:\{|<%|\?\?<|(?=;))|(?=(?<!\\)\n)
#enum_block.patterns[1].begin  (?<=\{|<%|\?\?<)
#enum_block.patterns[1].end  }|%>|\?\?>|(?=(?<!\\)\n)
#enum_block.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#enum_block.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)
#extern_block.begin  ((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\W…
#extern_block.end  (?:(?:(?<=}|%>|\?\?>)\s+{0,1}(;)|(;))|(?=[];=>\[]))|(?=(?<!…
#extern_block.patterns[0].begin  \G ?
#extern_block.patterns[0].end  (?:\{|<%|\?\?<|(?=;))|(?=(?<!\\)\n)
#extern_block.patterns[1].begin  (?<=\{|<%|\?\?<)
#extern_block.patterns[1].end  }|%>|\?\?>|(?=(?<!\\)\n)
#extern_block.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#extern_block.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)
#function_call.patterns[0].begin  groups 1, 2, 3, 6, 7, 8, 9, 10, 12, 13, 14
#function_call.patterns[1].begin  groups 2, 3, 6, 7, 8, 9, 10 (fuzz), 12, 13, 14
#function_definition.begin  (?:(?:^|\G|(?<=[;}]))|(?<=>|\*/))\s*+(?:((?<!\w)template(?!…
#function_definition.end  (?:(?<=}|%>|\?\?>)|(?=[];=>\[]))|(?=(?<!\\)\n)
#function_definition.patterns[0].begin  \G ?
#function_definition.patterns[0].end  (?:\{|<%|\?\?<|(?=;))|(?=(?<!\\)\n)
#function_definition.patterns[0].patterns[1].begin  \(
#function_definition.patterns[0].patterns[1].end  \)|(?=(?<!\\)\n)
#function_definition.patterns[0].patterns[2].match  (?<=^|\))\s+{0,1}(->)((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*…
#function_definition.patterns[0].patterns[2].captures[6].patterns[0].match  ::
#function_definition.patterns[0].patterns[2].captures[6].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#function_definition.patterns[0].patterns[2].captures[6].patterns[9].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#function_definition.patterns[0].patterns[2].captures[16].patterns[0].match  ::
#function_definition.patterns[0].patterns[2].captures[16].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#function_definition.patterns[1].begin  (?<=\{|<%|\?\?<)
#function_definition.patterns[1].end  }|%>|\?\?>|(?=(?<!\\)\n)
#function_definition.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#function_definition.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)
#function_definition.beginCaptures[7].patterns[0].match  ((?<!\w)(?:(?:constexpr|consteval|explicit|mutable|virtual|…
#function_definition.beginCaptures[13].patterns[0].match  ::
#function_definition.beginCaptures[13].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#function_definition.beginCaptures[13].patterns[9].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#function_definition.beginCaptures[23].patterns[0].match  ::
#function_definition.beginCaptures[23].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#function_definition.beginCaptures[31].patterns[0].match  \*
#function_definition.beginCaptures[31].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#function_definition.beginCaptures[31].patterns[2].match  &
#function_pointer.begin  (\s*+((?:(?:(?:\[\[.*?]]|__attribute(?:__)?\s*\(\s*\(.*?\)\…
#function_pointer.end  (\))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#function_pointer.beginCaptures[1].patterns[0].match  ::
#function_pointer.beginCaptures[1].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#function_pointer.beginCaptures[1].patterns[9].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#function_pointer.beginCaptures[11].patterns[0].match  ::
#function_pointer.beginCaptures[11].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#function_pointer.beginCaptures[19].patterns[0].match  \*
#function_pointer.beginCaptures[19].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#function_pointer.beginCaptures[19].patterns[2].match  &
#function_pointer_parameter.begin  (\s*+((?:(?:(?:\[\[.*?]]|__attribute(?:__)?\s*\(\s*\(.*?\)\…
#function_pointer_parameter.end  (\))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#function_pointer_parameter.beginCaptures[1].patterns[0].match  ::
#function_pointer_parameter.beginCaptures[1].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#function_pointer_parameter.beginCaptures[1].patterns[9].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#function_pointer_parameter.beginCaptures[11].patterns[0].match  ::
#function_pointer_parameter.beginCaptures[11].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#function_pointer_parameter.beginCaptures[19].patterns[0].match  \*
#function_pointer_parameter.beginCaptures[19].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#function_pointer_parameter.beginCaptures[19].patterns[2].match  &
#gcc_attributes.begin  __attribute(?:__)?\s*\(\s*\(
#gcc_attributes.end  \)\s*\)|(?=(?<!\\)\n)
#gcc_attributes.patterns[1].begin  \(
#gcc_attributes.patterns[1].end  \)|(?=(?<!\\)\n)
#gcc_attributes.patterns[2].match  (using)\s+((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A…
#gcc_attributes.patterns[3].match  ,
#gcc_attributes.patterns[4].match  :
#gcc_attributes.patterns[5].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#gcc_attributes.patterns[6].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#inheritance_context.patterns[1].match  ,
#inheritance_context.patterns[2].match  (?<!\w)p(?:rotected|rivate|ublic)(?!\w)  fuzz
#inheritance_context.patterns[3].match  (?<!\w)virtual(?!\w)
#inheritance_context.patterns[4].match  (?<=protected|virtual|private|public|[,:])\s+{0,1}(?!p(?:ro…  fuzz
#inheritance_context.patterns[4].captures[1].patterns[0].match  ::
#inheritance_context.patterns[4].captures[1].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#inheritance_context.patterns[4].captures[1].patterns[9].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…  fuzz
#inheritance_context.patterns[4].captures[4].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#inheritance_context.patterns[4].captures[6].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#inheritance_context.patterns[4].captures[7].patterns[0].match  ::
#inheritance_context.patterns[4].captures[7].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#inheritance_context.patterns[4].captures[11].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#lambdas.begin  (?:(?<=\S|^)(?<![]"\&)*>\[\w])|(?<=(?:\W|^)return))\s+{0,1}…
#lambdas.end  (?<=[;}])|(?=(?<!\\)\n)
#lambdas.patterns[0].begin  \(
#lambdas.patterns[0].end  \)|(?=(?<!\\)\n)
#lambdas.patterns[1].match  (?<!\w)(?:constexpr|consteval|mutable)(?!\w)
#lambdas.patterns[2].begin  ->
#lambdas.patterns[2].end  (?=\{)|(?=(?<!\\)\n)
#lambdas.patterns[2].patterns[1].match  \S+
#lambdas.patterns[3].begin  \{
#lambdas.patterns[3].end  }|(?=(?<!\\)\n)
#lambdas.beginCaptures[2].patterns[1].match  ((?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{…
#line.begin  ^((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\…
#line.end  (?<!\\)(?:(?=\n)|(?<=(?:^|[^\\])\n)(?=$))|(?=(?<!\\)\n)
#line_comment.begin  \s*+(//)  fuzz
#line_comment.end  (?<!\\)(?:(?=\n)|(?<=(?:^|[^\\])\n)(?=$))|(?=(?<!\\)\n)  fuzz
#macro.begin  ^(((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=…
#macro.end  (?<!\\)(?:(?=\n)|(?<=(?:^|[^\\])\n)(?=$))|(?=(?<!\\)\n)
#macro.patterns[0].match  \G\s+{0,1}(\()([^(]*)(\))
#macro.patterns[0].captures[2].patterns[0].match  (?<=[(,])\s+{0,1}((?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A…
#macro.patterns[0].captures[2].patterns[1].match  ,
#macro.patterns[0].captures[2].patterns[2].match  \.\.\.
#method_access.begin  groups 1, 2, 3, 4, 5, 7, 8, 9, 10, 12, 13
#method_access.beginCaptures[13].patterns[0].match  (?<=\.\*?|->\*??)\s+{0,1}(?:((?:\s*+(/\*)((?:[^*]++|\*+(?!/…
#method_access.beginCaptures[13].patterns[1].match  (?:((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<…
#ms_attributes.begin  __declspec\(
#ms_attributes.end  \)|(?=(?<!\\)\n)
#ms_attributes.patterns[1].begin  \(
#ms_attributes.patterns[1].end  \)|(?=(?<!\\)\n)
#ms_attributes.patterns[2].match  (using)\s+((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A…
#ms_attributes.patterns[3].match  ,
#ms_attributes.patterns[4].match  :
#ms_attributes.patterns[5].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#ms_attributes.patterns[6].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#namespace_block.begin  ((?<!\w)namespace(?!\w))  fuzz
#namespace_block.end  (?:(?<=}|%>|\?\?>)|(?=[];=>\[]))|(?=(?<!\\)\n)  fuzz
#namespace_block.patterns[0].begin  \G ?  fuzz
#namespace_block.patterns[0].end  (?:\{|<%|\?\?<|(?=;))|(?=(?<!\\)\n)  fuzz
#namespace_block.patterns[0].patterns[4].match  ((::)?(?:(?!\b(?:__has_cpp_attribute|reinterpret_cast|atomi…  fuzz
#namespace_block.patterns[1].begin  (?<=\{|<%|\?\?<)  fuzz
#namespace_block.patterns[1].end  }|%>|\?\?>|(?=(?<!\\)\n)  fuzz
#namespace_block.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#namespace_block.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)
#normal_variable_assignment.begin  ^((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\…
#normal_variable_assignment.end  (?=;)|(?=(?<!\\)\n)
#normal_variable_assignment.beginCaptures[14].patterns[0].match  ::
#normal_variable_assignment.beginCaptures[14].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#normal_variable_assignment.beginCaptures[14].patterns[9].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#normal_variable_assignment.beginCaptures[24].patterns[0].match  ::
#normal_variable_assignment.beginCaptures[24].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#normal_variable_assignment.beginCaptures[32].patterns[0].match  \*
#normal_variable_assignment.beginCaptures[32].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#normal_variable_assignment.beginCaptures[32].patterns[2].match  &
#normal_variable_declaration.begin  ^((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\…
#normal_variable_declaration.end  (?=;)|(?=(?<!\\)\n)
#normal_variable_declaration.beginCaptures[14].patterns[0].match  ::
#normal_variable_declaration.beginCaptures[14].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#normal_variable_declaration.beginCaptures[14].patterns[9].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#normal_variable_declaration.beginCaptures[24].patterns[0].match  ::
#normal_variable_declaration.beginCaptures[24].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#normal_variable_declaration.beginCaptures[32].patterns[0].match  \*
#normal_variable_declaration.beginCaptures[32].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#normal_variable_declaration.beginCaptures[32].patterns[2].match  &
#operator_overload.begin  ((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\W…
#operator_overload.end  (?:(?<=}|%>|\?\?>)|(?=[];=>\[]))|(?=(?<!\\)\n)
#operator_overload.patterns[0].begin  \G ?
#operator_overload.patterns[0].end  (?:\{|<%|\?\?<|(?=;))|(?=(?<!\\)\n)
#operator_overload.patterns[0].patterns[2].begin  \(
#operator_overload.patterns[0].patterns[2].end  \)|(?=(?<!\\)\n)
#operator_overload.patterns[0].patterns[4].match  (=)((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<…
#operator_overload.patterns[1].begin  (?<=\{|<%|\?\?<)
#operator_overload.patterns[1].end  }|%>|\?\?>|(?=(?<!\\)\n)
#operator_overload.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#operator_overload.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)
#operator_overload.beginCaptures[5].patterns[0].match  ::
#operator_overload.beginCaptures[5].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#operator_overload.beginCaptures[5].patterns[9].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#operator_overload.beginCaptures[15].patterns[0].match  ::
#operator_overload.beginCaptures[15].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#operator_overload.beginCaptures[23].patterns[0].match  \*
#operator_overload.beginCaptures[23].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#operator_overload.beginCaptures[23].patterns[2].match  &
#operator_overload.beginCaptures[45].patterns[0].match  ::
#operator_overload.beginCaptures[45].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#operator_overload.beginCaptures[58].patterns[0].match  ::
#operator_overload.beginCaptures[58].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#operator_overload.beginCaptures[63].patterns[0].match  \*
#operator_overload.beginCaptures[63].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#operator_overload.beginCaptures[63].patterns[2].match  &
#operators.patterns[0].begin  ((?<!\w)sizeof(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*…
#operators.patterns[0].end  \)|(?=(?<!\\)\n)
#operators.patterns[1].begin  ((?<!\w)alignof(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\…
#operators.patterns[1].end  \)|(?=(?<!\\)\n)
#operators.patterns[2].begin  ((?<!\w)alignas(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\…
#operators.patterns[2].end  \)|(?=(?<!\\)\n)
#operators.patterns[3].begin  ((?<!\w)typeid(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*…
#operators.patterns[3].end  \)|(?=(?<!\\)\n)
#operators.patterns[4].begin  ((?<!\w)noexcept(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(…
#operators.patterns[4].end  \)|(?=(?<!\\)\n)
#operators.patterns[5].begin  \b(sizeof\.\.\.)((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s…
#operators.patterns[5].end  \)|(?=(?<!\\)\n)
#operators.patterns[6].match  --  fuzz
#operators.patterns[7].match  \+\+
#operators.patterns[8].match  <<|>>
#operators.patterns[9].match  !=|<=|>=|==|[<>]  fuzz
#operators.patterns[12].match  ((?:[-%*+]|(?<!\()/)=)|((?:[\&^]|<<|>>|\|)=)|(=)  fuzz
#parameter.begin  ((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\W…
#parameter.end  (?:(?=\))|(,))|(?=(?<!\\)\n)
#parameter.patterns[5].match  ((?:(thread_local|volatile|register|restrict|static|extern|…
#parameter.patterns[8].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#parameter.patterns[9].begin  (?<==)
#parameter.patterns[9].end  (?:(?=\))|(,))|(?=(?<!\\)\n)
#parameter.patterns[10].match  =
#parameter.patterns[11].match  (?<![(,:\s])((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+…
#parameter.patterns[13].begin  \[
#parameter.patterns[13].end  ]|(?=(?<!\\)\n)
#parameter.patterns[14].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#parameter.patterns[16].match  ((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\W…
#parameter.patterns[16].captures[0].patterns[0].match  \*
#parameter.patterns[16].captures[0].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#parameter.patterns[16].captures[0].patterns[2].match  &
#parameter_or_maybe_value.begin  ((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\W…
#parameter_or_maybe_value.end  (?:(?=\))|(,))|(?=(?<!\\)\n)
#parameter_or_maybe_value.patterns[7].match  ((?:(thread_local|volatile|register|restrict|static|extern|…
#parameter_or_maybe_value.patterns[11].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#parameter_or_maybe_value.patterns[12].begin  (?<==)
#parameter_or_maybe_value.patterns[12].end  (?:(?=\))|(,))|(?=(?<!\\)\n)
#parameter_or_maybe_value.patterns[13].match  (?<![(,:\s])((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+…
#parameter_or_maybe_value.patterns[15].begin  \[
#parameter_or_maybe_value.patterns[15].end  ]|(?=(?<!\\)\n)
#parameter_or_maybe_value.patterns[16].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#parameter_or_maybe_value.patterns[18].match  ((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\W…
#parameter_or_maybe_value.patterns[18].captures[0].patterns[0].match  \*
#parameter_or_maybe_value.patterns[18].captures[0].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#parameter_or_maybe_value.patterns[18].captures[0].patterns[2].match  &
#parentheses.patterns[2].match  (?<!:):(?!:)  fuzz
#pragma.begin  ^((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\…
#pragma.end  (?<!\\)(?:(?=\n)|(?<=(?:^|[^\\])\n)(?=$))|(?=(?<!\\)\n)
#pragma.patterns[2].match  [$A-Z_a-z][-$\w]*
#preprocessor_conditional_defined.begin  ((?<!\w)defined(?!\w))(\()
#preprocessor_conditional_defined.end  (?:\)|(?<!\\)(?:(?=\n)|(?<=(?:^|[^\\])\n)(?=$)))|(?=(?<!\\)…
#preprocessor_conditional_range.begin  ^((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\…
#preprocessor_conditional_range.end  (?<!\\)(?:(?=\n)|(?<=(?:^|[^\\])\n)(?=$))|(?=(?<!\\)\n)
#qualifiers_and_specifiers_post_parameters.patterns[0].begin  ((?<!\w)requires(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(…
#qualifiers_and_specifiers_post_parameters.patterns[0].end  \)|(?=(?<!\\)\n)
#qualifiers_and_specifiers_post_parameters.patterns[1].match  ((?:(?:(?:\s*+/\*(?:[^*]++|\*+(?!/))*+\*/\s*+)+|\s++|(?<=\W…
#qualifiers_and_specifiers_post_parameters.patterns[1].captures[1].patterns[0].match  ((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\W…
#range_for_inner.begin  (?<=(?:\W|^)for ?\()
#range_for_inner.end  (?=\))|(?=(?<!\\)\n)
#range_for_inner.patterns[0].match  ((?:((?:(?:(?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\…
#range_for_inner.patterns[0].captures[10].patterns[0].match  ::
#range_for_inner.patterns[0].captures[10].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#range_for_inner.patterns[0].captures[10].patterns[9].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#range_for_inner.patterns[0].captures[20].patterns[0].match  ::
#range_for_inner.patterns[0].captures[20].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#range_for_inner.patterns[0].captures[28].patterns[0].match  \*
#range_for_inner.patterns[0].captures[28].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#range_for_inner.patterns[0].captures[28].patterns[2].match  &
#range_for_inner.patterns[1].match  ((?:((?:(?:(?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\…
#range_for_inner.patterns[1].captures[10].patterns[0].match  ::
#range_for_inner.patterns[1].captures[10].patterns[1].match  (?<!\w)(?:struct|class|union|enum)(?!\w)
#range_for_inner.patterns[1].captures[10].patterns[9].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#range_for_inner.patterns[1].captures[20].patterns[0].match  ::
#range_for_inner.patterns[1].captures[20].patterns[1].match  (?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(…
#range_for_inner.patterns[1].captures[28].patterns[0].match  \*
#range_for_inner.patterns[1].captures[28].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#range_for_inner.patterns[1].captures[28].patterns[2].match  &
#square_brackets.begin  ([A-Z_a-z][0-9A-Z_a-z]*|(?<=[])]))?(\[)(?!])  fuzz
#square_brackets.end  ]|(?=(?<!\\)\n)  fuzz
#static_assert.begin  ((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\W…
#static_assert.end  \)|(?=(?<!\\)\n)
#static_assert.patterns[0].begin  (,)\s+{0,1}(?=(?:L|u8?|U\s+{0,1}")?)
#static_assert.patterns[0].end  (?=\))|(?=(?<!\\)\n)
#string_context.patterns[0].begin  groups 1
#string_context.patterns[0].end  groups 2, 3
#string_context.patterns[0].patterns[0].match  \\(?:u\h{4}|U\h{8})
#string_context.patterns[0].patterns[1].match  \\["'?\\abfnrtv]
#string_context.patterns[0].patterns[2].match  \\[0-7]{1,3}
#string_context.patterns[0].patterns[3].match  (\\x0*\h{2}(?!\h))|(\\x\h*)
#string_context.patterns[1].begin  (?<!\h)((?:u8??|[LU])?)'  fuzz
#string_context.patterns[1].end  (')(?:((?:[A-Za-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?…  fuzz
#string_context.patterns[1].patterns[0].match  (\\x0*\h{2}(?!\h))|(\\x\h*)
#string_context.patterns[2].begin  ((?:[LUu]8?)?R)"(?:(?:_r|re)|regex)\(
#string_context.patterns[2].end  \)(?:(?:_r|re)|regex)"|(?=(?<!\\)\n)
#string_context.patterns[3].begin  ((?:[LUu]8?)?R)"(?:glsl|GLSL)\(
#string_context.patterns[3].end  \)(?:glsl|GLSL)"|(?=(?<!\\)\n)
#string_context.patterns[4].begin  ((?:[LUu]8?)?R)"\(  fuzz
#string_context.patterns[4].end  \)"|(?=(?<!\\)\n)  fuzz
#string_context.patterns[5].begin  ((?:u8??|[LU])?R)"(?:([^\t ()\\]{0,16})|([^\t ()\\]*))\(  fuzz
#string_context.patterns[5].end  (\)(\2)(\3)")(?:((?:[A-Za-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z…  fuzz
#struct_block.begin  ((?<!\w)struct(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*…
#struct_block.end  (?:(?:(?<=}|%>|\?\?>)\s+{0,1}(;)|(;))|(?=[];=>\[]))|(?=(?<!…
#struct_block.patterns[0].begin  \G ?
#struct_block.patterns[0].end  (?:\{|<%|\?\?<|(?=;))|(?=(?<!\\)\n)
#struct_block.patterns[1].begin  (?<=\{|<%|\?\?<)
#struct_block.patterns[1].end  }|%>|\?\?>|(?=(?<!\\)\n)
#struct_block.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#struct_block.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)
#struct_block.beginCaptures[11].patterns[0].match  ((?<!\w)final(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/…
#struct_block.beginCaptures[11].patterns[1].match  ((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\…
#struct_block.beginCaptures[11].patterns[2].match  DLLEXPORT
#struct_block.beginCaptures[11].patterns[3].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#switch_conditional_parentheses.begin  ((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\W…
#switch_conditional_parentheses.end  \)|(?=(?<!\\)\n)
#switch_statement.begin  ((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\W…
#switch_statement.end  (?:(?<=}|%>|\?\?>)|(?=[];=>\[]))|(?=(?<!\\)\n)
#switch_statement.patterns[0].begin  \G ?
#switch_statement.patterns[0].end  (?:\{|<%|\?\?<|(?=;))|(?=(?<!\\)\n)
#switch_statement.patterns[1].begin  (?<=\{|<%|\?\?<)
#switch_statement.patterns[1].end  }|%>|\?\?>|(?=(?<!\\)\n)
#switch_statement.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#switch_statement.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)
#template_call_range.begin  <  fuzz
#template_call_range.end  >|(?=(?<!\\)\n)  fuzz
#template_call_range_helper.patterns[0].match  \b((?<!\w)\.?\d(?:['.0-9A-Z_a-z]|(?<=[EPep])[-+])*)(((?:\s*…
#template_call_range_helper.patterns[0].captures[1].patterns[0].begin  (?=.)
#template_call_range_helper.patterns[0].captures[1].patterns[0].end  $|(?=(?<!\\)\n)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[0].match  \G(0[Xx])(\h(?:\h|((?<=\h)'(?=\h)))*)?((?<=\h)\.|\.(?=\h))(…
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[0].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[0].captures[5].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[0].captures[10].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[1].match  \G(?=[.0-9])(?!0[BXbx])([0-9](?:[0-9]|((?<=\h)'(?=\h)))*)?(…
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[1].captures[1].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[1].captures[4].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[1].captures[9].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[2].match  \G(0[Bb])([01](?:[01]|((?<=\h)'(?=\h)))*)((?:[Uu]|[Uu]ll?|[…
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[2].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[3].match  \G(0)((?:[0-7]|((?<=\h)'(?=\h)))+)((?:[Uu]|[Uu]ll?|[Uu]LL?|…
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[3].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[4].match  \G(0[Xx])(\h(?:\h|((?<=\h)'(?=\h)))*)(?:(?<!')([Pp])(\+?)(-…
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[4].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[4].captures[7].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[5].match  \G(?=[.0-9])(?!0[BXbx])([0-9](?:[0-9]|((?<=\h)'(?=\h)))*)(?…
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[5].captures[1].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[5].captures[6].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[0].captures[1].patterns[0].patterns[6].match  (?:['.0-9A-Z_a-z]|(?<=[EPep])[-+])+
#template_call_range_helper.patterns[0].captures[3].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#template_call_range_helper.patterns[1].match  \b((?<!\w)\.?\d(?:['.0-9A-Z_a-z]|(?<=[EPep])[-+])*)(((?:\s*…
#template_call_range_helper.patterns[1].captures[1].patterns[0].begin  (?=.)
#template_call_range_helper.patterns[1].captures[1].patterns[0].end  $|(?=(?<!\\)\n)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[0].match  \G(0[Xx])(\h(?:\h|((?<=\h)'(?=\h)))*)?((?<=\h)\.|\.(?=\h))(…
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[0].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[0].captures[5].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[0].captures[10].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[1].match  \G(?=[.0-9])(?!0[BXbx])([0-9](?:[0-9]|((?<=\h)'(?=\h)))*)?(…
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[1].captures[1].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[1].captures[4].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[1].captures[9].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[2].match  \G(0[Bb])([01](?:[01]|((?<=\h)'(?=\h)))*)((?:[Uu]|[Uu]ll?|[…
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[2].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[3].match  \G(0)((?:[0-7]|((?<=\h)'(?=\h)))+)((?:[Uu]|[Uu]ll?|[Uu]LL?|…
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[3].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[4].match  \G(0[Xx])(\h(?:\h|((?<=\h)'(?=\h)))*)(?:(?<!')([Pp])(\+?)(-…
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[4].captures[2].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[4].captures[7].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[5].match  \G(?=[.0-9])(?!0[BXbx])([0-9](?:[0-9]|((?<=\h)'(?=\h)))*)(?…
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[5].captures[1].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[5].captures[6].patterns[0].match  (?<=\h)'(?=\h)
#template_call_range_helper.patterns[1].captures[1].patterns[0].patterns[6].match  (?:['.0-9A-Z_a-z]|(?<=[EPep])[-+])+
#template_call_range_helper.patterns[1].captures[3].patterns[0].match  \s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+
#template_definition.begin  (?<!\w)(template)\s+{0,1}(<)
#template_definition.end  >|(?=(?<!\\)\n)
#template_definition.patterns[0].begin  (?<=\w)\s+{0,1}<
#template_definition.patterns[0].end  >|(?=(?<!\\)\n)
#ternary_operator.begin  \?  fuzz
#ternary_operator.end  :|(?=(?<!\\)\n)  fuzz
#typedef_class.begin  ((?<!\w)typedef(?!\w))\s+{0,1}(?=(?<!\w)class(?!\w))
#typedef_class.end  (?<=;)|(?=(?<!\\)\n)
#typedef_class.patterns[0].begin  ((?<!\w)class(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/…
#typedef_class.patterns[0].end  (?:(?:(?<=}|%>|\?\?>)\s+{0,1}(;)|(;))|(?=[];=>\[]))|(?=(?<!…
#typedef_class.patterns[0].patterns[0].begin  \G ?
#typedef_class.patterns[0].patterns[0].end  (?:\{|<%|\?\?<|(?=;))|(?=(?<!\\)\n)
#typedef_class.patterns[0].patterns[1].begin  (?<=\{|<%|\?\?<)
#typedef_class.patterns[0].patterns[1].end  }|%>|\?\?>|(?=(?<!\\)\n)
#typedef_class.patterns[0].patterns[2].begin  (?<=}|%>|\?\?>)\s*
#typedef_class.patterns[0].patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)
#typedef_class.patterns[0].patterns[2].patterns[0].match  (((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\…
#typedef_class.patterns[0].patterns[2].patterns[0].captures[1].patterns[0].match  \*
#typedef_class.patterns[0].patterns[2].patterns[0].captures[1].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#typedef_class.patterns[0].patterns[2].patterns[0].captures[1].patterns[2].match  &
#typedef_class.patterns[0].patterns[2].patterns[1].match  ,
#typedef_class.patterns[0].beginCaptures[11].patterns[0].match  ((?<!\w)final(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/…
#typedef_class.patterns[0].beginCaptures[11].patterns[1].match  ((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\…
#typedef_class.patterns[0].beginCaptures[11].patterns[2].match  DLLEXPORT
#typedef_class.patterns[0].beginCaptures[11].patterns[3].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#typedef_struct.begin  ((?<!\w)typedef(?!\w))\s+{0,1}(?=(?<!\w)struct(?!\w))
#typedef_struct.end  (?<=;)|(?=(?<!\\)\n)
#typedef_struct.patterns[0].begin  ((?<!\w)struct(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*…
#typedef_struct.patterns[0].end  (?:(?:(?<=}|%>|\?\?>)\s+{0,1}(;)|(;))|(?=[];=>\[]))|(?=(?<!…
#typedef_struct.patterns[0].patterns[0].begin  \G ?
#typedef_struct.patterns[0].patterns[0].end  (?:\{|<%|\?\?<|(?=;))|(?=(?<!\\)\n)
#typedef_struct.patterns[0].patterns[1].begin  (?<=\{|<%|\?\?<)
#typedef_struct.patterns[0].patterns[1].end  }|%>|\?\?>|(?=(?<!\\)\n)
#typedef_struct.patterns[0].patterns[2].begin  (?<=}|%>|\?\?>)\s*
#typedef_struct.patterns[0].patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)
#typedef_struct.patterns[0].patterns[2].patterns[0].match  (((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\…
#typedef_struct.patterns[0].patterns[2].patterns[0].captures[1].patterns[0].match  \*
#typedef_struct.patterns[0].patterns[2].patterns[0].captures[1].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#typedef_struct.patterns[0].patterns[2].patterns[0].captures[1].patterns[2].match  &
#typedef_struct.patterns[0].patterns[2].patterns[1].match  ,
#typedef_struct.patterns[0].beginCaptures[11].patterns[0].match  ((?<!\w)final(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/…
#typedef_struct.patterns[0].beginCaptures[11].patterns[1].match  ((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\…
#typedef_struct.patterns[0].beginCaptures[11].patterns[2].match  DLLEXPORT
#typedef_struct.patterns[0].beginCaptures[11].patterns[3].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#typedef_union.begin  ((?<!\w)typedef(?!\w))\s+{0,1}(?=(?<!\w)union(?!\w))
#typedef_union.end  (?<=;)|(?=(?<!\\)\n)
#typedef_union.patterns[0].begin  ((?<!\w)union(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/…
#typedef_union.patterns[0].end  (?:(?:(?<=}|%>|\?\?>)\s+{0,1}(;)|(;))|(?=[];=>\[]))|(?=(?<!…
#typedef_union.patterns[0].patterns[0].begin  \G ?
#typedef_union.patterns[0].patterns[0].end  (?:\{|<%|\?\?<|(?=;))|(?=(?<!\\)\n)
#typedef_union.patterns[0].patterns[1].begin  (?<=\{|<%|\?\?<)
#typedef_union.patterns[0].patterns[1].end  }|%>|\?\?>|(?=(?<!\\)\n)
#typedef_union.patterns[0].patterns[2].begin  (?<=}|%>|\?\?>)\s*
#typedef_union.patterns[0].patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)
#typedef_union.patterns[0].patterns[2].patterns[0].match  (((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\…
#typedef_union.patterns[0].patterns[2].patterns[0].captures[1].patterns[0].match  \*
#typedef_union.patterns[0].patterns[2].patterns[0].captures[1].patterns[1].match  (?:&((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?…
#typedef_union.patterns[0].patterns[2].patterns[0].captures[1].patterns[2].match  &
#typedef_union.patterns[0].patterns[2].patterns[1].match  ,
#typedef_union.patterns[0].beginCaptures[11].patterns[0].match  ((?<!\w)final(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/…
#typedef_union.patterns[0].beginCaptures[11].patterns[1].match  ((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\…
#typedef_union.patterns[0].beginCaptures[11].patterns[2].match  DLLEXPORT
#typedef_union.patterns[0].beginCaptures[11].patterns[3].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#union_block.begin  ((?<!\w)union(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/…
#union_block.end  (?:(?:(?<=}|%>|\?\?>)\s+{0,1}(;)|(;))|(?=[];=>\[]))|(?=(?<!…
#union_block.patterns[0].begin  \G ?
#union_block.patterns[0].end  (?:\{|<%|\?\?<|(?=;))|(?=(?<!\\)\n)
#union_block.patterns[1].begin  (?<=\{|<%|\?\?<)
#union_block.patterns[1].end  }|%>|\?\?>|(?=(?<!\\)\n)
#union_block.patterns[2].begin  (?<=}|%>|\?\?>)\s*
#union_block.patterns[2].end  \s*(?=;)|(?=(?<!\\)\n)
#union_block.beginCaptures[11].patterns[0].match  ((?<!\w)final(?!\w))((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/…
#union_block.beginCaptures[11].patterns[1].match  ((?<!\w)(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\…
#union_block.beginCaptures[11].patterns[2].match  DLLEXPORT
#union_block.beginCaptures[11].patterns[3].match  (?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4…
#using_namespace.begin  (?<!\w)(using)\s+(namespace)\s+((::)?(?:(?!\b(?:__has_cpp_a…
#using_namespace.end  ;|(?=(?<!\\)\n)
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
#at-rules.patterns[0].begin  \A\uFEFF?(?i:(?=\s*@charset\b))
#at-rules.patterns[0].end  ;|(?=$)
#at-rules.patterns[0].patterns[0].match  \G((?!@charset)@\w+)|\G(\s+)|(@charset\S[^;]*)|(?<=@charset…
#at-rules.patterns[0].patterns[1].match  ((@)charset)(?=\s)
#at-rules.patterns[0].patterns[2].begin  "
#at-rules.patterns[0].patterns[2].end  "|$
#at-rules.patterns[0].patterns[2].patterns[0].begin  (?:\G|^)(?=[^"]+$)
#at-rules.patterns[0].patterns[2].patterns[0].end  $
#at-rules.patterns[1].begin  (?i)((@)import)(?:\s+|$|(?=["']|/\*))
#at-rules.patterns[1].end  ;
#at-rules.patterns[1].patterns[0].begin  \G\s*(?=/\*)
#at-rules.patterns[1].patterns[0].end  (?<=\*/)\s*
#at-rules.patterns[1].patterns[3].match  \s+
#at-rules.patterns[2].begin  (?i)((@)font-face)(?=\s*|\{|/\*|$)
#at-rules.patterns[2].end  (?!\G)
#at-rules.patterns[3].begin  (?i)(@)page(?=[:{\s]|/\*|$)
#at-rules.patterns[3].end  (?=\s*($|[:;{]))
#at-rules.patterns[5].begin  (?i)(?=@counter-style(["';{\s]|/\*|$))
#at-rules.patterns[5].end  (?<=})(?!\G)
#at-rules.patterns[5].patterns[0].begin  (?i)\G(@)counter-style
#at-rules.patterns[5].patterns[0].end  (?=\s*\{)
#at-rules.patterns[5].patterns[0].patterns[2].match  [-A-Z_a-z[^\x00-\x7F]](?:[-0-9A-Z_a-z[^\x00-\x7F]]|\\(?:\h{…
#at-rules.patterns[5].patterns[1].begin  \{
#at-rules.patterns[5].patterns[1].end  }
#at-rules.patterns[6].begin  (?i)(?=@document(["';{\s]|/\*|$))
#at-rules.patterns[6].end  (?<=})(?!\G)
#at-rules.patterns[6].patterns[0].begin  (?i)\G(@)document
#at-rules.patterns[6].patterns[0].end  (?=\s*[;{])
#at-rules.patterns[6].patterns[0].patterns[0].begin  (?i)(?<![-\w])(url-prefix|domain|regexp)(\()
#at-rules.patterns[6].patterns[0].patterns[0].end  \)
#at-rules.patterns[6].patterns[0].patterns[0].patterns[3].match  [^"')\s]+
#at-rules.patterns[6].patterns[1].begin  \{
#at-rules.patterns[6].patterns[1].end  }
#at-rules.patterns[7].begin  (?i)(?=@(?:-(?:webkit|moz|o|ms)-)?keyframes(["';{\s]|/\*|$))
#at-rules.patterns[7].end  (?<=})(?!\G)
#at-rules.patterns[7].patterns[0].begin  (?i)\G(@)(?:-(?:webkit|moz|o|ms)-)?keyframes
#at-rules.patterns[7].patterns[0].end  (?=\s*\{)
#at-rules.patterns[7].patterns[0].patterns[2].match  [-A-Z_a-z[^\x00-\x7F]](?:[-0-9A-Z_a-z[^\x00-\x7F]]|\\(?:\h{…
#at-rules.patterns[7].patterns[1].begin  \{
#at-rules.patterns[7].patterns[1].end  }
#at-rules.patterns[7].patterns[1].patterns[2].match  (?i)(?<![-\w])(from|to)(?![-\w])|([-+]?(?:\d+(?:\.\d+)?|\.\…
#at-rules.patterns[9].begin  (?i)(?=@layer(?:[;{\s]|/\*|$))
#at-rules.patterns[9].end  (?<=})(?!\G)|;
#at-rules.patterns[9].patterns[0].begin  (?i)\G(@)layer
#at-rules.patterns[9].patterns[0].end  (?=\s*[;{])
#at-rules.patterns[9].patterns[1].begin  \{
#at-rules.patterns[9].patterns[1].end  }
#at-rules.patterns[10].begin  (?i)((@)(-(ms|o)-)?viewport)(?=["';{\s]|/\*|$)
#at-rules.patterns[10].end  (?=\s*[;@{])
#at-rules.patterns[11].begin  (?i)((@)font-feature-values)(?=["';{\s]|/\*|$)\s*
#at-rules.patterns[11].end  (?=\s*[;@{])
#at-rules.patterns[13].begin  (?i)((@)namespace)(?=["';\s]|/\*|$)
#at-rules.patterns[13].end  ;|(?=[@{])
#at-rules.patterns[13].patterns[1].match  (?i)(?:\G|^|(?<=\s))(?=(?<=\s|^)[-A-Z_a-z[^\x00-\x7F]]|\s*/…
#at-rules.patterns[14].begin  (?i)(?=@[-\w]+[^;]+;s*$)
#at-rules.patterns[14].end  (?<=;)(?!\G)
#at-rules.patterns[14].patterns[0].begin  (?i)\G(@)[-\w]+
#at-rules.patterns[14].patterns[0].end  ;
#at-rules.patterns[15].begin  (?i)(?=@[-\w]+([({\s]|/\*|$))  fuzz
#at-rules.patterns[15].end  (?<=})(?!\G)  fuzz
#at-rules.patterns[15].patterns[0].begin  (?i)\G(@)[-\w]+  fuzz
#at-rules.patterns[15].patterns[0].end  (?=\s*[;{])  fuzz
#at-rules.patterns[15].patterns[1].begin  \{  fuzz
#at-rules.patterns[15].patterns[1].end  }  fuzz
#color-keywords.patterns[1].match  (?i)(?<![-\w])(aliceblue|antiquewhite|aquamarine|azure|beig…
#combinator-invalid.match  /deep/|>>>
#escapes.patterns[1].begin  \\$\s*  fuzz
#escapes.patterns[1].end  ^(?<!\G)
#escapes.patterns[2].match  \\.  fuzz
#font-features.begin  (?i)((@)(annotation|character-variant|ornaments|styleset|st…
#font-features.end  (?<=})
#font-features.patterns[0].begin  \{
#font-features.patterns[0].end  }
#font-features.patterns[0].patterns[0].match  [-A-Z_a-z[^\x00-\x7F]](?:[-0-9A-Z_a-z[^\x00-\x7F]]|\\(?:\h{…
#function-nesting.begin  \(
#function-nesting.end  \)
#functional-pseudo-classes.patterns[1].begin  (?i)((:)lang)(\()
#functional-pseudo-classes.patterns[1].end  \)
#functional-pseudo-classes.patterns[1].patterns[0].match  (?<=[(,\s])[A-Za-z]+(-[0-9A-Za-z]*|\\(?:\h{1,6}|.))*(?=[),\…
#functional-pseudo-classes.patterns[1].patterns[1].begin  "
#functional-pseudo-classes.patterns[1].patterns[1].end  "
#functional-pseudo-classes.patterns[1].patterns[1].patterns[1].match  (?<=["\s])[*A-Za-z]+(-[*0-9A-Za-z]*)*(?=["\s])
#functional-pseudo-classes.patterns[1].patterns[2].begin  '
#functional-pseudo-classes.patterns[1].patterns[2].end  '
#functional-pseudo-classes.patterns[1].patterns[2].patterns[1].match  (?<=['\s])[*A-Za-z]+(-[*0-9A-Za-z]*)*(?=['\s])
#functions.patterns[2].begin  (?i)(?<![-\w])((?:-(?:webkit-|moz-|o-))?(?:repeating-)?(?:l…
#functions.patterns[2].end  \)
#functions.patterns[2].patterns[0].match  (?i)(?<![-\w])(from|to|at|in|hue)(?![-\w])
#functions.patterns[3].begin  (?i)(?<![-\w])(-webkit-gradient)(\()
#functions.patterns[3].end  \)
#functions.patterns[3].patterns[0].begin  (?i)(?<![-\w])(from|to|color-stop)(\()
#functions.patterns[3].patterns[0].end  \)
#functions.patterns[4].patterns[0].match  (?i)(?<=[",\s]|\*/|^)\d+x(?=["'),\s]|/\*|$)
#functions.patterns[5].begin  (?i)(?<![-\w])(circle|ellipse|inset|polygon|rect)(\()
#functions.patterns[5].end  \)
#functions.patterns[5].patterns[0].match  (?i)(?<=\s|^|\*/)(at|round)(?=\s|/\*|$)
#functions.patterns[6].patterns[0].match  (?i)(?<![-\w])(start|end)(?=\s*\)|$)
#layer-import.patterns[0].begin  (?i)(?<![-\w])(layer)(\()
#layer-import.patterns[0].end  \)
#layer-import.patterns[1].match  (?i)(?<![-\w])(layer)(?![-\w])
#layer-name-list.patterns[1].match  ,
#layer-name-list.patterns[2].match  \.
#layer-name-list.patterns[3].match  [-A-Z_a-z[^\x00-\x7F]](?:[-0-9A-Z_a-z[^\x00-\x7F]]|\\(?:\h{…
#media-feature-keywords.match  (?i)(?<=^|[:\s]|\*/)(?:portrait|landscape|progressive|inter…
#media-features.match  groups 2, 3
#media-query.patterns[5].match  ,(?:(?:\s*,)+|(?=\s*[);{]))
#media-query.patterns[7].patterns[3].match  >=|<=|[<=>]
#media-query.patterns[7].patterns[4].match  (\d+)\s*(/)\s*(\d+)
#media-types.match  groups 2
#pseudo-classes.match  groups 2
#selector-innards.patterns[4].match  (?:^|(?<=[(,;}\s]))(?![-*\w]+\|(?![-#\&*.:A-\[_a-z[^\x00-\x…
#selector-innards.patterns[6].match  &
#selector-innards.patterns[8].match  (?<![-@\w])([#.])((?:-?[0-9]|-(?=$|[#\&)+,.:>\[{|~\s]|/\*)|…  fuzz
#selector-innards.patterns[11].patterns[2].match  (?<=["'\s]|^|\*/)\s*([Ii])\s*(?=[]\s]|/\*|$)
#selector-innards.patterns[11].patterns[6].match  \|
#selector-innards.patterns[11].patterns[7].match  (-?(?!\d)(?:[-\w[^\x00-\x7F]]|\\(?:\h{1,6}|.))+|\*)(?=\|(?!…
#shared-names.patterns[1].begin  (?i)(?<=^|(^|[^:])\s|[{}]|\*/|\\(?:\h{1,6}|.))(?=(?:content…
#shared-names.patterns[1].end  (?i)(?=\s|/\*|[,{]|$)
#string.patterns[0].patterns[0].begin  (?:\G|^)(?=(?:[^"\\]|\\.)+$)
#string.patterns[0].patterns[0].end  $
#string.patterns[1].patterns[0].begin  (?:\G|^)(?=(?:[^'\\]|\\.)+$)  fuzz
#string.patterns[1].patterns[0].end  $  fuzz
#unicode-range.match  (?<![-\w])[Uu]\+[?\h]{1,6}(?:(-)\h{1,6})?(?![-\w])
```

### `diff`

```text
patterns[1].match  ^\d+(,\d+)*([acd])\d+(,\d+)*$\n?
patterns[3].match  ^(((-{3}) .+ (-{4}))|((\*{3}) .+ (\*{4})))$\n?
patterns[5].match  ^diff (-|\S+\s+\S+).*$\n?  fuzz
patterns[6].match  groups 7
patterns[7].match  groups 3, 4
patterns[8].match  groups 3
patterns[9].match  ^(!).*$\n?
patterns[10].match  groups 3
patterns[11].end  groups 1
patterns[13].match  ^Index(:) (.+)$\n?
patterns[14].match  ^Only in .*: .*$\n?
```

### `docker`

```text
patterns[1].match  groups 1
patterns[2].match  ^\s*(?i:(ONBUILD)\s+)?(?i:(CMD|ENTRYPOINT))\s
patterns[4].begin  groups 1
patterns[4].end  groups 1
patterns[5].begin  groups 1
patterns[5].end  groups 1
patterns[6].match  groups 1
```

### `erb`

```text
#comment.patterns[0].begin  <%+#  fuzz
#comment.patterns[0].end  %>  fuzz
#tags.patterns[0].patterns[0].match  (#).*?(?=-?%>)  fuzz
#tags.patterns[1].patterns[0].match  (#).*?(?=-?%>)
injections[text.html.erb - (meta.embedded.block.erb | meta.embedded.line.erb | comment)].patterns[0].begin  ^(\s*)(?=<%+#(?![^%]*%>))  fuzz
injections[text.html.erb - (meta.embedded.block.erb | meta.embedded.line.erb | comment)].patterns[0].end  (?!\G)(\s*$\n)?  fuzz
injections[text.html.erb - (meta.embedded.block.erb | meta.embedded.line.erb | comment)].patterns[1].begin  groups 0
injections[text.html.erb - (meta.embedded.block.erb | meta.embedded.line.erb | comment)].patterns[1].end  groups 0
```

### `go`

```text
#const_assignment.patterns[0].captures[2].patterns[2].match  \(
#const_assignment.patterns[0].captures[2].patterns[3].match  \)
#const_assignment.patterns[0].captures[2].patterns[4].match  \[
#const_assignment.patterns[0].captures[2].patterns[5].match  ]
#const_assignment.patterns[1].patterns[0].captures[2].patterns[2].match  \(
#const_assignment.patterns[1].patterns[0].captures[2].patterns[3].match  \)
#const_assignment.patterns[1].patterns[0].captures[2].patterns[4].match  \[
#const_assignment.patterns[1].patterns[0].captures[2].patterns[5].match  ]
#double_parentheses_types.match  (?<!\w)(\([]*\[]+{0,1}[.\w]+(?:\[(?:[]*.\[{}\w]+(?:,\s*[]*.…
#double_parentheses_types.captures[1].patterns[1].match  \(
#double_parentheses_types.captures[1].patterns[2].match  \)
#double_parentheses_types.captures[1].patterns[3].match  \[
#double_parentheses_types.captures[1].patterns[4].match  ]
#double_parentheses_types.captures[1].patterns[5].match  \{
#double_parentheses_types.captures[1].patterns[6].match  }
#double_parentheses_types.captures[1].patterns[7].match  \w+
#function_declaration.patterns[1].begin  groups 1
#function_declaration.patterns[1].beginCaptures[1].patterns[1].match  \w+
#function_declaration.patterns[2].match  (?<=\))\s*((?:\s*[]*\[]+{0,1}(?:<-\s*)?\bchan\b(?:\s*<-)?\s…
#function_declaration.patterns[2].captures[1].patterns[2].match  \w+
#function_declaration.beginCaptures[3].patterns[0].match  \d\w*
#function_param_types.patterns[4].match  (?:(?<=\()|^\s*)((?:\b\w+,\s*)+(?:/[*/].*)?)$
#function_param_types.patterns[4].captures[1].patterns[1].match  \w+
#function_param_types.patterns[6].begin  groups 1
#function_param_types.patterns[6].beginCaptures[1].patterns[1].match  \w+
#function_param_types.patterns[7].begin  \(
#function_param_types.patterns[7].end  \)
#functions.end  groups 1
#functions_inline.captures[2].patterns[2].match  \[
#functions_inline.captures[2].patterns[3].match  ]
#functions_inline.captures[2].patterns[4].match  \{
#functions_inline.captures[2].patterns[5].match  }
#functions_inline.captures[2].patterns[6].match  \w+
#generic_param_types.patterns[3].match  ((?:\b\w+,\s*)+{0,1}\b\w+)\s+(?=(?:\s*[]*\[]+{0,1}(?:<-\s*)…
#generic_param_types.patterns[3].captures[1].patterns[1].match  \w+
#generic_param_types.patterns[4].match  (?:(?<=\()|^\s*)((?:\b\w+,\s*)+(?:/[*/].*)?)$
#generic_param_types.patterns[4].captures[1].patterns[1].match  \w+
#generic_param_types.patterns[5].match  groups 3
#generic_param_types.patterns[5].captures[3].patterns[1].match  \w+
#generic_param_types.patterns[6].begin  ([*.\w]+)?(\[)
#generic_param_types.patterns[6].end  ]
#generic_param_types.patterns[6].beginCaptures[1].patterns[1].match  \w+
#generic_param_types.patterns[7].begin  \(
#generic_param_types.patterns[7].end  \)
#hover.patterns[0].match  (?<=^\bfield\b)\s+([*.\w]+)\s+([\s\S]+)
#hover.patterns[0].captures[1].patterns[1].match  \w+
#hover.patterns[0].captures[2].patterns[0].match  \binvalid\b\s+\btype\b
#hover.patterns[0].captures[2].patterns[3].match  \w+
#hover.patterns[1].match  (?<=^\breturns\b)\s+([\s\S]+)
#hover.patterns[1].captures[1].patterns[2].match  \w+
#interface_variables_types_field.patterns[2].begin  ([*.\w]+)?(\[)
#interface_variables_types_field.patterns[2].end  ]
#interface_variables_types_field.patterns[2].beginCaptures[1].patterns[1].match  \w+
#keywords.patterns[6].match  \bmap\b
#keywords.patterns[7].match  \bstruct\b
#keywords.patterns[8].match  \bimport\b
#keywords.patterns[9].match  \btype\b
#map_types.patterns[3].match  \[
#map_types.patterns[4].match  ]
#map_types.patterns[5].match  \{
#map_types.patterns[6].match  }
#map_types.patterns[7].match  \(
#map_types.patterns[8].match  \)
#multi_types.begin  \b(type)\b\s*(\()
#multi_types.end  \)
#multi_types.patterns[4].match  \w+
#numeric_literals.captures[0].patterns[0].patterns[0].match  \G(?:(?:(?:(?:(?:(?=[.0-9])(?!0[BOXbox])([0-9](?:[0-9]|((?<…
#numeric_literals.captures[0].patterns[0].patterns[0].captures[1].patterns[0].match  (?<=\h)_(?=\h)
#numeric_literals.captures[0].patterns[0].patterns[0].captures[4].patterns[0].match  (?<=\h)_(?=\h)
#numeric_literals.captures[0].patterns[0].patterns[0].captures[9].patterns[0].match  (?<=\h)_(?=\h)
#numeric_literals.captures[0].patterns[0].patterns[0].captures[11].patterns[0].match  (?<=\h)_(?=\h)
#numeric_literals.captures[0].patterns[0].patterns[0].captures[16].patterns[0].match  (?<=\h)_(?=\h)
#numeric_literals.captures[0].patterns[0].patterns[0].captures[19].patterns[0].match  (?<=\h)_(?=\h)
#numeric_literals.captures[0].patterns[0].patterns[0].captures[24].patterns[0].match  (?<=\h)_(?=\h)
#numeric_literals.captures[0].patterns[0].patterns[0].captures[27].patterns[0].match  (?<=\h)_(?=\h)
#numeric_literals.captures[0].patterns[0].patterns[0].captures[30].patterns[0].match  (?<=\h)_(?=\h)
#numeric_literals.captures[0].patterns[0].patterns[0].captures[35].patterns[0].match  (?<=\h)_(?=\h)
#numeric_literals.captures[0].patterns[0].patterns[0].captures[38].patterns[0].match  (?<=\h)_(?=\h)
#numeric_literals.captures[0].patterns[0].patterns[0].captures[43].patterns[0].match  (?<=\h)_(?=\h)
#numeric_literals.captures[0].patterns[0].patterns[0].captures[47].patterns[0].match  (?<=\h)_(?=\h)
#numeric_literals.captures[0].patterns[0].patterns[0].captures[52].patterns[0].match  (?<=\h)_(?=\h)
#numeric_literals.captures[0].patterns[0].patterns[1].match  groups 2, 3, 4, 5, 6, 7, 10, 11, 12, 13, 14, 15
#numeric_literals.captures[0].patterns[0].patterns[1].captures[1].patterns[0].match  (?<=\h)_(?=\h)
#numeric_literals.captures[0].patterns[0].patterns[1].captures[5].patterns[0].match  (?<=\h)_(?=\h)
#numeric_literals.captures[0].patterns[0].patterns[1].captures[9].patterns[0].match  (?<=\h)_(?=\h)
#numeric_literals.captures[0].patterns[0].patterns[1].captures[13].patterns[0].match  (?<=\h)_(?=\h)
#numeric_literals.captures[0].patterns[0].patterns[2].match  (?:[.0-9A-Z_a-z]|(?<=[EPep])[-+])+  fuzz
#other_struct_interface_expressions.patterns[1].match  groups 2
#other_struct_interface_expressions.patterns[1].captures[2].patterns[1].match  \[
#other_struct_interface_expressions.patterns[1].captures[2].patterns[2].match  ]
#other_struct_interface_expressions.patterns[1].captures[2].patterns[3].match  \{
#other_struct_interface_expressions.patterns[1].captures[2].patterns[4].match  }
#other_struct_interface_expressions.patterns[1].captures[2].patterns[5].match  \w+
#package_name.patterns[0].patterns[0].match  \d\w*
#parameter-variable-types.patterns[0].match  \{
#parameter-variable-types.patterns[1].match  }
#runes.patterns[0].patterns[1].match  [^']+  fuzz
#single_type.patterns[1].patterns[3].match  \[
#single_type.patterns[1].patterns[4].match  ]
#single_type.patterns[1].patterns[5].match  \{
#single_type.patterns[1].patterns[6].match  }
#single_type.patterns[1].patterns[7].match  \(
#single_type.patterns[1].patterns[8].match  \)
#single_type.patterns[1].patterns[9].match  \w+
#single_type.patterns[1].endCaptures[1].patterns[1].match  \[
#single_type.patterns[1].endCaptures[1].patterns[2].match  ]
#single_type.patterns[1].endCaptures[1].patterns[3].match  \w+
#string_escaped_char.patterns[1].match  \\[^"'0-7Uabfnrtuvx]
#struct_variable_types_fields_multi.patterns[1].begin  \b(\w+(?:,\s*\b\w+)*(?:\s*[]*\[]+{0,1}(?:<-\s*)?\bchan\b(?:…
#struct_variable_types_fields_multi.patterns[1].end  }
#struct_variable_types_fields_multi.patterns[1].beginCaptures[1].patterns[1].match  \w+
#struct_variables_types_fields.patterns[1].match  (?<=\{)\s*((?:\s*[]*\[]+{0,1}(?:<-\s*)?\bchan\b(?:\s*<-)?\s…
#struct_variables_types_fields.patterns[1].captures[1].patterns[1].match  \w+
#struct_variables_types_fields.patterns[2].match  (?<=\{)\s*((?:\w+,\s*)+{0,1}\w+\s+)((?:\s*[]*\[]+{0,1}(?:<-…
#struct_variables_types_fields.patterns[2].captures[1].patterns[1].match  \w+
#struct_variables_types_fields.patterns[2].captures[2].patterns[1].match  \w+
#struct_variables_types_fields.patterns[3].match  (?<=\{)((?:\s*(?:(?:\w+,\s*)+{0,1}\w+\s+)?(?:\s*[]*\[]+{0,1…
#struct_variables_types_fields.patterns[3].captures[1].patterns[0].match  ((?:\w+,\s*)+{0,1}\w+\s+)?((?:\s*[]*\[]+{0,1}(?:<-\s*)?\bch…
#struct_variables_types_fields.patterns[3].captures[1].patterns[0].captures[1].patterns[1].match  \w+
#struct_variables_types_fields.patterns[3].captures[1].patterns[0].captures[2].patterns[1].match  \w+
#support_functions.captures[2].patterns[1].match  \d\w*
#support_functions.captures[3].patterns[3].match  \{
#support_functions.captures[3].patterns[4].match  }
#switch_types.patterns[0].match  groups 4
#switch_types.patterns[1].begin  \bcase\b
#switch_types.patterns[1].end  :
#switch_types.patterns[1].patterns[1].match  \w+
#switch_types.beginCaptures[3].patterns[2].match  \btype\b
#syntax_errors.patterns[0].match  \[](\s+)
#syntax_errors.patterns[1].match  \b0[0-7]*[89]\d*\b
#type_assertion_inline.captures[2].patterns[1].match  \(
#type_assertion_inline.captures[2].patterns[2].match  \)
#type_assertion_inline.captures[2].patterns[5].match  \{
#type_assertion_inline.captures[2].patterns[6].match  }
#var_assignment.patterns[0].captures[2].patterns[2].match  \(
#var_assignment.patterns[0].captures[2].patterns[3].match  \)
#var_assignment.patterns[1].patterns[0].captures[2].patterns[2].match  \(
#var_assignment.patterns[1].patterns[0].captures[2].patterns[3].match  \)
#var_assignment.patterns[1].patterns[0].captures[2].patterns[4].match  \[
#var_assignment.patterns[1].patterns[0].captures[2].patterns[5].match  ]
#variable_assignment.patterns[0].captures[0].patterns[1].match  \d\w*
#variable_assignment.patterns[1].captures[0].patterns[2].match  \d\w*
```

### `graphql`

```text
#graphql-comma.match  \s*(,)  fuzz
#graphql-comment.patterns[0].match  groups 1
#graphql-comment.patterns[1].begin  (""")  fuzz
#graphql-comment.patterns[1].end  (""")  fuzz
#graphql-description-docstring.begin  """
#graphql-description-docstring.end  """
#graphql-description-singleline.match  #(?=([^"]*"[^"]*")*[^"]*$).*$
#graphql-directive-definition.begin  groups 3, 4
#graphql-fragment-definition.end  groups 1, 2, 3, 4
#graphql-fragment-spread.end  groups 1, 2
#graphql-inline-fragment.end  groups 1, 2, 3
#graphql-input-types.patterns[2].begin  groups 2
#graphql-null-value.match  \s*\b(null)\b  fuzz
#graphql-schema.patterns[0].patterns[0].end  groups 5
#graphql-string-content.patterns[0].match  \\["'/\\bfnrt]
#graphql-string-content.patterns[1].match  \\u(\h{4})
#graphql-string-value.end  groups 3
#graphql-type-definition.end  groups 5
#graphql-type-interface.end  (?=.)
#graphql-union.end  groups 1, 2
#graphql-union.patterns[0].end  groups 1, 2
#native-interpolation.begin  \s*(\$\{)
#native-interpolation.end  (})
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
#svg.patterns[0].patterns[0].begin  (?<!>)\G
#svg.patterns[0].patterns[0].end  >
#svg.repository.#attribute.patterns[1].end  (?=\s*+[^=\s])
#svg.repository.#attribute.patterns[2].match  [^>\s]+
#svg.repository.#tags.patterns[2].match  (?i)(<)(color-profile|desc|metadata|script|style|title)(?=\…
#svg.repository.#tags.patterns[3].begin  (?i)(<)(color-profile|desc|metadata|script|style|title)(?=\…
#svg.repository.#tags.patterns[3].end  (?i)(</)(\2)\s*(>)|(/>)|(?=</\w+)
#svg.repository.#tags.patterns[3].patterns[0].begin  (?<!>)\G
#svg.repository.#tags.patterns[3].patterns[0].end  (?=/>)|>
#svg.repository.#tags.patterns[4].match  (?i)(<)(animateMotion|clipPath|defs|feComponentTransfer|feD…
#svg.repository.#tags.patterns[5].begin  (?i)(<)(animateMotion|clipPath|defs|feComponentTransfer|feD…
#svg.repository.#tags.patterns[5].end  (?i)(</)(\2)\s*(>)|(/>)|(?=</\w+)
#svg.repository.#tags.patterns[5].patterns[0].begin  (?<!>)\G
#svg.repository.#tags.patterns[5].patterns[0].end  (?=/>)|>
#svg.repository.#tags.patterns[6].match  (?i)(<)(a|animate|discard|feBlend|feColorMatrix|feComposite…
#svg.repository.#tags.patterns[7].begin  (?i)(<)(a|animate|discard|feBlend|feColorMatrix|feComposite…
#svg.repository.#tags.patterns[7].end  (?i)(</)(\2)\s*(>)|(/>)|(?=</\w+)
#svg.repository.#tags.patterns[7].patterns[0].begin  (?<!>)\G
#svg.repository.#tags.patterns[7].patterns[0].end  (?=/>)|>
#svg.repository.#tags.patterns[8].match  (?i)(<)(circle|ellipse|feImage|foreignObject|image|line|pat…
#svg.repository.#tags.patterns[9].begin  (?i)(<)(a|circle|ellipse|feImage|foreignObject|image|line|p…
#svg.repository.#tags.patterns[9].end  (?i)(</)(\2)\s*(>)|(/>)|(?=</\w+)
#svg.repository.#tags.patterns[9].patterns[0].begin  (?<!>)\G
#svg.repository.#tags.patterns[9].patterns[0].end  (?=/>)|>
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
patterns[0].begin  groups 1
patterns[1].begin  groups 1
patterns[4].patterns[0].match  \\.
```

### `java`

```text
patterns[0].patterns[1].match  (?<=\.)\s*\.|\.(?=\s*;)
patterns[0].patterns[2].match  (?<!_)_(?=\s*([.;]))|\b\d+|-+
patterns[0].patterns[3].match  [A-Z]+
patterns[0].patterns[4].match  \b(?<!\$)(abstract|assert|boolean|break|byte|case|catch|cha…
patterns[1].patterns[1].match  (?<=\.)\s*\.|\.(?=\s*;)
patterns[1].patterns[2].match  (?<!\.)\s*\*
patterns[1].patterns[3].match  (?<!_)_(?=\s*([.;]))|\b\d+|-+
patterns[1].patterns[4].match  \b(?<!\$)(abstract|assert|boolean|break|byte|case|catch|cha…
#anonymous-classes-and-new.patterns[4].begin  (?<=])
#anonymous-classes-and-new.patterns[4].end  (?=[])-.:;?}]|/(?![*/])|[!%\&=^|])
#anonymous-classes-and-new.patterns[4].patterns[1].begin  \{
#anonymous-classes-and-new.patterns[4].patterns[1].end  }
#assertions.patterns[0].begin  \b(assert)\s
#assertions.patterns[0].end  $
#assertions.patterns[0].patterns[0].match  :
#class.patterns[6].begin  (permits)\s  fuzz
#class.patterns[6].end  (?=\s*extends|implements|\{)  fuzz
#comments.patterns[0].match  /\*\*/
#comments-inline.patterns[1].begin  groups 1
#enums.patterns[0].begin  \b(extends)\b
#enums.patterns[0].end  (?=\{|\bimplements\b)
#generics.patterns[5].match  &
#keywords.patterns[13].match  \b(const|goto)\b
#member-variables.begin  (?=private|protected|public|native|synchronized|abstract|th…
#member-variables.end  (?=[;=])
#module.begin  ((open)\s)?(module)\s+(\w+)
#module.end  }
#module.patterns[0].begin  \{
#module.patterns[0].end  (?=})
#module.patterns[0].patterns[2].match  \b(requires|transitive|exports|opens|to|uses|provides|with)…
#numbers.patterns[1].match  \b(?<!\$)0([Bb])[01]([01_]*[01])?[Ll]?\b(?!\$)
#numbers.patterns[2].match  \b(?<!\$)0[0-7]([0-7_]*[0-7])?[Ll]?\b(?!\$)  fuzz
#object-types.patterns[1].begin  groups 1
#object-types.patterns[1].beginCaptures[1].patterns[0].match  [A-Z_a-z]\w*
#object-types.patterns[1].beginCaptures[1].patterns[1].match  \.
#parens.patterns[2].begin  \{
#parens.patterns[2].end  }
#properties.patterns[3].match  (\.)\s*([0-9][$\w]*)
#record.begin  (?=\w?[\w\s]*\brecord\s+[$\w]+)  fuzz
#record.end  }  fuzz
#record.patterns[3].begin  (record)\s+([$\w]+)(<[$\w]+>)?(\()  fuzz
#record.patterns[3].end  \)  fuzz
#record.patterns[4].begin  (implements)\s  fuzz
#record.patterns[4].end  (?=\s*\{)  fuzz
#record-body.begin  \{  fuzz
#record-body.end  (?=})  fuzz
#record-constructor.begin  (?!new)(?=[<\w].*\s+)(?=([^(/=]|/(?!/))+(?=\{))
#record-constructor.end  (})|(?=;)
#record-constructor.patterns[1].begin  (\w+)
#record-constructor.patterns[1].end  (?=\s*\{)
#record-constructor.patterns[3].begin  \{
#record-constructor.patterns[3].end  (?=})
#strings.patterns[0].patterns[0].match  (\\""")(?!")|(\\.)
#try-catch-finally.patterns[0].patterns[0].begin  \(
#try-catch-finally.patterns[0].patterns[0].end  \)
#try-catch-finally.patterns[1].patterns[1].patterns[2].end  groups 1
#variables-local.begin  (?=\b(var)\b\s+[$A-Z_a-z][$\w]*\s*([:;=]))
#variables-local.end  (?=[:;=])
#variables-local.patterns[0].match  \bvar\b
#variables-local.patterns[1].match  ([$A-Z_a-z][$\w]*)(?=\s*(\[])*\s*([:;=]))
```

### `javascript`

```text
#access-modifier.match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(abstract|declare|…
#array-binding-pattern.begin  (?:(\.\.\.)\s*)?(\[)
#array-binding-pattern.end  ]
#array-binding-pattern-const.begin  (?:(\.\.\.)\s*)?(\[)
#array-binding-pattern-const.end  ]
#arrow-function.patterns[0].match  groups 1
#arrow-return-type.begin  (?<=\))\s*(:)
#arrow-return-type.end  (?==>|\{|^(\s*(export|function|class|interface|let|var|\bus…
#arrow-return-type-body.patterns[0].begin  (?<=:)(?=\s*\{)
#arrow-return-type-body.patterns[0].end  (?<=})
#async-modifier.match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(async)(?![$_[:aln…
#brackets.patterns[0].begin  \{
#brackets.patterns[0].end  }|(?=\*/)
#brackets.patterns[1].begin  \[
#brackets.patterns[1].end  ]|(?=\*/)
#class-declaration.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#class-declaration.end  (?<=})
#class-declaration-or-expression-patterns.patterns[2].match  [$_[:alpha:]][$_[:alnum:]]*
#class-expression.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(abstract)\s+)?…
#class-expression.end  (?<=})
#class-or-interface-body.begin  \{
#class-or-interface-body.end  }
#class-or-interface-body.patterns[2].begin  (?<=:)\s*
#class-or-interface-body.patterns[2].end  (?=[-\])+,:;}\s]|^\s*$|^\s*(?:abstract|async|\bawait\s+\bus…
#class-or-interface-heritage.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))\b(extends|impleme…
#class-or-interface-heritage.end  (?=\{)
#class-or-interface-heritage.patterns[4].match  ([$_[:alpha:]][$_[:alnum:]]*)\s*(?:(\.)|(\?\.(?!\s*\d)))(?=…
#class-or-interface-heritage.patterns[5].match  ([$_[:alpha:]][$_[:alnum:]]*)
#comment.patterns[1].begin  groups 2, 3
#comment.patterns[2].begin  groups 1, 4, 5
#control-statement.patterns[3].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(break|continue|go…
#control-statement.patterns[6].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(case|default|swit…  fuzz
#control-statement.patterns[9].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(with)(?![$_[:alnu…  fuzz
#control-statement.patterns[10].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(package)(?![$_[:a…
#control-statement.patterns[11].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(debugger)(?![$_[:…
#declaration.patterns[11].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(declare|export)(?…  fuzz
#decorator.end  (?=\s)  fuzz
#destructuring-const.patterns[1].begin  (?<![:=]|^of|[^$._[:alnum:]]of|^in|[^$._[:alnum:]]in)\s*(?=…
#destructuring-const.patterns[1].end  (?=$|^|[,;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(of…
#destructuring-parameter.patterns[0].begin  groups 1
#destructuring-parameter.patterns[1].begin  groups 1
#destructuring-parameter-rest.match  groups 1
#destructuring-variable.patterns[1].begin  (?<![:=]|^of|[^$._[:alnum:]]of|^in|[^$._[:alnum:]]in)\s*(?=…
#destructuring-variable.patterns[1].end  (?=$|^|[,;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(of…
#destructuring-variable-rest.match  groups 1
#destructuring-variable-rest-const.match  groups 1
#directives.begin  ^(///)\s*(?=<(reference|amd-dependency|amd-module)(\s+(path…
#directives.end  (?=$)
#directives.patterns[0].begin  (<)(reference|amd-dependency|amd-module)
#directives.patterns[0].end  />
#directives.patterns[0].patterns[0].match  path|types|no-default-lib|lib|name|resolution-mode
#directives.patterns[0].patterns[1].match  =
#docblock.patterns[0].match  ((@)a(?:ccess|pi))\s+(p(?:rivate|rotected|ublic))\b
#docblock.patterns[1].match  ((@)author)\s+([^*/<>@\s](?:[^*/<>@]|\*[^/])*)(?:\s*(<)([^>…
#docblock.patterns[2].match  ((@)borrows)\s+((?:[^*/@\s]|\*[^/])+)\s+(as)\s+((?:[^*/@\s]…
#docblock.patterns[3].begin  ((@)example)\s+
#docblock.patterns[3].end  (?=@|\*/)
#docblock.patterns[3].patterns[0].match  ^\s\*\s+
#docblock.patterns[3].patterns[1].begin  \G(<)caption(>)
#docblock.patterns[3].patterns[1].end  (</)caption(>)|(?=\*/)
#docblock.patterns[3].patterns[2].match  [^*@\s](?:[^*]|\*[^/])*
#docblock.patterns[4].match  ((@)kind)\s+(class|constant|event|external|file|function|me…
#docblock.patterns[5].match  groups 4
#docblock.patterns[6].match  ((@)template)\s+([$A-Z_a-z][]$.\[\w]*(?:\s*,\s*[$A-Z_a-z][]…
#docblock.patterns[7].begin  ((@)template)\s+(?=\{)
#docblock.patterns[7].end  (?=\s|\*/|[^]$A-\[_a-{}])
#docblock.patterns[7].patterns[1].match  ([$A-Z_a-z][]$.\[\w]*)
#docblock.patterns[8].match  ((@)(?:arg|argument|const|constant|member|namespace|param|v…
#docblock.patterns[10].patterns[2].match  groups 2, 3, 5
#docblock.patterns[12].match  ((@)(?:alias|augments|callback|constructs|emits|event|fires…
#docblock.patterns[13].begin  ((@)(?:default(?:value)?|license|version))\s+((["']))
#docblock.patterns[13].end  (\3)|(?=$|\*/)
#docblock.patterns[14].match  ((@)(?:default(?:value)?|license|tutorial|variation|version…
#docblock.patterns[17].match  ((@)[$_[:alpha:]][$_[:alnum:]]*)(?=\s+)  fuzz
#enum-declaration.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#enum-declaration.end  (?<=})
#enum-declaration.patterns[1].begin  \{
#enum-declaration.patterns[1].end  }
#enum-declaration.patterns[1].patterns[1].begin  ([$_[:alpha:]][$_[:alnum:]]*)
#enum-declaration.patterns[1].patterns[1].end  (?=[,}]|$)
#enum-declaration.patterns[1].patterns[2].begin  (?=(('([^'\\]|\\.)*')|("([^"\\]|\\.)*")|(`([^\\`]|\\.)*`)|(…
#enum-declaration.patterns[1].patterns[2].end  (?=[,}]|$)
#export-declaration.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(export)\s+(as)\s+…
#export-declaration.patterns[1].begin  groups 2, 3
#export-declaration.patterns[2].begin  groups 2
#expression-inside-possibly-arrow-parens.patterns[5].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(override|public|p…
#expression-inside-possibly-arrow-parens.patterns[6].match  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(override|publi…
#expression-inside-possibly-arrow-parens.patterns[7].match  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(override|publi…
#expression-inside-possibly-arrow-parens.patterns[10].match  ,
#expression-operators.patterns[1].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(yield)(?![$_[:aln…
#expression-operators.patterns[1].end  \*
#expression-operators.patterns[2].match  groups 2
#expression-operators.patterns[6].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))instanceof(?![$_[:…
#expression-operators.patterns[7].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))new(?![$_[:alnum:]…
#expression-operators.patterns[9].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))void(?![$_[:alnum:…
#expression-operators.patterns[10].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(as)\s+(const)(?=\…
#expression-operators.patterns[11].begin  groups 2
#expression-operators.patterns[14].match  (?:[\&^]|<<|>>>??|\|)=
#expression-operators.patterns[18].match  (?<=[$_[:alnum:]])(!)\s*(?:(/=)|(/)(?![*/]))
#expression-operators.patterns[25].begin  (?<=[]$)_[:alnum:]])\s*(?=(/\*([^*]|(\*[^/]))*\*/\s*)+(?:(/…
#expression-operators.patterns[25].end  (/=)|(/)(?!\*([^*]|(\*[^/]))*\*/)
#expression-operators.patterns[26].match  groups 1
#field-declaration.begin  groups 1
#field-declaration.patterns[6].match  (#?[$_[:alpha:]][$_[:alnum:]]*)(?:(\?)|(!))?(?=\s*\s*(=\s*(…
#field-declaration.patterns[8].match  \?
#field-declaration.patterns[9].match  !
#for-loop.patterns[1].match  await
#function-body.patterns[6].match  \*
#function-call.patterns[1].begin  (?=(((([$_[:alpha:]][$_[:alnum:]]*)(\s*\??\.\s*(#?[$_[:alph…
#function-call.patterns[1].end  (?<=>)(?!(((([$_[:alpha:]][$_[:alnum:]]*)(\s*\??\.\s*(#?[$_…
#function-call.patterns[1].patterns[0].begin  (?=(([$_[:alpha:]][$_[:alnum:]]*)(\s*\??\.\s*(#?[$_[:alpha:…
#function-call.patterns[1].patterns[0].end  (?=(<\s*[(\[{]\s*)$)
#function-call-optionals.patterns[0].match  \?\.
#function-call-optionals.patterns[1].match  !
#function-declaration.begin  groups 2
#function-expression.begin  groups 3
#function-name.match  [$_[:alpha:]][$_[:alnum:]]*  fuzz
#identifiers.patterns[1].match  groups 2
#identifiers.patterns[2].match  groups 2
#if-statement.patterns[0].patterns[2].begin  (?<=\))\s*/(?![*/])(?=(?:[^/\[\\]|\\.|\[([^]\\]|\\.)*])+/([…
#if-statement.patterns[0].patterns[2].end  (/)([dgimsuvy]*)
#import-declaration.begin  groups 1, 2, 4
#import-equals-declaration.patterns[0].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#import-equals-declaration.patterns[0].end  \)
#import-equals-declaration.patterns[1].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#import-equals-declaration.patterns[1].end  (?=;|$|^)
#import-equals-declaration.patterns[1].patterns[2].match  ([$_[:alpha:]][$_[:alnum:]]*)\s*(?:(\.)|(\?\.(?!\s*\d)))
#import-equals-declaration.patterns[1].patterns[3].match  ([$_[:alpha:]][$_[:alnum:]]*)
#import-export-assert-clause.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(with)|(assert)…
#import-export-assert-clause.end  }
#import-export-assert-clause.patterns[2].match  [$_[:alpha:]][$_[:alnum:]]*\s*(?=(/\*([^*]|(\*[^/]))*\*/\s*…
#import-export-assert-clause.patterns[3].match  :
#import-export-clause.patterns[1].match  groups 1, 2, 13, 15
#import-export-clause.patterns[4].match  \b(default)\b
#import-export-clause.patterns[5].match  groups 1, 3
#indexer-declaration.begin  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(readonly)\s*)?…
#indexer-declaration.end  (])\s*(\?\s*)?|$
#indexer-mapped-type-declaration.begin  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))([-+])?(readonl…
#indexer-mapped-type-declaration.end  (])([-+])?\s*(\?\s*)?|$
#indexer-mapped-type-declaration.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(as)\s+
#inline-tags.patterns[0].match  (\[)[^]]+(])(?=\{@(?:link|linkcode|linkplain|tutorial))
#inline-tags.patterns[1].begin  (\{)((@)(?:link(?:code|plain)?|tutorial))\s*
#inline-tags.patterns[1].end  }|(?=\*/)
#inline-tags.patterns[1].patterns[0].match  \G((?=https?://)(?:[^*|}\s]|\*/)+)(\|)?
#inline-tags.patterns[1].patterns[1].match  \G((?:[^*@{|}\s]|\*[^/])+)(\|)?
#instanceof-expr.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(instanceof)(?![$_…
#instanceof-expr.end  (?<=\))|(?=[-\])+,:;>?}]|\|\||&&|!==|$|([!=]==?)|(([\&^|~]\…
#interface-declaration.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#interface-declaration.end  (?<=})
#interface-declaration.patterns[2].match  [$_[:alpha:]][$_[:alnum:]]*
#jsx-entities.patterns[0].match  (&)([0-9A-Za-z]+|#[0-9]+|#x\h+)(;)
#jsx-string-single-quoted.begin  '
#jsx-string-single-quoted.end  '
#jsx-tag.end  groups 3, 4
#jsx-tag.patterns[0].begin  groups 2, 3
#jsx-tag-attribute-name.match  groups 1, 2
#jsx-tag-without-attributes.begin  groups 2, 3
#jsx-tag-without-attributes.end  groups 2, 3
#label.patterns[0].begin  ([$_[:alpha:]][$_[:alnum:]]*)\s*(:)(?=\s*\{)  fuzz
#label.patterns[0].end  (?<=})  fuzz
#method-declaration.patterns[0].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(override)\s+…
#method-declaration.patterns[0].end  (?=[,;}]|$)|(?<=})
#method-declaration.patterns[1].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(override)\s+…
#method-declaration.patterns[1].end  (?=[,;}]|$)|(?<=})
#method-declaration.patterns[2].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(override)\s+…  fuzz
#method-declaration.patterns[2].end  (?=[,;}]|$)|(?<=})  fuzz
#method-declaration-name.patterns[4].match  \?
#namespace-declaration.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#namespace-declaration.end  (?<=})|(?=;|^\s*$|^\s*(?:abstract|async|\bawait\s+\busing(?…
#namespace-declaration.patterns[2].match  ([$_[:alpha:]][$_[:alnum:]]*)
#numeric-literal.patterns[0].match  groups 1
#numeric-literal.patterns[1].match  \b(?<!\$)0[Bb][01][01_]*(n)?\b(?!\$)
#numeric-literal.patterns[2].match  \b(?<!\$)0[Oo]?[0-7][0-7_]*(n)?\b(?!\$)
#numeric-literal.patterns[3].match  groups 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 12, 13, 14
#numericConstant-literal.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))NaN(?![$_[:alnum:]…
#object-binding-element.patterns[1].begin  (?=(\b((?<!\$)0[Xx]\h[_\h]*(n)?\b(?!\$))|\b((?<!\$)0[Bb][01…
#object-binding-element.patterns[1].end  (?=[,}])
#object-binding-element-const.patterns[1].begin  (?=(\b((?<!\$)0[Xx]\h[_\h]*(n)?\b(?!\$))|\b((?<!\$)0[Bb][01…
#object-binding-element-const.patterns[1].end  (?=[,}])
#object-binding-element-propertyName.begin  (?=(\b((?<!\$)0[Xx]\h[_\h]*(n)?\b(?!\$))|\b((?<!\$)0[Bb][01…
#object-binding-element-propertyName.end  (:)
#object-binding-element-propertyName.patterns[3].match  ([$_[:alpha:]][$_[:alnum:]]*)
#object-binding-pattern.begin  groups 1
#object-binding-pattern-const.begin  groups 1
#object-identifiers.patterns[1].match  groups 2, 3
#object-literal-method-declaration.begin  groups 2, 3
#object-literal-method-declaration.patterns[2].begin  groups 1, 2, 3
#object-member.patterns[2].begin  (?=\[)  fuzz
#object-member.patterns[2].end  (?=:)|((?<=])(?=\s*[(<]))  fuzz
#object-member.patterns[5].begin  (?<=[]"'`])(?=\s*[(<])
#object-member.patterns[5].end  (?=[,;}])|(?<=})
#object-member.patterns[6].match  (?![$_[:alpha:]])(\d+)\s*(?=(/\*([^*]|(\*[^/]))*\*/\s*)*:)
#object-member.patterns[11].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(as)\s+(const)(?=\…
#object-member.patterns[12].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(as)|(satisfies…
#object-member.patterns[12].end  (?=[-\])+,:;>?}]|\|\||&&|!==|$|^|((?<![$_[:alnum:]])(?:(?<=…
#object-member.patterns[13].begin  (?=[$_[:alpha:]][$_[:alnum:]]*\s*=)  fuzz
#object-member.patterns[13].end  (?=[,}]|$|//|/\*)  fuzz
#object-member.patterns[14].patterns[0].begin  (?<=:)\s*(async)?(?=\s*(<\s*(((const\s+)?[$_[:alpha:]])|(\{…
#object-member.patterns[14].patterns[0].end  (?<=\))
#object-member.patterns[14].patterns[0].patterns[1].begin  \(
#object-member.patterns[14].patterns[0].patterns[1].end  \)
#object-member.patterns[14].patterns[1].begin  (?<=:)\s*(async)?\s*(\()(?=\s*((([\[{]\s*)?)$|((\{([^{}]|(\…
#object-member.patterns[14].patterns[1].end  \)
#object-member.patterns[14].patterns[2].begin  (?<=:)\s*(async)?\s*(?=<\s*$)
#object-member.patterns[14].patterns[2].end  (?<=>)
#object-member.patterns[14].patterns[3].begin  (?<=>)\s*(\()(?=\s*((([\[{]\s*)?)$|((\{([^{}]|(\{([^{}]|\{[…
#object-member.patterns[14].patterns[3].end  \)
#parameter-array-binding-pattern.begin  (?:(\.\.\.)\s*)?(\[)
#parameter-array-binding-pattern.end  ]
#parameter-name.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(override|public|p…
#parameter-name.patterns[1].match  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(override|publi…
#parameter-name.patterns[2].match  groups 1, 2, 3, 5
#parameter-object-binding-element.patterns[1].begin  (?=(\b((?<!\$)0[Xx]\h[_\h]*(n)?\b(?!\$))|\b((?<!\$)0[Bb][01…
#parameter-object-binding-element.patterns[1].end  (?=[,}])
#parameter-object-binding-pattern.begin  (?:(\.\.\.)\s*)?(\{)
#parameter-object-binding-pattern.end  }
#paren-expression-possibly-arrow.patterns[0].begin  (?<=[(,=])\s*(async)?(?=\s*((<\s*(((const\s+)?[$_[:alpha:]]…
#paren-expression-possibly-arrow.patterns[0].end  (?<=\))
#paren-expression-possibly-arrow.patterns[1].begin  groups 1
#possibly-arrow-return-type.begin  (?<=\)|^)\s*(:)(?=\s*([^()<>{}]|<([^<>]|<([^<>]|<[^<>]+>)+>…
#possibly-arrow-return-type.end  (?==>|\{|^(\s*(export|function|class|interface|let|var|\bus…
#property-accessor.match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(accessor|get|set)…
#qstring-double.end  groups 2 (fuzz)
#regex.patterns[1].end  groups 2
#regex-character-class.patterns[2].match  \\c[A-Z]
#regexp.patterns[1].match  \\(?:[1-9]\d*|k<([$A-Z_a-z][$\w]*)>)
#regexp.patterns[4].begin  (\()((\?=)|(\?!)|(\?<=)|(\?<!))
#regexp.patterns[4].end  (\))
#regexp.patterns[5].begin  groups 1, 2
#regexp.patterns[6].patterns[0].match  (?:.|(\\(?:[0-7]{3}|x\h{2}|u\h{4}))|(\\c[A-Z])|(\\.))-(?:[^…
#return-type.patterns[0].begin  (?<=\))\s*(:)(?=\s*\S)
#return-type.patterns[0].end  (?<![\&:|])(?=$|^|[,;{}]|//)
#return-type.patterns[1].begin  (?<=\))\s*(:)
#return-type.patterns[1].end  (?<![\&:|])((?=[,;{}]|//|^\s*$)|((?<=\S)(?=\s*$)))
#return-type-core.patterns[1].begin  (?<=[\&:|])(?=\s*\{)
#return-type-core.patterns[1].end  (?<=})
#shebang.match  \A(#!).*(?=$)
#single-line-comment-consuming-line-ending.begin  groups 1, 4, 5
#super-literal.match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))super\b(?!\$)
#support-function-call-identifiers.patterns[4].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))import(?=\s*\(\s*[…  fuzz
#support-objects.patterns[2].match  groups 3
#support-objects.patterns[3].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(new)\s*(?:(\.)|(\…
#support-objects.patterns[4].match  groups 2, 4
#support-objects.patterns[5].match  groups 4
#switch-statement.patterns[2].patterns[1].begin  (:)\s*(\{)
#switch-statement.patterns[2].patterns[1].end  }
#template.patterns[1].begin  groups 1
#template-call.patterns[1].begin  groups 1
#template-type.patterns[1].begin  ([$_[:alpha:]][$_[:alnum:]]*)?(`)
#template-type.patterns[1].end  `
#template-type-substitution-element.begin  \$\{
#template-type-substitution-element.end  }
#type.patterns[13].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(readonly)(?![$_[:…
#type-alias-declaration.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#type-alias-declaration.end  (?=[;}]|^\s*$|^\s*(?:abstract|async|\bawait\s+\busing(?=\s+…
#type-alias-declaration.patterns[2].begin  (=)\s*(intrinsic)(?![$_[:alnum:]])(?:(?=\.\.\.)|(?!\.))
#type-alias-declaration.patterns[2].end  (?=[;}]|^\s*$|^\s*(?:abstract|async|\bawait\s+\busing(?=\s+…
#type-alias-declaration.patterns[3].begin  (=)\s*
#type-alias-declaration.patterns[3].end  (?=[;}]|^\s*$|^\s*(?:abstract|async|\bawait\s+\busing(?=\s+…
#type-annotation.patterns[1].begin  (:)
#type-annotation.patterns[1].end  (?<![\&:|])((?=[]),;}]|//)|(?==[^>])|(?=^\s*$)|((?<=[]$)>_}…
#type-arguments-body.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(_)(?![$_[:alnum:]…
#type-builtin-literals.match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(this|true|false|u…
#type-conditional.patterns[0].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(extends)\s+
#type-conditional.patterns[0].end  (?<=:)
#type-conditional.patterns[0].patterns[0].begin  \?
#type-conditional.patterns[0].patterns[0].end  :
#type-fn-type-parameters.patterns[0].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(abstract)\s+)?…
#type-fn-type-parameters.patterns[0].end  (?<=>)
#type-fn-type-parameters.patterns[1].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(abstract)\s+)?…
#type-fn-type-parameters.patterns[1].end  (?<=\))
#type-fn-type-parameters.patterns[2].begin  ((?=\(\s*((\))|(\.\.\.)|([$_[:alnum:]]+\s*(([,:=?])|(\)\s*=…  fuzz
#type-fn-type-parameters.patterns[2].end  (?<=\))  fuzz
#type-function-return-type.patterns[0].begin  (=>)(?=\s*\S)  fuzz
#type-function-return-type.patterns[0].end  (?<!=>)(?<![\&|])(?=[]),:;=>?{}]|//|$)
#type-function-return-type.patterns[1].begin  =>
#type-function-return-type.patterns[1].end  (?<!=>)(?<![\&|])((?=[]),:;=>?{}]|//|^\s*$)|((?<=\S)(?=\s*$…
#type-function-return-type-core.patterns[1].begin  (?<==>)(?=\s*\{)  fuzz
#type-function-return-type-core.patterns[1].end  (?<=})
#type-infer.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(infer)\s+([$_[:al…
#type-name.patterns[0].begin  ([$_[:alpha:]][$_[:alnum:]]*)\s*(?:(\.)|(\?\.(?!\s*\d)))\s*…
#type-name.patterns[0].end  (>)
#type-name.patterns[2].match  groups 3
#type-object.patterns[6].begin  \.\.\.
#type-object.patterns[6].end  (?=[,;}]|$)|(?<=})
#type-operators.patterns[2].begin  ([\&|])(?=\s*\{)
#type-operators.patterns[2].end  (?<=})
#type-operators.patterns[3].begin  [\&|]
#type-operators.patterns[3].end  (?=\S)
#type-operators.patterns[4].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))keyof(?![$_[:alnum…
#type-operators.patterns[5].match  ([:?])
#type-operators.patterns[6].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))import(?=\s*\()
#type-parameters.begin  (<)
#type-parameters.end  (>)
#type-parameters.patterns[1].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(extends|in|out|co…
#type-parameters.patterns[4].match  (=)(?!>)
#type-paren-or-function-parameters.begin  \(
#type-paren-or-function-parameters.end  \)
#type-paren-or-function-parameters.patterns[0].match  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(public|private…
#type-paren-or-function-parameters.patterns[1].match  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(public|private…
#type-paren-or-function-parameters.patterns[3].match  ,
#type-predicate-operator.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(asserts)\s+)?(…
#type-predicate-operator.patterns[1].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(asserts)\s+(?!is)…
#type-predicate-operator.patterns[2].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))asserts(?![$_[:aln…
#type-predicate-operator.patterns[3].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))is(?![$_[:alnum:]]…
#type-tuple.begin  \[
#type-tuple.end  ]
#type-tuple.patterns[0].match  \.\.\.
#type-tuple.patterns[1].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))([$_[:alpha:]][$_[…
#var-expr.patterns[0].patterns[0].begin  groups 1, 2
#var-expr.patterns[1].begin  groups 2
#var-expr.patterns[1].patterns[0].begin  groups 2
#var-expr.patterns[1].patterns[5].begin  (,)\s*(?=$|//)  fuzz
#var-expr.patterns[1].patterns[5].end  (?<!,)(((?=[;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))…  fuzz
#var-expr.patterns[2].begin  (?=(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s…
#var-expr.patterns[2].end  (?!(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s…
#var-expr.patterns[2].patterns[0].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#var-expr.patterns[2].patterns[0].end  (?=\S)
#var-expr.patterns[2].patterns[4].begin  (,)\s*((?!\S)|(?=//))
#var-expr.patterns[2].patterns[4].end  (?<!,)(((?=[;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))…
#var-single-variable.patterns[0].begin  ([$_[:alpha:]][$_[:alnum:]]*)(!)?(?=\s*(=\s*(((async\s+)?((…
#var-single-variable.patterns[0].end  (?=$|^|[,;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(of…
#var-single-variable.patterns[1].begin  (\p{upper}[$_\d[:upper:]]*)(?![$_[:alnum:]])(!)?
#var-single-variable.patterns[1].end  (?=$|^|[,;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(of…
#var-single-variable.patterns[2].begin  groups 2
```

### `json`

```text
#array.patterns[2].match  [^]\s]  fuzz
#comments.patterns[0].begin  /\*\*(?!/)
#comments.patterns[0].end  \*/
#comments.patterns[1].begin  /\*  fuzz
#comments.patterns[1].end  \*/  fuzz
#comments.patterns[2].match  (//).*$\n?  fuzz
#object.patterns[2].patterns[1].match  [^,\s]  fuzz
#object.patterns[3].match  [^}\s]  fuzz
#stringcontent.patterns[1].match  \\.  fuzz
```

### `jsonc`

```text
#comments.patterns[0].begin  /\*\*(?!/)
#comments.patterns[0].end  \*/
#stringcontent.patterns[0].match  \\(?:["/\\bfnrt]|u\h{4})
#stringcontent.patterns[1].match  \\.
```

### `jsonl`

```text
#array.patterns[2].match  [^]\s]  fuzz
#comments.patterns[0].begin  /\*\*(?!/)
#comments.patterns[0].end  \*/
#comments.patterns[1].begin  /\*
#comments.patterns[1].end  \*/
#constant.match  \b(?:true|false|null)\b  fuzz
#number.match  -?(?:0|[1-9]\d*)(?:(?:\.\d+)?(?:[Ee][-+]?\d+)?)?  fuzz
#object.patterns[2].patterns[1].match  [^,\s]  fuzz
#object.patterns[3].match  [^}\s]  fuzz
#stringcontent.patterns[1].match  \\.  fuzz
```

### `jsx`

```text
#access-modifier.match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(abstract|declare|…
#array-binding-pattern.begin  (?:(\.\.\.)\s*)?(\[)
#array-binding-pattern.end  ]
#array-binding-pattern-const.begin  groups 1
#arrow-function.patterns[0].match  groups 1
#arrow-return-type.begin  (?<=\))\s*(:)
#arrow-return-type.end  (?==>|\{|^(\s*(export|function|class|interface|let|var|\bus…
#arrow-return-type-body.patterns[0].begin  (?<=:)(?=\s*\{)
#arrow-return-type-body.patterns[0].end  (?<=})
#async-modifier.match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(async)(?![$_[:aln…
#class-declaration.begin  groups 1, 2, 3
#class-expression.begin  groups 1
#class-or-interface-body.patterns[2].begin  (?<=:)\s*
#class-or-interface-body.patterns[2].end  (?=[-\])+,:;}\s]|^\s*$|^\s*(?:abstract|async|\bawait\s+\bus…
#class-or-interface-heritage.patterns[4].match  ([$_[:alpha:]][$_[:alnum:]]*)\s*(?:(\.)|(\?\.(?!\s*\d)))(?=…
#comment.patterns[1].begin  (/\*)(?:\s*((@)internal)(?=\s|(\*/)))?  fuzz
#comment.patterns[1].end  \*/
#comment.patterns[2].begin  groups 1, 4, 5
#control-statement.patterns[2].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(catch|finally|thr…
#control-statement.patterns[3].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(break|continue|go…
#control-statement.patterns[4].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(break|continue|do…
#control-statement.patterns[6].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(case|default|swit…  fuzz
#control-statement.patterns[9].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(with)(?![$_[:alnu…
#control-statement.patterns[10].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(package)(?![$_[:a…
#control-statement.patterns[11].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(debugger)(?![$_[:…
#declaration.patterns[11].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(declare|export)(?…  fuzz
#decorator.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))@
#decorator.end  (?=\s)
#destructuring-parameter.patterns[0].begin  groups 1
#destructuring-parameter.patterns[1].begin  (?<![:=])\s*(?:(\.\.\.)\s*)?(\[)  fuzz
#destructuring-parameter.patterns[1].end  ]  fuzz
#destructuring-variable.patterns[0].begin  (?<![:=]|^of|[^$._[:alnum:]]of|^in|[^$._[:alnum:]]in)\s*(?=…
#destructuring-variable.patterns[0].end  (?=$|^|[,;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(of…
#destructuring-variable.patterns[1].begin  (?<![:=]|^of|[^$._[:alnum:]]of|^in|[^$._[:alnum:]]in)\s*(?=…
#destructuring-variable.patterns[1].end  (?=$|^|[,;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(of…
#destructuring-variable-rest.match  (?:(\.\.\.)\s*)?([$_[:alpha:]][$_[:alnum:]]*)
#directives.begin  ^(///)\s*(?=<(reference|amd-dependency|amd-module)(\s+(path…
#directives.end  (?=$)
#directives.patterns[0].begin  (<)(reference|amd-dependency|amd-module)
#directives.patterns[0].end  />
#directives.patterns[0].patterns[0].match  path|types|no-default-lib|lib|name|resolution-mode
#directives.patterns[0].patterns[1].match  =
#docblock.patterns[0].match  ((@)a(?:ccess|pi))\s+(p(?:rivate|rotected|ublic))\b
#docblock.patterns[1].match  ((@)author)\s+([^*/<>@\s](?:[^*/<>@]|\*[^/])*)(?:\s*(<)([^>…
#docblock.patterns[2].match  ((@)borrows)\s+((?:[^*/@\s]|\*[^/])+)\s+(as)\s+((?:[^*/@\s]…
#docblock.patterns[3].begin  ((@)example)\s+
#docblock.patterns[3].end  (?=@|\*/)
#docblock.patterns[3].patterns[0].match  ^\s\*\s+
#docblock.patterns[3].patterns[1].begin  \G(<)caption(>)
#docblock.patterns[3].patterns[1].end  (</)caption(>)|(?=\*/)
#docblock.patterns[3].patterns[2].match  [^*@\s](?:[^*]|\*[^/])*
#docblock.patterns[4].match  ((@)kind)\s+(class|constant|event|external|file|function|me…
#docblock.patterns[5].match  ((@)see)\s+(?:((?=https?://)(?:[^*\s]|\*[^/])+)|((?!https?:…
#docblock.patterns[6].match  ((@)template)\s+([$A-Z_a-z][]$.\[\w]*(?:\s*,\s*[$A-Z_a-z][]…
#docblock.patterns[7].begin  ((@)template)\s+(?=\{)
#docblock.patterns[7].end  (?=\s|\*/|[^]$A-\[_a-{}])
#docblock.patterns[7].patterns[1].match  ([$A-Z_a-z][]$.\[\w]*)
#docblock.patterns[8].match  ((@)(?:arg|argument|const|constant|member|namespace|param|v…
#docblock.patterns[9].begin  ((@)typedef)\s+(?=\{)
#docblock.patterns[9].end  (?=\s|\*/|[^]$A-\[_a-{}])
#docblock.patterns[9].patterns[1].match  (?:[^*/@\s]|\*[^/])+
#docblock.patterns[10].patterns[2].match  (\[)\s*[$\w]+(?:(?:\[])?\.[$\w]+)*(?:\s*(=)\s*((?>"(?:\*(?!…
#docblock.patterns[12].match  ((@)(?:alias|augments|callback|constructs|emits|event|fires…
#docblock.patterns[13].begin  ((@)(?:default(?:value)?|license|version))\s+((["']))
#docblock.patterns[13].end  (\3)|(?=$|\*/)
#docblock.patterns[14].match  ((@)(?:default(?:value)?|license|tutorial|variation|version…
#docblock.patterns[15].match  (@)(?:abstract|access|alias|api|arg|argument|async|attribut…
#docblock.patterns[17].match  ((@)[$_[:alpha:]][$_[:alnum:]]*)(?=\s+)
#enum-declaration.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#enum-declaration.end  (?<=})
#enum-declaration.patterns[1].begin  \{
#enum-declaration.patterns[1].end  }
#enum-declaration.patterns[1].patterns[1].begin  ([$_[:alpha:]][$_[:alnum:]]*)
#enum-declaration.patterns[1].patterns[1].end  (?=[,}]|$)
#enum-declaration.patterns[1].patterns[2].begin  (?=(('([^'\\]|\\.)*')|("([^"\\]|\\.)*")|(`([^\\`]|\\.)*`)|(…
#enum-declaration.patterns[1].patterns[2].end  (?=[,}]|$)
#export-declaration.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(export)\s+(as)\s+…
#export-declaration.patterns[1].begin  groups 2, 3
#export-declaration.patterns[2].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(export)(?:\s+(typ…
#export-declaration.patterns[2].end  (?=$|;|^\s*$|^\s*(?:abstract|async|\bawait\s+\busing(?=\s+(…
#expression-inside-possibly-arrow-parens.patterns[5].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(override|public|p…
#expression-inside-possibly-arrow-parens.patterns[6].match  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(override|publi…
#expression-inside-possibly-arrow-parens.patterns[7].match  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(override|publi…
#expression-inside-possibly-arrow-parens.patterns[10].match  ,
#expression-operators.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(await)(?![$_[:aln…
#expression-operators.patterns[1].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(yield)(?![$_[:aln…
#expression-operators.patterns[1].end  \*
#expression-operators.patterns[2].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(yield)(?![$_[:aln…
#expression-operators.patterns[3].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))delete(?![$_[:alnu…
#expression-operators.patterns[6].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))instanceof(?![$_[:…
#expression-operators.patterns[7].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))new(?![$_[:alnum:]…
#expression-operators.patterns[9].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))void(?![$_[:alnum:…
#expression-operators.patterns[10].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(as)\s+(const)(?=\…
#expression-operators.patterns[11].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(as)|(satisfies…
#expression-operators.patterns[11].end  (?=^|[-\])+,:;>?}]|\|\||&&|!==|$|((?<![$_[:alnum:]])(?:(?<=…
#expression-operators.patterns[13].match  (?:\*|(?<!\()/|[-%+])=
#expression-operators.patterns[14].match  (?:[\&^]|<<|>>>??|\|)=
#expression-operators.patterns[15].match  <<|>>>?
#expression-operators.patterns[18].match  (?<=[$_[:alnum:]])(!)\s*(?:(/=)|(/)(?![*/]))
#expression-operators.patterns[20].match  [\&^|~]
#expression-operators.patterns[22].match  --
#expression-operators.patterns[25].begin  (?<=[]$)_[:alnum:]])\s*(?=(/\*([^*]|(\*[^/]))*\*/\s*)+(?:(/…
#expression-operators.patterns[25].end  (/=)|(/)(?!\*([^*]|(\*[^/]))*\*/)
#expression-operators.patterns[26].match  (?<=[]$)_[:alnum:]])\s*(?:(/=)|(/)(?![*/]))
#field-declaration.begin  groups 1
#field-declaration.patterns[6].match  (#?[$_[:alpha:]][$_[:alnum:]]*)(?:(\?)|(!))?(?=\s*\s*(=\s*(…
#field-declaration.patterns[8].match  \?
#field-declaration.patterns[9].match  !
#for-loop.patterns[1].match  await
#function-body.patterns[6].match  \*
#function-call.patterns[1].begin  (?=(((([$_[:alpha:]][$_[:alnum:]]*)(\s*\??\.\s*(#?[$_[:alph…
#function-call.patterns[1].end  (?<=>)(?!(((([$_[:alpha:]][$_[:alnum:]]*)(\s*\??\.\s*(#?[$_…
#function-call.patterns[1].patterns[0].begin  (?=(([$_[:alpha:]][$_[:alnum:]]*)(\s*\??\.\s*(#?[$_[:alpha:…
#function-call.patterns[1].patterns[0].end  (?=(<\s*[(\[{]\s*)$)
#function-call-optionals.patterns[0].match  \?\.
#function-call-optionals.patterns[1].match  !
#function-declaration.begin  groups 1, 2, 3, 5
#function-expression.begin  groups 1, 3
#function-name.match  [$_[:alpha:]][$_[:alnum:]]*  fuzz
#identifiers.patterns[1].match  groups 1, 2
#identifiers.patterns[2].match  groups 2
#identifiers.patterns[3].match  groups 2
#if-statement.patterns[0].patterns[2].begin  (?<=\))\s*/(?![*/])(?=(?:[^/\[\\]|\\.|\[([^]\\]|\\.)*])+/([…
#if-statement.patterns[0].patterns[2].end  (/)([dgimsuvy]*)
#import-declaration.begin  groups 1, 2, 4
#import-equals-declaration.patterns[0].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#import-equals-declaration.patterns[0].end  \)
#import-equals-declaration.patterns[1].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#import-equals-declaration.patterns[1].end  (?=;|$|^)
#import-equals-declaration.patterns[1].patterns[2].match  ([$_[:alpha:]][$_[:alnum:]]*)\s*(?:(\.)|(\?\.(?!\s*\d)))
#import-equals-declaration.patterns[1].patterns[3].match  ([$_[:alpha:]][$_[:alnum:]]*)
#import-export-assert-clause.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(with)|(assert)…
#import-export-assert-clause.end  }
#import-export-assert-clause.patterns[2].match  [$_[:alpha:]][$_[:alnum:]]*\s*(?=(/\*([^*]|(\*[^/]))*\*/\s*…
#import-export-assert-clause.patterns[3].match  :
#import-export-clause.patterns[1].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(type)\s+)?(?…
#import-export-clause.patterns[3].match  \*
#import-export-clause.patterns[4].match  \b(default)\b
#import-export-clause.patterns[5].match  groups 1, 3
#import-export-declaration.patterns[3].match  \bfrom\b
#indexer-declaration.begin  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(readonly)\s*)?…
#indexer-declaration.end  (])\s*(\?\s*)?|$
#indexer-mapped-type-declaration.begin  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))([-+])?(readonl…
#indexer-mapped-type-declaration.end  (])([-+])?\s*(\?\s*)?|$
#indexer-mapped-type-declaration.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(as)\s+
#inline-tags.patterns[0].match  (\[)[^]]+(])(?=\{@(?:link|linkcode|linkplain|tutorial))
#inline-tags.patterns[1].begin  (\{)((@)(?:link(?:code|plain)?|tutorial))\s*
#inline-tags.patterns[1].end  }|(?=\*/)
#inline-tags.patterns[1].patterns[0].match  \G((?=https?://)(?:[^*|}\s]|\*/)+)(\|)?
#inline-tags.patterns[1].patterns[1].match  \G((?:[^*@{|}\s]|\*[^/])+)(\|)?
#instanceof-expr.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(instanceof)(?![$_…
#instanceof-expr.end  (?<=\))|(?=[-\])+,:;>?}]|\|\||&&|!==|$|([!=]==?)|(([\&^|~]\…
#interface-declaration.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#interface-declaration.end  (?<=})
#interface-declaration.patterns[2].match  [$_[:alpha:]][$_[:alnum:]]*
#jsx-entities.patterns[0].match  (&)([0-9A-Za-z]+|#[0-9]+|#x\h+)(;)
#jsx-string-single-quoted.begin  '
#jsx-string-single-quoted.end  '
#jsx-tag.end  groups 3, 4
#jsx-tag.patterns[0].begin  groups 2, 3
#jsx-tag-attribute-name.match  groups 1, 2
#jsx-tag-attributes-illegal.match  \S+  fuzz
#jsx-tag-without-attributes.begin  groups 2, 3
#jsx-tag-without-attributes.end  groups 2, 3
#label.patterns[0].begin  ([$_[:alpha:]][$_[:alnum:]]*)\s*(:)(?=\s*\{)
#label.patterns[0].end  (?<=})
#label.patterns[1].match  ([$_[:alpha:]][$_[:alnum:]]*)\s*(:)  fuzz
#method-declaration.patterns[0].begin  groups 1, 2, 3, 4
#method-declaration.patterns[1].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(override)\s+…
#method-declaration.patterns[1].end  (?=[,;}]|$)|(?<=})
#method-declaration.patterns[2].begin  groups 1, 2, 3, 4, 5, 6
#method-declaration-name.patterns[4].match  \?
#namespace-declaration.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#namespace-declaration.end  (?<=})|(?=;|^\s*$|^\s*(?:abstract|async|\bawait\s+\busing(?…
#namespace-declaration.patterns[2].match  ([$_[:alpha:]][$_[:alnum:]]*)
#numeric-literal.patterns[0].match  \b(?<!\$)0[Xx]\h[_\h]*(n)?\b(?!\$)
#numeric-literal.patterns[1].match  \b(?<!\$)0[Bb][01][01_]*(n)?\b(?!\$)
#numeric-literal.patterns[2].match  \b(?<!\$)0[Oo]?[0-7][0-7_]*(n)?\b(?!\$)
#numeric-literal.patterns[3].match  groups 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 12, 13, 14
#numericConstant-literal.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))NaN(?![$_[:alnum:]…
#numericConstant-literal.patterns[1].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))Infinity(?![$_[:al…
#object-binding-element.patterns[1].begin  (?=(\b((?<!\$)0[Xx]\h[_\h]*(n)?\b(?!\$))|\b((?<!\$)0[Bb][01…
#object-binding-element.patterns[1].end  (?=[,}])
#object-binding-element-const.patterns[1].begin  (?=(\b((?<!\$)0[Xx]\h[_\h]*(n)?\b(?!\$))|\b((?<!\$)0[Bb][01…
#object-binding-element-const.patterns[1].end  (?=[,}])
#object-binding-pattern.begin  (?:(\.\.\.)\s*)?(\{)
#object-binding-pattern.end  }
#object-binding-pattern-const.begin  groups 1
#object-identifiers.patterns[1].match  groups 2
#object-identifiers.patterns[2].match  groups 1
#object-literal-method-declaration.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(async)\s+)?(…  fuzz
#object-literal-method-declaration.end  (?=[,;}])|(?<=})  fuzz
#object-literal-method-declaration.patterns[2].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(async)\s+)?(…  fuzz
#object-literal-method-declaration.patterns[2].end  (?=[(<])  fuzz
#object-member.patterns[2].begin  (?=\[)  fuzz
#object-member.patterns[2].end  (?=:)|((?<=])(?=\s*[(<]))
#object-member.patterns[4].begin  (?=\b((?<!\$)0[Xx]\h[_\h]*(n)?\b(?!\$))|\b((?<!\$)0[Bb][01]…
#object-member.patterns[4].end  (?=:)|(?=\s*([(,<}])|(\s+as|satisifies\s+))
#object-member.patterns[5].begin  (?<=[]"'`])(?=\s*[(<])
#object-member.patterns[5].end  (?=[,;}])|(?<=})
#object-member.patterns[6].match  (?![$_[:alpha:]])(\d+)\s*(?=(/\*([^*]|(\*[^/]))*\*/\s*)*:)
#object-member.patterns[7].match  ([$_[:alpha:]][$_[:alnum:]]*)\s*(?=(/\*([^*]|(\*[^/]))*\*/\…
#object-member.patterns[11].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(as)\s+(const)(?=\…
#object-member.patterns[12].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(as)|(satisfies…
#object-member.patterns[12].end  (?=[-\])+,:;>?}]|\|\||&&|!==|$|^|((?<![$_[:alnum:]])(?:(?<=…
#object-member.patterns[13].begin  (?=[$_[:alpha:]][$_[:alnum:]]*\s*=)  fuzz
#object-member.patterns[13].end  (?=[,}]|$|//|/\*)  fuzz
#object-member.patterns[14].patterns[0].begin  (?<=:)\s*(async)?(?=\s*(<\s*(((const\s+)?[$_[:alpha:]])|(\{…
#object-member.patterns[14].patterns[0].end  (?<=\))
#object-member.patterns[14].patterns[0].patterns[1].begin  \(
#object-member.patterns[14].patterns[0].patterns[1].end  \)
#object-member.patterns[14].patterns[1].begin  (?<=:)\s*(async)?\s*(\()(?=\s*((([\[{]\s*)?)$|((\{([^{}]|(\…
#object-member.patterns[14].patterns[1].end  \)
#object-member.patterns[14].patterns[2].begin  (?<=:)\s*(async)?\s*(?=<\s*$)
#object-member.patterns[14].patterns[2].end  (?<=>)
#object-member.patterns[14].patterns[3].begin  (?<=>)\s*(\()(?=\s*((([\[{]\s*)?)$|((\{([^{}]|(\{([^{}]|\{[…
#object-member.patterns[14].patterns[3].end  \)
#parameter-array-binding-pattern.begin  (?:(\.\.\.)\s*)?(\[)
#parameter-array-binding-pattern.end  ]
#parameter-name.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(override|public|p…
#parameter-name.patterns[1].match  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(override|publi…
#parameter-name.patterns[2].match  groups 1, 2, 3, 5
#parameter-object-binding-pattern.begin  (?:(\.\.\.)\s*)?(\{)  fuzz
#parameter-object-binding-pattern.end  }  fuzz
#parameter-type-annotation.patterns[0].begin  (:)
#parameter-type-annotation.patterns[0].end  (?=[),])|(?==[^>])
#paren-expression-possibly-arrow.patterns[0].begin  groups 1
#paren-expression-possibly-arrow.patterns[1].begin  groups 1
#possibly-arrow-return-type.begin  (?<=\)|^)\s*(:)(?=\s*([^()<>{}]|<([^<>]|<([^<>]|<[^<>]+>)+>…
#possibly-arrow-return-type.end  (?==>|\{|^(\s*(export|function|class|interface|let|var|\bus…
#property-accessor.match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(accessor|get|set)…
#punctuation-accessor.match  groups 2
#qstring-double.end  groups 2 (fuzz)
#qstring-single.end  groups 2 (fuzz)
#regex.patterns[1].begin  ((?<![]$)_[:alnum:]]|\+\+|--|}|\*/)|((?<=^return|[^$._[:aln…
#regex.patterns[1].end  (/)([dgimsuvy]*)
#regex-character-class.patterns[0].match  \\[DSWdfnrstvw]|\.
#regex-character-class.patterns[1].match  \\([0-7]{3}|x\h{2}|u\h{4})
#regex-character-class.patterns[2].match  \\c[A-Z]
#regex-character-class.patterns[3].match  \\.
#regexp.patterns[0].match  \\[Bb]|[$^]
#regexp.patterns[1].match  \\(?:[1-9]\d*|k<([$A-Z_a-z][$\w]*)>)
#regexp.patterns[2].match  [*+?]|\{(\d+,\d+|\d+,|,\d+|\d+)}\??
#regexp.patterns[3].match  \|
#regexp.patterns[4].begin  (\()((\?=)|(\?!)|(\?<=)|(\?<!))
#regexp.patterns[4].end  (\))
#regexp.patterns[5].begin  \((?:(\?:)|\?<([$A-Z_a-z][$\w]*)>)?
#regexp.patterns[5].end  \)
#regexp.patterns[6].begin  (\[)(\^)?
#regexp.patterns[6].end  (])
#regexp.patterns[6].patterns[0].match  (?:.|(\\(?:[0-7]{3}|x\h{2}|u\h{4}))|(\\c[A-Z])|(\\.))-(?:[^…
#return-type.patterns[0].begin  (?<=\))\s*(:)(?=\s*\S)
#return-type.patterns[0].end  (?<![\&:|])(?=$|^|[,;{}]|//)
#return-type.patterns[1].begin  (?<=\))\s*(:)
#return-type.patterns[1].end  (?<![\&:|])((?=[,;{}]|//|^\s*$)|((?<=\S)(?=\s*$)))
#return-type-core.patterns[1].begin  (?<=[\&:|])(?=\s*\{)
#return-type-core.patterns[1].end  (?<=})
#shebang.match  \A(#!).*(?=$)
#single-line-comment-consuming-line-ending.begin  (^[\t ]+)?((//)(?:\s*((@)internal)(?=\s|$))?)
#single-line-comment-consuming-line-ending.end  (?=^)
#string-character-escape.match  \\(x\h{2}|u\h{4}|u\{\h+}|[012][0-7]{0,2}|3[0-6][0-7]?|37[0-…
#support-function-call-identifiers.patterns[4].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))import(?=\s*\(\s*[…
#support-objects.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(arguments)\b(?!\$)
#support-objects.patterns[2].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(import)\s*(?:(\.)…
#support-objects.patterns[3].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(new)\s*(?:(\.)|(\…
#support-objects.patterns[4].match  groups 2, 4
#support-objects.patterns[5].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(exports)|(modu…
#switch-statement.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?=\bswitch\s*\()
#switch-statement.end  }
#switch-statement.patterns[1].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(switch)\s*(\()
#switch-statement.patterns[1].end  \)
#switch-statement.patterns[2].begin  \{
#switch-statement.patterns[2].end  (?=})
#switch-statement.patterns[2].patterns[0].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(case|default(?=:)…
#switch-statement.patterns[2].patterns[0].end  (?=:)
#switch-statement.patterns[2].patterns[1].begin  (:)\s*(\{)
#switch-statement.patterns[2].patterns[1].end  }
#switch-statement.patterns[2].patterns[2].match  (:)
#template.patterns[1].begin  groups 1
#template-call.patterns[0].begin  (?=(([$_[:alpha:]][$_[:alnum:]]*\s*\??\.\s*)*|(\??\.\s*)?)(…
#template-call.patterns[0].end  (?=`)
#template-call.patterns[0].patterns[0].begin  (?=(([$_[:alpha:]][$_[:alnum:]]*\s*\??\.\s*)*|(\??\.\s*)?)(…
#template-call.patterns[0].patterns[0].end  (?=(<\s*(((keyof|infer|typeof|readonly)\s+)|(([$_[:alpha:]]…
#template-call.patterns[0].patterns[0].patterns[1].match  ([$_[:alpha:]][$_[:alnum:]]*)
#template-call.patterns[1].begin  ([$_[:alpha:]][$_[:alnum:]]*)?\s*(?=(<\s*(((keyof|infer|typ…
#template-call.patterns[1].end  (?=`)
#template-type.patterns[1].begin  ([$_[:alpha:]][$_[:alnum:]]*)?(`)
#template-type.patterns[1].end  `
#template-type-substitution-element.begin  \$\{
#template-type-substitution-element.end  }
#type.patterns[13].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(readonly)(?![$_[:…
#type-alias-declaration.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#type-alias-declaration.end  (?=[;}]|^\s*$|^\s*(?:abstract|async|\bawait\s+\busing(?=\s+…
#type-alias-declaration.patterns[2].begin  (=)\s*(intrinsic)(?![$_[:alnum:]])(?:(?=\.\.\.)|(?!\.))
#type-alias-declaration.patterns[2].end  (?=[;}]|^\s*$|^\s*(?:abstract|async|\bawait\s+\busing(?=\s+…
#type-alias-declaration.patterns[3].begin  (=)\s*
#type-alias-declaration.patterns[3].end  (?=[;}]|^\s*$|^\s*(?:abstract|async|\bawait\s+\busing(?=\s+…
#type-annotation.patterns[0].begin  (:)(?=\s*\S)
#type-annotation.patterns[0].end  (?<![\&:|])(?!\s*[\&|]\s+)((?=^|[]),;}]|//)|(?==[^>])|((?<=…
#type-annotation.patterns[1].begin  (:)
#type-annotation.patterns[1].end  (?<![\&:|])((?=[]),;}]|//)|(?==[^>])|(?=^\s*$)|((?<=[]$)>_}…
#type-arguments.begin  <
#type-arguments.end  >
#type-arguments-body.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(_)(?![$_[:alnum:]…
#type-builtin-literals.match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(this|true|false|u…
#type-conditional.patterns[0].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(extends)\s+
#type-conditional.patterns[0].end  (?<=:)
#type-conditional.patterns[0].patterns[0].begin  \?
#type-conditional.patterns[0].patterns[0].end  :
#type-fn-type-parameters.patterns[0].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(abstract)\s+)?…
#type-fn-type-parameters.patterns[0].end  (?<=>)
#type-fn-type-parameters.patterns[1].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(abstract)\s+)?…
#type-fn-type-parameters.patterns[1].end  (?<=\))
#type-fn-type-parameters.patterns[2].begin  ((?=\(\s*((\))|(\.\.\.)|([$_[:alnum:]]+\s*(([,:=?])|(\)\s*=…  fuzz
#type-fn-type-parameters.patterns[2].end  (?<=\))  fuzz
#type-function-return-type.patterns[0].begin  (=>)(?=\s*\S)  fuzz
#type-function-return-type.patterns[0].end  (?<!=>)(?<![\&|])(?=[]),:;=>?{}]|//|$)
#type-function-return-type.patterns[1].begin  =>
#type-function-return-type.patterns[1].end  (?<!=>)(?<![\&|])((?=[]),:;=>?{}]|//|^\s*$)|((?<=\S)(?=\s*$…
#type-function-return-type-core.patterns[1].begin  (?<==>)(?=\s*\{)  fuzz
#type-function-return-type-core.patterns[1].end  (?<=})
#type-infer.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(infer)\s+([$_[:al…
#type-name.patterns[0].begin  ([$_[:alpha:]][$_[:alnum:]]*)\s*(?:(\.)|(\?\.(?!\s*\d)))\s*…
#type-name.patterns[0].end  (>)
#type-name.patterns[1].begin  ([$_[:alpha:]][$_[:alnum:]]*)\s*(<)
#type-name.patterns[1].end  (>)
#type-name.patterns[2].match  ([$_[:alpha:]][$_[:alnum:]]*)\s*(?:(\.)|(\?\.(?!\s*\d)))  fuzz
#type-name.patterns[3].match  [$_[:alpha:]][$_[:alnum:]]*  fuzz
#type-object.begin  \{  fuzz
#type-object.end  }
#type-object.patterns[6].begin  \.\.\.
#type-object.patterns[6].end  (?=[,;}]|$)|(?<=})
#type-operators.patterns[2].begin  ([\&|])(?=\s*\{)
#type-operators.patterns[2].end  (?<=})
#type-operators.patterns[3].begin  [\&|]
#type-operators.patterns[3].end  (?=\S)
#type-operators.patterns[4].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))keyof(?![$_[:alnum…
#type-operators.patterns[5].match  ([:?])
#type-operators.patterns[6].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))import(?=\s*\()
#type-parameters.begin  (<)  fuzz
#type-parameters.end  (>)
#type-parameters.patterns[1].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(extends|in|out|co…
#type-parameters.patterns[4].match  (=)(?!>)
#type-paren-or-function-parameters.begin  \(  fuzz
#type-paren-or-function-parameters.end  \)  fuzz
#type-paren-or-function-parameters.patterns[0].match  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(public|private…
#type-paren-or-function-parameters.patterns[1].match  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(public|private…
#type-paren-or-function-parameters.patterns[3].match  ,  fuzz
#type-predicate-operator.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(asserts)\s+)?(…
#type-predicate-operator.patterns[1].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(asserts)\s+(?!is)…
#type-predicate-operator.patterns[2].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))asserts(?![$_[:aln…
#type-predicate-operator.patterns[3].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))is(?![$_[:alnum:]]…
#type-primitive.match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(string|number|big…
#type-tuple.begin  \[  fuzz
#type-tuple.end  ]  fuzz
#type-tuple.patterns[0].match  \.\.\.
#type-tuple.patterns[1].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))([$_[:alpha:]][$_[…
#var-expr.patterns[0].patterns[0].begin  groups 1, 2
#var-expr.patterns[1].begin  groups 1, 2
#var-expr.patterns[1].patterns[0].begin  groups 1, 2
#var-expr.patterns[1].patterns[5].begin  (,)\s*(?=$|//)  fuzz
#var-expr.patterns[1].patterns[5].end  (?<!,)(((?=[;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))…  fuzz
#var-expr.patterns[2].begin  (?=(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s…
#var-expr.patterns[2].end  (?!(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s…
#var-expr.patterns[2].patterns[0].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#var-expr.patterns[2].patterns[0].end  (?=\S)
#var-expr.patterns[2].patterns[4].begin  (,)\s*((?!\S)|(?=//))
#var-expr.patterns[2].patterns[4].end  (?<!,)(((?=[;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))…
#var-single-variable.patterns[0].begin  groups 2
#var-single-variable.patterns[1].begin  (\p{upper}[$_\d[:upper:]]*)(?![$_[:alnum:]])(!)?
#var-single-variable.patterns[1].end  (?=$|^|[,;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(of…
#var-single-variable.patterns[2].begin  groups 2
```

### `kotlin`

```text
#annotation-site-list.begin  (?<!\w)(@\w+):\s*\[
#annotation-site-list.end  ]
#comment-javadoc.patterns[0].patterns[2].match  (@param)\[(\S+)]
#comment-javadoc.patterns[0].patterns[4].match  \{(@link)\s+(\S+)?#([$\w]+\s*\([^()]*\)).*}
#key.match  \b(\w=)\s*(=)
#operators.patterns[5].match  (--|\+\+)
#string-multiline.patterns[0].match  \\.
#type-alias.match  groups 3
#type-parameter.patterns[1].match  \b(in|out)\b
#variable-declaration.match  groups 2
```

### `less`

```text
#at-charset.begin  \s*((@)charset)\b\s*
#at-charset.end  \s*((?=;|$))
#at-container.end  \s*(})
#at-container.patterns[0].begin  groups 3
#at-container.patterns[0].end  (?=\{)
#at-container.patterns[0].patterns[0].patterns[1].patterns[0].match  \b(aspect-ratio|block-size|height|inline-size|orientation|w…
#at-container.patterns[0].patterns[0].patterns[1].patterns[2].match  :
#at-container.patterns[0].patterns[0].patterns[1].patterns[3].match  portrait|landscape
#at-container.patterns[0].patterns[0].patterns[1].patterns[5].match  /
#at-container.patterns[1].begin  \s*(\{)
#at-container.patterns[1].end  (?=})
#at-counter-style.begin  \s*((@)counter-style)\b\s+(?:(?i:\b(decimal|none)\b)|(-?(?:…
#at-counter-style.end  \s*(})
#at-custom-media.begin  (?=\s*@custom-media\b)
#at-custom-media.end  \s*(?=;)
#at-custom-media.patterns[0].match  \s*;
#at-custom-media.patterns[1].match  \s*((@)custom-media)(?=.*?)
#at-font-face.begin  \s*((@)font-face)\s*(?=\{|$)
#at-font-face.end  \s*(})
#at-import.patterns[2].begin  (?<=(["'])|(["']\)))\s*
#at-import.patterns[2].end  \s*(?=;)
#at-import.patterns[3].begin  \(
#at-import.patterns[3].end  \)
#at-import.patterns[3].patterns[0].match  reference|inline|less|css|once|multiple|optional
#at-keyframes.begin  \s*((@)keyframes)(?=.*?\{)
#at-keyframes.end  \s*(})
#at-keyframes.patterns[0].begin  \s*(\{)
#at-keyframes.patterns[0].end  (?=})
#at-keyframes.patterns[0].patterns[0].match  \s*(?:(from|to)|((?:\.[0-9]+|[0-9]+(?:\.[0-9]*)?)(%)))\s*,?…
#at-keyframes.patterns[1].begin  \s*(?=[^;{])
#at-keyframes.patterns[1].end  \s*(?=\{)
#at-media.patterns[0].begin  groups 3
#at-namespace.begin  \s*((@)namespace)\s+
#at-namespace.end  ;
#at-namespace.patterns[2].match  (-?(?:[A-Z_a-z[^\x00-\x7F]]|\\(?:\h{1,6}[\t\n\f\s]?|[^\n\f\…
#at-page.match  \s*((@)page)\s*(?:(:)(first|left|right))?\s*(?=\{|$)
#at-supports.begin  (?=\s*@supports\b)
#at-supports.end  (?=\s*)(})
#at-supports.patterns[0].begin  \s*((@)supports)
#at-supports.patterns[0].end  \s*(?=\{)
#at-supports.patterns[1].begin  \s*(\{)
#at-supports.patterns[1].end  (?=})
#at-supports-operators.match  \b(?:and|or|not)\b
#at-supports-parens.begin  \(
#at-supports-parens.end  \)
#attr-function.begin  \b(attr)(?=\()
#attr-function.end  \)
#attr-function.patterns[0].begin  \(
#attr-function.patterns[0].end  (?=\))
#attr-function.patterns[0].patterns[2].begin  (-?(?:[A-Z_a-z[^\x00-\x7F]]|\\(?:\h{1,6}[\t\n\f\s]?|[^\n\f\…
#attr-function.patterns[0].patterns[2].end  (?=\))
#attr-function.patterns[0].patterns[2].patterns[0].match  \b((?i:em|ex|ch|rem)|(?i:v(?:[hw]|min|max))|(?i:cm|mm|q|in|…
#calc-function.begin  \b(calc)(?=\()
#calc-function.end  \)
#calc-function.patterns[0].begin  \(
#calc-function.patterns[0].end  (?=\))
#color-functions.patterns[1].begin  \b(hsla?|hwb|oklab|oklch|lab|lch)(?=\()
#color-functions.patterns[1].end  \)
#color-functions.patterns[1].patterns[0].begin  \(
#color-functions.patterns[1].patterns[0].end  (?=\))
#color-functions.patterns[2].begin  \b(light-dark)(?=\()
#color-functions.patterns[2].end  \)
#color-functions.patterns[2].patterns[0].begin  \(
#color-functions.patterns[2].patterns[0].end  (?=\))
#counter-functions.patterns[0].begin  \b(counter)(?=\()
#counter-functions.patterns[0].end  \)
#counter-functions.patterns[0].patterns[0].begin  \(
#counter-functions.patterns[0].patterns[0].end  (?=\))
#counter-functions.patterns[0].patterns[0].patterns[3].match  --(?:[-\w[^\x00-\x7F]]|\\(?:\h{1,6}[\t\n\f\s]?|[^\n\f\h]))+…
#counter-functions.patterns[0].patterns[0].patterns[4].begin  (?=,)
#counter-functions.patterns[0].patterns[0].patterns[4].end  (?=\))
#counter-functions.patterns[0].patterns[0].patterns[4].patterns[1].match  \b((?i:arabic-indic|armenian|bengali|cambodian|circle|cjk-d…
#counter-functions.patterns[1].begin  \b(counters)(?=\()
#counter-functions.patterns[1].end  \)
#counter-functions.patterns[1].patterns[0].begin  \(
#counter-functions.patterns[1].patterns[0].end  (?=\))
#counter-functions.patterns[1].patterns[0].patterns[0].match  (-?(?:[A-Z_a-z[^\x00-\x7F]]|\\(?:\h{1,6}[\t\n\f\s]?|[^\n\f\…
#counter-functions.patterns[1].patterns[0].patterns[1].begin  (?=,)
#counter-functions.patterns[1].patterns[0].patterns[1].end  (?=\))
#counter-functions.patterns[1].patterns[0].patterns[1].patterns[5].match  \b((?i:arabic-indic|armenian|bengali|cambodian|circle|cjk-d…
#cross-fade-function.patterns[0].begin  \b(cross-fade)(?=\()
#cross-fade-function.patterns[0].end  \)
#cross-fade-function.patterns[0].patterns[0].begin  \(
#cross-fade-function.patterns[0].patterns[0].end  (?=\))
#cubic-bezier-function.begin  \b(cubic-bezier)(\()
#cubic-bezier-function.end  \)
#custom-property-name.match  \s*(--)((?:[-\w[^\x00-\x7F]]|\\(?:\h{1,6}[\t\n\f\s]?|[^\n\f…
#filter-function.begin  \b(filter)(?=\()
#filter-function.end  \)
#filter-function.patterns[0].begin  \(
#filter-function.patterns[0].end  (?=\))
#filter-functions.patterns[1].begin  \b(blur)(?=\()
#filter-functions.patterns[1].end  \)
#filter-functions.patterns[1].patterns[0].begin  \(
#filter-functions.patterns[1].patterns[0].end  (?=\))
#filter-functions.patterns[2].begin  \b(brightness|contrast|grayscale|invert|opacity|saturate|se…
#filter-functions.patterns[2].end  \)
#filter-functions.patterns[2].patterns[0].begin  \(
#filter-functions.patterns[2].patterns[0].end  (?=\))
#filter-functions.patterns[3].begin  \b(drop-shadow)(?=\()
#filter-functions.patterns[3].end  \)
#filter-functions.patterns[3].patterns[0].begin  \(
#filter-functions.patterns[3].patterns[0].end  (?=\))
#filter-functions.patterns[4].begin  \b(hue-rotate)(?=\()
#filter-functions.patterns[4].end  \)
#filter-functions.patterns[4].patterns[0].begin  \(
#filter-functions.patterns[4].patterns[0].end  (?=\))
#fit-content-function.begin  \b(fit-content)(?=\()
#fit-content-function.end  \)
#fit-content-function.patterns[0].begin  \(
#fit-content-function.patterns[0].end  (?=\))
#format-function.patterns[0].begin  \b(format)(?=\()
#format-function.patterns[0].end  \)
#format-function.patterns[0].patterns[0].begin  \(
#format-function.patterns[0].patterns[0].end  (?=\))
#frequency-type.match  (?i:[-+]?(?:\d*\.\d+(?:[Ee][-+]?\d+)*|[-+]?\d+)(k??Hz))\b
#gradient-functions.patterns[1].patterns[0].patterns[7].match  \b(top|right|bottom|left|center|((?:farth|clos)est)-(corner…
#grid-repeat-function.begin  \b(repeat)(?=\()
#grid-repeat-function.end  \)
#grid-repeat-function.patterns[0].begin  \(
#grid-repeat-function.patterns[0].end  (?=\))
#grid-repeat-function.patterns[0].patterns[6].match  \b(auto-(fi(?:ll|t)))\b
#grid-repeat-function.patterns[0].patterns[7].match  \b(((m(?:ax|in))-content)|auto)\b
#image-function.begin  \b(image)(?=\()
#image-function.end  \)
#image-function.patterns[0].begin  \(
#image-function.patterns[0].end  (?=\))
#integer-type.match  [-+]?\d+
#keyframe-name.begin  \s*(-?(?:[_a-z[^\x00-\x7F]]|(?:(:?\\[0-9a-f]{1,6}(\r\n|[\t\…
#keyframe-name.end  \s*(?:(,)|(?=[;{]))
#less-boolean-function.begin  \b(boolean)(?=\()
#less-boolean-function.end  \)
#less-boolean-function.patterns[0].begin  \(
#less-boolean-function.patterns[0].end  (?=\))
#less-color-blend-functions.patterns[0].begin  \b(multiply|screen|overlay|(soft|hard)light|difference|excl…
#less-color-blend-functions.patterns[0].end  \)
#less-color-blend-functions.patterns[0].patterns[0].begin  \(
#less-color-blend-functions.patterns[0].patterns[0].end  (?=\))
#less-color-channel-functions.patterns[0].begin  \b(hue|saturation|lightness|hsv(hue|saturation|value)|red|g…
#less-color-channel-functions.patterns[0].end  \)
#less-color-channel-functions.patterns[0].patterns[0].begin  \(
#less-color-channel-functions.patterns[0].patterns[0].end  (?=\))
#less-color-definition-functions.patterns[1].begin  \b(hsva?)(?=\()
#less-color-definition-functions.patterns[1].end  \)
#less-color-definition-functions.patterns[1].patterns[0].begin  \(
#less-color-definition-functions.patterns[1].patterns[0].end  (?=\))
#less-color-operation-functions.patterns[0].begin  \b(fade|shade|tint)(?=\()
#less-color-operation-functions.patterns[0].end  \)
#less-color-operation-functions.patterns[0].patterns[0].begin  \(
#less-color-operation-functions.patterns[0].patterns[0].end  (?=\))
#less-color-operation-functions.patterns[1].begin  \b(spin)(?=\()
#less-color-operation-functions.patterns[1].end  \)
#less-color-operation-functions.patterns[1].patterns[0].begin  \(
#less-color-operation-functions.patterns[1].patterns[0].end  (?=\))
#less-color-operation-functions.patterns[2].patterns[0].patterns[3].match  \brelative\b
#less-color-operation-functions.patterns[3].begin  \b(contrast)(?=\()
#less-color-operation-functions.patterns[3].end  \)
#less-color-operation-functions.patterns[3].patterns[0].begin  \(
#less-color-operation-functions.patterns[3].patterns[0].end  (?=\))
#less-color-operation-functions.patterns[4].begin  \b(greyscale)(?=\()
#less-color-operation-functions.patterns[4].end  \)
#less-color-operation-functions.patterns[4].patterns[0].begin  \(
#less-color-operation-functions.patterns[4].patterns[0].end  (?=\))
#less-color-operation-functions.patterns[5].begin  \b(mix)(?=\()
#less-color-operation-functions.patterns[5].end  \)
#less-color-operation-functions.patterns[5].patterns[0].begin  \(
#less-color-operation-functions.patterns[5].patterns[0].end  (?=\))
#less-if-function.begin  \b(if)(?=\()
#less-if-function.end  \)
#less-if-function.patterns[0].begin  \(
#less-if-function.patterns[0].end  (?=\))
#less-list-functions.patterns[0].begin  \b(length)(?=\()\b
#less-list-functions.patterns[0].end  \)
#less-list-functions.patterns[0].patterns[0].begin  \(
#less-list-functions.patterns[0].patterns[0].end  (?=\))
#less-list-functions.patterns[1].begin  \b(extract)(?=\()\b
#less-list-functions.patterns[1].end  \)
#less-list-functions.patterns[1].patterns[0].begin  \(
#less-list-functions.patterns[1].patterns[0].end  (?=\))
#less-list-functions.patterns[2].begin  \b(range)(?=\()\b
#less-list-functions.patterns[2].end  \)
#less-list-functions.patterns[2].patterns[0].begin  \(
#less-list-functions.patterns[2].patterns[0].end  (?=\))
#less-logical-comparisons.patterns[0].match  \s*(=|(([<>])=?))\s*
#less-logical-comparisons.patterns[1].begin  \(
#less-logical-comparisons.patterns[1].end  \)
#less-logical-comparisons.patterns[2].match  \btrue|false\b
#less-logical-comparisons.patterns[3].match  ,
#less-math-functions.patterns[1].match  ((pi)(\()(\)))
#less-misc-functions.patterns[0].begin  \b(color)(?=\()
#less-misc-functions.patterns[0].end  \)
#less-misc-functions.patterns[0].patterns[0].begin  \(
#less-misc-functions.patterns[0].patterns[0].end  (?=\))
#less-misc-functions.patterns[1].begin  \b(image-(size|width|height))(?=\()
#less-misc-functions.patterns[1].end  \)
#less-misc-functions.patterns[1].patterns[0].begin  \(
#less-misc-functions.patterns[1].patterns[0].end  (?=\))
#less-misc-functions.patterns[2].begin  \b(convert|unit)(?=\()
#less-misc-functions.patterns[2].end  \)
#less-misc-functions.patterns[2].patterns[0].begin  \(
#less-misc-functions.patterns[2].patterns[0].end  (?=\))
#less-misc-functions.patterns[2].patterns[0].patterns[4].match  (([cm])?m|in|p([ctx])|m?s|g?rad|deg|turn|%|r?em|ex|ch)
#less-misc-functions.patterns[3].begin  \b(data-uri)(?=\()
#less-misc-functions.patterns[3].end  \)
#less-misc-functions.patterns[3].patterns[0].begin  \(
#less-misc-functions.patterns[3].patterns[0].end  (?=\))
#less-misc-functions.patterns[3].patterns[0].patterns[2].match  \s*(,)
#less-misc-functions.patterns[4].match  \b(default(\()(\)))\b
#less-misc-functions.patterns[5].begin  \b(get-unit)(?=\()
#less-misc-functions.patterns[5].end  \)
#less-misc-functions.patterns[5].patterns[0].begin  \(
#less-misc-functions.patterns[5].patterns[0].end  (?=\))
#less-misc-functions.patterns[6].begin  \b(svg-gradient)(?=\()
#less-misc-functions.patterns[6].end  \)
#less-misc-functions.patterns[6].patterns[0].begin  \(
#less-misc-functions.patterns[6].patterns[0].end  (?=\))
#less-misc-functions.patterns[6].patterns[0].patterns[5].match  \bto\b
#less-misc-functions.patterns[6].patterns[0].patterns[6].match  \b(top|right|bottom|left|center)\b
#less-misc-functions.patterns[6].patterns[0].patterns[7].match  \b(at|circle|ellipse)\b
#less-mixin-guards.patterns[0].patterns[0].patterns[1].match  default((\()(\)))
#less-namespace-accessors.patterns[0].end  groups 1
#less-namespace-accessors.patterns[0].patterns[0].match  groups 2
#less-namespace-accessors.patterns[0].patterns[3].begin  \s*(\{)
#less-namespace-accessors.patterns[0].patterns[3].end  (?=})
#less-namespace-accessors.patterns[1].end  groups 2
#less-string-functions.patterns[1].begin  \s*(%)(?=\()\s*  fuzz
#less-string-functions.patterns[1].end  \)  fuzz
#less-string-functions.patterns[1].patterns[0].begin  \(  fuzz
#less-string-functions.patterns[1].patterns[0].end  (?=\))  fuzz
#less-string-functions.patterns[2].begin  \b(replace)(?=\()\b
#less-string-functions.patterns[2].end  \)
#less-string-functions.patterns[2].patterns[0].begin  \(
#less-string-functions.patterns[2].patterns[0].end  (?=\))
#less-strings.patterns[0].end  groups 2
#less-type-functions.patterns[0].begin  \b(is(number|string|color|keyword|url|pixel|em|percentage|r…
#less-type-functions.patterns[0].end  \)
#less-type-functions.patterns[0].patterns[0].begin  \(
#less-type-functions.patterns[0].patterns[0].end  (?=\))
#less-type-functions.patterns[1].begin  \b(isunit)(?=\()
#less-type-functions.patterns[1].end  \)
#less-type-functions.patterns[1].patterns[0].begin  \(
#less-type-functions.patterns[1].patterns[0].end  (?=\))
#less-type-functions.patterns[1].patterns[0].patterns[2].match  \b((?i:em|ex|ch|rem)|(?i:v(?:[hw]|min|max))|(?i:cm|mm|q|in|…
#less-type-functions.patterns[2].begin  \b(isdefined)(?=\()
#less-type-functions.patterns[2].end  \)
#less-type-functions.patterns[2].patterns[0].begin  \(
#less-type-functions.patterns[2].patterns[0].end  (?=\))
#less-variable-assignment.patterns[0].end  groups 2
#less-variable-assignment.patterns[0].patterns[0].match  groups 4
#less-variable-comparison.patterns[0].end  groups 1
#less-variable-comparison.patterns[0].patterns[1].match  \btrue\b
#less-variable-comparison.patterns[0].patterns[5].match  ,
#literal-string.patterns[0].end  groups 2
#literal-string.patterns[1].end  groups 2
#local-function.begin  \b(local)(?=\()
#local-function.end  \)
#local-function.patterns[0].begin  \(
#local-function.patterns[0].end  (?=\))
#media-query.begin  groups 1
#media-query.end  groups 1
#media-query.patterns[2].patterns[1].match  \b(portrait|landscape|progressive|interlace)
#media-query.patterns[2].patterns[2].match  \s*(\d+)(/)(\d+)
#minmax-function.begin  \b(minmax)(?=\()
#minmax-function.end  \)
#minmax-function.patterns[0].begin  \(
#minmax-function.patterns[0].end  (?=\))
#minmax-function.patterns[0].patterns[4].match  \b(m(?:ax|in)-content)\b
#property-value-constants.patterns[0].match  \b(flex-start|flex-end|start|end|space-between|space-around…
#property-value-constants.patterns[5].match  \b(?:replace|add|accumulate)\b
#property-value-constants.patterns[7].match  \b(?:forwards|backwards|both)\b
#property-value-constants.patterns[8].match  \binfinite\b
#property-value-constants.patterns[9].match  \b(?:running|paused)\b
#property-value-constants.patterns[10].match  \be(?:ntry|xit)(?:-crossing|)\b
#property-value-constants.patterns[11].match  \b(linear|ease-in-out|ease-in|ease-out|ease|step-start|step…
#pseudo-selectors.patterns[0].begin  (:)(dir)(?=\()
#pseudo-selectors.patterns[0].end  \)
#pseudo-selectors.patterns[0].patterns[0].begin  \(
#pseudo-selectors.patterns[0].patterns[0].end  (?=\))
#pseudo-selectors.patterns[0].patterns[0].patterns[0].match  ltr|rtl
#pseudo-selectors.patterns[1].begin  (:)(lang)(?=\()
#pseudo-selectors.patterns[1].end  \)
#pseudo-selectors.patterns[1].patterns[0].begin  \(
#pseudo-selectors.patterns[1].patterns[0].end  (?=\))
#pseudo-selectors.patterns[3].begin  (:)(nth(-last)?-(child|of-type))(?=\()
#pseudo-selectors.patterns[3].end  \)
#pseudo-selectors.patterns[3].patterns[0].begin  \(
#pseudo-selectors.patterns[3].patterns[0].end  (?=\))
#pseudo-selectors.patterns[3].patterns[0].patterns[0].match  \b(even|odd)\b
#pseudo-selectors.patterns[3].patterns[0].patterns[1].match  ([-+])?\d+{0,1}(n)(\s*([-+])\s*\d+)?|[-+]?\s*\d+
#pseudo-selectors.patterns[4].begin  (:)(host-context|host|has|is|not|where)(?=\()
#pseudo-selectors.patterns[4].end  \)
#pseudo-selectors.patterns[4].patterns[0].begin  \(
#pseudo-selectors.patterns[4].patterns[0].end  (?=\))
#pseudo-selectors.patterns[6].begin  (::?)(highlight|part|state)(?=\s*(\())
#pseudo-selectors.patterns[6].end  \)
#pseudo-selectors.patterns[6].patterns[0].begin  \(
#pseudo-selectors.patterns[6].patterns[0].end  (?=\))
#pseudo-selectors.patterns[6].patterns[0].patterns[0].match  --|-?(?:[A-Z_a-z·À-ÖØ-öø-ͽͿ-῿‌‍‿⁀⁰-↏Ⰰ-⿯、-퟿豈-﷏ﷰ-�𐀀-\x{EFFFF…
#pseudo-selectors.patterns[7].begin  (::?)slotted(?=\s*(\())
#pseudo-selectors.patterns[7].end  \)
#pseudo-selectors.patterns[7].patterns[0].begin  \(
#pseudo-selectors.patterns[7].patterns[0].end  (?=\))
#qualified-name.match  (?:(-?(?:[-\w[^\x00-\x7F]]|\\(?:\h{1,6}[\t\n\f\s]?|[^\n\f\h…
#regexp-function.begin  \b(regexp)(?=\()
#regexp-function.end  \)
#regexp-function.patterns[0].begin  \(
#regexp-function.patterns[0].end  (?=\))
#relative-color.patterns[0].match  from
#relative-color.patterns[1].match  \b[abchlsw]\b  fuzz
#rule-list-body.patterns[4].begin  (?=[-\w]*?@\{.*}[-\w]*?\s*:[^(;{]*(?=[);}]))
#rule-list-body.patterns[4].end  (?=\s*(;)|(?=[)}]))
#rule-list-body.patterns[4].patterns[0].begin  (?=[^:\s])
#rule-list-body.patterns[4].patterns[0].end  (?=(((\+_?)?):)[\t\s]*)
#rule-list-body.patterns[4].patterns[1].begin  (((\+_?)?):)(?=[\t\s]*)
#rule-list-body.patterns[4].patterns[1].end  (?=\s*(;)|(?=[)}]))
#rule-list-body.patterns[5].patterns[1].patterns[0].patterns[1].match  [-\w]+
#rule-list-body.patterns[5].patterns[4].match  \b(var-)(-?(?:[-\w[^\x00-\x{9F}]]|\\(?:\h{1,6}[\t\n\f\s]?|[…
#rule-list-body.patterns[5].patterns[5].begin  \bfont(-family)?(?!-)\b
#rule-list-body.patterns[5].patterns[5].end  \s*(;)|(?=[)}])
#rule-list-body.patterns[5].patterns[5].patterns[0].match  (((\+_?)?):)([\t\s]*)
#rule-list-body.patterns[5].patterns[5].patterns[2].match  -?(?:[A-Z_a-z[^\x00-\x{9F}]]|\\(?:\h{1,6}[\t\n\f\s]?|[^\n\f…
#rule-list-body.patterns[5].patterns[5].patterns[3].match  ,
#rule-list-body.patterns[5].patterns[6].begin  \banimation-timeline\b
#rule-list-body.patterns[5].patterns[6].end  \s*(;)|(?=[)}])
#rule-list-body.patterns[5].patterns[6].patterns[0].begin  (((\+_?)?):)(?=[\t\s]*)
#rule-list-body.patterns[5].patterns[6].patterns[0].end  (?=\s*(;)|(?=[)}]))
#rule-list-body.patterns[5].patterns[7].begin  \banimation(?:-name)?(?=(?:\+_?)?:)\b
#rule-list-body.patterns[5].patterns[7].end  \s*(;)|(?=[)}])
#rule-list-body.patterns[5].patterns[7].patterns[0].begin  (((\+_?)?):)(?=[\t\s]*)
#rule-list-body.patterns[5].patterns[7].patterns[0].end  (?=\s*(;)|(?=[)}]))
#rule-list-body.patterns[5].patterns[7].patterns[0].patterns[6].match  -?(?:[A-Z_a-z[^\x00-\x7F]]|(?:(:?\\[0-9a-f]{1,6}(\r\n|[\t\n…
#rule-list-body.patterns[5].patterns[9].end  groups 1
#rule-list-body.patterns[5].patterns[9].patterns[0].match  groups 4
#rule-list-body.patterns[5].patterns[9].patterns[1].match  \b(inherit|initial|unset|none)\b
#rule-list-body.patterns[5].patterns[10].begin  \bwill-change\b
#rule-list-body.patterns[5].patterns[10].end  \s*(;)|(?=[)}])
#rule-list-body.patterns[5].patterns[10].patterns[0].match  (((\+_?)?):)([\t\s]*)
#rule-list-body.patterns[5].patterns[10].patterns[1].match  unset|initial|inherit|will-change|auto|scroll-position|cont…
#rule-list-body.patterns[5].patterns[10].patterns[2].match  -?(?:[-\w[^\x00-\x{9F}]]|\\(?:\h{1,6}[\t\n\f\s]?|[^\n\f\h])…
#rule-list-body.patterns[5].patterns[11].begin  \bcounter-(increment|(re)?set)\b
#rule-list-body.patterns[5].patterns[11].end  \s*(;)|(?=[)}])
#rule-list-body.patterns[5].patterns[11].patterns[0].match  (((\+_?)?):)([\t\s]*)
#rule-list-body.patterns[5].patterns[11].patterns[1].match  -?(?:[-\w[^\x00-\x{9F}]]|\\(?:\h{1,6}[\t\n\f\s]?|[^\n\f\h])…
#rule-list-body.patterns[5].patterns[11].patterns[3].match  unset|initial|inherit|auto
#rule-list-body.patterns[5].patterns[12].begin  \bcontainer(?:-name)?(?=\s*?:)
#rule-list-body.patterns[5].patterns[12].end  \s*(;)|(?=[)}])
#rule-list-body.patterns[5].patterns[12].patterns[0].begin  (((\+_?)?):)(?=[\t\s]*)
#rule-list-body.patterns[5].patterns[12].patterns[0].end  (?=\s*(;)|(?=[)}]))
#rule-list-body.patterns[5].patterns[12].patterns[0].patterns[0].match  \bdefault\b
#rule-list-body.patterns[5].patterns[12].patterns[0].patterns[3].match  --|-?(?:[A-Z_a-z·À-ÖØ-öø-ͽͿ-῿‌‍‿⁀⁰-↏Ⰰ-⿯、-퟿豈-﷏ﷰ-�𐀀-\x{EFFFF…
#rule-list-body.patterns[5].patterns[14].match  \b(((contain-intrinsic|max|min)-)?(block|inline)?-size)\b
#rule-list-body.patterns[6].begin  groups 2
#scroll-function.begin  \b(scroll)(\()
#scroll-function.end  \)
#scroll-function.patterns[0].match  root|nearest|self
#scroll-function.patterns[1].match  block|inline|[xy]
#selectors.patterns[8].match  (>{1,3}|[+~]){2,}
#selectors.patterns[9].match  /deep/
#selectors.patterns[10].patterns[3].patterns[1].match  [^]"'\[\s]
#selectors.patterns[10].patterns[3].patterns[3].match  (?:\s+([Ii]))?
#selectors.patterns[10].patterns[3].patterns[4].match  ]
#shape-functions.patterns[0].begin  \b(rect)(?=\()
#shape-functions.patterns[0].end  \)
#shape-functions.patterns[0].patterns[0].begin  \(
#shape-functions.patterns[0].patterns[0].end  (?=\))
#shape-functions.patterns[0].patterns[0].patterns[0].match  \bauto\b
#shape-functions.patterns[1].begin  \b(inset)(?=\()
#shape-functions.patterns[1].end  \)
#shape-functions.patterns[1].patterns[0].begin  \(
#shape-functions.patterns[1].patterns[0].end  (?=\))
#shape-functions.patterns[1].patterns[0].patterns[0].match  \bround\b
#shape-functions.patterns[2].begin  \b(circle|ellipse)(?=\()
#shape-functions.patterns[2].end  \)
#shape-functions.patterns[2].patterns[0].begin  \(
#shape-functions.patterns[2].patterns[0].end  (?=\))
#shape-functions.patterns[2].patterns[0].patterns[0].match  \bat\b
#shape-functions.patterns[2].patterns[0].patterns[1].match  \b(top|right|bottom|left|center|closest-side|farthest-side)…
#shape-functions.patterns[3].begin  \b(polygon)(?=\()
#shape-functions.patterns[3].end  \)
#shape-functions.patterns[3].patterns[0].begin  \(
#shape-functions.patterns[3].patterns[0].end  (?=\))
#shape-functions.patterns[3].patterns[0].patterns[0].match  \b(nonzero|evenodd)\b
#steps-function.begin  \b(steps)(\()
#steps-function.end  \)
#steps-function.patterns[0].match  jump-start|jump-end|jump-none|jump-both|start|end
#string-content.patterns[1].match  \\\s*\n
#string-content.patterns[2].match  \\(\h{1,6}|.)
#style-function.begin  \b(style)(?=\()
#style-function.end  \)
#style-function.patterns[0].begin  \(
#style-function.patterns[0].end  (?=\))
#symbols-function.begin  \b(symbols)(?=\()
#symbols-function.end  \)
#symbols-function.patterns[0].begin  \(
#symbols-function.patterns[0].end  (?=\))
#symbols-function.patterns[0].patterns[0].match  \b(cyclic|numeric|alphabetic|symbolic|fixed)\b
#transform-functions.patterns[0].begin  \b((?:matrix|scale)(?:3d|))(?=\()
#transform-functions.patterns[0].end  \)
#transform-functions.patterns[0].patterns[0].begin  \(
#transform-functions.patterns[0].patterns[0].end  (?=\))
#transform-functions.patterns[1].begin  \b(translate(3d)?)(?=\()
#transform-functions.patterns[1].end  \)
#transform-functions.patterns[1].patterns[0].begin  \(
#transform-functions.patterns[1].patterns[0].end  (?=\))
#transform-functions.patterns[2].begin  \b(translate[XY])(?=\()
#transform-functions.patterns[2].end  \)
#transform-functions.patterns[2].patterns[0].begin  \(
#transform-functions.patterns[2].patterns[0].end  (?=\))
#transform-functions.patterns[3].begin  \b(rotate[XYZ]?|skew[XY])(?=\()
#transform-functions.patterns[3].end  \)
#transform-functions.patterns[3].patterns[0].begin  \(
#transform-functions.patterns[3].patterns[0].end  (?=\))
#transform-functions.patterns[4].begin  \b(skew)(?=\()
#transform-functions.patterns[4].end  \)
#transform-functions.patterns[4].patterns[0].begin  \(
#transform-functions.patterns[4].patterns[0].end  (?=\))
#transform-functions.patterns[5].begin  \b(translateZ|perspective)(?=\()
#transform-functions.patterns[5].end  \)
#transform-functions.patterns[5].patterns[0].begin  \(
#transform-functions.patterns[5].patterns[0].end  (?=\))
#transform-functions.patterns[6].begin  \b(rotate3d)(?=\()
#transform-functions.patterns[6].end  \)
#transform-functions.patterns[6].patterns[0].begin  \(
#transform-functions.patterns[6].patterns[0].end  (?=\))
#transform-functions.patterns[7].begin  \b(scale[XYZ])(?=\()
#transform-functions.patterns[7].end  \)
#transform-functions.patterns[7].patterns[0].begin  \(
#transform-functions.patterns[7].patterns[0].end  (?=\))
#unicode-range.match  (?i)(u\+)([0-9?a-f]{1,6}(?:(-)[0-9a-f]{1,6})?)
#value-separator.match  \s*(/)\s*
#var-function.begin  \b(var)(?=\()
#var-function.end  \)
#var-function.patterns[0].begin  \(
#var-function.patterns[0].end  (?=\))
#view-function.begin  \b(view)(?=\()
#view-function.end  \)
#view-function.patterns[0].begin  \(
#view-function.patterns[0].end  (?=\))
#view-function.patterns[0].patterns[0].match  block|inline|[xy]|auto
```

### `lua`

```text
patterns[0].patterns[1].patterns[1].match  (\.{3})\s*([A-Z_a-z][0-9A-Z_a-z]*)
patterns[0].patterns[1].patterns[4].begin  :
patterns[0].patterns[1].patterns[4].end  (?=[),])
patterns[2].match  (?<![.\w\d])0[Xx]\.\h+([Ee]-?\d*)?([Pp][-+]\d+)?
patterns[3].match  (?<![.\w\d])0[Xx]\h+(?![.0-9EPep])
patterns[6].match  (?<![.\w\d])\d+(?![.0-9EPep])
patterns[8].match  \A(#!).*$\n?
patterns[10].match  \b(goto)\s+([A-Z_a-z][0-9A-Z_a-z]*)
patterns[11].match  (::)\s*[A-Z_a-z][0-9A-Z_a-z]*\s*(::)
patterns[13].match  <[*A-Z_a-z][-*.0-9A-Z_a-z]*>
patterns[17].match  \b(function)\b(?![,:])
patterns[21].match  (?<![^.]\.|:)\b(async)\b(?!\s*=(?!=))
patterns[23].match  \b(\|\||&&|!)\b
#comment.patterns[0].begin  groups 1
#comment.patterns[0].end  groups 1
#comment.patterns[0].patterns[0].begin  --\[(=*)\[@@@
#comment.patterns[0].patterns[0].end  (--)?]\1]
#comment.patterns[1].begin  /\*
#comment.patterns[1].end  \*/
#emmydoc.patterns[0].patterns[1].match  [,:]
#emmydoc.patterns[1].begin  (?<=---)[\t ]*@enum
#emmydoc.patterns[1].end  (?=[\n#@])
#emmydoc.patterns[1].patterns[0].begin  \b([*A-Z_a-z][-*.0-9A-Z_a-z]*)
#emmydoc.patterns[1].patterns[0].end  (?=\n)
#emmydoc.patterns[2].begin  (?<=---)[\t ]*@type
#emmydoc.patterns[2].end  (?=[\n#@])
#emmydoc.patterns[3].begin  (?<=---)[\t ]*@alias
#emmydoc.patterns[3].end  (?=[\n#@])
#emmydoc.patterns[3].patterns[0].begin  \b([*A-Z_a-z][-*.0-9A-Z_a-z]*)
#emmydoc.patterns[3].patterns[0].end  (?=[\n#])
#emmydoc.patterns[4].begin  (?<=---)[\t ]*(@operator)\s*(\b[a-z]+)?
#emmydoc.patterns[4].end  (?=[\n#@])
#emmydoc.patterns[5].begin  (?<=---)[\t ]*@cast
#emmydoc.patterns[5].end  (?=[\n#@])
#emmydoc.patterns[5].patterns[0].begin  \b([*A-Z_a-z][-*.0-9A-Z_a-z]*)
#emmydoc.patterns[5].patterns[0].end  (?=\n)
#emmydoc.patterns[5].patterns[0].patterns[1].match  ([+-|])
#emmydoc.patterns[7].patterns[0].match  \?
#emmydoc.patterns[8].begin  (?<=---)[\t ]*@field
#emmydoc.patterns[8].end  (?=[\n#@])
#emmydoc.patterns[8].patterns[0].begin  (\b([A-Z_a-z][0-9A-Z_a-z]*)\b|(\[))(\??)
#emmydoc.patterns[8].patterns[0].end  (?=[\n#])
#emmydoc.patterns[8].patterns[0].patterns[2].match  ]
#emmydoc.patterns[9].patterns[0].patterns[0].match  :
#emmydoc.patterns[10].begin  (?<=---)[\t ]*@vararg
#emmydoc.patterns[10].end  (?=[\n#@])
#emmydoc.patterns[14].begin  (?<=---)[\t ]*@private
#emmydoc.patterns[14].end  (?=[\n#@])
#emmydoc.patterns[15].begin  (?<=---)[\t ]*@protected
#emmydoc.patterns[15].end  (?=[\n#@])
#emmydoc.patterns[16].begin  (?<=---)[\t ]*@package
#emmydoc.patterns[16].end  (?=[\n#@])
#emmydoc.patterns[18].begin  (?<=---)[\t ]*@see
#emmydoc.patterns[18].end  (?=[\n#@])
#emmydoc.patterns[18].patterns[0].match  \b([*A-Z_a-z][-*.0-9A-Z_a-z]*)
#emmydoc.patterns[18].patterns[1].match  #
#emmydoc.patterns[19].begin  (?<=---)[\t ]*@diagnostic
#emmydoc.patterns[19].end  (?=[\n#@])
#emmydoc.patterns[19].patterns[0].begin  ([-0-9A-Z_a-z]+)[\t ]*(:)?
#emmydoc.patterns[19].patterns[0].end  (?=\n)
#emmydoc.patterns[19].patterns[0].patterns[0].match  \b([*A-Z_a-z][-0-9A-Z_a-z]*)
#emmydoc.patterns[19].patterns[0].patterns[1].match  ,
#emmydoc.patterns[20].begin  (?<=---)[\t ]*@module
#emmydoc.patterns[20].end  (?=[\n#@])
#emmydoc.patterns[22].begin  (?<=---)\|\s*[+>]?
#emmydoc.patterns[22].end  (?=[\n#@])
#emmydoc.type.patterns[1].match  <[*A-Z_a-z][-*.0-9A-Z_a-z]*>
#emmydoc.type.patterns[4].patterns[1].match  (\.\.\.)[\t ]*
#escaped_char.patterns[5].match  \\.
#string.patterns[2].begin  `  fuzz
#string.patterns[2].end  `[\t ]*|(?=\n)  fuzz
#string.patterns[3].begin  (?<=\.cdef)\s*(\[(=*)\[)
#string.patterns[3].end  (]\2])[\t ]*
```

### `make`

```text
#another-variable-braces.patterns[0].patterns[1].match  \\\n
#another-variable-parentheses.patterns[0].patterns[1].match  \\\n
#builtin-variable-braces.patterns[0].match  (?<=\{)(MAKEFILES|VPATH|SHELL|MAKESHELL|MAKE|MAKELEVEL|MAKE…
#comment.begin  groups 1
#directives.patterns[0].patterns[2].match  %
#directives.patterns[1].begin  ^ *(vpath)\b
#directives.patterns[1].end  ^
#directives.patterns[1].patterns[2].match  %
#directives.patterns[2].begin  groups 1
#directives.patterns[2].end  groups 2, 3, 4
#directives.patterns[2].patterns[0].begin  \G(?!\n)
#directives.patterns[2].patterns[0].end  ^
#flavor-variable-braces.patterns[0].begin  (?<=\{)(origin|flavor)\s(?=[^}\s]+\s*})
#flavor-variable-braces.patterns[0].end  (?=})
#flavor-variable-parentheses.patterns[0].begin  (?<=\()(origin|flavor)\s(?=[^)\s]+\s*\))
#flavor-variable-parentheses.patterns[0].end  (?=\))
#function-variable-braces.patterns[0].begin  (?<=\{)(subst|patsubst|strip|findstring|filter(-out)?|sort|…
#function-variable-braces.patterns[0].end  (?=}|((?<!\\)\n))
#function-variable-braces.patterns[0].patterns[3].match  [%*]
#function-variable-braces.patterns[0].patterns[4].match  \\\n
```

### `markdown`

```text
#bold.patterns[0].begin  (?=<[^>]*?>)
#bold.patterns[0].end  (?<=>)
#fenced_code_block_abap.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(abap)((\s+|[,:?{])[^`]*)?$)
#fenced_code_block_abap.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_abap.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_abap.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_bibtex.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(bibtex)((\s+|[,:?{])[^`]*)?…
#fenced_code_block_bibtex.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_bibtex.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_bibtex.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_c.begin  groups 5
#fenced_code_block_clojure.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(cl(?:js??|ojure))((\s+|[,:?…
#fenced_code_block_clojure.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_clojure.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_clojure.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_coffee.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(coffee|Cakefile|coffee.erb)…
#fenced_code_block_coffee.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_coffee.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_coffee.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_cpp.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(c(?:pp|\+\+|xx))((\s+|[,:?{…  fuzz
#fenced_code_block_cpp.end  (^|\G)(\2|\s{0,3})(\3)\s*$  fuzz
#fenced_code_block_cpp.patterns[0].begin  (^|\G)(\s*)(.*)  fuzz
#fenced_code_block_cpp.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  fuzz
#fenced_code_block_csharp.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(c(?:s|sharp|#))((\s+|[,:?{]…
#fenced_code_block_csharp.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_csharp.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_csharp.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_dart.begin  groups 5
#fenced_code_block_dockerfile.begin  groups 5
#fenced_code_block_dosbatch.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(bat(?:|ch))((\s+|[,:?{])[^`…
#fenced_code_block_dosbatch.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_dosbatch.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_dosbatch.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_elixir.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(elixir)((\s+|[,:?{])[^`]*)?…
#fenced_code_block_elixir.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_elixir.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_elixir.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_erlang.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(erlang)((\s+|[,:?{])[^`]*)?…
#fenced_code_block_erlang.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_erlang.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_erlang.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_fsharp.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(f(?:s|sharp|#))((\s+|[,:?{]…
#fenced_code_block_fsharp.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_fsharp.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_fsharp.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_git_commit.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:((?:COMMIT_EDIT|MERGE_)MSG)(…
#fenced_code_block_git_commit.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_git_commit.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_git_commit.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_git_rebase.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(git-rebase-todo)((\s+|[,:?{…
#fenced_code_block_git_rebase.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_git_rebase.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_git_rebase.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_go.begin  groups 5
#fenced_code_block_groovy.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(g(?:roovy|vy))((\s+|[,:?{])…
#fenced_code_block_groovy.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_groovy.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_groovy.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_handlebars.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(h(?:andlebars|bs))((\s+|[,:…
#fenced_code_block_handlebars.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_handlebars.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_handlebars.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_ignore.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:((?:git|)ignore)((\s+|[,:?{]…
#fenced_code_block_ignore.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_ignore.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_ignore.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_ini.begin  groups 5
#fenced_code_block_java.begin  groups 5
#fenced_code_block_js_regexp.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(regexp)((\s+|[,:?{])[^`]*)?…
#fenced_code_block_js_regexp.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_js_regexp.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_js_regexp.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_jsonc.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(jsonc)((\s+|[,:?{])[^`]*)?$)
#fenced_code_block_jsonc.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_jsonc.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_jsonc.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_jsonl.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(jsonl(?:|ines))((\s+|[,:?{]…
#fenced_code_block_jsonl.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_jsonl.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_jsonl.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_julia.begin  groups 5
#fenced_code_block_latex.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:((?:la|)tex)((\s+|[,:?{])[^`…
#fenced_code_block_latex.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_latex.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_latex.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_less.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(less)((\s+|[,:?{])[^`]*)?$)
#fenced_code_block_less.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_less.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_less.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_log.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(log)((\s+|[,:?{])[^`]*)?$)
#fenced_code_block_log.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_log.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_log.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_lua.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(lua)((\s+|[,:?{])[^`]*)?$)
#fenced_code_block_lua.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_lua.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_lua.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_makefile.begin  groups 5
#fenced_code_block_markdown.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(m(?:arkdown|d))((\s+|[,:?{]…
#fenced_code_block_markdown.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_markdown.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_markdown.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_objc.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(objectivec|objective-c|mm|o…
#fenced_code_block_objc.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_objc.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_objc.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_perl.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(perl|pl|pm|pod|t|PL|psgi|vc…
#fenced_code_block_perl.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_perl.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_perl.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_perl6.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(perl6|p6|pl6|pm6|nqp)((\s+|…
#fenced_code_block_perl6.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_perl6.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_perl6.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_php.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(php3??|php4|php5|phpt|phtml…
#fenced_code_block_php.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_php.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_php.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_powershell.begin  groups 5
#fenced_code_block_pug.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(jade|pug)((\s+|[,:?{])[^`]*…
#fenced_code_block_pug.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_pug.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_pug.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_python.begin  groups 5 (fuzz)
#fenced_code_block_r.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:([RSrs]|Rprofile|\{\.r.+?})(…
#fenced_code_block_r.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_r.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_r.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_regexp_python.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(re)((\s+|[,:?{])[^`]*)?$)
#fenced_code_block_regexp_python.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_regexp_python.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_regexp_python.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_restructuredtext.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(r(?:estructuredtext|st))((\…
#fenced_code_block_restructuredtext.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_restructuredtext.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_restructuredtext.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_ruby.begin  groups 5
#fenced_code_block_scss.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(scss)((\s+|[,:?{])[^`]*)?$)
#fenced_code_block_scss.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_scss.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_scss.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_shell.begin  groups 5
#fenced_code_block_sql.begin  groups 5
#fenced_code_block_swift.begin  groups 5
#fenced_code_block_tsx.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(tsx)((\s+|[,:?{])[^`]*)?$)
#fenced_code_block_tsx.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_tsx.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_tsx.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_twig.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(twig)((\s+|[,:?{])[^`]*)?$)
#fenced_code_block_twig.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_twig.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_twig.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_vs_net.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(vb)((\s+|[,:?{])[^`]*)?$)
#fenced_code_block_vs_net.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_vs_net.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_vs_net.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_xml.begin  groups 5
#fenced_code_block_xsl.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(xslt??)((\s+|[,:?{])[^`]*)?…
#fenced_code_block_xsl.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_xsl.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_xsl.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#fenced_code_block_yaml.begin  groups 5
#fenced_code_block_yang.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(yang)((\s+|[,:?{])[^`]*)?$)
#fenced_code_block_yang.end  (^|\G)(\2|\s{0,3})(\3)\s*$
#fenced_code_block_yang.patterns[0].begin  (^|\G)(\s*)(.*)
#fenced_code_block_yang.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)
#frontMatter.patterns[0].begin  groups 2
#heading.captures[1].patterns[0].match  (#{6})\s+(.*?)(?:\s+(#+))?\s*$
#heading.captures[1].patterns[1].match  groups 3
#heading.captures[1].patterns[2].match  groups 3
#heading.captures[1].patterns[4].match  groups 3
#heading.captures[1].patterns[5].match  groups 3
#html.patterns[0].begin  groups 1
#html.patterns[0].end  groups 2
#html.patterns[1].begin  (?i)(^|\G)\s*(?=<(script|style|pre)(\s|$|>)(?!.*?</(script|…
#html.patterns[1].end  (?i)(.*)((</)(script|style|pre)(>))
#html.patterns[1].patterns[0].begin  (\s*|$)
#html.patterns[1].patterns[0].while  (?i)^(?!.*</(script|style|pre)>)
#html.patterns[3].begin  (^|\G)\s*(?=(<(?:[-0-9A-Za-z](/?>|\s.*?>)|/[-0-9A-Za-z]>))\…
#html.patterns[3].while  ^(?!\s*$)
#image-inline.match  groups 7, 8, 9, 12, 13, 14, 18, 19, 20
#image-ref.match  (!\[)((?<square>[^]\[\\]|\\.|\[\g<square>*+])*+)(]) ?(\[)(.…
#italic.patterns[0].begin  (?=<[^>]*?>)
#italic.patterns[0].end  (?<=>)
#link-def.match  groups 5, 6, 7, 9, 10, 11, 12, 13, 14, 15, 16, 17
#link-email.match  (<)((?:mailto:)?[!#-'*+\--9=?A-Z^-~]+@[-0-9A-Za-z]+(?:\.[-0…
#link-inline.match  groups 12, 13, 14
#list_paragraph.begin  (^|\G)(?=\S)(?![*->]\s|[0-9]+\.\s)
#list_paragraph.while  (^|\G)(?!\s*$|#| {0,3}([-*>_] {2,}){3,}[\t ]*$\n?| {0,3}[*-…
#strikethrough.match  (?<!\\)(~{2,})(?!(?<=\w~~)_)((?:[^~]|(?!(?<![\\~])\1(?!~))~…
#strikethrough.captures[2].patterns[0].begin  (?=<[^>]*?>)
#strikethrough.captures[2].patterns[0].end  (?<=>)
```

### `python`

```text
#backticks.begin  `  fuzz
#backticks.end  `|(?<!\\)(\n)  fuzz
#call-wrapper-inheritance.begin  \b(?=([_[:alpha:]]\w*)\s*(\())
#call-wrapper-inheritance.end  (\))
#class-inheritance.patterns[0].match  (\*\*?)
#class-inheritance.patterns[2].match  =(?!=)
#class-inheritance.patterns[3].match  \bmetaclass\b
#class-kwarg.match  \b([_[:alpha:]]\w*)\s*(=)(?!=)
#comments.patterns[0].patterns[1].match  (?<!\.)\b(bool|bytes|float|int|object|str|List|Dict|Iterabl…
#comments-string-single-three.begin  (#)
#comments-string-single-three.end  ($|(?='''))
#decorator.end  groups 2
#decorator-name.patterns[4].match  \s*([^#(.\\_[:alpha:]\s].*?)(?=#|$)  fuzz
#docstring.patterns[2].begin  (["'])  fuzz
#docstring.patterns[2].end  (\1)|(\n)  fuzz
#docstring.patterns[3].begin  ([Rr])(["'])  fuzz
#docstring.patterns[3].end  (\2)|(\n)  fuzz
#double-one-regexp-character-set.patterns[0].match  \[\^?](?!.*?])
#double-one-regexp-character-set.patterns[1].begin  groups 2, 3
#double-one-regexp-character-set.patterns[1].end  groups 2
#double-one-regexp-comments.begin  \(\?#
#double-one-regexp-comments.end  (\)|(?="))|((?=(?<!\\)\n))
#double-one-regexp-conditional.begin  (\()\?\((\w+(?:\s+\p{alnum}+)?|\d+)\)
#double-one-regexp-conditional.end  (\)|(?="))|((?=(?<!\\)\n))
#double-one-regexp-lookahead.begin  (\()\?=
#double-one-regexp-lookahead.end  (\)|(?="))|((?=(?<!\\)\n))
#double-one-regexp-lookahead-negative.begin  (\()\?!
#double-one-regexp-lookahead-negative.end  (\)|(?="))|((?=(?<!\\)\n))
#double-one-regexp-lookbehind.begin  (\()\?<=
#double-one-regexp-lookbehind.end  (\)|(?="))|((?=(?<!\\)\n))
#double-one-regexp-lookbehind-negative.begin  (\()\?<!
#double-one-regexp-lookbehind-negative.end  (\)|(?="))|((?=(?<!\\)\n))
#double-one-regexp-named-group.end  groups 2
#double-one-regexp-parentheses.end  groups 2
#double-one-regexp-parentheses-non-capturing.begin  \(\?:
#double-one-regexp-parentheses-non-capturing.end  (\)|(?="))|((?=(?<!\\)\n))
#double-three-regexp-character-set.patterns[0].match  \[\^?](?!.*?])
#double-three-regexp-character-set.patterns[1].begin  groups 3
#double-three-regexp-character-set.patterns[1].end  groups 2
#double-three-regexp-comments.begin  \(\?#
#double-three-regexp-comments.end  (\)|(?="""))
#double-three-regexp-conditional.begin  (\()\?\((\w+(?:\s+\p{alnum}+)?|\d+)\)
#double-three-regexp-conditional.end  (\)|(?="""))
#double-three-regexp-lookahead.begin  (\()\?=
#double-three-regexp-lookahead.end  (\)|(?="""))
#double-three-regexp-lookahead-negative.end  groups 2
#double-three-regexp-lookbehind.begin  (\()\?<=
#double-three-regexp-lookbehind.end  (\)|(?="""))
#double-three-regexp-lookbehind-negative.end  groups 2
#double-three-regexp-named-group.begin  (\()(\?P<\w+(?:\s+\p{alnum}+)?>)
#double-three-regexp-named-group.end  (\)|(?="""))
#double-three-regexp-parentheses.end  groups 2
#double-three-regexp-parentheses-non-capturing.end  groups 2
#escape-sequence-unicode.patterns[0].match  \\(u\h{4}|U\h{8}|N\{[\w\s]+?})
#fstring-fnorm-quoted-multi-line.begin  groups 2
#fstring-fnorm-quoted-multi-line.end  groups 2
#fstring-fnorm-quoted-single-line.begin  groups 2
#fstring-fnorm-quoted-single-line.end  groups 2
#fstring-formatting-braces.patterns[0].match  (\{)(\s*?)(})
#fstring-formatting-singe-brace.match  (}(?!}))
#fstring-illegal-single-brace.begin  (\{)(?=[^\n}]*$\n?)
#fstring-illegal-single-brace.end  (})|(?=\n)
#fstring-normf-quoted-multi-line.begin  \b([BUbu])([Ff])('''|""")
#fstring-normf-quoted-multi-line.end  (\3)
#fstring-normf-quoted-single-line.begin  \b([BUbu])([Ff])((["']))
#fstring-normf-quoted-single-line.end  (\3)|((?<!\\)\n)
#fstring-raw-multi-core.match  (.+?)($(\n?)|(?=[\\{}]|'''|"""))|\n
#fstring-raw-quoted-multi-line.begin  \b([Rr][Ff]|[Ff][Rr])('''|""")
#fstring-raw-quoted-multi-line.end  (\2)
#fstring-raw-quoted-single-line.end  groups 2
#fstring-terminator-multi.patterns[0].match  (=(![ars])?)(?=})
#fstring-terminator-multi.patterns[1].match  (=?![ars])(?=})
#fstring-terminator-multi.patterns[2].match  (=?(?:![ars])?)(:\w?[<=>^]?[- +]?#?\d*,?(\.\d+)?[%EFGXb-gno…
#fstring-terminator-multi-tail.begin  (=?(?:![ars])?)(:)(?=.*?\{)
#fstring-terminator-multi-tail.end  (?=})
#fstring-terminator-multi-tail.patterns[2].match  ([%EFGXb-gnosx])(?=})
#fstring-terminator-multi-tail.patterns[3].match  (\.\d+)
#fstring-terminator-multi-tail.patterns[4].match  (,)
#fstring-terminator-multi-tail.patterns[5].match  (\d+)
#fstring-terminator-multi-tail.patterns[6].match  (#)
#fstring-terminator-multi-tail.patterns[7].match  ([- +])
#fstring-terminator-multi-tail.patterns[8].match  ([<=>^])
#fstring-terminator-multi-tail.patterns[9].match  (\w)
#fstring-terminator-single.patterns[2].match  (=?(?:![ars])?)(:\w?[<=>^]?[- +]?#?\d*,?(\.\d+)?[%EFGXb-gno…
#fstring-terminator-single-tail.begin  (=?(?:![ars])?)(:)(?=.*?\{)
#fstring-terminator-single-tail.end  (?=})|(?=\n)
#fstring-terminator-single-tail.patterns[2].match  ([%EFGXb-gnosx])(?=})
#fstring-terminator-single-tail.patterns[3].match  (\.\d+)
#fstring-terminator-single-tail.patterns[4].match  (,)
#fstring-terminator-single-tail.patterns[5].match  (\d+)
#fstring-terminator-single-tail.patterns[6].match  (#)
#fstring-terminator-single-tail.patterns[7].match  ([- +])
#fstring-terminator-single-tail.patterns[8].match  ([<=>^])
#fstring-terminator-single-tail.patterns[9].match  (\w)
#function-arguments.patterns[5].match  =(?!=)  fuzz
#illegal-anno.match  ->  fuzz
#illegal-names.match  groups 2 (fuzz)
#illegal-object-name.match  \b(True|False|None)\b
#illegal-operator.patterns[0].match  &&|\|\||--|\+\+
#illegal-operator.patterns[1].match  [$?]  fuzz
#illegal-operator.patterns[2].match  !\b
#impossible.match  $.^
#lambda.patterns[0].match  ((?<=\.)lambda|lambda(?=\s*[.=]))
#lambda.patterns[1].match  \b(lambda)\s*?(?=[\n,]|$)
#lambda.patterns[2].patterns[0].match  /
#lambda.patterns[2].patterns[1].match  (\*\*?)
#lambda.patterns[2].patterns[4].match  groups 2
#lambda-incomplete.match  \blambda(?=\s*[),])
#lambda-nested-incomplete.match  \blambda(?=\s*[),:])
#lambda-parameter-with-default.begin  \b([_[:alpha:]]\w*)\s*(=)
#lambda-parameter-with-default.end  (,)|(?=:|$)
#line-continuation.patterns[0].match  (\\)\s*(\S.*$\n?)  fuzz
#line-continuation.patterns[1].begin  (\\)\s*$\n?  fuzz
#line-continuation.patterns[1].end  (?=^\s*$)|(?!(\s*[Rr]?('''|"""|["']))|\G()$)  fuzz
#number.patterns[6].match  \b[0-9]+\w+
#number-bin.match  (?<![.\w])(0[Bb])(_?[01])+\b
#number-dec.match  groups 1, 2
#number-float.match  groups 1
#number-hex.match  (?<![.\w])(0[Xx])(_?\h)+\b
#number-long.match  (?<![.\w])([1-9][0-9]*|0)([Ll])\b
#number-oct.match  (?<![.\w])(0[Oo])(_?[0-7])+\b
#regexp-backreference.match  (\()(\?P=\w+(?:\s+\p{alnum}+)?)(\))
#regexp-backreference-number.match  (\\[1-9]\d?)
#regexp-charecter-set-escapes.patterns[2].match  \\([0-7]{1,3})
#regexp-double-one-line.begin  groups 2, 3
#regexp-double-one-line.end  groups 2
#regexp-double-three-line.begin  groups 2, 3
#regexp-double-three-line.end  groups 2
#regexp-escape-character.match  \\(x\h{2}|0[0-7]{1,2}|[0-7]{3})
#regexp-escape-special.match  \\([ABDSWZbdsw])
#regexp-escape-unicode.match  \\(u\h{4}|U\h{8})
#regexp-flags.match  \(\?[Laimsux]+\)
#regexp-single-one-line.begin  \b(([Uu]r)|([Bb]r)|(r[Bb]?))(')
#regexp-single-one-line.end  (')|(?<!\\)(\n)
#regexp-single-three-line.begin  \b(([Uu]r)|([Bb]r)|(r[Bb]?))(''')
#regexp-single-three-line.end  (''')
#semicolon.patterns[0].match  ;$
#single-one-regexp-character-set.patterns[0].match  \[\^?](?!.*?])
#single-one-regexp-character-set.patterns[1].begin  (\[)(\^)?(])?
#single-one-regexp-character-set.patterns[1].end  (]|(?='))|((?=(?<!\\)\n))
#single-one-regexp-character-set.patterns[1].patterns[1].match  \N
#single-one-regexp-comments.begin  \(\?#
#single-one-regexp-comments.end  (\)|(?='))|((?=(?<!\\)\n))
#single-one-regexp-conditional.begin  (\()\?\((\w+(?:\s+\p{alnum}+)?|\d+)\)
#single-one-regexp-conditional.end  (\)|(?='))|((?=(?<!\\)\n))
#single-one-regexp-lookahead.begin  (\()\?=
#single-one-regexp-lookahead.end  (\)|(?='))|((?=(?<!\\)\n))
#single-one-regexp-lookahead-negative.begin  (\()\?!
#single-one-regexp-lookahead-negative.end  (\)|(?='))|((?=(?<!\\)\n))
#single-one-regexp-lookbehind.begin  (\()\?<=
#single-one-regexp-lookbehind.end  (\)|(?='))|((?=(?<!\\)\n))
#single-one-regexp-lookbehind-negative.begin  (\()\?<!
#single-one-regexp-lookbehind-negative.end  (\)|(?='))|((?=(?<!\\)\n))
#single-one-regexp-named-group.begin  (\()(\?P<\w+(?:\s+\p{alnum}+)?>)
#single-one-regexp-named-group.end  (\)|(?='))|((?=(?<!\\)\n))
#single-one-regexp-parentheses.begin  \(
#single-one-regexp-parentheses.end  (\)|(?='))|((?=(?<!\\)\n))
#single-one-regexp-parentheses-non-capturing.begin  \(\?:
#single-one-regexp-parentheses-non-capturing.end  (\)|(?='))|((?=(?<!\\)\n))
#single-three-regexp-character-set.patterns[0].match  \[\^?](?!.*?])
#single-three-regexp-character-set.patterns[1].begin  (\[)(\^)?(])?
#single-three-regexp-character-set.patterns[1].end  (]|(?='''))
#single-three-regexp-character-set.patterns[1].patterns[1].match  \N
#single-three-regexp-comments.begin  \(\?#
#single-three-regexp-comments.end  (\)|(?='''))
#single-three-regexp-conditional.begin  (\()\?\((\w+(?:\s+\p{alnum}+)?|\d+)\)
#single-three-regexp-conditional.end  (\)|(?='''))
#single-three-regexp-lookahead.begin  (\()\?=
#single-three-regexp-lookahead.end  (\)|(?='''))
#single-three-regexp-lookahead-negative.begin  (\()\?!
#single-three-regexp-lookahead-negative.end  (\)|(?='''))
#single-three-regexp-lookbehind.begin  (\()\?<=
#single-three-regexp-lookbehind.end  (\)|(?='''))
#single-three-regexp-lookbehind-negative.begin  (\()\?<!
#single-three-regexp-lookbehind-negative.end  (\)|(?='''))
#single-three-regexp-named-group.begin  (\()(\?P<\w+(?:\s+\p{alnum}+)?>)
#single-three-regexp-named-group.end  (\)|(?='''))
#single-three-regexp-parentheses.begin  \(
#single-three-regexp-parentheses.end  (\)|(?='''))
#single-three-regexp-parentheses-non-capturing.begin  \(\?:
#single-three-regexp-parentheses-non-capturing.end  (\)|(?='''))
#statement-keyword.patterns[0].match  \b((async\s+)?\s*def)\b  fuzz
#statement-keyword.patterns[2].match  \b(?<!\.)as\b  fuzz
#statement-keyword.patterns[5].match  \b(?<!\.)(class)\b  fuzz
#string-bin-quoted-multi-line.begin  \b([Bb])('''|""")
#string-bin-quoted-multi-line.end  (\2)
#string-bin-quoted-single-line.end  groups 2
#string-brace-formatting.patterns[0].match  groups 4
#string-brace-formatting.patterns[1].match  groups 3
#string-line-continuation.match  \\$  fuzz
#string-multi-bad-brace1-formatting-raw.begin  (?=\{%(.*?(?!'''|"""))%})
#string-multi-bad-brace1-formatting-raw.end  (?='''|""")
#string-multi-bad-brace1-formatting-unicode.begin  (?=\{%(.*?(?!'''|"""))%})
#string-multi-bad-brace1-formatting-unicode.end  (?='''|""")
#string-multi-bad-brace2-formatting-raw.begin  (?!\{\{)(?=\{(\w*?(?!'''|""")[^!.:\[}\w]).*?(?!'''|""")})
#string-multi-bad-brace2-formatting-raw.end  (?='''|""")
#string-multi-bad-brace2-formatting-unicode.begin  (?!\{\{)(?=\{(\w*?(?!'''|""")[^!.:\[}\w]).*?(?!'''|""")})
#string-multi-bad-brace2-formatting-unicode.end  (?='''|""")
#string-quoted-multi-line.begin  groups 1, 2
#string-quoted-multi-line.end  groups 2
#string-quoted-single-line.begin  groups 1, 2
#string-quoted-single-line.end  groups 2
#string-raw-bin-quoted-multi-line.begin  \b(R[Bb]|[Bb]R)('''|""")
#string-raw-bin-quoted-multi-line.end  (\2)
#string-raw-bin-quoted-single-line.begin  \b(R[Bb]|[Bb]R)((["']))
#string-raw-bin-quoted-single-line.end  (\2)|((?<!\\)\n)
#string-raw-quoted-multi-line.begin  \b(([Uu]R)|(R))('''|""")
#string-raw-quoted-multi-line.end  (\4)
#string-raw-quoted-single-line.begin  \b(([Uu]R)|(R))((["']))
#string-raw-quoted-single-line.end  (\4)|((?<!\\)\n)
#string-single-bad-brace1-formatting-raw.begin  (?=\{%(.*?(?!(["'])|((?<!\\)\n)))%})
#string-single-bad-brace1-formatting-raw.end  (?=(["'])|((?<!\\)\n))
#string-single-bad-brace1-formatting-unicode.begin  (?=\{%(.*?(?!(["'])|((?<!\\)\n)))%})
#string-single-bad-brace1-formatting-unicode.end  (?=(["'])|((?<!\\)\n))
#string-single-bad-brace2-formatting-raw.begin  (?!\{\{)(?=\{(\w*?(?!(["'])|((?<!\\)\n))[^!.:\[}\w]).*?(?!(…
#string-single-bad-brace2-formatting-raw.end  (?=(["'])|((?<!\\)\n))
#string-single-bad-brace2-formatting-unicode.begin  (?!\{\{)(?=\{(\w*?(?!(["'])|((?<!\\)\n))[^!.:\[}\w]).*?(?!(…
#string-single-bad-brace2-formatting-unicode.end  (?=(["'])|((?<!\\)\n))
```

### `ruby`

```text
patterns[1].match  groups 5
patterns[3].match  (?<!\.)\belse(\s)+if\b
patterns[5].match  (?<!\.)\b(case|if|elsif|unless|until|while)\b\s*(\()*?\s*([…
patterns[7].match  (?<!\.)\b(case|if|elsif|unless|until|while)\b\s*(\()*?\s*([…
patterns[21].end  groups 1
patterns[30].begin  (->)\(
patterns[30].end  \)
patterns[30].patterns[0].begin  (?=[\&*A-Z_a-z])
patterns[30].patterns[0].end  (?=[),])
patterns[32].begin  (?=def\b)(?<=^|\s)(def)\s+((?>[A-Z_a-z]\w*(?>\.|::))?(?>[A-…  fuzz
patterns[32].end  (?=;)|(?<=[]!"')?`}\w])(?=\s*#|\s*$)  fuzz
patterns[32].patterns[0].begin  (?=[\&*A-Z_a-z])
patterns[32].patterns[0].end  (?=[,;]|\s*#|\s*$)
patterns[35].begin  :'
patterns[35].end  '
patterns[35].patterns[0].match  \\['\\]
patterns[37].match  (?<!\()/=
patterns[38].patterns[0].match  \\['\\]
patterns[40].begin  (?<!\.)`  fuzz
patterns[40].end  `  fuzz
patterns[42].begin  %r\{
patterns[42].end  }[eimnosux]*
patterns[43].begin  %r\[
patterns[43].end  ][eimnosux]*
patterns[44].begin  %r\(
patterns[44].end  \)[eimnosux]*
patterns[45].begin  %r<
patterns[45].end  >[eimnosux]*
patterns[46].begin  %r(\W)
patterns[46].end  \1[eimnosux]*
patterns[47].begin  %I\[
patterns[47].end  ]
patterns[48].begin  %I\(
patterns[48].end  \)
patterns[49].begin  %I<
patterns[49].end  >
patterns[50].begin  %I\{
patterns[50].end  }
patterns[51].begin  %I(\W)
patterns[51].end  \1
patterns[52].begin  %i\[
patterns[52].end  ]
patterns[52].patterns[0].match  \\[]\\]
patterns[53].begin  %i\(
patterns[53].end  \)
patterns[53].patterns[0].match  \\[)\\]
patterns[54].begin  %i<
patterns[54].end  >
patterns[54].patterns[0].match  \\[>\\]
patterns[55].begin  %i\{
patterns[55].end  }
patterns[55].patterns[0].match  \\[\\}]
patterns[56].begin  %i(\W)
patterns[56].end  \1
patterns[56].patterns[0].match  \\.
patterns[57].begin  %W\[
patterns[57].end  ]
patterns[58].begin  %W\(
patterns[58].end  \)
patterns[59].begin  %W<
patterns[59].end  >
patterns[60].begin  %W\{
patterns[60].end  }
patterns[61].begin  %W(\W)
patterns[61].end  \1
patterns[62].begin  %w\[
patterns[62].end  ]
patterns[62].patterns[0].match  \\[]\\]
patterns[63].patterns[0].match  \\[)\\]
patterns[64].begin  %w<
patterns[64].end  >
patterns[64].patterns[0].match  \\[>\\]
patterns[65].begin  %w\{
patterns[65].end  }
patterns[65].patterns[0].match  \\[\\}]
patterns[66].begin  %w(\W)
patterns[66].end  \1
patterns[66].patterns[0].match  \\.
patterns[67].begin  %[Qx]?\(
patterns[67].end  \)
patterns[69].begin  %[Qx]?\{
patterns[69].end  }
patterns[70].begin  %[Qx]?<
patterns[70].end  >
patterns[71].begin  %[Qx](\W)
patterns[71].end  \1
patterns[73].begin  %q\(
patterns[73].end  \)
patterns[73].patterns[0].match  \\[)\\]
patterns[74].begin  %q<
patterns[74].end  >
patterns[74].patterns[0].match  \\[>\\]
patterns[75].begin  %q\[
patterns[75].end  ]
patterns[75].patterns[0].match  \\[]\\]
patterns[76].begin  %q\{
patterns[76].end  }
patterns[76].patterns[0].match  \\[\\}]
patterns[77].begin  %q(\W)
patterns[77].end  \1
patterns[77].patterns[0].match  \\.
patterns[78].begin  %s\(
patterns[78].end  \)
patterns[78].patterns[0].match  \\[)\\]
patterns[79].begin  %s<
patterns[79].end  >
patterns[79].patterns[0].match  \\[>\\]
patterns[80].begin  %s\[
patterns[80].end  ]
patterns[80].patterns[0].match  \\[]\\]
patterns[81].begin  %s\{
patterns[81].end  }
patterns[81].patterns[0].match  \\[\\}]
patterns[82].begin  %s(\W)
patterns[82].end  \1
patterns[82].patterns[0].match  \\.
patterns[84].begin  ^=begin
patterns[84].end  ^=end
patterns[86].begin  groups 1
patterns[88].begin  ^__END__\n
patterns[88].end  (?=not)impossible
patterns[88].patterns[0].begin  (?=<?xml|<(?i:html\b)|!DOCTYPE (?i:html\b))
patterns[88].patterns[0].end  (?=not)impossible
patterns[90].begin  (?=(?><<[-~]?(["'`]?)((?:[_\w]+_|)HAML)\b\1))
patterns[90].end  ^\s*((?:[_\w]+_|)HAML)$\n?
patterns[90].patterns[0].begin  (?><<[-~]?(["'`]?)((?:[_\w]+_|)HAML)\b\1)
patterns[90].patterns[0].while  ^(?!\s*((?:[_\w]+_|)HAML)\s*$)
patterns[104].begin  (?>=\s*<<(["'`]?)([A-Z_a-z]\w*)\1)
patterns[104].end  ^\2$
#heredoc.begin  ^<<[-~]?[A-Z_a-z]\w*
#heredoc.end  $
#interpolated_ruby.patterns[2].match  (#@@)[A-Z_a-z]\w*
#interpolated_ruby.patterns[3].match  (#\$)[A-Z_a-z]\w*
#method_parameters.repository.#brackets.begin  \[  fuzz
#method_parameters.repository.#brackets.end  ]  fuzz
#method_parameters.repository.#parens.begin  \(  fuzz
#method_parameters.repository.#parens.end  \)  fuzz
#nest_brackets.begin  \[
#nest_brackets.end  ]
#nest_brackets_i.begin  \[
#nest_brackets_i.end  ]
#nest_brackets_r.begin  \[
#nest_brackets_r.end  ]
#nest_curly.begin  \{
#nest_curly.end  }
#nest_curly_and_self.patterns[0].begin  \{
#nest_curly_and_self.patterns[0].end  }
#nest_curly_i.begin  \{
#nest_curly_i.end  }
#nest_curly_r.begin  \{
#nest_curly_r.end  }
#nest_ltgt.begin  <
#nest_ltgt.end  >
#nest_ltgt_i.begin  <
#nest_ltgt_i.end  >
#nest_ltgt_r.begin  <
#nest_ltgt_r.end  >
#nest_parens.begin  \(
#nest_parens.end  \)
#nest_parens_i.begin  \(
#nest_parens_i.end  \)
#nest_parens_r.begin  \(
#nest_parens_r.end  \)
#regex_sub.patterns[4].begin  \(\?#
#regex_sub.patterns[4].end  \)
#regex_sub.patterns[6].begin  (?<=^|\s)(#)\s(?=[-\t !,.0-9?A-Za-z[^\x00-\x7F]]*$)
#regex_sub.patterns[6].end  $\n?
#yard_directive.begin  ^(\s*)(#)(\s*)(@!)(endgroup|group|method|parse|scope|visibi…
#yard_directive.end  ^(?!\s*#\3\s{2,}|\s*#\s*$)
#yard_option.begin  ^(\s*)(#)(\s*)(@)(option)(?=\s)(?>\s+([_a-z]\w*:?))?(?>\s+(…
#yard_option.end  ^(?!\s*#\3\s{2,}|\s*#\s*$)
```

### `rust`

```text
#functions.patterns[1].match  \b(pub)(\()  waived: `\b(pub)\b`, the pattern before it, matches wherever `\b(pub)(\()` does
```

### `scss`

```text
#at_rule_at_root.begin  \s*((@)(at-root))(\s+|$)
#at_rule_at_root.end  \s*(?=\{)
#at_rule_charset.begin  \s*((@)charset)\b\s*
#at_rule_charset.end  \s*((?=;|$))
#at_rule_content.end  groups 1
#at_rule_each.end  groups 1, 2
#at_rule_else.end  groups 1, 2
#at_rule_extend.end  groups 1, 2
#at_rule_fontface.patterns[0].begin  ^\s*((@)font-face)\b
#at_rule_fontface.patterns[0].end  \s*(?=\{)
#at_rule_for.end  groups 1, 2
#at_rule_forward.end  groups 1, 2
#at_rule_forward.patterns[1].match  \b([-\w]+)(\*)
#at_rule_function.patterns[0].begin  groups 3
#at_rule_function.patterns[0].end  groups 1, 2, 3
#at_rule_function.patterns[1].match  \s*((@)function)\b\s*
#at_rule_if.end  groups 1, 2
#at_rule_import.begin  \s*((@)import)\b\s*
#at_rule_import.end  \s*((?=;)|(?=}))
#at_rule_include.patterns[1].match  (?<=@include)\s+(?:([-\w]+)\s*(\.))?([-\w]+)
#at_rule_keyframes.begin  (?<=^|\s)(@)(?:-(?:webkit|moz)-)?keyframes\b
#at_rule_keyframes.end  (?<=})
#at_rule_keyframes.patterns[0].match  (?<=@keyframes)\s+((?:[A-Z_a-z][-\w]|-[A-Z_a-z])[-\w]*)
#at_rule_keyframes.patterns[1].begin  (?<=@keyframes)\s+(")
#at_rule_keyframes.patterns[1].end  "
#at_rule_keyframes.patterns[1].patterns[0].match  \\(\h{1,6}|.)
#at_rule_keyframes.patterns[2].begin  (?<=@keyframes)\s+(')
#at_rule_keyframes.patterns[2].end  '
#at_rule_keyframes.patterns[2].patterns[0].match  \\(\h{1,6}|.)
#at_rule_keyframes.patterns[3].begin  \{
#at_rule_keyframes.patterns[3].end  }
#at_rule_keyframes.patterns[3].patterns[0].match  \b(?:(?:100|[1-9]\d|\d)%|from|to)(?=\s*\{)
#at_rule_media.patterns[0].patterns[3].match  \b(only)\b
#at_rule_media.patterns[0].patterns[4].patterns[1].end  groups 1
#at_rule_namespace.patterns[0].begin  (?<=@namespace)\s+(?=url)
#at_rule_namespace.patterns[0].end  (?=;|$)
#at_rule_namespace.patterns[1].begin  (?<=@namespace)\s+([-\w]*)
#at_rule_namespace.patterns[1].end  (?=;|$)
#at_rule_namespace.patterns[2].match  ((@)namespace)\b
#at_rule_option.match  ^\s*((@)option)\b\s*
#at_rule_page.patterns[0].begin  ^\s*((@)page)(?=[:\s])\s*([-:\w]*)
#at_rule_page.patterns[0].end  \s*(?=\{)
#at_rule_return.end  groups 1, 2
#at_rule_supports.begin  (?<=^|\s)(@)supports\b
#at_rule_supports.end  (?=\{)|$
#at_rule_supports.patterns[2].match  \(
#at_rule_supports.patterns[3].match  \)
#at_rule_use.end  groups 1, 2
#at_rule_use.patterns[7].begin  \(
#at_rule_use.patterns[7].end  \)
#at_rule_warn.end  groups 1, 2
#at_rule_while.begin  \s*((@)while)\b\s*
#at_rule_while.end  \s*(?=})
#comment_block.begin  /\*  fuzz
#comment_block.end  \*/
#comment_docblock.begin  ///
#comment_docblock.end  (?=$)
#constant_optional.match  !optional
#constant_sass_functions.begin  (headings|stylesheet-url|rgba?|hsla?|ie-hex-str|red|green|b…
#constant_sass_functions.end  (\))
#function_attributes.patterns[3].match  [;=?@{}]  fuzz
#functions.patterns[0].begin  ([-\w]+)(\()\s*
#functions.patterns[0].end  (\))
#functions.patterns[1].match  ([-\w]+)
#map.patterns[3].match  \b([-\w]+)\s*(:)
#parent_selector_suffix.match  (?<=&)((?:[-0-9A-Z_a-z[^\x00-\x7F]]|\\(?:\h{1,6}|.)|#\{|[$}…
#parent_selector_suffix.captures[2].patterns[1].match  \\(\h{1,6}|.)
#parent_selector_suffix.captures[2].patterns[2].match  [$}]
#selector_attribute.match  groups 4, 9, 10, 11, 12
#selector_attribute.captures[2].patterns[1].match  \\(\h{1,6}|.)
#selector_attribute.captures[2].patterns[2].match  [$}]
#selector_attribute.captures[4].patterns[1].match  \\(\h{1,6}|.)
#selector_attribute.captures[4].patterns[2].match  [$}]
#selector_attribute.captures[7].patterns[1].match  \\(\h{1,6}|.)
#selector_attribute.captures[7].patterns[2].match  [$}]
#selector_attribute.captures[11].patterns[1].match  \\(\h{1,6}|.)
#selector_attribute.captures[11].patterns[2].match  [$}]
#selector_class.captures[2].patterns[1].match  \\(\h{1,6}|.)
#selector_class.captures[2].patterns[2].match  [$}]
#selector_custom.match  \b([0-9A-Za-z]+(-[0-9A-Za-z]+)+)(?=\.|\s++[^:]|\s*[,\[{]|:(…  fuzz
#selector_id.match  (#)((?:[-0-9A-Z_a-z[^\x00-\x7F]]|\\(?:\h{1,6}|.)|#\{|\.?\$|…  fuzz
#selector_id.captures[2].patterns[1].match  \\(\h{1,6}|.)
#selector_id.captures[2].patterns[2].match  [$}]
#selector_placeholder.captures[2].patterns[1].match  \\(\h{1,6}|.)
#selector_placeholder.captures[2].patterns[2].match  [$}]
#selector_pseudo_class.patterns[0].patterns[2].match  (?:(?<=\d)n|\b(n|even|odd))\b
#selector_pseudo_class.patterns[0].patterns[3].match  \w+
#string_double.patterns[0].match  \\(\h{1,6}|.)
#string_single.begin  '  fuzz
#string_single.end  '
#string_single.patterns[0].match  \\(\h{1,6}|.)
#variable_setting.patterns[1].patterns[6].match  ,  fuzz
```

### `shellscript`

```text
#alias_statement.begin  [\t ]*+(alias)[\t ]*+((?:((?<!\w)-\w+)\b[\t ]*+)*)[\t ]*+((…
#alias_statement.end  (?=[\t ]|$)|(?:(?:(?:(;)|(&&))|(\|\|))|(&))
#alias_statement.beginCaptures[2].patterns[0].match  (?<!\w)-\w+\b
#array_value.begin  groups 2, 3, 4, 5, 8
#array_value.patterns[1].match  ((?<!\w)[-0-9A-Z_a-z]+(?!\w))(=)
#array_value.patterns[2].match  (\[)(.+?)(])(=)
#case_statement_context.patterns[1].match  \+  fuzz
#case_statement_context.patterns[2].match  \?
#case_statement_context.patterns[3].match  @
#case_statement_context.patterns[6].match  (?<=\tin| in|[\t ]|;;)\(
#case_statement_context.patterns[7].begin  (?<=\S)(\()  fuzz
#case_statement_context.patterns[7].end  \)  fuzz
#case_statement_context.patterns[8].patterns[0].match  \\.
#command_name_range.patterns[4].begin  groups 1, 4 (fuzz)
#continuation_of_double_quoted_command_name.patterns[0].match  \\[\n"$\\`]
#continuation_of_single_quoted_command_name.begin  \G(?<=')  fuzz
#continuation_of_single_quoted_command_name.end  '  fuzz
#double_quote_context.patterns[0].match  \\[\n"$\\`]
#function_definition.patterns[2].begin  \(
#function_definition.patterns[2].end  \)
#heredoc.patterns[0].begin  ((?<!<)<<-)[\t ]*+(["'])[\t ]*+([^"']+?)(?=["\&';<\s])(\2)(…
#heredoc.patterns[0].end  ^\t*\3(?=[\&;\s]|$)
#heredoc.patterns[1].begin  ((?<!<)<<(?!<))[\t ]*+(["'])[\t ]*+([^"']+?)(?=["\&';<\s])(…
#heredoc.patterns[1].end  ^\3(?=[\&;\s]|$)
#heredoc.patterns[2].begin  ((?<!<)<<-)[\t ]*+([^\t "']+)(?=["\&';<\s])(.*)
#heredoc.patterns[2].end  ^\t*\2(?=[\&;\s]|$)
#herestring.patterns[0].begin  (<<<)\s*(('))
#herestring.patterns[0].end  (')
#herestring.patterns[2].match  (<<<)\s*(([^)\\\s]|\\.)+)
#interpolation.patterns[2].patterns[0].match  \\[$\\`]
#interpolation.patterns[2].patterns[1].begin  (?<=\W)(?=#)(?!#\{)
#interpolation.patterns[2].patterns[1].end  (?!\G)
#interpolation.patterns[2].patterns[1].patterns[0].begin  #
#interpolation.patterns[2].patterns[1].patterns[0].end  (?=`)
#loop.patterns[0].begin  (?<=^|[\&;\s])(for)\s+(.+?)\s+(in)(?=[\&;\s]|$)
#loop.patterns[0].end  (?<=^|[\&;\s])done(?=[\&;\s]|$|\))
#math.patterns[2].match  0[Xx]\h+
#math.patterns[4].match  0\d+
#math.patterns[5].match  \d{1,2}#[0-9@-Z_a-z]+
#math_operators.patterns[1].match  0[Xx]\h+
#math_operators.patterns[2].match  0\d+
#math_operators.patterns[3].match  \d{1,2}#[0-9@-Z_a-z]+
#modified_assignment_statement.patterns[2].match  groups 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13
#normal_assignment_statement.begin  groups 2, 3, 4, 5, 7, 8
#normal_statement_seperator.match  groups 3
#numeric_literal.match  groups 1, 2, 3
#option.begin  groups 2
#pathname.patterns[0].match  (?<=[:=\s]|^)~  fuzz
#pathname.patterns[2].begin  ([!*+?@])(\()
#pathname.patterns[2].end  \)
#pipeline.patterns[0].match  (?<=^|[\&;\s])(time)(?=[\&;\s]|$)
#redirect_fix.match  (>>?)[\t ]*+([^\t\n "$\&-);<>\\`|]+)
#redirection.patterns[0].begin  [<>]\(
#redirection.patterns[0].end  \)
#regex_comparison.match  =~
#string.patterns[3].begin  \$'
#string.patterns[3].end  '
#string.patterns[3].patterns[0].match  \\['\\abefnrtv]
#string.patterns[3].patterns[1].match  \\[0-9]{3}"
#string.patterns[3].patterns[2].match  \\x\h{2}"
#string.patterns[3].patterns[3].match  \\c."
#support.patterns[0].match  (?<=^|[\&;\s])[.:](?=[\&;\s]|$)
```

### `sql`

```text
patterns[1].patterns[0].match  \[|]]
patterns[5].match  groups 4
patterns[7].match  groups 2, 3, 9, 10, 11, 13, 14, 15
patterns[14].match  (?i:\b(grant(\swith\sgrant\soption)?|revoke)\b)
patterns[24].match  (?i)\b(cume_dist|first_value|lag|last_value|lead|percent_ra…
patterns[25].match  (?i)\b((?:bit_coun|get_bi|left_shif|right_shif|set_bi)t)\b\…
patterns[27].match  (?i)\b(collationproperty|tertiary_weights)\b\s*\(
patterns[28].match  (?i)\b(asymkey_id|asymkeyproperty|certproperty|cert_id|cryp…
patterns[29].match  (?i)\b(cursor_status)\b\s*\(
patterns[33].match  (?<!@)@@(?i)\b(cursor_rows|connections|cpu_busy|datefirst|d…
patterns[34].match  (?i)\b(json|isjson|json_object|json_array|json_value|json_q…
patterns[35].match  (?i)\b(choose|iif|greatest|least)\b\s*\(
patterns[40].match  (?i)\b(certencoded|certprivatekey|current_user|database_pri…
patterns[42].match  (?i)\b(binary_checksum|checksum|compress|connectionproperty…
patterns[43].match  (?i)\b(patindex|textptr|textvalid)\b\s*\(
patterns[44].match  (?i)\b(vector_(?:distance|norm|normalize))\b\s*\(
#comments.patterns[0].begin  groups 1
#comments.patterns[1].begin  groups 1
#regexps.patterns[0].begin  /(?=\S.*/)
#regexps.patterns[0].end  /
#regexps.patterns[0].patterns[1].match  \\/
#regexps.patterns[1].begin  %r\{
#regexps.patterns[1].end  }
#string_escape.match  \\.
#string_interpolation.match  (#\{)([^}]*)(})
#strings.patterns[2].match  (`)[^\\`]*(`)
#strings.patterns[3].begin  `  fuzz
#strings.patterns[3].end  `
#strings.patterns[5].begin  "  fuzz
#strings.patterns[5].end  "
#strings.patterns[6].begin  %\{
#strings.patterns[6].end  }
```

### `swift`

```text
#async-throws.match  groups 1, 3
#attributes.patterns[0].patterns[3].match  groups 3
#attributes.patterns[1].begin  ((@)objc)(\()
#attributes.patterns[1].end  \)
#attributes.patterns[1].patterns[0].match  \w*(?::(?:\w*:)*(\w*))?
#attributes.patterns[2].begin  groups 2, 3
#builtin-functions.patterns[2].match  (?<=\.)(?:s(?:uperclassMirror|amePositionIn|tartsWith)|next…
#builtin-global-functions.patterns[0].begin  \b(type)(\()\s*(of)(:)
#builtin-global-functions.patterns[0].end  \)
#builtin-global-functions.patterns[1].match  \ba(?:nyGenerator|utoreleasepool)(?=\s*[({])\b
#builtin-global-functions.patterns[3].match  \b(?:s(?:ort|uffix|pli(?:ce|t))|insert|overlaps|d(?:istance…
#builtin-properties.patterns[0].match  (?<=(?:^|\W)(?:Process\.|CommandLine\.))(arguments|argc|uns…
#builtin-types-builtin-class-type.match  \b(Managed((?:|Proto)Buffer)|NonObjectiveCBase|AnyGenerator…
#builtin-types-builtin-enum-type.patterns[0].match  \b(?:CommandLine|Process(?=\.))\b
#builtin-types-builtin-enum-type.patterns[3].match  \b(?:MirrorDisposition|QuickLookObject)\b
#builtin-types-builtin-protocol-type.patterns[0].match  \bActor\b
#builtin-types-builtin-protocol-type.patterns[2].match  \b(?:Ran(?:domAccessIndex|geReplaceableCollection)Type|Gene…
#builtin-types-builtin-struct-type.patterns[1].match  \b(?:R(?:everse(?:RandomAccess(?:Collection|Index)|Collecti…
#builtin-types-builtin-typealias.patterns[1].match  \b(?:Generator|PlaygroundQuickLook|UWord|Word)\b
#comments.patterns[0].match  \A^(#!).*$\n?
#comments.patterns[1].begin  /\*\*(?!/)
#comments.patterns[1].end  \*/
#comments.patterns[2].begin  /\*:
#comments.patterns[2].end  \*/
#comments.patterns[3].begin  /\*  fuzz
#comments.patterns[3].end  \*/
#comments.patterns[4].match  \*/  fuzz
#comments.patterns[5].begin  groups 1
#comments.patterns[5].patterns[1].begin  //:
#comments.patterns[5].patterns[1].end  $
#comments-nested.begin  /\*
#comments-nested.end  \*/
#compiler-control.patterns[1].end  groups 1, 2
#compiler-control.patterns[1].patterns[2].match  \b(arch)\s*(\()\s*(?:(arm|arm64|powerpc64|powerpc64le|i386|…
#compiler-control.patterns[1].patterns[5].patterns[0].match  \b(simulator|UIKitForMac)\b
#compiler-control.patterns[2].match  groups 3
#compiler-control.patterns[2].captures[3].patterns[0].match  \S+
#compiler-control.patterns[3].match  groups 7
#compiler-control.patterns[3].captures[5].patterns[3].match  \S+
#compiler-control.patterns[3].captures[7].patterns[0].match  \S+
#declarations-available-types.patterns[3].match  \basync\b
#declarations-available-types.patterns[7].match  \b(?:repeat|each)\b
#declarations-available-types.patterns[9].match  \bnonisolated(?:\(nonsending\)|\b)
#declarations-available-types.patterns[15].match  \bprotocol\b
#declarations-available-types-collection-type.patterns[2].match  \b_\b
#declarations-available-types-collection-type.patterns[3].match  (?<=\s)\bof\b(?=\s+[(\[_\p{L}\d\p{N}\p{M}])
#declarations-available-types-collection-type.patterns[4].patterns[0].match  :
#declarations-function.begin  groups 3, 4
#declarations-function-initializer.beginCaptures[1].patterns[0].match  (?<=[!?])[!?]+
#declarations-function-result.patterns[0].match  \bsending\b
#declarations-generic-parameter-clause.patterns[2].match  \blet\b
#declarations-generic-parameter-clause.patterns[3].match  \beach\b
#declarations-import.end  groups 1
#declarations-import.patterns[0].begin  groups 1
#declarations-import.patterns[0].patterns[0].match  groups 1, 2
#declarations-import.patterns[0].patterns[1].match  (?<=\G|\.)\$[0-9]+
#declarations-import.patterns[0].patterns[2].match  (?<=\G|\.)(?:((?<oph>[-!%\&*+/<-?^|~¡-§©«¬®°±¶»¿×÷‖‗†-‧‰-‾⁁…
#declarations-import.patterns[0].patterns[2].captures[1].patterns[0].match  \.
#declarations-import.patterns[0].patterns[3].match  \.
#declarations-import.patterns[0].patterns[4].begin  (?!\s*(;|$|//|/\*))
#declarations-import.patterns[0].patterns[4].end  (?=\s*(;|$|//|/\*))
#declarations-inheritance-clause.begin  groups 1
#declarations-inheritance-clause.patterns[0].begin  \bclass\b
#declarations-inheritance-clause.patterns[0].end  (?=[={}]|(?!\G)\bwhere\b)
#declarations-macro.begin  \b(macro)\s+((?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>))\s*…
#declarations-macro.end  $|(?=;|//|/\*|[=}])
#declarations-operator.end  groups 1
#declarations-operator.patterns[2].match  ((?!$|;|//|/\*)\S)+
#declarations-operator.beginCaptures[4].patterns[0].match  \.
#declarations-operator-swift2.begin  \G(\{)
#declarations-operator-swift2.end  (})
#declarations-operator-swift2.patterns[1].match  \b(associativity)\s+(left|right)\b
#declarations-operator-swift2.patterns[2].match  \b(precedence)\s+([0-9]+)\b
#declarations-operator-swift2.patterns[3].match  \b(assignment)\b
#declarations-operator-swift3.match  groups 3, 4
#declarations-parameter-list.patterns[0].match  groups 2, 3, 5, 6
#declarations-parameter-list.patterns[1].match  groups 3, 4
#declarations-parameter-list.patterns[2].patterns[0].match  \bsending\b
#declarations-parameter-list.patterns[2].patterns[2].match  :  fuzz
#declarations-precedencegroup.begin  \b(precedencegroup)\s+((?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(…
#declarations-precedencegroup.end  (?!\G)
#declarations-precedencegroup.patterns[0].begin  \{
#declarations-precedencegroup.patterns[0].end  }
#declarations-precedencegroup.patterns[0].patterns[1].match  \b((?:high|low)erThan)\s*:\s*((?<q>`?)[_\p{L}][_\p{L}\p{N}\…
#declarations-precedencegroup.patterns[0].patterns[2].match  \b(associativity)\b(?:\s*:\s*(right|left|none)\b)?
#declarations-precedencegroup.patterns[0].patterns[3].match  \b(assignment)\b(?:\s*:\s*(true|false)\b)?
#declarations-protocol.begin  groups 3, 4
#declarations-protocol-associated-type.begin  groups 3, 4
#declarations-protocol-protocol-initializer.begin  (?<!\.)\b(init[!?]*)\s*(?=[(<])
#declarations-protocol-protocol-initializer.end  $|(?=;|//|/\*|})
#declarations-protocol-protocol-initializer.patterns[5].begin  \{
#declarations-protocol-protocol-initializer.patterns[5].end  }
#declarations-protocol-protocol-initializer.beginCaptures[1].patterns[0].match  (?<=[!?])[!?]+
#declarations-protocol-protocol-method.begin  groups 3, 4
#declarations-protocol-protocol-method.patterns[6].begin  \{
#declarations-protocol-protocol-method.patterns[6].end  }
#declarations-type.patterns[0].begin  groups 3, 4
#declarations-type-enum.begin  groups 3, 4
#declarations-type-enum-associated-values.patterns[1].begin  (?:(_)|((?<q1>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*\k<q1>))\s+(((?…
#declarations-type-enum-associated-values.patterns[1].end  (?=[]),])
#declarations-type-enum-associated-values.patterns[3].patterns[1].match  :
#declarations-type-identifier.begin  groups 2, 3
#declarations-typealias.begin  groups 3, 4
#declarations-typed-variable-declaration.begin  groups 1
#expressions-trailing-closure.patterns[0].match  groups 2, 3
#expressions-trailing-closure.patterns[1].match  groups 2, 3
#expressions-without-trailing-closures-or-member-references-availability-condition.patterns[1].match  groups 2
#expressions-without-trailing-closures-or-member-references-compound-name.match  groups 2, 3
#expressions-without-trailing-closures-or-member-references-compound-name.captures[4].patterns[0].match  groups 1, 2
#expressions-without-trailing-closures-or-member-references-expression-element-list.patterns[1].begin  groups 2, 3
#expressions-without-trailing-closures-or-member-references-function-or-macro-call-expression.patterns[0].begin  groups 2, 3
#expressions-without-trailing-closures-or-member-references-parenthesized-expression.end  groups 2
#expressions-without-trailing-closures-or-member-references-parenthesized-expression.endCaptures[2].patterns[0].match  \brethrows\b
#keywords.patterns[3].match  (?<=\s)\bof\b(?=\s+[(\[_\p{L}\d\p{N}\p{M}])
#keywords.patterns[5].match  groups 2
#keywords.patterns[7].match  groups 1
#keywords.patterns[9].match  (?<!\.)\b(?:|re)throws\b  fuzz
#keywords.patterns[10].match  groups 2
#keywords.patterns[11].match  groups 1
#keywords.patterns[12].match  (?<!\.)\b(?:associatedtype|operator|typealias)\b
#keywords.patterns[13].match  (?<!\.)\b(class|enum|extension|precedencegroup|protocol|str…  fuzz
#keywords.patterns[17].match  groups 2, 3, 4
#keywords.patterns[23].match  (?<!\.)\bimport\b
#keywords.patterns[25].match  (?<!\.)\bcopy(?=\s+`?[_\p{L}])
#literals.patterns[4].match  \B#((?:color|image|file)Literal)\b
#literals.patterns[5].match  \B#externalMacro\b
#literals.patterns[6].match  \B#keyPath\b
#literals.patterns[7].begin  \B(#selector)(\()(?:\s*([gs]etter)\s*(:))?
#literals.patterns[7].end  \)
#literals-numeric.patterns[1].match  (\B-|\b)(?<![]()\[_{}\p{L}\p{N}\p{M}]\.)(0x\h[_\h]*)(?:\.\h…
#literals-numeric.patterns[2].match  (\B-|\b)(?<![]()\[_{}\p{L}\p{N}\p{M}]\.)(0x\h[_\h]*)(?:\.\h…
#literals-numeric.patterns[3].match  (\B-|\b)(?<![]()\[_{}\p{L}\p{N}\p{M}]\.)(0x\h[_\h]*)\.[0-9]…
#literals-numeric.patterns[4].match  (?<=\s|^)-?\.[0-9][.\w]*
#literals-numeric.patterns[5].match  (\B-|\b)0[box]_[_\h]*(?:[EPep][-+]?\w+)?[.\w]+
#literals-numeric.patterns[7].match  (\B-|\b)(?<![]()\[_{}\p{L}\p{N}\p{M}]\.)0b[01][01_]*\b(?!\.…
#literals-numeric.patterns[8].match  (\B-|\b)(?<![]()\[_{}\p{L}\p{N}\p{M}]\.)0o[0-7][0-7_]*\b(?!…
#literals-numeric.patterns[11].match  (\B-|\b)[0-9][.\w]*
#literals-regular-expression-literal.patterns[0].begin  (#+)/\n
#literals-regular-expression-literal.patterns[0].end  /\1
#literals-regular-expression-literal.patterns[1].match  (/)(?!\s)(?!/)(?:\\\s(?=/)|(?<guts>(?>(?:\\Q(?:(?!\\E)(?!/)…  fuzz
#literals-regular-expression-literal.patterns[2].match  groups 5
#literals-regular-expression-literal-backreference-or-subpattern.patterns[0].match  (\\g\{)(?:((?!\d)\w+)(?:([-+])(\d+))?|([-+]?\d+)(?:([-+])(\…
#literals-regular-expression-literal-backreference-or-subpattern.patterns[1].match  (\\g)([-+]?\d+)(?:([-+])(\d+))?
#literals-regular-expression-literal-backreference-or-subpattern.patterns[2].match  (\\[gk]<)(?:((?!\d)\w+)(?:([-+])(\d+))?|([-+]?\d+)(?:([-+])…
#literals-regular-expression-literal-backreference-or-subpattern.patterns[3].match  (\\[gk]')(?:((?!\d)\w+)(?:([-+])(\d+))?|([-+]?\d+)(?:([-+])…
#literals-regular-expression-literal-backreference-or-subpattern.patterns[4].match  (\\k\{)((?!\d)\w+)(?:([-+])(\d+))?(})
#literals-regular-expression-literal-backreference-or-subpattern.patterns[5].match  \\[1-9][0-9]+
#literals-regular-expression-literal-backreference-or-subpattern.patterns[6].match  (\(\?(?:P[=>]|&))((?!\d)\w+)(?:([-+])(\d+))?(\))
#literals-regular-expression-literal-backreference-or-subpattern.patterns[7].match  \(\?R\)
#literals-regular-expression-literal-backreference-or-subpattern.patterns[8].match  (\(\?)([-+]?\d+)(?:([-+])(\d+))?(\))
#literals-regular-expression-literal-backtracking-directive-or-global-matching-option.match  (\(\*)(?:(ACCEPT|FAIL|F|MARK(?=:)|(?=:)|COMMIT|PRUNE|SKIP|T…
#literals-regular-expression-literal-callout.match  (\()(?<keyw>\?C)(?:(?<num>\d+)|`(?<name>(?:[^`]|``)*)`|'(?<…
#literals-regular-expression-literal-character-properties.match  \\[Pp]\{([-\s\w]+(?:=[-\s\w]+)?)}|(\[:)([-\s\w]+(?:=[-\s\w]…
#literals-regular-expression-literal-custom-char-class.patterns[0].begin  (\[)(\^)?
#literals-regular-expression-literal-custom-char-class.patterns[0].end  ]
#literals-regular-expression-literal-custom-char-class-members.patterns[0].match  \\b
#literals-regular-expression-literal-group-option-toggle.match  \(\?(?:\^(?:[DJPSUWimnswx]|xx|y\{[gw]})*|(?:[DJPSUWimnswx]|…
#literals-regular-expression-literal-group-or-conditional.patterns[0].begin  (\()(\?~)
#literals-regular-expression-literal-group-or-conditional.patterns[0].end  \)
#literals-regular-expression-literal-group-or-conditional.patterns[1].begin  (\()(?<cond>\?\()(?:(?<NumberRef>(?<num>[-+]?\d+)(?:(?<op>[…
#literals-regular-expression-literal-group-or-conditional.patterns[1].end  \)
#literals-regular-expression-literal-group-or-conditional.patterns[2].begin  groups 4, 5, 6, 8, 9, 10
#literals-regular-expression-literal-line-comment.match  (#).*$
#literals-regular-expression-literal-quote.begin  \\Q
#literals-regular-expression-literal-quote.end  \\E|(\n)
#literals-regular-expression-literal-regex-guts.patterns[1].begin  \(\?#
#literals-regular-expression-literal-regex-guts.patterns[1].end  \)
#literals-regular-expression-literal-regex-guts.patterns[2].begin  <\{
#literals-regular-expression-literal-regex-guts.patterns[2].end  }>
#literals-regular-expression-literal-regex-guts.patterns[5].match  [$^]|\\[ABGYZbyz]|\\K
#literals-regular-expression-literal-regex-guts.patterns[10].match  \\c.
#literals-regular-expression-literal-regex-guts.patterns[11].match  \\[^c]
#literals-regular-expression-literal-regex-guts.patterns[12].match  \|
#literals-regular-expression-literal-regex-guts.patterns[14].match  \{(?:\s*\d+\s*(?:,\s*\d*\s*)?}|\s*,\s*\d+\s*})
#literals-regular-expression-literal-set-operators.patterns[0].match  &&
#literals-regular-expression-literal-set-operators.patterns[1].match  --
#literals-regular-expression-literal-set-operators.patterns[2].match  ~~
#literals-regular-expression-literal-unicode-scalars.match  \\(?:u\{\s*(?:\h+\s*)+}|u\h{4}|x\{\h+}|x\h{0,2}|U\h{8}|o\{[…
#literals-string.patterns[0].end  groups 1
#literals-string.patterns[0].patterns[3].match  \S((?!\\\().)*(?=""")
#literals-string.patterns[1].begin  #"""(?!#)(?=(?:[^"]|"(?!#))*$)
#literals-string.patterns[1].end  """#(#*)
#literals-string.patterns[1].patterns[0].match  \G(?:.+(?=""")|.+)
#literals-string.patterns[1].patterns[1].match  \\#\s*\n
#literals-string.patterns[1].patterns[3].match  \S((?!\\#\().)*(?=""")
#literals-string.patterns[2].begin  (?<!#)(##+)"""(?!\1)(?=(?:[^"]|"(?!\1))*$)
#literals-string.patterns[2].end  """\1(#*)
#literals-string.patterns[2].patterns[0].match  \G(?:.+(?=""")|.+)
#literals-string.patterns[3].end  groups 1
#literals-string.patterns[3].patterns[0].match  [\n\r]  fuzz
#literals-string.patterns[4].begin  (?<!#)(##+)"
#literals-string.patterns[4].end  "\1(#*)
#literals-string.patterns[4].patterns[0].match  [\n\r]
#literals-string.patterns[5].end  groups 1
#literals-string.patterns[5].patterns[0].match  [\n\r]
#literals-string-raw-string-guts.patterns[0].match  \\#["'0\\nrt]
#literals-string-raw-string-guts.patterns[1].match  \\#u\{\h{1,8}}
#literals-string-raw-string-guts.patterns[2].begin  \\#\(
#literals-string-raw-string-guts.patterns[2].end  \)
#literals-string-raw-string-guts.patterns[2].patterns[1].begin  \(
#literals-string-raw-string-guts.patterns[2].patterns[1].end  \)
#literals-string-raw-string-guts.patterns[3].match  \\#.
#literals-string-string-guts.patterns[2].patterns[1].begin  \(
#literals-string-string-guts.patterns[2].patterns[1].end  \)
#literals-string-string-guts.patterns[3].match  \\.
#member-reference.patterns[0].match  groups 2, 3
#operators.patterns[1].patterns[0].captures[0].patterns[0].match  \G(\+\+|--)$
#operators.patterns[1].patterns[0].captures[0].patterns[1].match  \G([-+])$
#operators.patterns[1].patterns[0].captures[0].patterns[2].match  \G!$
#operators.patterns[1].patterns[0].captures[0].patterns[3].match  \G~$
#operators.patterns[1].patterns[1].captures[0].patterns[0].match  \G(\+\+|--)$
#operators.patterns[1].patterns[1].captures[0].patterns[1].match  \G!$
#operators.patterns[1].patterns[2].captures[0].patterns[0].match  \G=$
#operators.patterns[1].patterns[2].captures[0].patterns[1].match  \G([-%*+/]|<<|>>|[\&^|]|&&|\|\|)=$
#operators.patterns[1].patterns[2].captures[0].patterns[2].match  \G([-*+/])$
#operators.patterns[1].patterns[2].captures[0].patterns[3].match  \G&([-*+])$
#operators.patterns[1].patterns[2].captures[0].patterns[4].match  \G%$
#operators.patterns[1].patterns[2].captures[0].patterns[5].match  \G(==|!=|[<>]|>=|<=|~=)$
#operators.patterns[1].patterns[2].captures[0].patterns[6].match  \G\?\?$
#operators.patterns[1].patterns[2].captures[0].patterns[7].match  \G(&&|\|\|)$
#operators.patterns[1].patterns[2].captures[0].patterns[8].match  \G([\&^|]|<<|>>)$
#operators.patterns[1].patterns[2].captures[0].patterns[9].match  \G([!=]==)$
#operators.patterns[1].patterns[2].captures[0].patterns[10].match  \G\?$
#operators.patterns[1].patterns[5].captures[0].patterns[0].match  \G\.\.[.<]$
```

### `toml`

```text
#comments.begin  groups 1
#invalid.match  \S+(\s*(?=\S))?  fuzz
#key_pair.patterns[0].end  groups 1, 2
#key_pair.patterns[1].end  groups 1, 2, 3, 4, 5
#key_pair.patterns[1].captures[3].patterns[0].match  \\(["\\bfnrt]|u\h{4}|U\h{8})
#key_pair.patterns[1].captures[3].patterns[1].match  \\[^"\\bfnrt]
#key_pair.patterns[1].captures[3].patterns[2].match  "
#key_pair.patterns[2].begin  ((')([^']*)('))\s*(=)\s*
#key_pair.patterns[2].end  (?<=\S)(?<!=)|$
#key_pair.patterns[3].end  groups 1, 3
#key_pair.patterns[3].captures[1].patterns[1].match  (")((?:[^"\\]|\\.)*)(")
#key_pair.patterns[3].captures[1].patterns[1].captures[2].patterns[0].match  \\(["\\bfnrt]|u\h{4}|U\h{8})
#key_pair.patterns[3].captures[1].patterns[1].captures[2].patterns[1].match  \\[^"\\bfnrt]
#key_pair.patterns[3].captures[1].patterns[2].match  (')[^']*(')
#primatives.patterns[0].patterns[0].match  \\(["\\bfnrt]|u\h{4}|U\h{8})
#primatives.patterns[0].patterns[1].match  \\[^\n"\\bfnrt]
#primatives.patterns[1].patterns[1].match  \\[^"\\bfnrt]  fuzz
#primatives.patterns[10].match  \G[-+]?(inf|nan)
```

### `tsx`

```text
#access-modifier.match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(abstract|declare|…
#array-binding-pattern.begin  (?:(\.\.\.)\s*)?(\[)
#array-binding-pattern.end  ]
#array-binding-pattern-const.begin  groups 1
#arrow-function.patterns[0].match  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))\b(async)\s+)?(…
#arrow-function.patterns[1].begin  groups 1
#arrow-return-type.begin  (?<=\))\s*(:)
#arrow-return-type.end  (?==>|\{|^(\s*(export|function|class|interface|let|var|\bus…
#arrow-return-type-body.patterns[0].begin  (?<=:)(?=\s*\{)
#arrow-return-type-body.patterns[0].end  (?<=})
#async-modifier.match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(async)(?![$_[:aln…
#brackets.patterns[0].begin  \{
#brackets.patterns[0].end  }|(?=\*/)
#brackets.patterns[1].begin  \[
#brackets.patterns[1].end  ]|(?=\*/)
#class-declaration.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#class-declaration.end  (?<=})
#class-declaration-or-expression-patterns.patterns[2].match  [$_[:alpha:]][$_[:alnum:]]*
#class-expression.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(abstract)\s+)?…
#class-expression.end  (?<=})
#class-or-interface-body.begin  \{
#class-or-interface-body.end  }
#class-or-interface-body.patterns[2].begin  (?<=:)\s*
#class-or-interface-body.patterns[2].end  (?=[-\])+,:;}\s]|^\s*$|^\s*(?:abstract|async|\bawait\s+\bus…
#class-or-interface-heritage.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))\b(extends|impleme…
#class-or-interface-heritage.end  (?=\{)
#class-or-interface-heritage.patterns[4].match  ([$_[:alpha:]][$_[:alnum:]]*)\s*(?:(\.)|(\?\.(?!\s*\d)))(?=…
#class-or-interface-heritage.patterns[5].match  ([$_[:alpha:]][$_[:alnum:]]*)
#comment.patterns[0].begin  /\*\*(?!/)
#comment.patterns[0].end  \*/
#comment.patterns[1].begin  groups 2, 3
#comment.patterns[2].begin  groups 1, 4, 5
#control-statement.patterns[3].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(break|continue|go…
#control-statement.patterns[4].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(break|continue|do…
#control-statement.patterns[6].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(case|default|swit…  fuzz
#control-statement.patterns[9].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(with)(?![$_[:alnu…
#control-statement.patterns[10].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(package)(?![$_[:a…
#control-statement.patterns[11].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(debugger)(?![$_[:…
#declaration.patterns[11].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(declare|export)(?…
#decorator.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))@
#decorator.end  (?=\s)
#destructuring-parameter.patterns[0].begin  groups 1
#destructuring-parameter.patterns[1].begin  groups 1
#destructuring-variable.patterns[0].begin  (?<![:=]|^of|[^$._[:alnum:]]of|^in|[^$._[:alnum:]]in)\s*(?=…
#destructuring-variable.patterns[0].end  (?=$|^|[,;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(of…
#destructuring-variable.patterns[1].begin  (?<![:=]|^of|[^$._[:alnum:]]of|^in|[^$._[:alnum:]]in)\s*(?=…
#destructuring-variable.patterns[1].end  (?=$|^|[,;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(of…
#destructuring-variable-rest.match  (?:(\.\.\.)\s*)?([$_[:alpha:]][$_[:alnum:]]*)
#destructuring-variable-rest-const.match  groups 1
#directives.begin  ^(///)\s*(?=<(reference|amd-dependency|amd-module)(\s+(path…
#directives.end  (?=$)
#directives.patterns[0].begin  (<)(reference|amd-dependency|amd-module)
#directives.patterns[0].end  />
#directives.patterns[0].patterns[0].match  path|types|no-default-lib|lib|name|resolution-mode
#directives.patterns[0].patterns[1].match  =
#docblock.patterns[0].match  ((@)a(?:ccess|pi))\s+(p(?:rivate|rotected|ublic))\b
#docblock.patterns[1].match  ((@)author)\s+([^*/<>@\s](?:[^*/<>@]|\*[^/])*)(?:\s*(<)([^>…
#docblock.patterns[2].match  ((@)borrows)\s+((?:[^*/@\s]|\*[^/])+)\s+(as)\s+((?:[^*/@\s]…
#docblock.patterns[3].begin  ((@)example)\s+
#docblock.patterns[3].end  (?=@|\*/)
#docblock.patterns[3].patterns[0].match  ^\s\*\s+
#docblock.patterns[3].patterns[1].begin  \G(<)caption(>)
#docblock.patterns[3].patterns[1].end  (</)caption(>)|(?=\*/)
#docblock.patterns[3].patterns[2].match  [^*@\s](?:[^*]|\*[^/])*
#docblock.patterns[4].match  ((@)kind)\s+(class|constant|event|external|file|function|me…
#docblock.patterns[5].match  ((@)see)\s+(?:((?=https?://)(?:[^*\s]|\*[^/])+)|((?!https?:…
#docblock.patterns[6].match  ((@)template)\s+([$A-Z_a-z][]$.\[\w]*(?:\s*,\s*[$A-Z_a-z][]…
#docblock.patterns[7].begin  ((@)template)\s+(?=\{)
#docblock.patterns[7].end  (?=\s|\*/|[^]$A-\[_a-{}])
#docblock.patterns[7].patterns[1].match  ([$A-Z_a-z][]$.\[\w]*)
#docblock.patterns[8].match  ((@)(?:arg|argument|const|constant|member|namespace|param|v…
#docblock.patterns[9].begin  ((@)typedef)\s+(?=\{)
#docblock.patterns[9].end  (?=\s|\*/|[^]$A-\[_a-{}])
#docblock.patterns[9].patterns[1].match  (?:[^*/@\s]|\*[^/])+
#docblock.patterns[10].begin  ((@)(?:arg|argument|const|constant|member|namespace|param|p…
#docblock.patterns[10].end  (?=\s|\*/|[^]$A-\[_a-{}])
#docblock.patterns[10].patterns[1].match  ([$A-Z_a-z][]$.\[\w]*)
#docblock.patterns[10].patterns[2].match  (\[)\s*[$\w]+(?:(?:\[])?\.[$\w]+)*(?:\s*(=)\s*((?>"(?:\*(?!…
#docblock.patterns[11].begin  ((@)(?:define|enum|exception|export|extends|lends|implement…
#docblock.patterns[11].end  (?=\s|\*/|[^]$A-\[_a-{}])
#docblock.patterns[12].match  ((@)(?:alias|augments|callback|constructs|emits|event|fires…
#docblock.patterns[13].begin  ((@)(?:default(?:value)?|license|version))\s+((["']))
#docblock.patterns[13].end  (\3)|(?=$|\*/)
#docblock.patterns[14].match  ((@)(?:default(?:value)?|license|tutorial|variation|version…
#docblock.patterns[15].match  (@)(?:abstract|access|alias|api|arg|argument|async|attribut…
#docblock.patterns[17].match  ((@)[$_[:alpha:]][$_[:alnum:]]*)(?=\s+)
#enum-declaration.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#enum-declaration.end  (?<=})
#enum-declaration.patterns[1].begin  \{
#enum-declaration.patterns[1].end  }
#enum-declaration.patterns[1].patterns[1].begin  ([$_[:alpha:]][$_[:alnum:]]*)
#enum-declaration.patterns[1].patterns[1].end  (?=[,}]|$)
#enum-declaration.patterns[1].patterns[2].begin  (?=(('([^'\\]|\\.)*')|("([^"\\]|\\.)*")|(`([^\\`]|\\.)*`)|(…
#enum-declaration.patterns[1].patterns[2].end  (?=[,}]|$)
#export-declaration.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(export)\s+(as)\s+…
#export-declaration.patterns[1].begin  groups 2, 3
#export-declaration.patterns[2].begin  groups 2
#expression-inside-possibly-arrow-parens.patterns[5].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(override|public|p…
#expression-inside-possibly-arrow-parens.patterns[6].match  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(override|publi…
#expression-inside-possibly-arrow-parens.patterns[7].match  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(override|publi…  fuzz
#expression-inside-possibly-arrow-parens.patterns[10].match  ,  fuzz
#expression-operators.patterns[1].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(yield)(?![$_[:aln…
#expression-operators.patterns[1].end  \*
#expression-operators.patterns[2].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(yield)(?![$_[:aln…
#expression-operators.patterns[3].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))delete(?![$_[:alnu…
#expression-operators.patterns[5].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))of(?![$_[:alnum:]]…
#expression-operators.patterns[6].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))instanceof(?![$_[:…
#expression-operators.patterns[7].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))new(?![$_[:alnum:]…
#expression-operators.patterns[9].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))void(?![$_[:alnum:…  fuzz
#expression-operators.patterns[11].begin  groups 2
#expression-operators.patterns[13].match  (?:\*|(?<!\()/|[-%+])=
#expression-operators.patterns[14].match  (?:[\&^]|<<|>>>??|\|)=
#expression-operators.patterns[15].match  <<|>>>?
#expression-operators.patterns[17].match  <=|>=|<>|[<>]  fuzz
#expression-operators.patterns[18].match  (?<=[$_[:alnum:]])(!)\s*(?:(/=)|(/)(?![*/]))
#expression-operators.patterns[20].match  [\&^|~]  fuzz
#expression-operators.patterns[22].match  --
#expression-operators.patterns[23].match  \+\+
#expression-operators.patterns[25].begin  (?<=[]$)_[:alnum:]])\s*(?=(/\*([^*]|(\*[^/]))*\*/\s*)+(?:(/…
#expression-operators.patterns[25].end  (/=)|(/)(?!\*([^*]|(\*[^/]))*\*/)
#expression-operators.patterns[26].match  (?<=[]$)_[:alnum:]])\s*(?:(/=)|(/)(?![*/]))
#field-declaration.begin  groups 1
#field-declaration.patterns[6].match  groups 3
#field-declaration.patterns[9].match  !
#for-loop.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))for(?=((\s+|(\s*/\…
#for-loop.end  (?<=\))
#for-loop.patterns[1].match  await
#for-loop.patterns[2].begin  \(
#for-loop.patterns[2].end  \)
#function-body.patterns[6].match  \*
#function-call.patterns[1].begin  (?=(((([$_[:alpha:]][$_[:alnum:]]*)(\s*\??\.\s*(#?[$_[:alph…
#function-call.patterns[1].end  (?<=>)(?!(((([$_[:alpha:]][$_[:alnum:]]*)(\s*\??\.\s*(#?[$_…
#function-call.patterns[1].patterns[0].begin  (?=(([$_[:alpha:]][$_[:alnum:]]*)(\s*\??\.\s*(#?[$_[:alpha:…
#function-call.patterns[1].patterns[0].end  (?=(<\s*[(\[{]\s*)$)
#function-call-optionals.patterns[1].match  !
#function-declaration.begin  groups 2, 5
#function-expression.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(async)\s+)?(fu…  fuzz
#function-expression.end  (?=;)|(?<=})  fuzz
#function-name.match  [$_[:alpha:]][$_[:alnum:]]*  fuzz
#identifiers.patterns[1].match  (?:(?:(\.)|(\?\.(?!\s*\d)))\s*)?([$_[:alpha:]][$_[:alnum:]]…
#identifiers.patterns[2].match  (?:(\.)|(\?\.(?!\s*\d)))\s*(#?\p{upper}[$_\d[:upper:]]*)(?!…
#if-statement.patterns[0].patterns[2].begin  (?<=\))\s*/(?![*/])(?=(?:[^/\[\\]|\\.|\[([^]\\]|\\.)*])+/([…
#if-statement.patterns[0].patterns[2].end  (/)([dgimsuvy]*)
#import-declaration.begin  groups 1, 2
#import-equals-declaration.patterns[0].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#import-equals-declaration.patterns[0].end  \)
#import-equals-declaration.patterns[1].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#import-equals-declaration.patterns[1].end  (?=;|$|^)
#import-equals-declaration.patterns[1].patterns[2].match  ([$_[:alpha:]][$_[:alnum:]]*)\s*(?:(\.)|(\?\.(?!\s*\d)))
#import-equals-declaration.patterns[1].patterns[3].match  ([$_[:alpha:]][$_[:alnum:]]*)
#import-export-assert-clause.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(with)|(assert)…
#import-export-assert-clause.end  }
#import-export-assert-clause.patterns[2].match  [$_[:alpha:]][$_[:alnum:]]*\s*(?=(/\*([^*]|(\*[^/]))*\*/\s*…
#import-export-assert-clause.patterns[3].match  :
#import-export-clause.patterns[1].match  groups 1, 2, 5, 13, 15
#import-export-clause.patterns[3].match  \*
#import-export-clause.patterns[4].match  \b(default)\b
#import-export-clause.patterns[5].match  groups 3
#indexer-declaration.begin  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(readonly)\s*)?…
#indexer-declaration.end  (])\s*(\?\s*)?|$
#indexer-mapped-type-declaration.begin  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))([-+])?(readonl…
#indexer-mapped-type-declaration.end  (])([-+])?\s*(\?\s*)?|$
#indexer-mapped-type-declaration.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(as)\s+
#inline-tags.patterns[0].match  (\[)[^]]+(])(?=\{@(?:link|linkcode|linkplain|tutorial))
#inline-tags.patterns[1].begin  (\{)((@)(?:link(?:code|plain)?|tutorial))\s*
#inline-tags.patterns[1].end  }|(?=\*/)
#inline-tags.patterns[1].patterns[0].match  \G((?=https?://)(?:[^*|}\s]|\*/)+)(\|)?
#inline-tags.patterns[1].patterns[1].match  \G((?:[^*@{|}\s]|\*[^/])+)(\|)?
#instanceof-expr.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(instanceof)(?![$_…
#instanceof-expr.end  (?<=\))|(?=[-\])+,:;>?}]|\|\||&&|!==|$|([!=]==?)|(([\&^|~]\…
#interface-declaration.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#interface-declaration.end  (?<=})
#interface-declaration.patterns[2].match  [$_[:alpha:]][$_[:alnum:]]*
#jsdoctype.patterns[0].begin  \G(\{)
#jsdoctype.patterns[0].end  ((}))\s*|(?=\*/)
#jsx-entities.patterns[0].match  (&)([0-9A-Za-z]+|#[0-9]+|#x\h+)(;)
#jsx-string-single-quoted.begin  '
#jsx-string-single-quoted.end  '
#jsx-tag.end  groups 3, 4
#jsx-tag.patterns[0].begin  groups 2, 3
#jsx-tag-attribute-name.match  groups 1 (fuzz), 2 (fuzz)
#jsx-tag-attributes-illegal.match  \S+  fuzz
#jsx-tag-without-attributes.begin  groups 2, 3
#jsx-tag-without-attributes.end  groups 2, 3
#label.patterns[0].begin  ([$_[:alpha:]][$_[:alnum:]]*)\s*(:)(?=\s*\{)  fuzz
#label.patterns[0].end  (?<=})  fuzz
#label.patterns[1].match  ([$_[:alpha:]][$_[:alnum:]]*)\s*(:)  fuzz
#method-declaration.patterns[0].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(override)\s+…
#method-declaration.patterns[0].end  (?=[,;}]|$)|(?<=})
#method-declaration.patterns[1].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(override)\s+…  fuzz
#method-declaration.patterns[1].end  (?=[,;}]|$)|(?<=})
#method-declaration.patterns[2].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(override)\s+…  fuzz
#method-declaration.patterns[2].end  (?=[,;}]|$)|(?<=})  fuzz
#method-declaration-name.begin  (?=(\b((?<!\$)0[Xx]\h[_\h]*(n)?\b(?!\$))|\b((?<!\$)0[Bb][01…  fuzz
#method-declaration-name.end  (?=[(<])  fuzz
#method-declaration-name.patterns[3].match  [$_[:alpha:]][$_[:alnum:]]*  fuzz
#method-declaration-name.patterns[4].match  \?
#namespace-declaration.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#namespace-declaration.end  (?<=})|(?=;|^\s*$|^\s*(?:abstract|async|\bawait\s+\busing(?…
#namespace-declaration.patterns[2].match  ([$_[:alpha:]][$_[:alnum:]]*)
#numeric-literal.patterns[0].match  \b(?<!\$)0[Xx]\h[_\h]*(n)?\b(?!\$)
#numeric-literal.patterns[1].match  \b(?<!\$)0[Bb][01][01_]*(n)?\b(?!\$)
#numeric-literal.patterns[2].match  \b(?<!\$)0[Oo]?[0-7][0-7_]*(n)?\b(?!\$)
#numeric-literal.patterns[3].match  groups 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14
#numericConstant-literal.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))NaN(?![$_[:alnum:]…
#numericConstant-literal.patterns[1].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))Infinity(?![$_[:al…
#object-binding-element.patterns[1].begin  (?=(\b((?<!\$)0[Xx]\h[_\h]*(n)?\b(?!\$))|\b((?<!\$)0[Bb][01…
#object-binding-element.patterns[1].end  (?=[,}])
#object-binding-element-const.patterns[1].begin  (?=(\b((?<!\$)0[Xx]\h[_\h]*(n)?\b(?!\$))|\b((?<!\$)0[Bb][01…
#object-binding-element-const.patterns[1].end  (?=[,}])
#object-binding-pattern.begin  (?:(\.\.\.)\s*)?(\{)
#object-binding-pattern.end  }
#object-binding-pattern-const.begin  groups 1
#object-identifiers.patterns[0].match  ([$_[:alpha:]][$_[:alnum:]]*)(?=\s*\??\.\s*prototype\b(?!\$…
#object-identifiers.patterns[1].match  groups 2, 3
#object-identifiers.patterns[2].match  groups 1
#object-literal-method-declaration.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(async)\s+)?(…  fuzz
#object-literal-method-declaration.end  (?=[,;}])|(?<=})  fuzz
#object-literal-method-declaration.patterns[2].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(async)\s+)?(…  fuzz
#object-literal-method-declaration.patterns[2].end  (?=[(<])  fuzz
#object-member.patterns[2].begin  (?=\[)  fuzz
#object-member.patterns[2].end  (?=:)|((?<=])(?=\s*[(<]))  fuzz
#object-member.patterns[4].begin  (?=\b((?<!\$)0[Xx]\h[_\h]*(n)?\b(?!\$))|\b((?<!\$)0[Bb][01]…
#object-member.patterns[4].end  (?=:)|(?=\s*([(,<}])|(\s+as|satisifies\s+))
#object-member.patterns[5].begin  (?<=[]"'`])(?=\s*[(<])
#object-member.patterns[5].end  (?=[,;}])|(?<=})
#object-member.patterns[6].match  (?![$_[:alpha:]])(\d+)\s*(?=(/\*([^*]|(\*[^/]))*\*/\s*)*:)
#object-member.patterns[7].match  ([$_[:alpha:]][$_[:alnum:]]*)\s*(?=(/\*([^*]|(\*[^/]))*\*/\…
#object-member.patterns[11].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(as)\s+(const)(?=\…
#object-member.patterns[12].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(as)|(satisfies…  fuzz
#object-member.patterns[12].end  (?=[-\])+,:;>?}]|\|\||&&|!==|$|^|((?<![$_[:alnum:]])(?:(?<=…  fuzz
#object-member.patterns[13].begin  (?=[$_[:alpha:]][$_[:alnum:]]*\s*=)  fuzz
#object-member.patterns[13].end  (?=[,}]|$|//|/\*)  fuzz
#object-member.patterns[14].patterns[0].begin  (?<=:)\s*(async)?(?=\s*(<\s*(((const\s+)?[$_[:alpha:]])|(\{…
#object-member.patterns[14].patterns[0].end  (?<=\))
#object-member.patterns[14].patterns[0].patterns[1].begin  \(
#object-member.patterns[14].patterns[0].patterns[1].end  \)
#object-member.patterns[14].patterns[1].begin  (?<=:)\s*(async)?\s*(\()(?=\s*((([\[{]\s*)?)$|((\{([^{}]|(\…
#object-member.patterns[14].patterns[1].end  \)
#object-member.patterns[14].patterns[2].begin  (?<=:)\s*(async)?\s*(?=<\s*$)
#object-member.patterns[14].patterns[2].end  (?<=>)
#object-member.patterns[14].patterns[3].begin  (?<=>)\s*(\()(?=\s*((([\[{]\s*)?)$|((\{([^{}]|(\{([^{}]|\{[…
#object-member.patterns[14].patterns[3].end  \)
#parameter-array-binding-pattern.begin  (?:(\.\.\.)\s*)?(\[)
#parameter-array-binding-pattern.end  ]
#parameter-name.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(override|public|p…
#parameter-name.patterns[1].match  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(override|publi…
#parameter-name.patterns[2].match  groups 1, 2 (fuzz), 3, 5
#parameter-object-binding-pattern.begin  (?:(\.\.\.)\s*)?(\{)  fuzz
#parameter-object-binding-pattern.end  }  fuzz
#paren-expression-possibly-arrow.patterns[0].begin  groups 1
#paren-expression-possibly-arrow.patterns[1].begin  groups 1
#possibly-arrow-return-type.begin  (?<=\)|^)\s*(:)(?=\s*([^()<>{}]|<([^<>]|<([^<>]|<[^<>]+>)+>…
#possibly-arrow-return-type.end  (?==>|\{|^(\s*(export|function|class|interface|let|var|\bus…
#property-accessor.match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(accessor|get|set)…
#qstring-double.end  groups 2 (fuzz)
#qstring-single.end  groups 2 (fuzz)
#regex.patterns[1].begin  ((?<![]$)_[:alnum:]]|\+\+|--|}|\*/)|((?<=^return|[^$._[:aln…
#regex.patterns[1].end  (/)([dgimsuvy]*)
#regex-character-class.patterns[0].match  \\[DSWdfnrstvw]|\.
#regex-character-class.patterns[1].match  \\([0-7]{3}|x\h{2}|u\h{4})
#regex-character-class.patterns[2].match  \\c[A-Z]
#regex-character-class.patterns[3].match  \\.
#regexp.patterns[0].match  \\[Bb]|[$^]
#regexp.patterns[1].match  \\(?:[1-9]\d*|k<([$A-Z_a-z][$\w]*)>)
#regexp.patterns[2].match  [*+?]|\{(\d+,\d+|\d+,|,\d+|\d+)}\??
#regexp.patterns[3].match  \|
#regexp.patterns[4].begin  (\()((\?=)|(\?!)|(\?<=)|(\?<!))
#regexp.patterns[4].end  (\))
#regexp.patterns[5].begin  \((?:(\?:)|\?<([$A-Z_a-z][$\w]*)>)?
#regexp.patterns[5].end  \)
#regexp.patterns[6].begin  (\[)(\^)?
#regexp.patterns[6].end  (])
#regexp.patterns[6].patterns[0].match  (?:.|(\\(?:[0-7]{3}|x\h{2}|u\h{4}))|(\\c[A-Z])|(\\.))-(?:[^…
#return-type.patterns[0].begin  (?<=\))\s*(:)(?=\s*\S)  fuzz
#return-type.patterns[0].end  (?<![\&:|])(?=$|^|[,;{}]|//)
#return-type.patterns[1].begin  (?<=\))\s*(:)
#return-type.patterns[1].end  (?<![\&:|])((?=[,;{}]|//|^\s*$)|((?<=\S)(?=\s*$)))
#return-type-core.patterns[1].begin  (?<=[\&:|])(?=\s*\{)
#return-type-core.patterns[1].end  (?<=})
#shebang.match  \A(#!).*(?=$)
#single-line-comment-consuming-line-ending.begin  (^[\t ]+)?((//)(?:\s*((@)internal)(?=\s|$))?)
#single-line-comment-consuming-line-ending.end  (?=^)
#super-literal.match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))super\b(?!\$)
#support-function-call-identifiers.patterns[4].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))import(?=\s*\(\s*[…
#support-objects.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(arguments)\b(?!\$)
#support-objects.patterns[1].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(Promise)\b(?!\$)
#support-objects.patterns[2].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(import)\s*(?:(\.)…
#support-objects.patterns[3].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(new)\s*(?:(\.)|(\…
#support-objects.patterns[4].match  groups 4
#support-objects.patterns[5].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(exports)|(modu…
#switch-statement.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?=\bswitch\s*\()
#switch-statement.end  }
#switch-statement.patterns[1].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(switch)\s*(\()
#switch-statement.patterns[1].end  \)
#switch-statement.patterns[2].begin  \{
#switch-statement.patterns[2].end  (?=})
#switch-statement.patterns[2].patterns[0].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(case|default(?=:)…
#switch-statement.patterns[2].patterns[0].end  (?=:)
#switch-statement.patterns[2].patterns[1].begin  (:)\s*(\{)
#switch-statement.patterns[2].patterns[1].end  }
#switch-statement.patterns[2].patterns[2].match  (:)
#template.patterns[1].begin  groups 1
#template-call.patterns[0].begin  (?=(([$_[:alpha:]][$_[:alnum:]]*\s*\??\.\s*)*|(\??\.\s*)?)(…  fuzz
#template-call.patterns[0].end  (?=`)  fuzz
#template-call.patterns[0].patterns[0].begin  (?=(([$_[:alpha:]][$_[:alnum:]]*\s*\??\.\s*)*|(\??\.\s*)?)(…  fuzz
#template-call.patterns[0].patterns[0].end  (?=(<\s*(((keyof|infer|typeof|readonly)\s+)|(([$_[:alpha:]]…  fuzz
#template-call.patterns[0].patterns[0].patterns[1].match  ([$_[:alpha:]][$_[:alnum:]]*)  fuzz
#template-call.patterns[1].begin  ([$_[:alpha:]][$_[:alnum:]]*)?\s*(?=(<\s*(((keyof|infer|typ…
#template-call.patterns[1].end  (?=`)
#template-type.patterns[1].begin  ([$_[:alpha:]][$_[:alnum:]]*)?(`)
#template-type.patterns[1].end  `
#template-type-substitution-element.begin  \$\{
#template-type-substitution-element.end  }
#this-literal.match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))this\b(?!\$)
#type.patterns[13].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(readonly)(?![$_[:…
#type-alias-declaration.begin  groups 2
#type-alias-declaration.patterns[2].begin  (=)\s*(intrinsic)(?![$_[:alnum:]])(?:(?=\.\.\.)|(?!\.))
#type-alias-declaration.patterns[2].end  (?=[;}]|^\s*$|^\s*(?:abstract|async|\bawait\s+\busing(?=\s+…
#type-annotation.patterns[1].begin  (:)
#type-annotation.patterns[1].end  (?<![\&:|])((?=[]),;}]|//)|(?==[^>])|(?=^\s*$)|((?<=[]$)>_}…
#type-arguments-body.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(_)(?![$_[:alnum:]…
#type-conditional.patterns[0].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(extends)\s+
#type-conditional.patterns[0].end  (?<=:)
#type-conditional.patterns[0].patterns[0].begin  \?
#type-conditional.patterns[0].patterns[0].end  :
#type-fn-type-parameters.patterns[0].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(abstract)\s+)?…
#type-fn-type-parameters.patterns[0].end  (?<=>)
#type-fn-type-parameters.patterns[1].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(abstract)\s+)?…
#type-fn-type-parameters.patterns[1].end  (?<=\))
#type-function-return-type.patterns[1].begin  =>
#type-function-return-type.patterns[1].end  (?<!=>)(?<![\&|])((?=[]),:;=>?{}]|//|^\s*$)|((?<=\S)(?=\s*$…
#type-function-return-type-core.patterns[1].begin  (?<==>)(?=\s*\{)
#type-function-return-type-core.patterns[1].end  (?<=})
#type-infer.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(infer)\s+([$_[:al…
#type-name.patterns[0].begin  ([$_[:alpha:]][$_[:alnum:]]*)\s*(?:(\.)|(\?\.(?!\s*\d)))\s*…
#type-name.patterns[0].end  (>)
#type-name.patterns[2].match  groups 3 (fuzz)
#type-object.patterns[6].begin  \.\.\.  fuzz
#type-object.patterns[6].end  (?=[,;}]|$)|(?<=})  fuzz
#type-operators.patterns[5].match  ([:?])  fuzz
#type-operators.patterns[6].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))import(?=\s*\()
#type-parameters.begin  (<)  fuzz
#type-parameters.end  (>)  fuzz
#type-parameters.patterns[1].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(extends|in|out|co…
#type-parameters.patterns[4].match  (=)(?!>)  fuzz
#type-paren-or-function-parameters.patterns[0].match  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(public|private…
#type-paren-or-function-parameters.patterns[1].match  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(public|private…
#type-paren-or-function-parameters.patterns[3].match  ,
#type-predicate-operator.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(asserts)\s+)?(…
#type-predicate-operator.patterns[1].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(asserts)\s+(?!is)…
#type-predicate-operator.patterns[2].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))asserts(?![$_[:aln…
#type-predicate-operator.patterns[3].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))is(?![$_[:alnum:]]…
#type-tuple.patterns[0].match  \.\.\.
#type-tuple.patterns[1].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))([$_[:alpha:]][$_[…
#var-expr.patterns[0].patterns[0].begin  groups 1, 2
#var-expr.patterns[0].patterns[5].begin  (,)\s*(?=$|//)
#var-expr.patterns[0].patterns[5].end  (?<!,)(((?=[;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))…
#var-expr.patterns[1].begin  groups 1, 2
#var-expr.patterns[1].patterns[0].begin  groups 1, 2
#var-expr.patterns[1].patterns[5].begin  (,)\s*(?=$|//)
#var-expr.patterns[1].patterns[5].end  (?<!,)(((?=[;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))…
#var-expr.patterns[2].begin  (?=(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s…
#var-expr.patterns[2].end  (?!(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s…
#var-expr.patterns[2].patterns[0].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#var-expr.patterns[2].patterns[0].end  (?=\S)
#var-expr.patterns[2].patterns[4].begin  (,)\s*((?!\S)|(?=//))
#var-expr.patterns[2].patterns[4].end  (?<!,)(((?=[;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))…
#var-single-variable.patterns[0].begin  ([$_[:alpha:]][$_[:alnum:]]*)(!)?(?=\s*(=\s*(((async\s+)?((…
#var-single-variable.patterns[0].end  (?=$|^|[,;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(of…
#var-single-variable.patterns[1].begin  (\p{upper}[$_\d[:upper:]]*)(?![$_[:alnum:]])(!)?
#var-single-variable.patterns[1].end  (?=$|^|[,;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(of…
#var-single-variable.patterns[2].begin  groups 2
```

### `typescript`

```text
#array-binding-pattern.begin  (?:(\.\.\.)\s*)?(\[)
#array-binding-pattern.end  ]
#array-binding-pattern-const.begin  groups 1
#arrow-function.patterns[0].match  groups 1
#arrow-return-type-body.patterns[0].begin  (?<=:)(?=\s*\{)
#arrow-return-type-body.patterns[0].end  (?<=})
#brackets.patterns[0].begin  \{
#brackets.patterns[0].end  }|(?=\*/)
#cast.patterns[0].match  \s*(<)\s*(const)\s*(>)
#cast.patterns[1].begin  (?<!\+\+|--)(?<=^return|[^$._[:alnum:]]return|^throw|[^$._[…
#cast.patterns[1].end  (>)
#cast.patterns[2].begin  (?<=^)\s*(<)(?=[$_[:alpha:]][$_[:alnum:]]*\s*>)
#cast.patterns[2].end  (>)
#class-declaration.begin  groups 2, 3
#class-expression.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(abstract)\s+)?…  fuzz
#class-expression.end  (?<=})
#class-or-interface-body.patterns[2].begin  (?<=:)\s*
#class-or-interface-body.patterns[2].end  (?=[-\])+,:;}\s]|^\s*$|^\s*(?:abstract|async|\bawait\s+\bus…
#class-or-interface-heritage.patterns[4].match  groups 3
#comment.patterns[1].begin  groups 2, 3
#comment.patterns[2].begin  groups 1, 4, 5
#control-statement.patterns[3].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(break|continue|go…
#control-statement.patterns[6].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(case|default|swit…
#control-statement.patterns[9].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(with)(?![$_[:alnu…  fuzz
#control-statement.patterns[10].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(package)(?![$_[:a…
#control-statement.patterns[11].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(debugger)(?![$_[:…
#declaration.patterns[11].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(declare|export)(?…
#destructuring-parameter.patterns[0].begin  groups 1
#destructuring-parameter.patterns[1].begin  (?<![:=])\s*(?:(\.\.\.)\s*)?(\[)  fuzz
#destructuring-parameter.patterns[1].end  ]  fuzz
#destructuring-parameter-rest.match  groups 1
#destructuring-variable.patterns[0].begin  (?<![:=]|^of|[^$._[:alnum:]]of|^in|[^$._[:alnum:]]in)\s*(?=…
#destructuring-variable.patterns[0].end  (?=$|^|[,;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(of…
#destructuring-variable.patterns[1].begin  (?<![:=]|^of|[^$._[:alnum:]]of|^in|[^$._[:alnum:]]in)\s*(?=…
#destructuring-variable.patterns[1].end  (?=$|^|[,;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(of…
#destructuring-variable-rest.match  (?:(\.\.\.)\s*)?([$_[:alpha:]][$_[:alnum:]]*)
#docblock.patterns[0].match  ((@)a(?:ccess|pi))\s+(p(?:rivate|rotected|ublic))\b
#docblock.patterns[1].match  ((@)author)\s+([^*/<>@\s](?:[^*/<>@]|\*[^/])*)(?:\s*(<)([^>…
#docblock.patterns[2].match  ((@)borrows)\s+((?:[^*/@\s]|\*[^/])+)\s+(as)\s+((?:[^*/@\s]…
#docblock.patterns[3].begin  ((@)example)\s+
#docblock.patterns[3].end  (?=@|\*/)
#docblock.patterns[3].patterns[0].match  ^\s\*\s+
#docblock.patterns[3].patterns[1].begin  \G(<)caption(>)
#docblock.patterns[3].patterns[1].end  (</)caption(>)|(?=\*/)
#docblock.patterns[3].patterns[2].match  [^*@\s](?:[^*]|\*[^/])*
#docblock.patterns[4].match  ((@)kind)\s+(class|constant|event|external|file|function|me…
#docblock.patterns[5].match  ((@)see)\s+(?:((?=https?://)(?:[^*\s]|\*[^/])+)|((?!https?:…
#docblock.patterns[7].begin  ((@)template)\s+(?=\{)
#docblock.patterns[7].end  (?=\s|\*/|[^]$A-\[_a-{}])
#docblock.patterns[7].patterns[1].match  ([$A-Z_a-z][]$.\[\w]*)
#docblock.patterns[9].begin  ((@)typedef)\s+(?=\{)
#docblock.patterns[9].end  (?=\s|\*/|[^]$A-\[_a-{}])
#docblock.patterns[9].patterns[1].match  (?:[^*/@\s]|\*[^/])+
#docblock.patterns[10].patterns[2].match  (\[)\s*[$\w]+(?:(?:\[])?\.[$\w]+)*(?:\s*(=)\s*((?>"(?:\*(?!…
#docblock.patterns[12].match  ((@)(?:alias|augments|callback|constructs|emits|event|fires…
#docblock.patterns[13].begin  ((@)(?:default(?:value)?|license|version))\s+((["']))
#docblock.patterns[13].end  (\3)|(?=$|\*/)
#docblock.patterns[14].match  ((@)(?:default(?:value)?|license|tutorial|variation|version…
#enum-declaration.begin  groups 2, 3
#enum-declaration.patterns[1].patterns[2].begin  (?=(('([^'\\]|\\.)*')|("([^"\\]|\\.)*")|(`([^\\`]|\\.)*`)|(…
#enum-declaration.patterns[1].patterns[2].end  (?=[,}]|$)
#export-declaration.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(export)\s+(as)\s+…
#export-declaration.patterns[1].begin  groups 2, 3
#expression-inside-possibly-arrow-parens.patterns[5].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(override|public|p…
#expression-inside-possibly-arrow-parens.patterns[6].match  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(override|publi…
#expression-inside-possibly-arrow-parens.patterns[7].match  groups 1, 2, 3, 5
#expression-inside-possibly-arrow-parens.patterns[10].match  ,  fuzz
#expression-operators.patterns[1].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(yield)(?![$_[:aln…
#expression-operators.patterns[1].end  \*
#expression-operators.patterns[2].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(yield)(?![$_[:aln…
#expression-operators.patterns[6].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))instanceof(?![$_[:…
#expression-operators.patterns[7].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))new(?![$_[:alnum:]…
#expression-operators.patterns[18].match  (?<=[$_[:alnum:]])(!)\s*(?:(/=)|(/)(?![*/]))
#expression-operators.patterns[25].begin  (?<=[]$)_[:alnum:]])\s*(?=(/\*([^*]|(\*[^/]))*\*/\s*)+(?:(/…
#expression-operators.patterns[25].end  (/=)|(/)(?!\*([^*]|(\*[^/]))*\*/)
#expression-operators.patterns[26].match  (?<=[]$)_[:alnum:]])\s*(?:(/=)|(/)(?![*/]))
#field-declaration.patterns[6].match  groups 3
#field-declaration.patterns[9].match  !
#for-loop.patterns[1].match  await
#function-body.patterns[6].match  \*
#function-call.patterns[1].begin  (?=(((([$_[:alpha:]][$_[:alnum:]]*)(\s*\??\.\s*(#?[$_[:alph…
#function-call.patterns[1].end  (?<=>)(?!(((([$_[:alpha:]][$_[:alnum:]]*)(\s*\??\.\s*(#?[$_…
#function-call.patterns[1].patterns[0].begin  (?=(([$_[:alpha:]][$_[:alnum:]]*)(\s*\??\.\s*(#?[$_[:alpha:…
#function-call.patterns[1].patterns[0].end  (?=(<\s*[(\[{]\s*)$)
#function-call-optionals.patterns[1].match  !
#function-declaration.begin  groups 2, 3, 5
#function-expression.begin  groups 3
#identifiers.patterns[1].match  groups 2
#identifiers.patterns[2].match  groups 2
#if-statement.patterns[0].patterns[2].begin  (?<=\))\s*/(?![*/])(?=(?:[^/\[\\]|\\.|\[([^]\\]|\\.)*])+/([…
#if-statement.patterns[0].patterns[2].end  (/)([dgimsuvy]*)
#import-declaration.begin  groups 1, 2
#import-equals-declaration.patterns[0].begin  groups 1, 2
#import-equals-declaration.patterns[1].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#import-equals-declaration.patterns[1].end  (?=;|$|^)
#import-equals-declaration.patterns[1].patterns[2].match  ([$_[:alpha:]][$_[:alnum:]]*)\s*(?:(\.)|(\?\.(?!\s*\d)))
#import-equals-declaration.patterns[1].patterns[3].match  ([$_[:alpha:]][$_[:alnum:]]*)
#import-export-assert-clause.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(with)|(assert)…
#import-export-assert-clause.end  }
#import-export-assert-clause.patterns[2].match  [$_[:alpha:]][$_[:alnum:]]*\s*(?=(/\*([^*]|(\*[^/]))*\*/\s*…
#import-export-assert-clause.patterns[3].match  :
#import-export-clause.patterns[1].match  groups 1, 2, 5, 13, 15
#import-export-clause.patterns[3].match  \*
#import-export-clause.patterns[4].match  \b(default)\b
#import-export-clause.patterns[5].match  groups 3
#indexer-declaration.begin  (?:(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(readonly)\s*)?…
#indexer-declaration.end  (])\s*(\?\s*)?|$
#indexer-mapped-type-declaration.end  groups 2
#inline-tags.patterns[0].match  (\[)[^]]+(])(?=\{@(?:link|linkcode|linkplain|tutorial))
#inline-tags.patterns[1].patterns[0].match  \G((?=https?://)(?:[^*|}\s]|\*/)+)(\|)?
#inline-tags.patterns[1].patterns[1].match  groups 2
#interface-declaration.begin  groups 2, 3
#label.patterns[0].begin  ([$_[:alpha:]][$_[:alnum:]]*)\s*(:)(?=\s*\{)
#label.patterns[0].end  (?<=})
#label.patterns[1].match  ([$_[:alpha:]][$_[:alnum:]]*)\s*(:)  fuzz
#method-declaration.patterns[0].begin  groups 1, 2, 3, 4
#method-declaration.patterns[1].begin  groups 1, 2, 3, 4, 5, 6
#method-declaration.patterns[2].begin  groups 1, 2, 3, 4, 6
#namespace-declaration.begin  groups 1
#namespace-declaration.patterns[2].match  ([$_[:alpha:]][$_[:alnum:]]*)
#numeric-literal.patterns[0].match  \b(?<!\$)0[Xx]\h[_\h]*(n)?\b(?!\$)
#numeric-literal.patterns[1].match  \b(?<!\$)0[Bb][01][01_]*(n)?\b(?!\$)
#numeric-literal.patterns[2].match  \b(?<!\$)0[Oo]?[0-7][0-7_]*(n)?\b(?!\$)
#numeric-literal.patterns[3].match  groups 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14
#numericConstant-literal.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))NaN(?![$_[:alnum:]…
#numericConstant-literal.patterns[1].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))Infinity(?![$_[:al…
#object-binding-element.patterns[1].begin  (?=(\b((?<!\$)0[Xx]\h[_\h]*(n)?\b(?!\$))|\b((?<!\$)0[Bb][01…
#object-binding-element.patterns[1].end  (?=[,}])
#object-binding-pattern.begin  (?:(\.\.\.)\s*)?(\{)
#object-binding-pattern.end  }
#object-binding-pattern-const.begin  groups 1
#object-identifiers.patterns[0].match  ([$_[:alpha:]][$_[:alnum:]]*)(?=\s*\??\.\s*prototype\b(?!\$…
#object-identifiers.patterns[1].match  groups 3
#object-literal-method-declaration.begin  groups 3
#object-literal-method-declaration.patterns[2].begin  groups 1, 2, 3
#object-member.patterns[2].begin  (?=\[)  fuzz
#object-member.patterns[2].end  (?=:)|((?<=])(?=\s*[(<]))  fuzz
#object-member.patterns[3].begin  (?=["'`])  fuzz
#object-member.patterns[3].end  (?=:)|((?<=["'`])(?=((\s*[(,<}])|(\s+(as|satisifies)\s+))))  fuzz
#object-member.patterns[4].begin  (?=\b((?<!\$)0[Xx]\h[_\h]*(n)?\b(?!\$))|\b((?<!\$)0[Bb][01]…
#object-member.patterns[4].end  (?=:)|(?=\s*([(,<}])|(\s+as|satisifies\s+))
#object-member.patterns[5].begin  (?<=[]"'`])(?=\s*[(<])
#object-member.patterns[5].end  (?=[,;}])|(?<=})
#object-member.patterns[6].match  (?![$_[:alpha:]])(\d+)\s*(?=(/\*([^*]|(\*[^/]))*\*/\s*)*:)
#object-member.patterns[11].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(as)\s+(const)(?=\…
#object-member.patterns[12].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(as)|(satisfies…
#object-member.patterns[12].end  (?=[-\])+,:;>?}]|\|\||&&|!==|$|^|((?<![$_[:alnum:]])(?:(?<=…
#object-member.patterns[13].begin  (?=[$_[:alpha:]][$_[:alnum:]]*\s*=)  fuzz
#object-member.patterns[13].end  (?=[,}]|$|//|/\*)  fuzz
#object-member.patterns[14].patterns[0].begin  (?<=:)\s*(async)?(?=\s*(<\s*(((const\s+)?[$_[:alpha:]])|(\{…
#object-member.patterns[14].patterns[0].end  (?<=\))
#object-member.patterns[14].patterns[0].patterns[1].begin  \(
#object-member.patterns[14].patterns[0].patterns[1].end  \)
#object-member.patterns[14].patterns[1].begin  (?<=:)\s*(async)?\s*(\()(?=\s*((([\[{]\s*)?)$|((\{([^{}]|(\…  fuzz
#object-member.patterns[14].patterns[1].end  \)  fuzz
#object-member.patterns[14].patterns[2].begin  (?<=:)\s*(async)?\s*(?=<\s*$)
#object-member.patterns[14].patterns[2].end  (?<=>)
#object-member.patterns[14].patterns[3].begin  (?<=>)\s*(\()(?=\s*((([\[{]\s*)?)$|((\{([^{}]|(\{([^{}]|\{[…
#object-member.patterns[14].patterns[3].end  \)
#parameter-array-binding-pattern.begin  (?:(\.\.\.)\s*)?(\[)
#parameter-array-binding-pattern.end  ]
#parameter-name.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(override|public|p…
#parameter-name.patterns[1].match  groups 2, 3
#parameter-name.patterns[2].match  groups 1, 3
#parameter-object-binding-element.patterns[1].begin  (?=(\b((?<!\$)0[Xx]\h[_\h]*(n)?\b(?!\$))|\b((?<!\$)0[Bb][01…  fuzz
#parameter-object-binding-element.patterns[1].end  (?=[,}])  fuzz
#parameter-object-binding-pattern.begin  (?:(\.\.\.)\s*)?(\{)
#parameter-object-binding-pattern.end  }
#paren-expression-possibly-arrow.patterns[0].begin  groups 1
#paren-expression-possibly-arrow.patterns[1].begin  (?<=[(,=]|=>|^return|[^$._[:alnum:]]return)\s*(async)?(?=\s…
#paren-expression-possibly-arrow.patterns[1].end  (?<=\))
#possibly-arrow-return-type.begin  (?<=\)|^)\s*(:)(?=\s*([^()<>{}]|<([^<>]|<([^<>]|<[^<>]+>)+>…  fuzz
#possibly-arrow-return-type.end  (?==>|\{|^(\s*(export|function|class|interface|let|var|\bus…  fuzz
#qstring-double.end  groups 2 (fuzz)
#qstring-single.end  groups 2 (fuzz)
#regex-character-class.patterns[1].match  \\([0-7]{3}|x\h{2}|u\h{4})
#regex-character-class.patterns[2].match  \\c[A-Z]
#regexp.patterns[1].match  \\(?:[1-9]\d*|k<([$A-Z_a-z][$\w]*)>)
#regexp.patterns[4].begin  groups 3, 5, 6
#regexp.patterns[5].begin  groups 2
#regexp.patterns[6].patterns[0].match  groups 1, 2, 5, 6
#return-type-core.patterns[1].begin  (?<=[\&:|])(?=\s*\{)
#return-type-core.patterns[1].end  (?<=})
#shebang.match  \A(#!).*(?=$)
#single-line-comment-consuming-line-ending.begin  groups 1, 4, 5
#super-literal.match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))super\b(?!\$)
#support-function-call-identifiers.patterns[4].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))import(?=\s*\(\s*[…
#support-objects.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(arguments)\b(?!\$)
#support-objects.patterns[2].match  groups 3
#support-objects.patterns[3].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(new)\s*(?:(\.)|(\…
#support-objects.patterns[4].match  groups 4
#support-objects.patterns[5].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(exports)|(modu…
#switch-statement.begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?=\bswitch\s*\()
#switch-statement.end  }
#switch-statement.patterns[1].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(switch)\s*(\()
#switch-statement.patterns[1].end  \)
#switch-statement.patterns[2].begin  \{
#switch-statement.patterns[2].end  (?=})
#switch-statement.patterns[2].patterns[0].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(case|default(?=:)…
#switch-statement.patterns[2].patterns[0].end  (?=:)
#switch-statement.patterns[2].patterns[1].begin  (:)\s*(\{)
#switch-statement.patterns[2].patterns[1].end  }
#switch-statement.patterns[2].patterns[2].match  (:)
#template.patterns[1].begin  groups 1
#template-call.patterns[0].begin  (?=(([$_[:alpha:]][$_[:alnum:]]*\s*\??\.\s*)*|(\??\.\s*)?)(…
#template-call.patterns[0].end  (?=`)
#template-call.patterns[0].patterns[0].begin  (?=(([$_[:alpha:]][$_[:alnum:]]*\s*\??\.\s*)*|(\??\.\s*)?)(…
#template-call.patterns[0].patterns[0].end  (?=(<\s*(((keyof|infer|typeof|readonly)\s+)|(([$_[:alpha:]]…
#template-call.patterns[0].patterns[0].patterns[1].match  ([$_[:alpha:]][$_[:alnum:]]*)
#template-call.patterns[1].begin  ([$_[:alpha:]][$_[:alnum:]]*)?\s*(?=(<\s*(((keyof|infer|typ…
#template-call.patterns[1].end  (?=`)
#template-type.patterns[1].begin  groups 1
#type-alias-declaration.begin  groups 2
#type-alias-declaration.patterns[2].begin  (=)\s*(intrinsic)(?![$_[:alnum:]])(?:(?=\.\.\.)|(?!\.))
#type-alias-declaration.patterns[2].end  (?=[;}]|^\s*$|^\s*(?:abstract|async|\bawait\s+\busing(?=\s+…
#type-annotation.patterns[1].begin  (:)
#type-annotation.patterns[1].end  (?<![\&:|])((?=[]),;}]|//)|(?==[^>])|(?=^\s*$)|((?<=[]$)>_}…
#type-arguments-body.patterns[0].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(_)(?![$_[:alnum:]…
#type-fn-type-parameters.patterns[0].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(abstract)\s+)?…
#type-fn-type-parameters.patterns[0].end  (?<=>)
#type-fn-type-parameters.patterns[1].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:(abstract)\s+)?…
#type-fn-type-parameters.patterns[1].end  (?<=\))
#type-function-return-type-core.patterns[1].begin  (?<==>)(?=\s*\{)
#type-function-return-type-core.patterns[1].end  (?<=})
#type-infer.patterns[0].match  groups 3
#type-name.patterns[0].begin  ([$_[:alpha:]][$_[:alnum:]]*)\s*(?:(\.)|(\?\.(?!\s*\d)))\s*…
#type-name.patterns[0].end  (>)
#type-name.patterns[2].match  groups 3
#type-object.patterns[6].begin  \.\.\.
#type-object.patterns[6].end  (?=[,;}]|$)|(?<=})
#type-paren-or-function-parameters.patterns[0].match  groups 1, 2, 3, 5
#type-paren-or-function-parameters.patterns[1].match  groups 1, 2, 3
#type-predicate-operator.patterns[0].match  groups 1, 2
#type-predicate-operator.patterns[1].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(asserts)\s+(?!is)…
#type-predicate-operator.patterns[2].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))asserts(?![$_[:aln…
#type-predicate-operator.patterns[3].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))is(?![$_[:alnum:]]…
#type-tuple.patterns[1].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))([$_[:alpha:]][$_[…
#var-expr.patterns[0].patterns[0].begin  groups 2
#var-expr.patterns[0].patterns[5].begin  (,)\s*(?=$|//)
#var-expr.patterns[0].patterns[5].end  (?<!,)(((?=[;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))…
#var-expr.patterns[1].begin  groups 2
#var-expr.patterns[1].patterns[0].begin  groups 2
#var-expr.patterns[1].patterns[5].begin  (,)\s*(?=$|//)
#var-expr.patterns[1].patterns[5].end  (?<!,)(((?=[;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))…
#var-expr.patterns[2].begin  (?=(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s…
#var-expr.patterns[2].end  (?!(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s…
#var-expr.patterns[2].patterns[0].begin  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(?:\b(export)\s+)?…
#var-expr.patterns[2].patterns[0].end  (?=\S)
#var-expr.patterns[2].patterns[4].begin  (,)\s*((?!\S)|(?=//))
#var-expr.patterns[2].patterns[4].end  (?<!,)(((?=[;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))…
#var-single-variable.patterns[0].begin  ([$_[:alpha:]][$_[:alnum:]]*)(!)?(?=\s*(=\s*(((async\s+)?((…
#var-single-variable.patterns[0].end  (?=$|^|[,;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(of…
#var-single-variable.patterns[1].begin  (\p{upper}[$_\d[:upper:]]*)(?![$_[:alnum:]])(!)?
#var-single-variable.patterns[1].end  (?=$|^|[,;=}]|((?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))(of…
#var-single-variable.patterns[2].begin  groups 2
```

### `xml`

```text
patterns[0].end  groups 2
patterns[1].end  groups 2, 3
patterns[4].end  groups 2, 3, 4, 5
patterns[7].begin  <%@
patterns[7].end  %>
patterns[7].patterns[0].match  page|include|taglib
patterns[8].begin  <%[!=]?(?!--)
patterns[8].end  (?!--)%>
#EntityDecl.begin  groups 3
#EntityDecl.end  groups 2, 3, 4, 5
#bare-ampersand.match  &  fuzz
#comments.patterns[0].begin  <%--
#comments.patterns[1].patterns[0].begin  --(?!>)
#parameterEntity.match  (%)([:A-Z_a-z][-.0-:A-Z_a-z]*)(;)
```

### `yaml`

```text
patterns[4].match  ^\.{3}
#block-pair.patterns[0].begin  \?
#block-pair.patterns[0].end  (?=\?)|^ *(:)|(:)
#block-scalar.begin  groups 2 (fuzz), 3, 5 (fuzz)
#block-scalar.beginCaptures[5].patterns[1].match  .+  fuzz
#comment.begin  groups 1
#directive.begin  ^%
#directive.end  (?=$|[\t ]+($|#))
#directive.patterns[0].match  \G(YAML)[\t ]+(\d+\.\d+)
#directive.patterns[1].match  \G(TAG)(?:[\t ]+(!(?:[-0-9A-Za-z]*!)?)(?:[\t ]+(!(?:%\h{2}|…
#directive.patterns[2].match  \G(\w+)(?:[\t ]+(\w+)(?:[\t ]+(\w+))?)?
#directive.patterns[3].match  \S+
#flow-alias.match  groups 4
#flow-mapping.patterns[1].match  ,
#flow-pair.patterns[0].begin  \?
#flow-pair.patterns[0].end  (?=[],}])
#flow-pair.patterns[0].patterns[3].begin  :(?=\s|$|[],\[{}])
#flow-pair.patterns[0].patterns[3].end  (?=[],}])
#flow-pair.patterns[1].begin  (?=(?:[^-\]!"#%\&'*,:>?@\[`{|}\s]|[-:?][^],\[{}\s])([^],:\[…  fuzz
#flow-pair.patterns[1].end  (?=\s*$|\s+#|\s*:(\s|$)|\s*:[],\[{}]|\s*[],\[{}])  fuzz
#flow-pair.patterns[1].patterns[1].begin  [^-\]!"#%\&'*,:>?@\[`{|}\s]|[-:?][^],\[{}\s]  fuzz
#flow-pair.patterns[1].patterns[1].end  (?=\s*$|\s+#|\s*:(\s|$)|\s*:[],\[{}]|\s*[],\[{}])  fuzz
#flow-pair.patterns[3].begin  :(?=\s|$|[],\[{}])  fuzz
#flow-pair.patterns[3].end  (?=[],}])
#flow-scalar-double-quoted.patterns[0].match  \\([ "/0LN\\_abefnprtv]|x\d\d|u\d{4}|U\d{8})
#flow-scalar-double-quoted.patterns[1].match  \\\n  fuzz
#flow-scalar-plain-in-implicit-type.patterns[0].match  (?:(null|Null|NULL|~)|([Yy]|yes|Yes|YES|[Nn]|no|No|NO|true|…  fuzz
#flow-scalar-plain-out-implicit-type.patterns[0].match  groups 1, 5, 6
#flow-scalar-single-quoted.patterns[0].match  ''
#flow-value.patterns[0].begin  \G(?![],}])  fuzz
#flow-value.patterns[0].end  (?=[],}])
#property.patterns[0].match  groups 4
#property.patterns[1].match  \G!(?:<(?:%\h{2}|[]!#$\&-;=?-\[_a-z~])+>|(?:[-0-9A-Za-z]*!)…
#property.patterns[2].match  \S+
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
