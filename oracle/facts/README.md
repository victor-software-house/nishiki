# Fact files

What Shiki and Oniguruma answer, as JSON indented two spaces.
A Rust writer reads these files and writes the tables under `src/`.
Until it does, each script also writes its Rust table.

| File | Script | Holds |
|:--|:--|:--|
| `unicode-sets.json` | [`unicode-tables.ts`](../unicode-tables.ts) | the sets that `\p{..}`, `[[:..:]]`, `\w`, `\d` and `\s` match, as ranges |
| `case-folds.json` | [`case-folds.ts`](../case-folds.ts) | the orbits of `(?i)`, and the folds of more than one character |
| `classes.json` | [`classes.ts`](../classes.ts) | the five classes read by tokenizing, over the code points probed |

- Each file names what answered in `measured`, and the script that asked in `script`.
- A code point is written in Unicode's notation: four hex digits or more, upper case, such as `00AA`.
  A range is two of them joined by `..`, such as `0041..005A`.
- [`facts.ts`](../facts.ts) writes them.
- Each script's check, such as `mise run check:case-folds`, fails when a file differs from what the script writes.
- Never edit a file here by hand: run its script.
