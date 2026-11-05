# Per-function splits

Self cost per function, head minus base, from callgrind_annotate at one codegen unit.

## LaTeX, first document

```text
change base head function
net 40219857
  35023215    40818944    75842159 nishiki::rules::tables::candidate_lists
   2161324     3912220     6073544 __memcmp_avx2_movbe
   1551562     2728713     4280275 <std::sync::once::Once>::call_once_force::<<std::sync::once_lock::OnceLock<nishiki::rules::tables::Tables<
   1176280     1864260     3040540 __memset_avx2_unaligned_erms
     84491      181783      266274 <std::sync::once::Once>::call_once_force::<<std::sync::once_lock::OnceLock<nishiki::rules::tables::Tables<
     52094      167303      219397 <std::sync::once::Once>::call_once_force::<<std::sync::once_lock::OnceLock<nishiki::rules::tables::Tables<
```

## Bat, lexing

```text
change base head function
net 13966
     14251     1828547     1842798 <nishiki::bat::engine::Bat as nishiki::rules::Grammar>::scan
     -1339      134264      132925 __memcmp_avx2_movbe
       500       10573       11073 _int_malloc
       192        1522        1714 unlink_chunk.isra.0
       166        2997        3163 _int_free_merge_chunk
       138        2449        2587 _int_free_create_chunk
```

## Kotlin, lexing

```text
change base head function
net 116309
    112313    14774753    14887066 <nishiki::kotlin::engine::Kotlin as nishiki::rules::Grammar>::scan
      3048       18672       21720 _int_malloc
       840        2627        3467 unlink_chunk.isra.0
      -168         168           0 0x00000000052076c0 [???]
       168           0         168 0x000000000520a6c0 [???]
       108        5425        5533 _int_free_create_chunk
```
