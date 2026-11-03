# Tasks

## 1. Foundations

- [x] 1.1 Port the prototype's Oniguruma-dialect parser and matcher into the crate, to the coding standards, with the prototype's tests → verify: the regex tests pass, and every pattern of the four prototype grammars compiles
  - A bytecode machine with an explicit stack in reused scratch replaced the prototype's recursive matcher (nishiki-archive#138).
  - An ignored survey compiles the patterns of Shiki's grammars, and another compares the machine with Oniguruma's recorded answers (nishiki-archive#163).
- [x] 1.2 Grammar tables: a generator writes each grammar's rules, patterns and prefilters as tables the crate includes → verify: regenerating gives the same files
  - `bun oracle/engine-tables.ts <dir> <spec>...` writes `src/<language>/grammar.rs` and the test-only tables in `tests/engine/`; regenerating PowerShell's table changes no byte.
- [x] 1.3 The tokenizer on the arena: frames as hash-consed states, the `Limits` work budget, the endless-loop guards → verify: the four prototype grammars match the oracle through the engine
  - The engine matches Shiki on the Rust, shell, TypeScript, JavaScript, Markdown, Python, YAML, make and TOML fixtures, with the scanners on and off (`src/textmate/tests.rs`).

## 2. Speed

- [ ] 2.1 Literal prefilters, first-byte sets and anchor classes from the tables → verify: bench table before and after, output unchanged
  - The prefilters were in the machine from its first commit (nishiki-archive#138), so no table isolates them.
- [x] 2.2 The `regex-automata` fast path for lookaround-free patterns → verify: bench table with and without it, reported separately
  - One `regex-automata` automaton per rule list finds where a rule can start; the machine checks each candidate (nishiki-archive#158).
- [x] 2.3 Rule-level skipping: search only the rules that can match at the cursor, and reuse a match the cursor has not passed → verify: bench table
  - Each pattern's next match, and each list's winner, is kept for the rest of the line (nishiki-archive#158).
  - Together with 2.2, on one Namespace 8x16: Markdown 1.7x, Rust 10.9x, shell 2.1x, TypeScript 2.1x; 1.9x across 61 fixtures (nishiki-archive#158's table).
- [ ] 2.4 No allocation per line or token → verify: the allocation count of a lexed fixture is zero after the first line
  - `tests/it/alloc.rs` (nishiki-archive#130) lexes each fixture twice and counts the second pass's allocations, PowerShell included.

## 3. Languages

- [x] 3.1 The engine runs a language the hand lexers have not reached, with oracle inputs and held-out inputs → verify: 100% on both
  - PowerShell: two development inputs, two held out, and two Markdown documents with PowerShell fences, all matching Shiki in scopes and colours.
- [x] 3.2 The engine and the hand lexers compared on every oracle fixture → verify: every difference is explained or fixed
  - Both match Shiki on every fixture of the nine languages in 1.3, so they agree with each other.

## 4. Limits

- [x] 4.1 Hostile lines for the engine in `tests/it/limits.rs` → verify: each finishes within the default limits
  - `tests/it/limits/powershell.rs`: brackets, quotes, here-strings, comments and blanks; a line that would cost more than the default work ends at the work limit with tokens that cover it.
- [ ] 4.2 p99 per line under `bench/targets.toml` on every fixture → verify: a recorded run
