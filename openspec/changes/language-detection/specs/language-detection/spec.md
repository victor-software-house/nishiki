## Purpose

nishiki names the language of a text, a file or a shell command's output from the evidence it carries, so a caller does not have to.

## ADDED Requirements

### Requirement: A file name names its language

Given a file name or path, the detector SHALL name the language Shiki associates with its extension or full name.

#### Scenario: A file with a known extension

- **WHEN** the detector is given `src/deploy.py`
- **THEN** it names Python

### Requirement: An extensionless file is named from its content

Given text with no file name, or a file name with no known extension, the detector SHALL name a language from a shebang, then a modeline, then the content, and SHALL report how sure it is.

#### Scenario: A script with a shebang

- **WHEN** the detector is given an extensionless file starting with `#!/usr/bin/env bash`
- **THEN** it names shell with full confidence

### Requirement: Shell commands carry their file's language

When a shell command writes a here-document to a file or prints a file, the lines it writes or prints SHALL be highlighted as the language of that file's name.

#### Scenario: A here-document written to a Python file

- **WHEN** a shell session contains `cat > deploy.py <<'EOF'`, Python lines, then `EOF`
- **THEN** the lines between are highlighted as Python
