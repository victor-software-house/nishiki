# Tasks

## 1. Field-pattern fixes

- [ ] 1.1 `codegen/` drops a repeat's arm under the unit and order condition ([design][design], section 2).
  → verify: the arm-drop scenario passes, and the differential test shows 0 differences for Protocol Buffers
- [ ] 1.2 The generated scan skips starts under the four conditions ([design][design], section 3).
  → verify: the keyword scenario passes, and the differential test shows 0 differences for Protocol Buffers
- [ ] 1.3 Protocol Buffers' field lines end uncut.
  → verify: the field lines of 101 and 12,001 bytes end uncut under the default limits, in release and debug builds

Protocol Buffers moves to generated matchers in [rule-table-codegen][rule-table-codegen] task 1.1 only after these.

[design]: design.md
[rule-table-codegen]: ../rule-table-codegen/tasks.md
