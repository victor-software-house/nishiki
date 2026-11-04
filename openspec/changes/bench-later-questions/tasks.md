# Tasks

Each group is one pull request unless it says otherwise.

## 1. Hostile lines

- [ ] 1.1 Q7 of `bench-methodology`: move each language's hostile lines from `tests/it/limits/` into data that both the limit tests and a `hostile` bench read, and report the slowest line per language under the default limits → verify: the limit tests pass in CI unchanged in what they check, and a recorded run shows the new figure against the 4 ms target

## 2. Binary size

- [ ] 2.1 The stripped binary size each language feature adds, in recorded runs → verify: a record lists the size of a build with no language, and the growth for each language alone
