# mise-tasks

The repository's tasks that are longer than one command, as scripts that [mise](https://mise.jdx.dev) runs.
A script's path is its task name: `check/features.sh` runs as `mise run check:features`.
Each script's `#MISE description=` line says what it does.
`lib/` holds shell functions that tasks source.
Its files are not executable.
mise runs only executable files as tasks.

Short tasks, and `verify`, which runs the checks CI runs, are in [`mise.toml`](../mise.toml).

```sh
mise tasks              # every task, with its description
mise run verify         # what the pull request's CI runs
mise run oracle         # regenerate the Shiki fixtures
mise run callgrind      # instruction counts against origin/main
```
