# Tasks

## 1. Foundations

- [ ] 1.1 Port the prototype's Oniguruma-dialect parser and matcher into the crate, to the coding standards, with the prototype's tests → verify: the regex tests pass, and every pattern of the four prototype grammars compiles
- [ ] 1.2 Grammar tables: a generator writes each grammar's rules, patterns and prefilters as tables the crate includes → verify: regenerating gives the same files
- [ ] 1.3 The tokenizer on the arena: frames as hash-consed states, the `Limits` work budget, the endless-loop guards → verify: the four prototype grammars match the oracle through the engine

## 2. Speed

- [ ] 2.1 Literal prefilters, first-byte sets and anchor classes from the tables → verify: bench table before and after, output unchanged
- [ ] 2.2 The `regex-automata` fast path for lookaround-free patterns → verify: bench table with and without it, reported separately
- [ ] 2.3 Rule-level skipping: search only the rules that can match at the cursor, and reuse a match the cursor has not passed → verify: bench table
- [ ] 2.4 No allocation per line or token → verify: the allocation count of a lexed fixture is zero after the first line

## 3. Languages

- [ ] 3.1 The engine runs a language the hand lexers have not reached, with oracle inputs and held-out inputs → verify: 100% on both
- [ ] 3.2 The engine and the hand lexers compared on every oracle fixture → verify: every difference is explained or fixed

## 4. Limits

- [ ] 4.1 Hostile lines for the engine in `tests/it/limits.rs` → verify: each finishes within the default limits
- [ ] 4.2 p99 per line under `bench/targets.toml` on every fixture → verify: a recorded run
