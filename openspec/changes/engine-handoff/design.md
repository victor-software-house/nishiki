# Design

One contract for a language that hands a region of its line to another language and takes the line back: the halts, what they mean, and the state at a halt.
The TextMate engine is its first new user; ERB's injection at any depth (xml-html) and Ruby's language-named heredocs ([#159][pr159], go-lexer) use the same one.

## 1. Where the engine hands off

The generator reads which grammars are hand-lexed from one list beside `oracle/licences.ts`: Shiki name, scope name, `Language` variant, and the entries the language offers (section 3).
An include in an engine grammar is one of three kinds:

1. An include of a whole hand-lexed grammar (`source.python`, `text.html.basic`, or `$self` of one) hands off to that language's root.
   Julia's `py"""` rule and Vue's block bodies are this kind.
2. A rule whose only pattern includes one repository rule of a hand-lexed grammar hands off to that rule, when the language offers it as an entry.
   Vue's `source.ts.embedded.html.vue`, whose patterns are `[include source.ts#expression]`, hands off to TypeScript's `expression` entry.
3. Repository rules of a hand-lexed grammar mixed into an engine list, as Vue's style attribute mixes `source.css#comment-block`, `#property-names` and its own matches, cannot be one hand-off.
   The generator compiles those rules into the engine's table as it compiles any grammar, so the grammar needs a licence entry for bundling like any other.

A grammar whose rules are only reached through hand-offs is not compiled into the table, and needs no licence entry for bundling: the hand lexer's own notice covers its scopes.

## 2. What the table holds

A hand-off is one more `RuleKind`, `HandOff`, standing where the include stood in its list, with:

1. the target: a `Language` and an entry (section 3);
2. `before`: the patterns listed before the include in the same list, and the embedding rule's end unless the rule sets `applyEndLast`;
3. `after`: the patterns listed after the include, and the end when the rule sets `applyEndLast`.

Vue's template body is `[vue template-tag, vue capitalized-tag, include text.html.derivative, include text.html.basic]`: the two Vue rules and the body's end are `before` for the hand-off to `Language::HtmlDerivative`, so the engine takes the line back where one of them matches, opens it, and hands back after it.

Two hand-offs in one list are ordered like any other patterns: the second is tried only where the first did not match, so it sits in the first's `after`.
Vue's second include, `text.html.basic` after html-derivative, is redundant: html-derivative holds all of HTML's root patterns but `tags-invalid`, and its own tag rule matches wherever `tags-invalid` would (xml-html).
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
A language starts an entry it does not offer as `Root`; the generator hands off only to entries the list in section 1 names, so the case is not reached.
xml-html adds TypeScript's `expression`; no other entry is planned.
Shiki's `html-derivative` is a language of its own, `Language::HtmlDerivative` (approved as xml-html's language PR), so Vue's `include text.html.derivative` is a hand-off to its root, not an entry of HTML.

## 4. Halts

Today a halt is `fn(&[u8], usize) -> bool` ([`src/language.rs`][language]), and [#159][pr159] passes `end` and `after`.
An engine end is a regular expression whose back-references are filled when the rule opens, so it cannot be a function known at compile time.
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

`any` is an injection with no `L:` priority, as ERB's `text.html.erb - (meta.embedded.*.erb | comment)`: in JavaScript code `<%=` loses to the `<` operator, and in a string it wins, because the string's patterns do not match at `<` (xml-html, from Shiki probes).
The halt cannot see the scope stack, so skipping comments is the embedded lexer's job: the rule engine marks each rule whose scopes name `comment`, and TypeScript skips `any` in its comment kinds.

Where the halts live, in the rule engine (go-lexer):

1. On the `Lexer`, not in `rules::Scratch`: a `&'a dyn Fn` cannot live in the long-lived scratch.
   `Lexer::new`, `run_halting`, `open` and `finish` are `#[inline(always)]`, so the extra fields cost the plain path nothing; they must not move back into the scratch.
2. `before` and `after` keep today's root-only test (`root_halt`, one frame open).
   `any` has its own test outside that filter, and it lives in the `HALTING` instance only, so the plain scan stays as it is.
3. SQL and C have scans of their own, generic over `const HALTING`, and shell and GraphQL have private engines; each gets the same split.
   A callee that a split scan doubles is `#[inline(always)]`.

Where the halts come from: a grammar maps each embedding rule to its halts, `Grammar::rule_halts(rule) -> Halts<'static>` beside `rule_payload`, replacing nishiki-archive#159's single `end` and `after` pair for every language a grammar embeds (primitives).
One grammar can embed through rules with different ends, as the engine's `HandOff` rules do.

Five rules hold for every halt:

1. A halt is a pure function of the line's bytes and the place, with no state between calls (engine-spans).
2. A call through `&dyn Fn` costs what today's `fn` pointer costs; both are one indirect call (engine-spans, xml-html).
3. A lexer called with no halts pays nothing for them: the rule engine keeps `scan::<G, HALTING>`, TypeScript its const-generic copies, and a line with no halt keeps today's single `Option` branch where it is.
4. Halts are tried at place 0 too, blank or not: ERB's first injection patterns are `^(\s*)(?=<%…)`, and the rule engine's scan skips blank runs, so it tests the halts at 0 before skipping.
5. The change is labelled `bench`, and the Callgrind job judges it; a monomorphised halt test is added only if a profile shows it hot.

## 5. What a halt returns

```rust
let (state, halt) = language.lex_until(cx, line, state, &halts).expect("a hand-off target halts");
```

1. Tokens are appended to `cx.out` with ends counted from the line's start, on the base stack the entry started on, with no root scope of the language's own, as Shiki embeds a grammar.
2. At a `before` or `after` halt only the entry's own context is open, so the state is the entry's.
3. At an `any` halt the lexer may be deep inside its own contexts, as ERB code inside a CSS string.
   It returns its full context stack, interning the contexts opened on the line as it does at a line's end, and resumes from that state later on the same line, after the injection's end, or on a later line.
   Same-line anchor state, `\G` and the place of the last match, resets on that resume.
   In TextMate an injection's match does not move the outer rule's `\G`, so the reset can differ from Shiki where an embedded pattern anchors on `\G` right after the injection; each user checks its fixtures for it.

## 6. Nested hand-offs

A language can be embedded and embed on the same line: GraphQL inside a Ruby heredoc embeds JavaScript, and HTML under ERB or Vue embeds CSS and JavaScript (primitives).
The inner language receives only the embedding rule's own `before` and `after`, never the outer ones.
The outer `any` passes through every level unchanged: ERB's `<%=` inside a JavaScript string inside HTML halts JavaScript only because HTML forwards `any` to it.

## 7. `Language::lex_until`

[#159][pr159] adds `Language::lex_until`, one arm per row of `languages!`, `None` for a row without `halts: true`.
This change keeps that dispatch, marks it `#[inline]` ([`bench/AGENTS.md`][bench] rule 13; go-lexer adds it when next touching the dispatch), and changes its halts to `&Halts`.
Each language's own `lex_until` follows: the rule engine's, TypeScript's, CSS's, Java's, Ruby's and those nishiki-archive#159 adds take `&Halts`, and the engine's own languages gain one, so an engine language can be embedded as well as embed.
Ruby's `lex_until` keeps embedding its heredocs, so a Ruby region inside ERB or the engine keeps `<<~SQL`; that needs primitives' `lex_embedding_until`, which halts its own grammar and embeds (go-lexer).
The engine's turn at a halt in section 8 is nishiki-archive#159's `host` flag.
The generator hands off only to a language whose row has `halts: true`.

## 8. State through the arena

The engine keeps its frame for the embedding rule as today: rule, interned end pattern and content stack.
Above it, a hand-off frame:

| Field | Value |
|:--|:--|
| `parent` | the engine's frame for the embedding rule |
| `kind` | `tag::TEXTMATE`, with the hand-off bit |
| `payload` | the embedded language's state |
| `stack` | the embedding rule's content stack |

Each line whose embedded state changes interns one new hand-off frame on the same parent, as Markdown's fence body frame carries its language's state in its payload today.
When the embedded lexer halts at `before` or `after`, the engine tries its own patterns at the halt, as vscode-textmate does after the embedded patterns fail; when the end matches, it pops the hand-off frame with the rule, and otherwise it hands the rest of the line back after its own match.
A state is still one integer, and two lines that reach the same embedded state reach the same engine state.

## 9. Limits

1. Depth stays per language, as the rule engine counts it today: each language counts only its own contexts, so a pinned depth keeps giving the same output.
2. The languages on one line share its work budget, and a hand-off does not reset it.
   nishiki-archive#159 carried the host's steps across segments through `run_embedding(spent)`, while each embedded `lex_until` started its own counter.
   core-fix's `Context { steps }` (`feat/shared-line-budget`) replaces both with one budget for the line: each lexer counts in a field of its own, an embedding lexer lends that count, and the embedded lexer gives it back when dropped.
   `run_embedding(spent)` is gone, and depth stays per language (1 above).
3. Limits reached by the embedded lexer go to the same `cx.report`, as a Markdown fence's do.

## 10. Agreed with

1. engine-spans (2026-11-03): the engine profile is a callgrind measurement only and changes nothing about how a lexer is entered or how work and depth are counted; nothing of theirs is open in the rule engine; `&dyn Fn` halts under rules 1, 2, 3 and 5 of section 4.
2. xml-html (2026-11-03): `any` with injection priority and the comment skip, the full stack at an `any` halt, the test at place 0, TypeScript's `expression` as the one planned entry, html-derivative as a language reached at its root, Vue's second include redundant, and the third kind of include compiled into the engine's table; ERB moves onto `Halts` in its second PR.
   xml-html read the design at 5ff6726a and confirmed sections 4 and 5 for ERB.
3. primitives (2026-11-03): the GraphQL port's `lex_embedding_until` maps onto `Halts` and its internal `Stop` stays the rule engine's; nested hand-offs pass only the embedding rule's own `before` and `after` and forward the outer `any`; halts per rule from the grammar.
4. go-lexer (2026-11-03): nishiki-archive#159's dispatch, `None` arm and `scan::<G, HALTING>` carry over; halts on the `Lexer`; `any` only in the halting instance; the same split for SQL, C, shell and GraphQL; Ruby's heredocs through `lex_embedding_until`; `#[inline]` on the dispatch; the shared work budget waits on core-fix.
5. The operator's lead (2026-11-03): depth stays per language, and only the work budget is shared.

## 11. Order of work

1. `Halts`, its contract, `Grammar::rule_halts` and `Language::lex_until(…, &Halts)`, in one PR on top of nishiki-archive#159 and primitives' GraphQL port, with the rule engine, SQL, C, shell, GraphQL and TypeScript honouring all three tests; HTML, GraphQL and Ruby heredoc fixtures unchanged.
2. Entries: `start_embedded(arena, base, entry)`; xml-html adds TypeScript's `expression`.
3. core-fix's shared line budget, `Context { steps }` (`feat/shared-line-budget`); depth stays per language (section 9).
4. The engine's hand-off: `RuleKind::HandOff`, the generator's list, the frame and the resume; Julia's table without its engine copies first.
5. ERB's any-depth injection (xml-html), and Perl, Elixir, Objective-C, PHP, Vue and Svelte, each its own PR.

## 12. Open questions

1. Whether the same-line `\G` reset after an `any` halt (section 5) changes any fixture against Shiki; ERB's second PR measures it first.
2. Whether `Entry::Named` stays a string or becomes an enum per language once more than two entries exist.
3. Which grammar moves first after the engine side lands: a queue decision.

## 13. Not decided here

1. An embedded language the crate runs on the engine rather than by hand, such as Scala inside another grammar, stays an include compiled into the embedding table.

## 14. The engine's hand-off on main (approved by the lead, 2026-11-04)

Sections 1, 2, 8 and 9, made concrete for main after nishiki-archive#228 and nishiki-archive#325, with HAML as the first user.
Sections 2 and 3 of the tasks (`Halts`, entries) stay deferred; this needs neither.

### 14.1 What HAML asks for

HAML's includes of hand-lexed grammars fall into two shapes (Shiki's `haml` grammar, 4.4.3):

| Shape | Rules | Where the embedding rule's end can match |
|:--|:--|:--|
| Filters | `:ruby`, `:javascript`, `:css`, `:markdown` (`^(\s*):name`, end `^(?!\1\s+\|$\n*)`) | only at a line's start |
| Inline | `rubyline` (`= expr`, end `…$\|$`), `#{…}` through `nest_curly_and_self` | anywhere on the line |

1. A filter's end needs the indentation its begin captured, so the halt is a back-referenced pattern filled when the rule opens.
   A `fn(&[u8], usize) -> bool` cannot hold it.
2. A filter's end and its `before` patterns can match only at a line's start.
   The engine tries them there itself and hands over the rest of the line whole, so the target needs no halting entry: `Language::lex_embedded` serves, Markdown included.
3. An inline hand-off halts mid-line, so its target needs `lex_until`: Ruby gets `halts: true` on its row (its `lex_until` exists), and JavaScript and CSS have theirs.
4. Sass and CoffeeScript have no hand lexer; they stay includes compiled into HAML's table and become engine rows of their own (13.1).

### 14.2 A halt that carries its context

`Halt` becomes a trait object instead of a function pointer:

```rust
pub(crate) trait Halt {
    /// The first place in `from..=to` where the halt holds, in `s`.
    fn first(&self, s: &[u8], from: usize, to: usize) -> Option<usize>;

    /// Whether the halt holds at `p`.
    fn holds(&self, s: &[u8], p: usize) -> bool {
        self.first(s, p, p) == Some(p)
    }
}

impl Halt for fn(&[u8], usize) -> bool { /* today's per-place loop */ }
```

1. Lexers take `&dyn Halt` where they take `Halt` today; `halt_before` calls `first`, and a lexer that tests one place, as YAML does, calls `holds`.
2. Today's halts are `fn` items, which keep working through the blanket impl: one indirect call per place, as now.
3. The engine's halt holds the filled end pattern and its `before` list, and answers `first` with one forward search instead of a test per place.
   It keeps the next match it found in a `Cell`, so a later call inside that span costs a comparison; the answer stays a pure function of the bytes and the place (4, rule 1).
4. csharp's Lua halt, which needs the `]=*]` level its begin read, is the same shape: a struct holding the level.
   One change of `Halt` serves both: csharp makes it in its Lua cdef PR, with the `]=*]` halt as its first stateful user, and the engine builds on it (lead, 2026-11-04).
   The blanket impl stays, so today's halts compile unchanged, and no `&dyn` call lands on the plain path of a lexer that does not halt; Callgrind on the hand lexers checks it.
5. Section 4's `before`, `after` and `any` become three `Option<&dyn Halt>` when `Halts` lands; nothing here depends on the struct.

### 14.3 The table

1. `RuleKind::HandOff` stands where the include stood in its list.
   Its `begin` is an index into a new `Grammar::hand_offs: &'static [HandOff]`, so `Rule` gains no field:

   ```rust
   pub(crate) struct HandOff {
       /// The language that lexes the region.
       pub(crate) language: Language,
       /// Whether the embedding rule's end and `before` match only at a
       /// line's start, so the region is handed over a line at a time.
       pub(crate) whole_lines: bool,
       /// The rules listed before the include, and the end unless
       /// `applyEndLast`: a list of the grammar, with its scanner.
       pub(crate) before: usize,
       /// The rules listed after it, and the end with `applyEndLast`.
       pub(crate) after: usize,
   }
   ```

2. `oracle/hand-lexed.ts`, beside `oracle/licences.ts`, lists the hand-lexed grammars: scope name, `Language` variant, and whether it offers a halting entry.
   An include of one becomes a `HandOff`; an include of a repository rule of one is compiled into the table as today (1.3).
3. `whole_lines` is true when every pattern in `before` and `after`, and the end, starts with `^` outside a group, which the generator reads from the pattern text.
4. One `HandOff` per list: the generator fails on a second, and drops Vue's redundant second include first (2).
5. A language whose table holds a hand-off needs its target's feature: `lang-haml` turns on `lang-ruby`, `lang-javascript`, `lang-css` and `lang-markdown`.

### 14.4 The tokenizer

1. When the scan's best candidate is a `HandOff`, the engine pushes a hand-off frame (8) and calls the target:
   - `whole_lines`: `lex_embedded` on the rest of the line; the hand-off ends only when the end matches at a later line's start.
   - otherwise `lex_until` with the engine's halt (14.2) as `end`, and the `after` list as `after`.
2. At a halt the engine runs its own step at that place: the end pops the rule with the hand-off frame, and a `before` rule matches and applies, after which the region resumes from the entry's state.
3. A target needs the whole crate `Scratch`, which the engine borrows only as `scratch.textmate` today.
   `line_until` takes the `Context`, and before a hand-off the engine moves its line's own buffers (text, frames, captures) out of `scratch.textmate` and back after, so an engine fence inside a hand-lexed region, as Nix in Markdown in HAML, reuses the scratch without overwriting the outer line.
   The move is `mem::take` and back, which allocates nothing; an allocation test on a nested engine fence checks it.
4. The plain path pays nothing: a grammar with no `HandOff` never reaches the branch, and the check sits after the scan, where `apply` already matches on the rule's kind.
   The Callgrind gate is every engine language at or under 1.000.

### 14.5 State and limits

1. The hand-off frame is section 8's: `kind` `tag::TEXTMATE | 1`, `payload` the target's state, `stack` the embedding rule's content stack, `parent` the embedding rule's frame.
   The target is the one `HandOff` in the parent rule's list, so the frame stores no index.
2. Work: the engine lends what its line has left, in steps of the work limit, and charges what the target returns in `Finished`, times `STEP`.
3. Depth stays per language (9.1); the target's limits go to the same `cx.report` (9.3).

### 14.6 Order

1. `Halt` as a trait object, in csharp's Lua cdef PR, with every lexer's `lex_until` taking `&dyn Halt`, and fixtures unchanged.
2. The generator's hand-lexed list and `RuleKind::HandOff`, with tables unchanged for every grammar that includes no hand-lexed one.
3. The tokenizer's hand-off, the frame and the resume, tested on a test grammar that hands off to JavaScript inline and to Markdown by whole lines.
4. HAML on it, with Sass and CoffeeScript as engine rows, each after its licence is read at the source.

[language]: https://github.com/victor-software-house/nishiki-archive/blob/8f346d8e/src/language.rs
[bench]: https://github.com/victor-software-house/nishiki-archive/blob/8f346d8e/bench/AGENTS.md
[pr159]: https://github.com/victor-software-house/nishiki-archive/pull/159
