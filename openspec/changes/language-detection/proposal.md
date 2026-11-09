# Proposal

## Motivation

A caller must name the language today.
`Language::from_name` takes a Shiki id or alias and nothing else.

Most text a caller has comes without a language name:

- a file path;
- a script with no extension;
- a shell session, where `cat > deploy.py <<'EOF'` writes Python and `cat setup.sh` prints shell.

nishiki should find the language itself wherever the text gives it away.
Then anything it shows is highlighted (operator, 2026-11-02).

This change is on the horizon.
It records the goal.
Work starts after the TextMate engine covers the long tail of languages.

## Changes

- A detector names the language of a text from the evidence it carries.
  It weighs the evidence strongest first:
  a file name or extension, a shebang, an editor modeline, then the content itself.
- Shell commands that move a file's text name its language.
  - A here-document written to a file carries the file name, as in `cat > x.py <<'EOF'` and `tee x.ts`.
  - `cat`, `head`, `less` and their kin carry the name of the file they print.
  - The lines they write or print are highlighted as that file's language.
- An extensionless file or a bare snippet falls back to content detection.
  Content detection reports how sure it is.
  A caller can then choose plain text instead.

## Capabilities

### New Capabilities

- `language-detection`: nishiki names the language of a text, a file or a shell command's output from the evidence it carries.

### Modified Capabilities

None.

## Impact

- A new public API sits beside `Language::from_name`.
  Its shape is decided in `design.md` when the work starts.
- The shell and Markdown lexers report the file names that commands and here-documents write or print.
  Embedded text can then switch language.
- No dependency is added without the operator's approval.
