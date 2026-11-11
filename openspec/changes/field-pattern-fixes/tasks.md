# Tasks

## 1. Field-pattern fixes

- [ ] 1.1 `codegen/` drops a repeat's arm under the unit and order condition ([design][design], section 2).
  → verify: the arm-drop scenario passes, and the differential test shows 0 differences for Protocol Buffers
  - 2026-11-10: prototyped in the writer of `research/default-matchers`, at [9d804fae][fix-1-prototype], not yet in `codegen/`.
    On a box, its unit tests pass, and the differential test shows 0 differences in 8,292,000 Protocol Buffers matches.
- [ ] 1.2 The generated scan skips starts under the four conditions ([design][design], section 3).
  → verify: the keyword scenario passes, and the differential test shows 0 differences for Protocol Buffers
  - 2026-11-11: prototyped in the writer of `research/matchers-on-main`, at [30390626][fix-2-prototype], not yet in `codegen/`.
    On a box, the keyword case passes, and the differential test shows 0 differences in 8,292,000 Protocol Buffers matches.
    [The design][design], section 3.4, has the rows.
- [ ] 1.3 Protocol Buffers' field lines end uncut.
  → verify: the field lines of 101 and 12,001 bytes end uncut under the default limits, in release and debug builds
  - 2026-11-10: with the prototype of fix 1 alone, in release and debug builds on a box:
    - the 101-byte line ends uncut;
    - the 12,001-byte line still ends at `Limit::Work`, so it waits for task 1.2 ([design][design], section 3).
  - 2026-11-11: with the prototypes of both fixes, at [30390626][fix-2-prototype], both lines end uncut in release and debug builds on a box.
    The task stays open until both fixes are in `codegen/`.

Protocol Buffers moves to generated matchers in [rule-table-codegen][rule-table-codegen] task 1.1 only after these.

[design]: design.md
[fix-1-prototype]: https://github.com/victor-software-house/nishiki-archive/commit/9d804fae0de3b6073f38e3faf57d77ca915583cf
[fix-2-prototype]: https://github.com/victor-software-house/nishiki-archive/commit/30390626b75a582a60da875573339ee08c0286a9
[rule-table-codegen]: ../rule-table-codegen/tasks.md
