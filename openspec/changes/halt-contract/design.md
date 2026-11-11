# Design

This design sets one contract for a language that hands a region of its line to another language and takes the line back.
It covers the halts, what they mean, and the state at a halt.

The TextMate engine is its first new user.
ERB's injection at any depth (xml-html) and Ruby's language-named heredocs ([#159][pr159], go-lexer) use the same one.

## 1. Where the engine hands off

The generator reads which grammars are hand-lexed from one list beside `oracle/licences.ts`: Shiki name, scope name, `Language` variant, and the entries the language offers (section 3).
An include in an engine grammar is one of three kinds:

1. An include of a whole hand-lexed grammar (`source.python`, `text.html.basic`, or `$self` of one) hands off to that language's root.
   Julia's `py"""` rule and Vue's block bodies take this form.
2. A rule whose only pattern includes one repository rule of a hand-lexed grammar hands off to that rule.
   It does so when the language offers that rule as an entry.
   Vue's `source.ts.embedded.html.vue`, whose patterns are `[include source.ts#expression]`, hands off to TypeScript's `expression` entry.
3. Repository rules of a hand-lexed grammar mixed into an engine list cannot be one hand-off.
   Vue's style attribute mixes them: `source.css#comment-block`, `#property-names` and its own matches.
   The generator compiles those rules into the engine's table as it compiles any grammar.
   So the grammar needs a licence entry for bundling, like any other.

A grammar whose rules are only reached through hand-offs is not compiled into the table.
It needs no licence entry for bundling: the hand lexer's own notice covers its scopes.

## 2. What the table holds

A hand-off is one more `RuleKind`, `HandOff`, standing where the include stood in its list, with:

1. the target: a `Language` and an entry (section 3);
2. `before`: the patterns listed before the include in the same list, and the embedding rule's end unless the rule sets `applyEndLast`;
3. `after`: the patterns listed after the include, and the end when the rule sets `applyEndLast`.

Vue's template body is `[vue template-tag, vue capitalized-tag, include text.html.derivative, include text.html.basic]`.
The two Vue rules and the body's end are `before` for the hand-off to `Language::HtmlDerivative`.
So the engine takes the line back where one of them matches, opens it, and hands back after it.

Two hand-offs in one list are ordered like any other patterns.
The second is tried only where the first did not match, so it sits in the first's `after`.

Vue's second include, `text.html.basic` after html-derivative, is redundant.
html-derivative holds all of HTML's root patterns but `tags-invalid`.
Its own tag rule matches wherever `tags-invalid` would (xml-html).
Julia's `py"""` rule keeps `#string_dollar_sign_interpolate` as an `after` pattern.

## 3. Entries

A language that can be handed a region offers one or more entries:

```rust
pub(crate) enum Entry {
    /// The grammar's root, as `include source.x` reaches it.
    Root,
    /// A repository rule the language exposes, such as TypeScript's
    /// `expression`.
    Named(&'static str),
}
```

`Language::start_embedded` gains the entry: `start_embedded(arena, base, entry)`.
A language starts an entry it does not offer as `Root`.
The generator hands off only to entries the list in section 1 names, so the case is not reached.

xml-html adds TypeScript's `expression`; no other entry is planned.
Shiki's `html-derivative` is a language of its own, `Language::HtmlDerivative` (approved as xml-html's language PR).
So Vue's `include text.html.derivative` is a hand-off to its root and no entry of HTML.

## 4. Halts

Today a halt is `fn(&[u8], usize) -> bool` ([`src/language.rs`][language]), and [#159][pr159] passes `end` and `after`.
An engine end is a regular expression whose back-references are filled when the rule opens.
So it cannot be a function known at compile time.
`Halts` becomes one struct of three tests:

```rust
pub(crate) struct Halts<'a> {
    pub(crate) before: &'a dyn Fn(&[u8], usize) -> bool,
    pub(crate) after: Option<&'a dyn Fn(&[u8], usize) -> bool>,
    pub(crate) any: Option<&'a dyn Fn(&[u8], usize) -> bool>,
}
```

What each means, for the embedded lexer that receives them:

| Test | Where it is tried | Priority |
|:--|:--|:--|
| `before` | at each place while only the entry's own context is open | before the embedded language's patterns: a match of theirs that starts earlier still wins |
| `after` | at each place while only the entry's own context is open | only where none of the embedded language's patterns match |
| `any` | at each place, at every depth, outside the embedded language's comments | only where none of the current context's patterns match |

`any` is an injection with no `L:` priority, as ERB's `text.html.erb - (meta.embedded.*.erb | comment)`.
In JavaScript code `<%=` loses to the `<` operator.
In a string it wins, because the string's patterns do not match at `<` (xml-html, from Shiki probes).

The halt cannot see the scope stack, so skipping comments is the embedded lexer's job.
The rule engine marks each rule whose scopes name `comment`.
TypeScript skips `any` in its comment kinds.

Where the halts live, in the rule engine (go-lexer):

1. On the `Lexer`, not in `rules::Scratch`: a `&'a dyn Fn` cannot live in the long-lived scratch.
   `Lexer::new`, `run_halting`, `open` and `finish` are `#[inline(always)]`, so the extra fields cost the plain path nothing.
   They must not move back into the scratch.
2. `before` and `after` keep today's root-only test (`root_halt`, one frame open).
   `any` has its own test outside that filter, and it lives in the `HALTING` instance only, so the plain scan stays as it is.
3. SQL and C have scans of their own, generic over `const HALTING`, and shell and GraphQL have private engines; each gets the same split.
   A callee that a split scan doubles is `#[inline(always)]`.

Where the halts come from: a grammar maps each embedding rule to its halts, `Grammar::rule_halts(rule) -> Halts<'static>` beside `rule_carried_state`.
It replaces [#159][pr159]'s single `end` and `after` pair for every language a grammar embeds (primitives).
One grammar can embed through rules with different ends, as the engine's `HandOff` rules do.

Five rules hold for every halt:

1. A halt is a pure function of the line's bytes and the place, with no state between calls (engine-spans).
2. A call through `&dyn Fn` costs what today's `fn` pointer costs; both are one indirect call (engine-spans, xml-html).
3. A lexer called with no halts pays nothing for them.
   The rule engine keeps `scan::<G, HALTING>`, and TypeScript its const-generic copies.
   A line with no halt keeps today's single `Option` branch where it is.
4. Halts are tried at place 0 too, blank or not.
   ERB's first injection patterns are `^(\s*)(?=<%…)`.
   The rule engine's scan skips blank runs, so it tests the halts at 0 before skipping.
5. The change is labelled `bench`, and the Callgrind job judges it; a monomorphised halt test is added only if a profile shows it hot.

## 5. What a halt returns

```rust
let (state, halt) = language.lex_until(cx, line, state, &halts).expect("a hand-off target halts");
```

1. Tokens are appended to `cx.out` with ends counted from the line's start.
   They sit on the base stack the entry started on, with no root scope of the language's own, as Shiki embeds a grammar.
2. At a `before` or `after` halt only the entry's own context is open, so the state is the entry's.
3. At an `any` halt the lexer may be deep inside its own contexts, as ERB code inside a CSS string.
   - It returns its full context stack, interning the contexts opened on the line as it does at a line's end.
   - It resumes from that state later on the same line, after the injection's end, or on a later line.
   - Same-line anchor state, `\G` and the place of the last match, resets on that resume.
   - In TextMate an injection's match does not move the outer rule's `\G`.
     So the reset can differ from Shiki where an embedded pattern anchors on `\G` right after the injection.
     Each user checks its fixtures for it.

## 6. Nested hand-offs

A language can be embedded and embed on the same line (primitives):

- GraphQL inside a Ruby heredoc embeds JavaScript;
- HTML under ERB or Vue embeds CSS and JavaScript.

The inner language receives only the embedding rule's own `before` and `after`, never the outer ones.
The outer `any` passes through every level unchanged.
ERB's `<%=` inside a JavaScript string inside HTML halts JavaScript only because HTML forwards `any` to it.

## 7. `Language::lex_until`

[#159][pr159] adds `Language::lex_until`, one arm per row of `languages!`, `None` for a row without `halts: true`.
This change keeps that dispatch and changes its halts to `&Halts`.
It marks the dispatch `#[inline]` ([`bench/AGENTS.md`][bench] rule 13); go-lexer adds it when next touching the dispatch.

Each language's own `lex_until` follows:

- The rule engine's, TypeScript's, CSS's, Java's, Ruby's and those [#159][pr159] adds take `&Halts`.
- The engine's own languages gain one, so an engine language can be embedded as well as embed.
- Ruby's `lex_until` keeps embedding its heredocs, so a Ruby region inside ERB or the engine keeps `<<~SQL`.
  That needs primitives' `lex_embedding_until`, which halts its own grammar and embeds (go-lexer).

The engine's turn at a halt in section 8 is [#159][pr159]'s `host` flag.
The generator hands off only to a language whose row has `halts: true`.

## 8. State through the arena

The engine keeps its frame for the embedding rule as today: rule, interned end pattern and content stack.
Above it, a hand-off frame:

| Field | Value |
|:--|:--|
| `parent` | the engine's frame for the embedding rule |
| `kind` | `tag::TEXTMATE`, with the hand-off bit |
| `carried` | the embedded language's state |
| `stack` | the embedding rule's content stack |

Each line whose embedded state changes interns one new hand-off frame on the same parent.
Markdown's fence body frame does the same today: it carries its language's state in its `carried` value.

When the embedded lexer halts at `before` or `after`, the engine tries its own patterns at the halt.
vscode-textmate does the same after the embedded patterns fail.
When the end matches, the engine pops the hand-off frame with the rule.
Otherwise it hands the rest of the line back after its own match.

A state is still one integer.
Two lines that reach the same embedded state reach the same engine state.

## 9. Limits

1. Depth stays per language, as the rule engine counts it today.
   Each language counts only its own contexts, so a pinned depth keeps giving the same output.
2. The languages on one line share its work budget, and a hand-off does not reset it.
   - [#159][pr159] carried the host's steps across segments through `run_embedding(spent)`, while each embedded `lex_until` started its own counter.
   - core-fix's `Context { steps }` (`feat/shared-line-budget`) replaces both with one budget for the line.
     Each lexer counts in a field of its own.
     An embedding lexer lends that count, and the embedded lexer gives it back when dropped.
   - `run_embedding(spent)` is gone, and depth stays per language (1 above).
3. Limits reached by the embedded lexer go to the same `cx.report`, as a Markdown fence's do.

## 10. Agreed with

1. engine-spans (2026-11-03):
   - The engine profile is a callgrind measurement only.
     It changes nothing about how a lexer is entered or how work and depth are counted.
   - Nothing of theirs is open in the rule engine.
   - `&dyn Fn` halts under rules 1, 2, 3 and 5 of section 4.
2. xml-html (2026-11-03):
   - `any` with injection priority and the comment skip;
   - the full stack at an `any` halt, and the test at place 0;
   - TypeScript's `expression` as the one planned entry;
   - html-derivative as a language reached at its root;
   - Vue's second include redundant;
   - the third kind of include compiled into the engine's table;
   - ERB moves onto `Halts` in its second PR.

   xml-html read the design at 5ff6726a and confirmed sections 4 and 5 for ERB.
3. primitives (2026-11-03):
   - the GraphQL port's `lex_embedding_until` maps onto `Halts`, and its internal `Stop` stays the rule engine's;
   - nested hand-offs pass only the embedding rule's own `before` and `after` and forward the outer `any`;
   - halts per rule from the grammar.
4. go-lexer (2026-11-03):
   - [#159][pr159]'s dispatch, `None` arm and `scan::<G, HALTING>` carry over;
   - halts on the `Lexer`, with `any` only in the halting instance;
   - the same split for SQL, C, shell and GraphQL;
   - Ruby's heredocs through `lex_embedding_until`;
   - `#[inline]` on the dispatch;
   - the shared work budget waits on core-fix.
5. The operator's lead (2026-11-03): depth stays per language, and only the work budget is shared.

## 11. Order of work

1. `Halts`, its contract, `Grammar::rule_halts` and `Language::lex_until(…, &Halts)`, in one PR stacked on [#159][pr159] and primitives' GraphQL port.
   The rule engine, SQL, C, shell, GraphQL and TypeScript honour all three tests.
   HTML, GraphQL and Ruby heredoc fixtures stay unchanged.
2. Entries: `start_embedded(arena, base, entry)`; xml-html adds TypeScript's `expression`.
3. core-fix's shared line budget, `Context { steps }` (`feat/shared-line-budget`); depth stays per language (section 9).
4. The engine's hand-off: `RuleKind::HandOff`, the generator's list, the frame and the resume; Julia's table without its engine copies first.
5. ERB's any-depth injection (xml-html), and Perl, Elixir, Objective-C, PHP, Vue and Svelte, each its own PR.

## 12. Open questions

1. Whether the same-line `\G` reset after an `any` halt (section 5) changes any fixture against Shiki; ERB's second PR measures it first.
2. Whether `Entry::Named` stays a string or becomes an enum per language once more than two entries exist.
3. Which grammar moves first after the engine side lands: a queue decision.

## 13. Not decided here

1. An embedded language the crate runs on the engine rather than by hand stays an include compiled into the embedding table.
   Scala inside another grammar is an example.

[language]: https://github.com/victor-software-house/nishiki-archive/blob/8f346d8e/src/language.rs
[bench]: https://github.com/victor-software-house/nishiki-archive/blob/8f346d8e/bench/AGENTS.md
[pr159]: https://github.com/victor-software-house/nishiki-archive/pull/159
