<!--
  Capture groups that Svelte's grammar lexes again with TypeScript: the names
  of `{@debug}` and the index of `{#each}`.
  TextMate reads no line feed after such a group, so a keyword at its end
  stays a name where a line feed would make it a keyword.
  Each group ends where TypeScript might read past it: a keyword, blanks, a
  `$` prefix, a member access, a comma, or the line's end.
-->
{@debug a}
{@debug a, b}
{@debug $a, $$props}
{@debug a.b}
{@debug a // c}
{@debug a /* c */}
{@debug a /* c}
{@debug a   }
{@debug as}
{@debug a as}
{@debug in}
{@debug a in}
{@debug of}
{@debug a of}
{@debug is}
{@debug a is}
{@debug satisfies}
{@debug a satisfies}
{@debug keyof}
{@debug typeof}
{@debug await}
{@debug a, }
{@debug a,b,}
{@debug}
{@debug a
}

{#each items as item, i}{/each}
{#each items as item, $i}{/each}
{#each items as item, i (item.id)}{/each}
{#each items as item, i // c}{/each}
{#each items as item, i /* c */}{/each}
{#each items as item, i  }{/each}
{#each items as item, as}{/each}
{#each items as item, in}{/each}
{#each items as item, of}{/each}
{#each items as item, is}{/each}
{#each items as item, satisfies}{/each}
{#each items as item, keyof}{/each}
{#each items as item, await}{/each}
{#each items as item, typeof}{/each}
{#each items as { a, b }, index}{/each}
{#each items as item, i
}{/each}
{#each items as item,}{/each}
