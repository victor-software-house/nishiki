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
| C | `c` | 321 of 342 | 93.9% | 318 of 341 | 93.3% | 44 | 0 | yes |
| C++ | `cpp` | 877 of 912 | 96.2% | 2480 of 2679 | 92.6% | 234 | 0 | yes |
| C++ | `cpp-macro` | 593 of 611 | 97.1% | 1532 of 1645 | 93.1% | 131 | 0 | yes |
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
| Lua | `lua` | 147 of 152 | 96.7% | 72 of 74 | 97.3% | 7 | 0 | ready |
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
