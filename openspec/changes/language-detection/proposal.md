# Proposal

## Why

A caller must name the language today: `Language::from_name` takes a Shiki id or alias and nothing else.
Most text a caller has comes without one: a file path, a script with no extension, or a shell session where `cat > deploy.py <<'EOF'` writes Python and `cat setup.sh` prints shell.
nishiki should find the language itself wherever the text gives it away, so anything it shows is highlighted (operator, 2026-11-02).

This change is on the horizon: it records the goal, and work starts after the TextMate engine covers the long tail of languages.

## What Changes

- A detector that names the language of a text from the evidence it carries, strongest first: a file name or extension, a shebang, an editor modeline, then the content itself.
- Shell commands that move a file's text name its language: a here-document written to a file (`cat > x.py <<'EOF'`, `tee x.ts`), and `cat`, `head`, `less` and their kin printing a file, carry the file name, so the lines they write or print are highlighted as that file's language.
- An extensionless file or a bare snippet falls back to content detection, which reports how sure it is so a caller can choose plain text instead.

## Capabilities

### New Capabilities

- `language-detection`: nishiki names the language of a text, a file or a shell command's output from the evidence it carries.

### Modified Capabilities

None.

## Impact

- New public API beside `Language::from_name`; its shape is decided in `design.md` when the work starts.
- The shell and Markdown lexers report the file names that commands and here-documents write or print, so embedded text can switch language.
- No dependency is added without the operator's approval.
