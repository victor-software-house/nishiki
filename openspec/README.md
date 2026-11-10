# openspec

What nishiki does, what is changing, and why.
The [OpenSpec](https://github.com/Fission-AI/OpenSpec) CLI manages it.

- [`specs/`](specs): what nishiki does now, one capability a directory.
- [`changes/`](changes): work in flight, each with a proposal, a design and tasks.
  Finished changes move to [`changes/archive/`](changes/archive).
- [`research/`](research): research notes that feed the changes.
- [`AGENTS.md`](AGENTS.md): the rules for working here.

## Capabilities

| Capability | Spec |
|:--|:--|
| Scopes match Shiki | [`shiki-fidelity`](specs/shiki-fidelity/spec.md) |
| Colours match Shiki | [`theme-colouring`](specs/theme-colouring/spec.md) |
| Bounded work per line | [`line-limits`](specs/line-limits/spec.md) |
| nishiki's TextMate engine | [`textmate-engine`](specs/textmate-engine/spec.md) |
| Grammars a caller brings | [`grammar-loading`](specs/grammar-loading/spec.md) |
| Speed and memory, measured | [`benchmarks`](specs/benchmarks/spec.md) |

## Commands

The `openspec` CLI comes through mise.
Prefix each `openspec` command with `mise x github:Fission-AI/OpenSpec@1.16.0 --`, the version `mise-tasks/specs.sh` pins.

```sh
openspec list                        # changes in flight
openspec list --specs                # capabilities
openspec show <change>               # one change
mise run specs                       # validate everything, strictly
openspec archive <change> -y         # archive a finished change
```
