# tests

Rules for tests and their data, beside the root guide and [CODING_STANDARDS.md](../CODING_STANDARDS.md).
[README.md](README.md) says what each suite checks.

1. `tests/it/oracle.rs` checks every language in `Language::ALL`, in parallel.
   It fails when a non-whitespace byte gets a different scope stack or colour, so a new language needs only its fixtures.
2. Fixtures and engine tables are generated; regenerate them with their scripts in [`oracle/`](../oracle).
3. `tests/oracle/.ignore` keeps typos, `rg` and `fd` out of the fixtures, which keep their sources' spelling.
   Search them with `rg -u`.
4. A new language adds a `tests/it/limits/<language>.rs` with its hostile lines.
5. The Identity workflow and when to run what are in the root guide's Verify section.
