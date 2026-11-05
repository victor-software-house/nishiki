# Binary size

`size -A` of examples/lex, default features plus json, at one codegen unit (CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1).
a251d5ef is B1 and 1f9d2a72 the prototype.
The other 27 of the binary's 31 sections have equal sizes on both sides.

| Section | a251d5ef | 1f9d2a72 | Change |
|:--|--:|--:|--:|
| .gcc_except_table | 33992 | 34000 | +8 |
| .rodata | 1174076 | 1182268 | +8192 |
| .text | 6362387 | 6365315 | +2928 |
| .relro_padding | 1352 | 2520 | +1168 |
| Total | 9824636 | 9836932 | +12296 |
