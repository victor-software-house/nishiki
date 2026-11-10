---
"nishiki": patch
---

A loaded grammar that hands a region to Less now ends that region where Less stops at a `)` or `}` at its root, as in Shiki.
The rest of the line takes the scope of the rule around the region.
This covers a Markdown fence and a Vue `<style lang="less">` body loaded through `Grammar::from_json`.

Before, Less went on past that character and kept the region open.
