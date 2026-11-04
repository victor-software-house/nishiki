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
| CSS | `css` | 219 of 222 | 98.6% | 181 of 184 | 98.4% | 6 | 0 | yes |
| diff | `diff` | 10 of 16 | 62.5% | 10 of 23 | 43.5% | 0 | 1 |  |
| Dockerfile | `docker` | 8 of 9 | 88.9% | 5 of 13 | 38.5% | 0 | 0 |  |
| ERB | `erb` | 6 of 12 | 50.0% | 6 of 14 | 42.9% | 0 | 8 |  |
| Go | `go` | 201 of 336 | 59.8% | 145 of 250 | 58.0% | 0 | 2 |  |
| GraphQL | `graphql` | 76 of 88 | 86.4% | 83 of 113 | 73.5% | 0 | 7 |  |
| HTML | `html` | 131 of 227 | 57.7% | 162 of 368 | 44.0% | 0 | 2 |  |
| HTML | `html-derivative` | 2 of 3 | 66.7% | 3 of 3 | 100.0% | 0 | 1 |  |
| INI | `ini` | 14 of 15 | 93.3% | 10 of 12 | 83.3% | 0 | 0 |  |
| Java | `java` | 157 of 214 | 73.4% | 102 of 134 | 76.1% | 0 | 19 |  |
| TypeScript | `javascript` | 532 of 534 | 99.6% | 600 of 605 | 99.2% | 7 | 0 | yes |
| JSON | `json` | 14 of 23 | 60.9% | 10 of 15 | 66.7% | 0 | 10 |  |
| JSON | `jsonc` | 19 of 23 | 82.6% | 13 of 15 | 86.7% | 0 | 0 |  |
| JSON | `jsonl` | 13 of 23 | 56.5% | 11 of 15 | 73.3% | 0 | 6 |  |
| TypeScript | `jsx` | 532 of 534 | 99.6% | 600 of 605 | 99.2% | 7 | 0 | yes |
| Kotlin | `kotlin` | 57 of 65 | 87.7% | 25 of 35 | 71.4% | 0 | 0 |  |
| Less | `less` | 581 of 590 | 98.5% | 417 of 437 | 95.4% | 29 | 0 | yes |
| Lua | `lua` | 81 of 152 | 53.3% | 32 of 74 | 43.2% | 0 | 3 |  |
| Make | `make` | 58 of 75 | 77.3% | 23 of 32 | 71.9% | 0 | 0 |  |
| Markdown | `markdown` | 205 of 311 | 65.9% | 266 of 373 | 71.3% | 213 | 0 | yes |
| Python | `python` | 349 of 350 | 99.7% | 311 of 372 | 83.6% | 62 | 0 | yes |
| Ruby | `ruby` | 168 of 336 | 50.0% | 139 of 293 | 47.4% | 0 | 8 |  |
| Rust | `rust` | 94 of 95 | 98.9% | 97 of 99 | 98.0% | 3 | 0 | yes |
| SCSS | `scss` | 161 of 168 | 95.8% | 136 of 179 | 76.0% | 50 | 0 | yes |
| shell | `shellscript` | 119 of 174 | 68.4% | 106 of 199 | 53.3% | 0 | 10 |  |
| SQL | `sql` | 47 of 75 | 62.7% | 42 of 79 | 53.2% | 0 | 9 |  |
| Swift | `swift` | 425 of 448 | 94.9% | 441 of 448 | 98.4% | 30 | 0 | yes |
| TOML | `toml` | 54 of 54 | 100.0% | 39 of 53 | 73.6% | 14 | 0 | yes |
| TypeScript | `tsx` | 532 of 534 | 99.6% | 600 of 605 | 99.2% | 7 | 0 | yes |
| TypeScript | `typescript` | 513 of 515 | 99.6% | 571 of 576 | 99.1% | 7 | 0 | yes |
| XML | `xml` | 23 of 32 | 71.9% | 46 of 66 | 69.7% | 0 | 1 |  |
| YAML | `yaml` | 60 of 62 | 96.8% | 51 of 55 | 92.7% | 6 | 0 | yes |

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
#at-rules.patterns[0].patterns[0].match  groups 2 (waived: the group matches only blanks, and the oracle compares no blank), 4 (waived: the group matches only blanks, and the oracle compares no blank)
#at-rules.patterns[0].patterns[2].patterns[0].begin  (?:\G|^)(?=[^"]+$)  waived: group 6 of `#at-rules.patterns[0].patterns[0]`, earlier in the `@charset` rule, takes an unclosed string from the same `"`, and a closed one fails the lookahead
#at-rules.patterns[0].patterns[2].patterns[0].end  $  waived: its begin never matches
#at-rules.patterns[3].end  groups 0 (waived: the expression is a lookahead, so the match holds no text)
#escapes.patterns[1].end  ^(?<!\G)  waived: the begin, `\\$\s*`, takes the line feed, so `\G` stands at the start of the next line and `^(?<!\G)` never holds
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
#comment.patterns[2].begin  groups 1 (waived: the group matches only the blanks that lead the line, and the oracle compares no blank)
#expression-operators.patterns[6].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))instanceof(?![$_[:…  waived: `#instanceof-expr` begins with the same expression and comes before `#expression-operators` in `#expressionWithoutIdentifiers`, the only rule that includes it
#expression-operators.patterns[7].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))new(?![$_[:alnum:]…  waived: `#new-expr` begins with the same expression and comes before `#expression-operators` in `#expressionWithoutIdentifiers`, the only rule that includes it
#single-line-comment-consuming-line-ending.begin  groups 1 (waived: the group matches only the blanks that lead the line, and the oracle compares no blank)
#support-objects.patterns[4].match  groups 4 (waived: the group sits inside the negative lookahead after the property names, so it never holds text; the constant names are group 53)
#template.patterns[1].begin  groups 1 (waived: `#template-call`, the pattern before it, starts at the same name wherever this group would match one)
#template-type.patterns[1].begin  groups 1 (waived: `#template-call`, the pattern before it, starts at the same name wherever this group would match one)
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
#comment.patterns[2].begin  groups 1 (waived: the group matches only the blanks that lead the line, and the oracle compares no blank)
#expression-operators.patterns[6].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))instanceof(?![$_[:…  waived: `#instanceof-expr` begins with the same expression and comes before `#expression-operators` in `#expressionWithoutIdentifiers`, the only rule that includes it
#expression-operators.patterns[7].match  (?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))new(?![$_[:alnum:]…  waived: `#new-expr` begins with the same expression and comes before `#expression-operators` in `#expressionWithoutIdentifiers`, the only rule that includes it
#single-line-comment-consuming-line-ending.begin  groups 1 (waived: the group matches only the blanks that lead the line, and the oracle compares no blank)
#support-objects.patterns[4].match  groups 4 (waived: the group sits inside the negative lookahead after the property names, so it never holds text; the constant names are group 53)
#template.patterns[1].begin  groups 1 (waived: `#template-call`, the pattern before it, starts at the same name wherever this group would match one)
#template-type.patterns[1].begin  groups 1 (waived: `#template-call`, the pattern before it, starts at the same name wherever this group would match one)
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
#fenced_code_block_coffee.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(coffee|Cakefile|coffee.erb)…  waived: the fence's only pattern includes `source.coffee`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_coffee.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `source.coffee`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_coffee.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `source.coffee`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_coffee.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `source.coffee`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_elixir.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(elixir)((\s+|[,:?{])[^`]*)?…  waived: the fence's only pattern includes `source.elixir`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_elixir.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `source.elixir`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_elixir.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `source.elixir`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_elixir.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `source.elixir`, which the oracle does not load, so vscode-textmate drops the rule
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
#fenced_code_block_objc.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(objectivec|objective-c|mm|o…  waived: the fence's only pattern includes `source.objc`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_objc.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `source.objc`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_objc.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `source.objc`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_objc.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `source.objc`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_perl.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(perl|pl|pm|pod|t|PL|psgi|vc…  waived: the fence's only pattern includes `source.perl`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_perl.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `source.perl`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_perl.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `source.perl`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_perl.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `source.perl`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_perl6.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(perl6|p6|pl6|pm6|nqp)((\s+|…  waived: the fence's only pattern includes `source.perl.6`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_perl6.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `source.perl.6`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_perl6.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `source.perl.6`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_perl6.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `source.perl.6`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_pug.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(jade|pug)((\s+|[,:?{])[^`]*…  waived: the fence's only pattern includes `text.pug`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_pug.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `text.pug`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_pug.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `text.pug`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_pug.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `text.pug`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_r.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:([RSrs]|Rprofile|\{\.r.+?})(…  waived: the fence's only pattern includes `source.r`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_r.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `source.r`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_r.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `source.r`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_r.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `source.r`, which the oracle does not load, so vscode-textmate drops the rule
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
#fenced_code_block_vs_net.begin  (^|\G)(\s*)(`{3,}|~{3,})\s*(?i:(vb)((\s+|[,:?{])[^`]*)?$)  waived: the fence's only pattern includes `source.asp.vb.net`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_vs_net.end  (^|\G)(\2|\s{0,3})(\3)\s*$  waived: the fence's only pattern includes `source.asp.vb.net`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_vs_net.patterns[0].begin  (^|\G)(\s*)(.*)  waived: the fence's only pattern includes `source.asp.vb.net`, which the oracle does not load, so vscode-textmate drops the rule
#fenced_code_block_vs_net.patterns[0].while  (^|\G)(?!\s*([`~]{3,})\s*$)  waived: the fence's only pattern includes `source.asp.vb.net`, which the oracle does not load, so vscode-textmate drops the rule
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
patterns[32].begin  (?=def\b)(?<=^|\s)(def)\s+((?>[A-Z_a-z]\w*(?>\.|::))?(?>[A-…
patterns[32].end  (?=;)|(?<=[]!"')?`}\w])(?=\s*#|\s*$)
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
#method_parameters.repository.#brackets.begin  \[
#method_parameters.repository.#brackets.end  ]
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
#strings.patterns[2].match  (`)[^\\`]*(`)  fuzz
#strings.patterns[3].begin  `  fuzz
#strings.patterns[3].end  `  fuzz
#strings.patterns[5].begin  "  fuzz
#strings.patterns[5].end  "
#strings.patterns[6].begin  %\{
#strings.patterns[6].end  }
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
#block-pair.patterns[0].begin  groups 1 (waived: the grammar names the group, but the expression has no groups)
#comment.begin  groups 1 (waived: the group matches only blanks, and the oracle compares no blank)
#flow-pair.patterns[0].patterns[3].begin  :(?=\s|$|[],\[{}])  waived: `#flow-pair`, earlier in the same list, includes `#flow-pair.patterns[3]`, whose begin is the same expression
#flow-pair.patterns[0].patterns[3].end  (?=[],}])  waived: its begin never matches
#flow-pair.patterns[3].end  groups 0 (waived: the group is a lookahead, so it never holds text)
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
