L # pgvector
T 1 text.html.markdown markup.heading.markdown heading.1.markdown punctuation.definition.heading.markdown
T 2 text.html.markdown markup.heading.markdown heading.1.markdown
T 10 text.html.markdown markup.heading.markdown heading.1.markdown entity.name.section.markdown
L 
L Open-source vector similarity search for Postgres
T 49 text.html.markdown meta.paragraph.markdown
L 
L Store your vectors with the rest of your data. Supports:
T 56 text.html.markdown meta.paragraph.markdown
L 
L - exact and approximate nearest neighbor search
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 47 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - single-precision, half-precision, binary, and sparse vectors
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 62 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - L2 distance, inner product, cosine distance, L1 distance, Hamming distance, and Jaccard distance
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 98 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - any [language](#languages) with a Postgres client
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 6 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 7 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 51 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L Plus [ACID](https://en.wikipedia.org/wiki/ACID) compliance, point-in-time recovery, JOINs, and all of the other [great features](https://www.postgresql.org/about/) of Postgres
T 5 text.html.markdown meta.paragraph.markdown
T 6 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 11 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 12 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 46 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 47 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 112 text.html.markdown meta.paragraph.markdown
T 113 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 127 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 128 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 129 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 162 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 163 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 175 text.html.markdown meta.paragraph.markdown
L 
L Have a lot of vectors? Use [quantization](#scaling) to scale
T 27 text.html.markdown meta.paragraph.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 40 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 41 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 42 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 50 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 51 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 60 text.html.markdown meta.paragraph.markdown
L 
L [![Build Status](https://github.com/pgvector/pgvector/actions/workflows/build.yml/badge.svg)](https://github.com/pgvector/pgvector/actions)
T 1 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 3 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.link.description.begin.markdown
T 15 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown string.other.link.description.markdown
T 16 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.link.description.end.markdown
T 17 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.metadata.markdown
T 91 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown markup.underline.link.image.markdown
T 92 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.metadata.markdown
T 93 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 94 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 138 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 139 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L ## Installation
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 15 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L ### Linux and Mac
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 17 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Compile and install the extension (supports Postgres 13+)
T 57 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L cd /tmp
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell support.function.builtin.shell
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L git clone --branch v0.8.7 https://github.com/pgvector/pgvector.git
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L cd pgvector
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell support.function.builtin.shell
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L make
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
L make install # may need sudo
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell punctuation.definition.comment.shell
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L See the [installation notes](#installation-notes---linux-and-mac) if you run into issues
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 64 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 65 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 88 text.html.markdown meta.paragraph.markdown
L 
L You can also install it with [Docker](#docker), [Homebrew](#homebrew), [PGXN](#pgxn), [APT](#apt), [Yum](#yum), [pkg](#pkg), [APK](#apk), or [conda-forge](#conda-forge), and it comes preinstalled with [Postgres.app](#postgresapp) and many [hosted providers](#hosted-postgres). There are also instructions for [GitHub Actions](https://github.com/pgvector/setup-pgvector).
T 29 text.html.markdown meta.paragraph.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 38 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 45 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 46 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 48 text.html.markdown meta.paragraph.markdown
T 49 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 57 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 58 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 59 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 68 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 69 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 71 text.html.markdown meta.paragraph.markdown
T 72 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 76 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 77 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 78 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 83 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 84 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 86 text.html.markdown meta.paragraph.markdown
T 87 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 90 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 91 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 92 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 96 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 97 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 99 text.html.markdown meta.paragraph.markdown
T 100 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 103 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 104 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 105 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 109 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 110 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 112 text.html.markdown meta.paragraph.markdown
T 113 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 116 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 117 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 118 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 122 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 123 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 125 text.html.markdown meta.paragraph.markdown
T 126 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 129 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 130 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 131 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 135 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 136 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 141 text.html.markdown meta.paragraph.markdown
T 142 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 153 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 154 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 155 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 167 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 168 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 201 text.html.markdown meta.paragraph.markdown
T 202 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 214 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 215 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 216 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 228 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 229 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 239 text.html.markdown meta.paragraph.markdown
T 240 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 256 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 257 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 258 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 274 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 275 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 309 text.html.markdown meta.paragraph.markdown
T 310 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 324 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 325 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 326 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 368 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 369 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 370 text.html.markdown meta.paragraph.markdown
L 
L ### Windows
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 11 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Ensure [C++ support in Visual Studio](https://learn.microsoft.com/en-us/cpp/build/building-on-the-command-line?view=msvc-170#download-and-install-the-tools) is installed and run `x64 Native Tools Command Prompt for VS [version]` as administrator. Then use `nmake` to build:
T 7 text.html.markdown meta.paragraph.markdown
T 8 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 38 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 155 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 156 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 178 text.html.markdown meta.paragraph.markdown
T 179 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 227 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 228 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 256 text.html.markdown meta.paragraph.markdown
T 257 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 262 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 263 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 273 text.html.markdown meta.paragraph.markdown
L 
L ```cmd
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language
L set "PGROOT=C:\Program Files\PostgreSQL\18"
T 43 text.html.markdown markup.fenced_code.block.markdown
L cd %TEMP%
T 9 text.html.markdown markup.fenced_code.block.markdown
L git clone --branch v0.8.7 https://github.com/pgvector/pgvector.git
T 66 text.html.markdown markup.fenced_code.block.markdown
L cd pgvector
T 11 text.html.markdown markup.fenced_code.block.markdown
L nmake /F Makefile.win
T 21 text.html.markdown markup.fenced_code.block.markdown
L nmake /F Makefile.win install
T 29 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L See the [installation notes](#installation-notes---windows) if you run into issues
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 58 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 59 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 82 text.html.markdown meta.paragraph.markdown
L 
L You can also install it with [Docker](#docker) or [conda-forge](#conda-forge).
T 29 text.html.markdown meta.paragraph.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 38 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 45 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 46 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 50 text.html.markdown meta.paragraph.markdown
T 51 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 62 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 63 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 64 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 76 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 77 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 78 text.html.markdown meta.paragraph.markdown
L 
L ## Getting Started
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 18 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Enable the extension (do this once in each database where you want to use it)
T 77 text.html.markdown meta.paragraph.markdown
L 
L ```tsql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language
L CREATE EXTENSION vector;
T 24 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Create a vector column with 3 dimensions
T 40 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE TABLE items (id bigserial PRIMARY KEY, embedding vector(3));
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.type.sql
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.modifier.sql
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Insert vectors
T 14 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L INSERT INTO items (embedding) VALUES ('[1,2,3]'), ('[4,5,6]');
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.II.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Get the nearest neighbors by L2 distance
T 40 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT * FROM items ORDER BY embedding <-> '[3,1,2]' LIMIT 5;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.math.sql
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Also supports inner product (`<#>`), cosine distance (`<=>`), and L1 distance (`<+>`)
T 29 text.html.markdown meta.paragraph.markdown
T 30 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 33 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 34 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 54 text.html.markdown meta.paragraph.markdown
T 55 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 58 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 59 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 79 text.html.markdown meta.paragraph.markdown
T 80 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 83 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 84 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 85 text.html.markdown meta.paragraph.markdown
L 
L Note: `<#>` returns the negative inner product since Postgres only supports `ASC` order index scans on operators
T 6 text.html.markdown meta.paragraph.markdown
T 7 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 10 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 11 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 76 text.html.markdown meta.paragraph.markdown
T 77 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 80 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 81 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 112 text.html.markdown meta.paragraph.markdown
L 
L ## Storing
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 10 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Create a new table with a vector column
T 39 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE TABLE items (id bigserial PRIMARY KEY, embedding vector(3));
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.type.sql
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.modifier.sql
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Or add a vector column to an existing table
T 43 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L ALTER TABLE items ADD COLUMN embedding vector(3);
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.alter.sql keyword.other.create.sql
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.alter.sql
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.alter.sql keyword.other.table.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.alter.sql
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Also supports [half-precision](#half-precision-vectors), [binary](#binary-vectors), and [sparse](#sparse-vectors) vectors
T 14 text.html.markdown meta.paragraph.markdown
T 15 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 31 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 54 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 55 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 57 text.html.markdown meta.paragraph.markdown
T 58 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 64 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 65 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 66 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 81 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 82 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 88 text.html.markdown meta.paragraph.markdown
T 89 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 95 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 96 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 97 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 112 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 113 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 121 text.html.markdown meta.paragraph.markdown
L 
L Insert vectors
T 14 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L INSERT INTO items (embedding) VALUES ('[1,2,3]'), ('[4,5,6]');
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.II.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Or load vectors in bulk using `COPY` ([example](https://github.com/pgvector/pgvector-python/blob/master/examples/loading/example.py))
T 30 text.html.markdown meta.paragraph.markdown
T 31 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 36 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 38 text.html.markdown meta.paragraph.markdown
T 39 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 46 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 47 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 48 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 131 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 132 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 133 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L COPY items (embedding) FROM STDIN WITH (FORMAT BINARY);
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Upsert vectors
T 14 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L INSERT INTO items (id, embedding) VALUES (1, '[1,2,3]'), (2, '[4,5,6]')
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.II.sql
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L     ON CONFLICT (id) DO UPDATE SET embedding = EXCLUDED.embedding;
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DDL.create.II.sql
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.database-name.sql
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.table-name.sql
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Update vectors
T 14 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L UPDATE items SET embedding = '[1,2,3]' WHERE id = 1;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Delete vectors
T 14 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L DELETE FROM items WHERE id = 1;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Querying
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 11 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Get the nearest neighbors to a vector
T 37 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT * FROM items ORDER BY embedding <-> '[3,1,2]' LIMIT 5;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.math.sql
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Supported distance functions are:
T 33 text.html.markdown meta.paragraph.markdown
L 
L - `<->` - L2 distance
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 6 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 7 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - `<#>` - (negative) inner product
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 6 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 7 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 34 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - `<=>` - cosine distance
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 6 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 7 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 25 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - `<+>` - L1 distance
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 6 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 7 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - `<~>` - Hamming distance (binary vectors)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 6 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 7 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - `<%>` - Jaccard distance (binary vectors)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 6 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 7 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L Get the nearest neighbors to a row
T 34 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT * FROM items WHERE id != 1 ORDER BY embedding <-> (SELECT embedding FROM items WHERE id = 1) LIMIT 5;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.math.sql
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 79 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 86 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 91 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 95 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 96 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 97 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 98 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 100 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 105 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 106 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 107 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 108 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Get rows within a certain distance
T 34 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT * FROM items WHERE embedding <-> '[3,1,2]' < 5;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.math.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Note: Combine with `ORDER BY` and `LIMIT` to use an index
T 19 text.html.markdown meta.paragraph.markdown
T 20 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 28 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 29 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 34 text.html.markdown meta.paragraph.markdown
T 35 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 40 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 41 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 57 text.html.markdown meta.paragraph.markdown
L 
L #### Distances
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 14 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L Get the distance
T 16 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT embedding <-> '[3,1,2]' AS distance FROM items;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.math.sql
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.alias.sql
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L For inner product, multiply by -1 (since `<#>` returns the negative inner product)
T 41 text.html.markdown meta.paragraph.markdown
T 42 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 45 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 82 text.html.markdown meta.paragraph.markdown
L 
L ```tsql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language
L SELECT (embedding <#> '[3,1,2]') * -1 AS inner_product FROM items;
T 66 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L For cosine similarity, use 1 - cosine distance
T 46 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT 1 - (embedding <=> '[3,1,2]') AS cosine_similarity FROM items;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.math.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.alias.sql
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L #### Aggregates
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 15 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L Average vectors
T 15 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT AVG(embedding) FROM items;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql support.function.aggregate.sql
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Average groups of vectors
T 25 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT category_id, AVG(embedding) FROM items GROUP BY category_id;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql support.function.aggregate.sql
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Indexing
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 11 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L By default, pgvector performs exact nearest neighbor search, which provides perfect recall.
T 91 text.html.markdown meta.paragraph.markdown
L 
L You can add an index to use approximate nearest neighbor search, which trades some recall for speed. Unlike typical indexes, you will see different results for queries after adding an approximate index.
T 202 text.html.markdown meta.paragraph.markdown
L 
L Supported index types are:
T 26 text.html.markdown meta.paragraph.markdown
L 
L - [HNSW](#hnsw)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 7 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L - [IVFFlat](#ivfflat)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L ## HNSW
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 7 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L An HNSW index creates a multilayer graph. It has better query performance than IVFFlat (in terms of speed-recall tradeoff), but has slower build times and uses more memory. Also, an index can be created without any data in the table since there isn’t a training step like IVFFlat.
T 282 text.html.markdown meta.paragraph.markdown
L 
L Add an index for each distance function you want to use.
T 56 text.html.markdown meta.paragraph.markdown
L 
L L2 distance
T 11 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON items USING hnsw (embedding vector_l2_ops);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Note: Use `halfvec_l2_ops` for `halfvec` and `sparsevec_l2_ops` for `sparsevec` (and similar with the other distance functions)
T 10 text.html.markdown meta.paragraph.markdown
T 11 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 25 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 26 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 31 text.html.markdown meta.paragraph.markdown
T 32 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 39 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 40 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 45 text.html.markdown meta.paragraph.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 62 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 63 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 68 text.html.markdown meta.paragraph.markdown
T 69 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 78 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 79 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 127 text.html.markdown meta.paragraph.markdown
L 
L Inner product
T 13 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON items USING hnsw (embedding vector_ip_ops);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Cosine distance
T 15 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON items USING hnsw (embedding vector_cosine_ops);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L L1 distance
T 11 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON items USING hnsw (embedding vector_l1_ops);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Hamming distance
T 16 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON items USING hnsw (embedding bit_hamming_ops);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Jaccard distance
T 16 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON items USING hnsw (embedding bit_jaccard_ops);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Supported types are:
T 20 text.html.markdown meta.paragraph.markdown
L 
L - `vector` - up to 2,000 dimensions
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - `halfvec` - up to 4,000 dimensions
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - `bit` - up to 64,000 dimensions
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 6 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 7 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 33 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - `sparsevec` - up to 1,000 non-zero elements
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 13 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 45 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ### Index Options
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 17 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Specify HNSW parameters
T 23 text.html.markdown meta.paragraph.markdown
L 
L - `m` - the max number of connections per layer (16 by default)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 5 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 63 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - `ef_construction` - the size of the dynamic candidate list for constructing the graph (64 by default)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 103 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON items USING hnsw (embedding vector_l2_ops) WITH (m = 16, ef_construction = 64);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 89 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 90 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 91 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 93 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 95 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L A higher value of `ef_construction` provides better recall at the cost of index build time / insert speed.
T 18 text.html.markdown meta.paragraph.markdown
T 19 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 34 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 35 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 106 text.html.markdown meta.paragraph.markdown
L 
L ### Query Options
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 17 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Specify the size of the dynamic candidate list for search (40 by default)
T 73 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SET hnsw.ef_search = 100;
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.database-name.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.table-name.sql
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L A higher value provides better recall at the cost of speed.
T 59 text.html.markdown meta.paragraph.markdown
L 
L Use `SET LOCAL` inside a transaction to set it for a single query
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 14 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 15 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 65 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L BEGIN;
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.LUW.sql
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L SET LOCAL hnsw.ef_search = 100;
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.database-name.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.table-name.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L SELECT ...
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L COMMIT;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.LUW.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Index Build Time
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 20 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Indexes build significantly faster when the graph fits into `maintenance_work_mem`
T 60 text.html.markdown meta.paragraph.markdown
T 61 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 81 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 82 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SET maintenance_work_mem = '8GB';
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L A notice is shown when the graph no longer fits
T 47 text.html.markdown meta.paragraph.markdown
L 
L ```text
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language
L NOTICE:  hnsw graph no longer fits into maintenance_work_mem after 100000 tuples
T 80 text.html.markdown markup.fenced_code.block.markdown
L DETAIL:  Building will take significantly more time.
T 52 text.html.markdown markup.fenced_code.block.markdown
L HINT:  Increase maintenance_work_mem to speed up builds.
T 56 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Note: Do not set `maintenance_work_mem` so high that it exhausts the memory on the server
T 17 text.html.markdown meta.paragraph.markdown
T 18 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 38 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 39 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 89 text.html.markdown meta.paragraph.markdown
L 
L Like other index types, it’s faster to create an index after loading your initial data
T 88 text.html.markdown meta.paragraph.markdown
L 
L You can also speed up index creation by increasing the number of parallel workers (2 by default)
T 96 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SET max_parallel_maintenance_workers = 7; -- plus leader
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql comment.line.double-dash.sql punctuation.definition.comment.sql
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql comment.line.double-dash.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L For a large number of workers, you may need to increase `max_parallel_workers` (8 by default)
T 56 text.html.markdown meta.paragraph.markdown
T 57 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 77 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 78 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 93 text.html.markdown meta.paragraph.markdown
L 
L The [index options](#index-options) also have a significant impact on build time (use the defaults unless seeing low recall)
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 18 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 34 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 124 text.html.markdown meta.paragraph.markdown
L 
L Use [binary quantization](#binary-quantization) for faster build times at scale
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 24 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 46 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 47 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 79 text.html.markdown meta.paragraph.markdown
L 
L ### Indexing Progress
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 21 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Check [indexing progress](https://www.postgresql.org/docs/current/progress-reporting.html#CREATE-INDEX-PROGRESS-REPORTING)
T 6 text.html.markdown meta.paragraph.markdown
T 7 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 24 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 121 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 122 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT phase, round(100.0 * blocks_done / nullif(blocks_total, 0), 1) AS "%" FROM pg_stat_progress_create_index;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql support.function.mathematical.sql
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.math.sql
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql support.function.expression.sql
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.alias.sql
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.double.sql punctuation.definition.string.begin.sql
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.double.sql
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.double.sql punctuation.definition.string.end.sql
T 77 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 81 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 112 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L The phases for HNSW are:
T 24 text.html.markdown meta.paragraph.markdown
L 
L 1. `initializing`
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 4 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 2. `loading tuples`
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 4 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 18 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 19 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L ## IVFFlat
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 10 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L An IVFFlat index divides vectors into lists, and then searches a subset of those lists that are closest to the query vector. It has faster build times and uses less memory than HNSW, but has lower query performance (in terms of speed-recall tradeoff).
T 251 text.html.markdown meta.paragraph.markdown
L 
L Three keys to achieving good recall are:
T 40 text.html.markdown meta.paragraph.markdown
L 
L 1. Create the index *after* the table has some data
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 20 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 21 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 26 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.italic.markdown
T 27 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 51 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 2. Choose an appropriate number of lists - a good place to start is `rows / 1000` for up to 1M rows and `sqrt(rows)` for over 1M rows
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 68 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 69 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 80 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 81 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 104 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 105 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 115 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 116 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 133 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 3. When querying, specify an appropriate number of [probes](#query-options) (higher is better for recall, lower is better for speed) - a good place to start is `sqrt(lists)`
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 51 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 52 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 58 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 59 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 60 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 74 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 75 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 160 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 161 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 172 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 173 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L Add an index for each distance function you want to use.
T 56 text.html.markdown meta.paragraph.markdown
L 
L L2 distance
T 11 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON items USING ivfflat (embedding vector_l2_ops) WITH (lists = 100);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 79 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 81 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Note: Use `halfvec_l2_ops` for `halfvec` (and similar with the other distance functions)
T 10 text.html.markdown meta.paragraph.markdown
T 11 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 25 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 26 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 31 text.html.markdown meta.paragraph.markdown
T 32 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 39 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 40 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 88 text.html.markdown meta.paragraph.markdown
L 
L Inner product
T 13 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON items USING ivfflat (embedding vector_ip_ops) WITH (lists = 100);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 79 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 81 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Cosine distance
T 15 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON items USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 78 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 79 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 80 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 83 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 85 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Hamming distance
T 16 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON items USING ivfflat (embedding bit_hamming_ops) WITH (lists = 100);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 77 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 78 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 81 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 83 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Supported types are:
T 20 text.html.markdown meta.paragraph.markdown
L 
L - `vector` - up to 2,000 dimensions
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - `halfvec` - up to 4,000 dimensions
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - `bit` - up to 64,000 dimensions
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 6 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 7 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 33 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ### Query Options
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 17 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Specify the number of probes (1 by default)
T 43 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SET ivfflat.probes = 10;
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.database-name.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.table-name.sql
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L A higher value provides better recall at the cost of speed, and it can be set to the number of lists for exact nearest neighbor search (at which point the planner won’t use the index)
T 185 text.html.markdown meta.paragraph.markdown
L 
L Use `SET LOCAL` inside a transaction to set it for a single query
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 14 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 15 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 65 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L BEGIN;
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.LUW.sql
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L SET LOCAL ivfflat.probes = 10;
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.database-name.sql
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.table-name.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L SELECT ...
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L COMMIT;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.LUW.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Index Build Time
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 20 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Speed up index creation on large tables by increasing the number of parallel workers (2 by default)
T 99 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SET max_parallel_maintenance_workers = 7; -- plus leader
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql comment.line.double-dash.sql punctuation.definition.comment.sql
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql comment.line.double-dash.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L For a large number of workers, you may also need to increase `max_parallel_workers` (8 by default)
T 61 text.html.markdown meta.paragraph.markdown
T 62 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 82 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 83 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 98 text.html.markdown meta.paragraph.markdown
L 
L ### Indexing Progress
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 21 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Check [indexing progress](https://www.postgresql.org/docs/current/progress-reporting.html#CREATE-INDEX-PROGRESS-REPORTING)
T 6 text.html.markdown meta.paragraph.markdown
T 7 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 24 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 121 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 122 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT phase, round(100.0 * tuples_done / nullif(tuples_total, 0), 1) AS "%" FROM pg_stat_progress_create_index;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql support.function.mathematical.sql
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.math.sql
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql support.function.expression.sql
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.alias.sql
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.double.sql punctuation.definition.string.begin.sql
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.double.sql
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.double.sql punctuation.definition.string.end.sql
T 77 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 81 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 112 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L The phases for IVFFlat are:
T 27 text.html.markdown meta.paragraph.markdown
L 
L 1. `initializing`
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 4 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 2. `performing k-means`
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 4 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 22 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 23 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 3. `assigning tuples`
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 4 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 20 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 21 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 4. `loading tuples`
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 4 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 18 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 19 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L Note: `%` is only populated during the `loading tuples` phase
T 6 text.html.markdown meta.paragraph.markdown
T 7 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 8 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 39 text.html.markdown meta.paragraph.markdown
T 40 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 54 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 55 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown meta.paragraph.markdown
L 
L ## Filtering
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 12 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L There are a few ways to index nearest neighbor queries with a `WHERE` clause.
T 62 text.html.markdown meta.paragraph.markdown
T 63 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 68 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 69 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 77 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT * FROM items WHERE category_id = 123 ORDER BY embedding <-> '[3,1,2]' LIMIT 5;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.math.sql
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 77 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 82 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 83 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 84 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 85 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L A good place to start is creating an index on the filter column. This can provide fast, exact nearest neighbor search in many cases. Postgres has a number of [index types](https://www.postgresql.org/docs/current/indexes-types.html) for this: B-tree (default), hash, GiST, SP-GiST, GIN, and BRIN.
T 158 text.html.markdown meta.paragraph.markdown
T 159 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 170 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 171 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 172 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 230 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 231 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 295 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON items (category_id);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L For multiple columns, consider a [multicolumn index](https://www.postgresql.org/docs/current/indexes-multicolumn.html).
T 33 text.html.markdown meta.paragraph.markdown
T 34 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 51 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 52 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 53 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 117 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 118 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 119 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON items (location_id, category_id);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Exact indexes work well for conditions that match a low percentage of rows. Otherwise, [approximate indexes](#indexing) can work better.
T 87 text.html.markdown meta.paragraph.markdown
T 88 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 107 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 108 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 109 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 118 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 119 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 136 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON items USING hnsw (embedding vector_l2_ops);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L With approximate indexes, filtering is applied *after* the index is scanned. If a condition matches 10% of rows, with HNSW and the default `hnsw.ef_search` of 40, only 4 rows will match on average. For more rows, enable [iterative index scans](#iterative-index-scans), which will automatically scan more of the index when needed.
T 47 text.html.markdown meta.paragraph.markdown
T 48 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 53 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 54 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 139 text.html.markdown meta.paragraph.markdown
T 140 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 154 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 155 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 220 text.html.markdown meta.paragraph.markdown
T 221 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 242 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 243 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 244 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 266 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 267 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 329 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SET hnsw.iterative_scan = strict_order;
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.database-name.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.table-name.sql
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L If filtering by only a few distinct values, consider [partial indexing](https://www.postgresql.org/docs/current/indexes-partial.html).
T 53 text.html.markdown meta.paragraph.markdown
T 54 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 70 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 71 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 72 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 132 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 133 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 134 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON items USING hnsw (embedding vector_l2_ops) WHERE (category_id = 123);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 78 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 79 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 80 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 83 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 85 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L If filtering by many different values, consider [partitioning](https://www.postgresql.org/docs/current/ddl-partitioning.html).
T 48 text.html.markdown meta.paragraph.markdown
T 49 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 61 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 62 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 63 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 124 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 125 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 126 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE TABLE items (embedding vector(3), category_id int) PARTITION BY LIST(category_id);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.type.sql
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 89 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Multitenancy
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 15 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L For applications with multiple tenants, sharing an approximate index between tenants means vectors from one tenant can affect recall (and speed) for other tenants.
T 163 text.html.markdown meta.paragraph.markdown
L 
L For tenant isolation, use [list partitioning](https://www.postgresql.org/docs/current/ddl-partitioning.html) or separate tables.
T 26 text.html.markdown meta.paragraph.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 44 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 45 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 46 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 107 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 108 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 128 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE TABLE items (customer_id int, embedding vector(3)) PARTITION BY LIST(customer_id);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.type.sql
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 89 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Iterative Index Scans
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 24 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L With approximate indexes, queries with filtering can return less results since filtering is applied *after* the index is scanned. Starting with 0.8.0, you can enable iterative index scans, which will automatically scan more of the index until enough results are found (or it reaches `hnsw.max_scan_tuples` or `ivfflat.max_probes`).
T 100 text.html.markdown meta.paragraph.markdown
T 101 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 106 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 107 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 283 text.html.markdown meta.paragraph.markdown
T 284 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 304 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 305 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 309 text.html.markdown meta.paragraph.markdown
T 310 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 328 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 329 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 331 text.html.markdown meta.paragraph.markdown
L 
L Iterative scans can use strict or relaxed ordering.
T 51 text.html.markdown meta.paragraph.markdown
L 
L Strict ensures results are in the exact order by distance
T 57 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SET hnsw.iterative_scan = strict_order;
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.database-name.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.table-name.sql
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Relaxed allows results to be slightly out of order by distance, but provides better recall
T 90 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SET hnsw.iterative_scan = relaxed_order;
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.database-name.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.table-name.sql
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L # or
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
L SET ivfflat.iterative_scan = relaxed_order;
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.database-name.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.table-name.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L With relaxed ordering, you can use a [materialized CTE](https://www.postgresql.org/docs/current/queries-with.html#QUERIES-WITH-CTE-MATERIALIZATION) to get strict ordering
T 37 text.html.markdown meta.paragraph.markdown
T 38 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 54 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 55 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 56 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 146 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 147 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 170 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L WITH relaxed_results AS MATERIALIZED (
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.alias.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L     SELECT id, embedding <-> '[1,2,3]' AS distance FROM items WHERE category_id = 123 ORDER BY distance LIMIT 5
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.math.sql
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.alias.sql
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 80 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 81 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 82 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 85 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 86 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 94 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 104 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 109 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 110 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 111 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
L ) SELECT * FROM relaxed_results ORDER BY distance + 0;
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.math.sql
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Note: `+ 0` is needed for Postgres 17+
T 6 text.html.markdown meta.paragraph.markdown
T 7 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 10 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 11 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 38 text.html.markdown meta.paragraph.markdown
L 
L For queries that filter by distance, use a materialized CTE and place the distance filter outside of it for best performance (due to the [current behavior](https://www.postgresql.org/message-id/flat/CAOdR5yGUoMQ6j7M5hNUXrySzaqZVGf_Ne%2B8fwZMRKTFxU1nbJg%40mail.gmail.com) of the Postgres executor)
T 137 text.html.markdown meta.paragraph.markdown
T 138 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 154 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 155 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 156 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 269 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 270 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 296 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L WITH nearest_results AS MATERIALIZED (
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.alias.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L     SELECT id, embedding <-> '[1,2,3]' AS distance FROM items ORDER BY distance LIMIT 5
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.math.sql
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.alias.sql
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 80 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 85 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 86 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 87 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
L ) SELECT * FROM nearest_results WHERE distance < 5 ORDER BY distance;
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Note: Place any other filters inside the CTE
T 44 text.html.markdown meta.paragraph.markdown
L 
L ### Iterative Scan Options
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 26 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Since scanning a large portion of an approximate index is expensive, there are options to control when a scan ends.
T 115 text.html.markdown meta.paragraph.markdown
L 
L #### HNSW
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 9 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L Specify the max number of tuples to visit (20,000 by default)
T 61 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SET hnsw.max_scan_tuples = 20000;
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.database-name.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.table-name.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Note: This is approximate and does not affect the initial scan
T 62 text.html.markdown meta.paragraph.markdown
L 
L Specify the max amount of memory to use, as a multiple of `work_mem` (1 by default)
T 58 text.html.markdown meta.paragraph.markdown
T 59 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 67 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 68 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 83 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SET hnsw.scan_mem_multiplier = 2;
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.database-name.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.table-name.sql
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Note: Try increasing this if increasing `hnsw.max_scan_tuples` does not improve recall
T 40 text.html.markdown meta.paragraph.markdown
T 41 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 62 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 86 text.html.markdown meta.paragraph.markdown
L 
L #### IVFFlat
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 12 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L Specify the max number of probes
T 32 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SET ivfflat.max_probes = 100;
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.database-name.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.other.table-name.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Note: If this is lower than `ivfflat.probes`, `ivfflat.probes` will be used
T 28 text.html.markdown meta.paragraph.markdown
T 29 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 43 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 44 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 46 text.html.markdown meta.paragraph.markdown
T 47 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 62 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 75 text.html.markdown meta.paragraph.markdown
L 
L ## Half-Precision Vectors
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 25 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Use the `halfvec` type to store half-precision vectors
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 54 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE TABLE items (id bigserial PRIMARY KEY, embedding halfvec(3));
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.type.sql
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.modifier.sql
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Half-Precision Indexing
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 26 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Index vectors at half precision for smaller indexes
T 51 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON items USING hnsw ((embedding::halfvec(3)) halfvec_l2_ops);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Get the nearest neighbors
T 25 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT * FROM items ORDER BY embedding::halfvec(3) <-> '[1,2,3]' LIMIT 5;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.math.sql
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Binary Vectors
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 17 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Use the `bit` type to store binary vectors ([example](https://github.com/pgvector/pgvector-python/blob/master/examples/imagehash/example.py))
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 12 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 13 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 44 text.html.markdown meta.paragraph.markdown
T 45 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 52 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 53 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 54 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 139 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 140 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 141 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE TABLE items (id bigserial PRIMARY KEY, embedding bit(3));
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.type.sql
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.modifier.sql
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.type.sql
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L INSERT INTO items (embedding) VALUES ('000'), ('111');
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.II.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Get the nearest neighbors by Hamming distance
T 45 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT * FROM items ORDER BY embedding <~> '101' LIMIT 5;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Also supports Jaccard distance (`<%>`)
T 32 text.html.markdown meta.paragraph.markdown
T 33 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 36 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 37 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 38 text.html.markdown meta.paragraph.markdown
L 
L ## Binary Quantization
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 22 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Use expression indexing for binary quantization
T 47 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON items USING hnsw ((binary_quantize(embedding)::bit(3)) bit_hamming_ops);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.type.sql
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 88 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Get the nearest neighbors by Hamming distance
T 45 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT * FROM items ORDER BY binary_quantize(embedding)::bit(3) <~> binary_quantize('[1,-2,3]') LIMIT 5;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.type.sql
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 84 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 85 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 93 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 94 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 96 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 101 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 102 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 103 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 104 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Re-rank by the original vectors for better recall
T 49 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT * FROM (
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L     SELECT * FROM items ORDER BY binary_quantize(embedding)::bit(3) <~> binary_quantize('[1,-2,3]') LIMIT 20
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.type.sql
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 88 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 89 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 97 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 98 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 100 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 105 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 106 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 108 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
L ) ORDER BY embedding <=> '[1,-2,3]' LIMIT 5;
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Sparse Vectors
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 17 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Use the `sparsevec` type to store sparse vectors
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 18 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 19 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 48 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE TABLE items (id bigserial PRIMARY KEY, embedding sparsevec(5));
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.type.sql
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.modifier.sql
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Insert vectors
T 14 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L INSERT INTO items (embedding) VALUES ('{1:1,3:2,5:3}/5'), ('{1:4,3:5,5:6}/5');
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.II.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 78 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L The format is `{index1:value1,index2:value2}/dimensions` and indices start at 1 like SQL arrays
T 14 text.html.markdown meta.paragraph.markdown
T 15 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 55 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 56 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 95 text.html.markdown meta.paragraph.markdown
L 
L Get the nearest neighbors by L2 distance
T 40 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT * FROM items ORDER BY embedding <-> '{1:3,3:1,5:2}/5' LIMIT 5;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.math.sql
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Hybrid Search
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 16 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Use together with Postgres [full-text search](https://www.postgresql.org/docs/current/textsearch-intro.html) for hybrid search.
T 27 text.html.markdown meta.paragraph.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 44 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 45 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 46 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 107 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 108 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 127 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT id, content FROM items, plainto_tsquery('hello search') query
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L     WHERE textsearch @@ query ORDER BY ts_rank_cd(textsearch, query) DESC LIMIT 5;
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.order.sql
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 79 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 80 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 81 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 82 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L You can use [Reciprocal Rank Fusion](https://github.com/pgvector/pgvector-python/blob/master/examples/hybrid_search/rrf.py) or a [cross-encoder](https://github.com/pgvector/pgvector-python/blob/master/examples/hybrid_search/cross_encoder.py) to combine results.
T 12 text.html.markdown meta.paragraph.markdown
T 13 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 122 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 123 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 129 text.html.markdown meta.paragraph.markdown
T 130 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 143 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 144 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 145 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 240 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 241 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 261 text.html.markdown meta.paragraph.markdown
L 
L ## Indexing Subvectors
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 22 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Use expression indexing to index subvectors
T 43 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON items USING hnsw ((subvector(embedding, 1, 3)::vector(3)) vector_cosine_ops);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 93 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Get the nearest neighbors by cosine distance
T 44 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT * FROM items ORDER BY subvector(embedding, 1, 3)::vector(3) <=> subvector('[1,2,3,4,5]'::vector, 1, 3) LIMIT 5;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 81 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 82 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 93 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 94 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 96 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 102 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 104 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 105 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 107 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 108 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 110 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 115 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 116 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 117 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 118 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Re-rank by the full vectors for better recall
T 45 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT * FROM (
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L     SELECT * FROM items ORDER BY subvector(embedding, 1, 3)::vector(3) <=> subvector('[1,2,3,4,5]'::vector, 1, 3) LIMIT 20
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 85 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 86 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 97 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 98 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 100 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 106 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 108 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 109 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 111 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 112 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 114 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 119 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 120 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 122 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
L ) ORDER BY embedding <=> '[1,2,3,4,5]' LIMIT 5;
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Performance
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 14 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L ### Tuning
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 10 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Use a tool like [PgTune](https://pgtune.leopard.in.ua/) to set initial values for Postgres server parameters. For instance, `shared_buffers` should typically be 25% of the server’s memory. You can find the config file with:
T 16 text.html.markdown meta.paragraph.markdown
T 17 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 24 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 54 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 55 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 124 text.html.markdown meta.paragraph.markdown
T 125 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 139 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 140 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 225 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SHOW config_file;
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L And check individual settings with:
T 35 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SHOW shared_buffers;
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Be sure to restart Postgres for changes to take effect.
T 55 text.html.markdown meta.paragraph.markdown
L 
L ### Storing
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 11 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Use the `halfvec` type instead of `vector` for a smaller working set.
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 34 text.html.markdown meta.paragraph.markdown
T 35 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 41 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 42 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 69 text.html.markdown meta.paragraph.markdown
L 
L ### Loading
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 11 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Use `COPY` for bulk loading data ([example](https://github.com/pgvector/pgvector-python/blob/master/examples/loading/example.py)).
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 10 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 34 text.html.markdown meta.paragraph.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 42 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 43 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 44 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 127 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 128 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 130 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L COPY items (embedding) FROM STDIN WITH (FORMAT BINARY);
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Add any indexes *after* loading the initial data for best performance.
T 16 text.html.markdown meta.paragraph.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 22 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 23 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 70 text.html.markdown meta.paragraph.markdown
L 
L ### Indexing
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 12 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L See index build time for [HNSW](#index-build-time) and [IVFFlat](#index-build-time-1).
T 25 text.html.markdown meta.paragraph.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 31 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 32 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 49 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 50 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 55 text.html.markdown meta.paragraph.markdown
T 56 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 63 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 64 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 65 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 84 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 85 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 86 text.html.markdown meta.paragraph.markdown
L 
L Use [binary quantization](#binary-quantization) for smaller indexes and faster build times at scale.
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 24 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 46 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 47 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 100 text.html.markdown meta.paragraph.markdown
L 
L In production environments, create indexes concurrently to avoid blocking writes.
T 81 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX CONCURRENTLY ...
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Querying
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 12 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Use `EXPLAIN (ANALYZE, BUFFERS)` to debug performance.
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 31 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 32 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 54 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L EXPLAIN (ANALYZE, BUFFERS) SELECT * FROM items ORDER BY embedding <-> '[3,1,2]' LIMIT 5;
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.math.sql
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 78 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 79 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 80 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 85 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 86 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 87 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 88 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L #### Exact Search
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 17 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L To speed up queries without an index, increase `max_parallel_workers_per_gather`.
T 47 text.html.markdown meta.paragraph.markdown
T 48 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 79 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 80 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 81 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SET max_parallel_workers_per_gather = 4;
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L If vectors are normalized to length 1 (like [OpenAI embeddings](https://platform.openai.com/docs/guides/embeddings#which-distance-function-should-i-use)), use inner product for best performance.
T 44 text.html.markdown meta.paragraph.markdown
T 45 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 62 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 63 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 64 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 151 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 152 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 194 text.html.markdown meta.paragraph.markdown
L 
L ```tsql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language
L SELECT * FROM items ORDER BY embedding <#> '[3,1,2]' LIMIT 5;
T 61 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L #### Approximate Search
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 23 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L Use [binary quantization](#binary-quantization) with re-ranking to keep indexes in-memory at scale.
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 24 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 46 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 47 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 99 text.html.markdown meta.paragraph.markdown
L 
L To speed up queries with an IVFFlat index, increase the number of inverted lists (at the expense of recall).
T 108 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON items USING ivfflat (embedding vector_l2_ops) WITH (lists = 1000);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 80 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 82 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Vacuuming
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 13 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Vacuuming can take a while for HNSW indexes. Speed it up by reindexing first.
T 77 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L REINDEX INDEX CONCURRENTLY index_name;
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L VACUUM table_name;
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Scaling
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 10 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L For a smaller working set:
T 26 text.html.markdown meta.paragraph.markdown
L 
L 1. Use the `halfvec` type instead of `vector` for tables
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 11 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 12 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 20 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 37 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 38 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 44 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 45 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 56 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 2. Use [binary quantization](#binary-quantization) for indexes (with re-ranking for search)
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 7 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 8 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 27 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 28 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 29 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 49 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 50 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 91 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 
L Scale vertically by increasing memory, CPU, and storage on a single instance. Use existing tools to [tune parameters](#tuning) and [monitor performance](#monitoring).
T 100 text.html.markdown meta.paragraph.markdown
T 101 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 116 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 117 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 118 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 125 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 126 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 131 text.html.markdown meta.paragraph.markdown
T 132 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 151 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 152 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 153 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 164 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 165 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 166 text.html.markdown meta.paragraph.markdown
L 
L Scale horizontally with [replicas](https://www.postgresql.org/docs/current/hot-standby.html), or use [Citus](https://github.com/citusdata/citus), [PgDog](https://github.com/pgdogdev/pgdog), or another approach for sharding ([example](https://github.com/pgvector/pgvector-python/blob/master/examples/citus/example.py)).
T 24 text.html.markdown meta.paragraph.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 33 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 34 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 91 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 92 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 101 text.html.markdown meta.paragraph.markdown
T 102 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 107 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 108 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 109 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 143 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 144 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 146 text.html.markdown meta.paragraph.markdown
T 147 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 152 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 153 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 154 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 187 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 188 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 224 text.html.markdown meta.paragraph.markdown
T 225 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 232 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 233 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 234 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 315 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 316 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 318 text.html.markdown meta.paragraph.markdown
L 
L ## Monitoring
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 13 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Use existing tools like [pg_stat_statements](https://www.postgresql.org/docs/current/pgstatstatements.html) or [PgHero](https://github.com/ankane/pghero) to monitor performance.
T 24 text.html.markdown meta.paragraph.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 43 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 44 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 45 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 106 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 107 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 111 text.html.markdown meta.paragraph.markdown
T 112 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 118 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 119 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 120 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 152 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 153 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 177 text.html.markdown meta.paragraph.markdown
L 
L Monitor recall by comparing results from approximate search with exact search.
T 78 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L BEGIN;
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.LUW.sql
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L SET LOCAL enable_indexscan = off; -- use exact search
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DDL.create.II.sql
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql comment.line.double-dash.sql punctuation.definition.comment.sql
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql comment.line.double-dash.sql
L SELECT ...
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L COMMIT;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.LUW.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Languages
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 12 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Use pgvector from any language with a Postgres client. You can even generate and store vectors in one language and query them in another.
T 137 text.html.markdown meta.paragraph.markdown
L 
L Language | Libraries / Examples
T 31 text.html.markdown meta.paragraph.markdown
L --- | ---
T 9 text.html.markdown meta.paragraph.markdown
L Ada | [pgvector-ada](https://github.com/pgvector/pgvector-ada)
T 6 text.html.markdown meta.paragraph.markdown
T 7 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 61 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 62 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Algol | [pgvector-algol](https://github.com/pgvector/pgvector-algol)
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 24 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 67 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 68 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L C | [pgvector-c](https://github.com/pgvector/pgvector-c)
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 15 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 16 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 17 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 55 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 56 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L C++ | [pgvector-cpp](https://github.com/pgvector/pgvector-cpp)
T 6 text.html.markdown meta.paragraph.markdown
T 7 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 61 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 62 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L C#, F#, Visual Basic | [pgvector-dotnet](https://github.com/pgvector/pgvector-dotnet)
T 23 text.html.markdown meta.paragraph.markdown
T 24 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 39 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 40 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 41 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 84 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 85 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L COBOL | [pgvector-cobol](https://github.com/pgvector/pgvector-cobol)
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 24 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 67 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 68 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Crystal | [pgvector-crystal](https://github.com/pgvector/pgvector-crystal)
T 10 text.html.markdown meta.paragraph.markdown
T 11 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 73 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 74 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L D | [pgvector-d](https://github.com/pgvector/pgvector-d)
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 15 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 16 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 17 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 55 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 56 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Dart | [pgvector-dart](https://github.com/pgvector/pgvector-dart)
T 7 text.html.markdown meta.paragraph.markdown
T 8 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 64 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 65 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Elixir | [pgvector-elixir](https://github.com/pgvector/pgvector-elixir)
T 9 text.html.markdown meta.paragraph.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 70 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 71 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Erlang | [pgvector-erlang](https://github.com/pgvector/pgvector-erlang)
T 9 text.html.markdown meta.paragraph.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 70 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 71 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Fortran | [pgvector-fortran](https://github.com/pgvector/pgvector-fortran)
T 10 text.html.markdown meta.paragraph.markdown
T 11 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 73 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 74 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Gleam | [pgvector-gleam](https://github.com/pgvector/pgvector-gleam)
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 24 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 67 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 68 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Go | [pgvector-go](https://github.com/pgvector/pgvector-go)
T 5 text.html.markdown meta.paragraph.markdown
T 6 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 17 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 18 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 58 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 59 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Haskell | [pgvector-haskell](https://github.com/pgvector/pgvector-haskell)
T 10 text.html.markdown meta.paragraph.markdown
T 11 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 73 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 74 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Java, Kotlin, Groovy, Scala | [pgvector-java](https://github.com/pgvector/pgvector-java)
T 30 text.html.markdown meta.paragraph.markdown
T 31 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 44 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 45 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 46 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 87 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 88 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L JavaScript, TypeScript | [pgvector-node](https://github.com/pgvector/pgvector-node)
T 25 text.html.markdown meta.paragraph.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 39 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 40 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 41 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 82 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 83 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Julia | [Pgvector.jl](https://github.com/pgvector/Pgvector.jl)
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 61 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 62 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Lisp | [pgvector-lisp](https://github.com/pgvector/pgvector-lisp)
T 7 text.html.markdown meta.paragraph.markdown
T 8 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 64 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 65 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Lua | [pgvector-lua](https://github.com/pgvector/pgvector-lua)
T 6 text.html.markdown meta.paragraph.markdown
T 7 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 61 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 62 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Nim | [pgvector-nim](https://github.com/pgvector/pgvector-nim)
T 6 text.html.markdown meta.paragraph.markdown
T 7 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 61 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 62 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L OCaml | [pgvector-ocaml](https://github.com/pgvector/pgvector-ocaml)
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 24 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 67 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 68 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Pascal | [pgvector-pascal](https://github.com/pgvector/pgvector-pascal)
T 9 text.html.markdown meta.paragraph.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 70 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 71 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Perl | [pgvector-perl](https://github.com/pgvector/pgvector-perl)
T 7 text.html.markdown meta.paragraph.markdown
T 8 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 64 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 65 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L PHP | [pgvector-php](https://github.com/pgvector/pgvector-php)
T 6 text.html.markdown meta.paragraph.markdown
T 7 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 61 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 62 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Prolog | [pgvector-prolog](https://github.com/pgvector/pgvector-prolog)
T 9 text.html.markdown meta.paragraph.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 70 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 71 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Python | [pgvector-python](https://github.com/pgvector/pgvector-python)
T 9 text.html.markdown meta.paragraph.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 70 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 71 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L R | [pgvector-r](https://github.com/pgvector/pgvector-r)
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 15 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 16 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 17 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 55 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 56 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Racket | [pgvector-racket](https://github.com/pgvector/pgvector-racket)
T 9 text.html.markdown meta.paragraph.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 70 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 71 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Raku | [pgvector-raku](https://github.com/pgvector/pgvector-raku)
T 7 text.html.markdown meta.paragraph.markdown
T 8 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 64 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 65 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Ruby | [pgvector-ruby](https://github.com/pgvector/pgvector-ruby), [Neighbor](https://github.com/ankane/neighbor)
T 7 text.html.markdown meta.paragraph.markdown
T 8 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 64 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 65 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 67 text.html.markdown meta.paragraph.markdown
T 68 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 76 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 77 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 78 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 112 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 113 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Rust | [pgvector-rust](https://github.com/pgvector/pgvector-rust)
T 7 text.html.markdown meta.paragraph.markdown
T 8 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 64 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 65 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Swift | [pgvector-swift](https://github.com/pgvector/pgvector-swift)
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 24 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 67 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 68 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Tcl | [pgvector-tcl](https://github.com/pgvector/pgvector-tcl)
T 6 text.html.markdown meta.paragraph.markdown
T 7 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 61 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 62 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L Zig | [pgvector-zig](https://github.com/pgvector/pgvector-zig)
T 6 text.html.markdown meta.paragraph.markdown
T 7 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 61 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 62 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L ## Frequently Asked Questions
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 29 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L #### How many vectors can be stored in a single table?
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 54 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L A non-partitioned table has a limit of 32 TB by default in Postgres. A partitioned table can have thousands of partitions of that size.
T 135 text.html.markdown meta.paragraph.markdown
L 
L #### Is replication supported?
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 30 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L Yes, pgvector uses the write-ahead log (WAL), which allows for replication and point-in-time recovery.
T 102 text.html.markdown meta.paragraph.markdown
L 
L #### What if I want to index vectors with more than 2,000 dimensions?
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 69 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L You can use [half-precision vectors](#half-precision-vectors) or [half-precision indexing](#half-precision-indexing) to index up to 4,000 dimensions or [binary quantization](#binary-quantization) to index up to 64,000 dimensions. Other options are [indexing subvectors](#indexing-subvectors) (for models that support it) or [dimensionality reduction](https://en.wikipedia.org/wiki/Dimensionality_reduction).
T 12 text.html.markdown meta.paragraph.markdown
T 13 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 60 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 61 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 65 text.html.markdown meta.paragraph.markdown
T 66 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 89 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 90 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 91 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 115 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 116 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 152 text.html.markdown meta.paragraph.markdown
T 153 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 172 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 173 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 174 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 194 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 195 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 248 text.html.markdown meta.paragraph.markdown
T 249 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 268 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 269 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 270 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 290 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 291 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 324 text.html.markdown meta.paragraph.markdown
T 325 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 349 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 350 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 351 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 405 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 406 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 407 text.html.markdown meta.paragraph.markdown
L 
L #### Can I store vectors with different dimensions in the same column?
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 70 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L You can use `vector` as the type (instead of `vector(n)`).
T 12 text.html.markdown meta.paragraph.markdown
T 13 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 20 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 45 text.html.markdown meta.paragraph.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 55 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 56 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 58 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE TABLE embeddings (model_id bigint, item_id bigint, embedding vector, PRIMARY KEY (model_id, item_id));
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.type.sql
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.type.sql
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 87 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.modifier.sql
T 109 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L However, you can only create indexes on rows with the same number of dimensions (using [expression](https://www.postgresql.org/docs/current/indexes-expressional.html) and [partial](https://www.postgresql.org/docs/current/indexes-partial.html) indexing):
T 87 text.html.markdown meta.paragraph.markdown
T 88 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 98 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 99 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 100 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 165 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 166 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 171 text.html.markdown meta.paragraph.markdown
T 172 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 179 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 180 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 181 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 241 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 242 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 253 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON embeddings USING hnsw ((embedding::vector(3)) vector_l2_ops) WHERE (model_id = 123);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 77 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 82 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 93 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 94 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 95 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 98 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 100 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L and query with:
T 15 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT * FROM embeddings WHERE model_id = 123 ORDER BY embedding::vector(3) <-> '[3,1,2]' LIMIT 5;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 77 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 78 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.math.sql
T 79 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 80 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 81 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 88 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 89 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 90 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 95 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 96 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 97 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 98 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L #### Can I store vectors with more precision?
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 45 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L You can use the `double precision[]` or `numeric[]` type to store vectors with more precision.
T 16 text.html.markdown meta.paragraph.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 36 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 40 text.html.markdown meta.paragraph.markdown
T 41 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 50 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 51 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 94 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE TABLE items (id bigserial PRIMARY KEY, embedding double precision[]);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.type.sql
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.modifier.sql
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.type.sql
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql text.bracketed punctuation.definition.bracket.begin.sql
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql text.bracketed punctuation.definition.bracket.end.sql
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L 
L -- use {} instead of [] for Postgres arrays
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql comment.line.double-dash.sql punctuation.definition.comment.sql
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql comment.line.double-dash.sql
L INSERT INTO items (embedding) VALUES ('{1,2,3}'), ('{4,5,6}');
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.II.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Optionally, add a [check constraint](https://www.postgresql.org/docs/current/ddl-constraints.html) to ensure data can be converted to the `vector` type and has the expected dimensions.
T 18 text.html.markdown meta.paragraph.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 97 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 98 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 138 text.html.markdown meta.paragraph.markdown
T 139 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 145 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 146 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 184 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L ALTER TABLE items ADD CHECK (vector_dims(embedding::vector) = 3);
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.alter.sql keyword.other.create.sql
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.alter.sql
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.alter.sql keyword.other.table.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.alter.sql
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql storage.modifier.sql
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Use [expression indexing](https://www.postgresql.org/docs/current/indexes-expressional.html) to index (at a lower precision):
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 24 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 91 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 92 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 125 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CREATE INDEX ON items USING hnsw ((embedding::vector(3)) vector_l2_ops);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.create.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql keyword.other.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.create.sql entity.name.function.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L and query with:
T 15 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT * FROM items ORDER BY embedding::vector(3) <-> '[3,1,2]' LIMIT 5;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.star.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.math.sql
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L #### Do indexes need to fit into memory?
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 40 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L No, but like other index types, you’ll likely see better performance if they do. You can get the size of an index with:
T 121 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT pg_size_pretty(pg_relation_size('index_name'));
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Use [half-precision indexing](#half-precision-indexing) or [binary quantization](#binary-quantization) for smaller indexes.
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 54 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 55 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 59 text.html.markdown meta.paragraph.markdown
T 60 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 79 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 80 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 81 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 101 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 102 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 123 text.html.markdown meta.paragraph.markdown
L 
L ## Troubleshooting
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 18 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L #### Why isn’t a query using an index?
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 40 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L The query needs to have an `ORDER BY` and `LIMIT`, and the `ORDER BY` must be the result of a distance operator (not an expression) in ascending order.
T 27 text.html.markdown meta.paragraph.markdown
T 28 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 36 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 37 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 42 text.html.markdown meta.paragraph.markdown
T 43 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 48 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 49 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 59 text.html.markdown meta.paragraph.markdown
T 60 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 68 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 69 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 151 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L -- index
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql comment.line.double-dash.sql punctuation.definition.comment.sql
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql comment.line.double-dash.sql
L ORDER BY embedding <=> '[3,1,2]' LIMIT 5;
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L 
L -- no index
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql comment.line.double-dash.sql punctuation.definition.comment.sql
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql comment.line.double-dash.sql
L ORDER BY 1 - (embedding <=> '[3,1,2]') DESC LIMIT 5;
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.math.sql
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.order.sql
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L You can encourage the planner to use an index for a query with:
T 63 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L BEGIN;
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.LUW.sql
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L SET LOCAL enable_seqscan = off;
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DDL.create.II.sql
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L SELECT ...
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L COMMIT;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.LUW.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Also, if the table is small, a table scan may be faster.
T 56 text.html.markdown meta.paragraph.markdown
L 
L #### Why isn’t a query using a parallel table scan?
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 53 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L The planner doesn’t consider [out-of-line storage](https://www.postgresql.org/docs/current/storage-toast.html) in cost estimates, which can make a serial scan look cheaper. You can reduce the cost of a parallel scan for a query with:
T 31 text.html.markdown meta.paragraph.markdown
T 32 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 51 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 52 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 53 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 111 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 112 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 235 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L BEGIN;
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.LUW.sql
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L SET LOCAL min_parallel_table_scan_size = 1;
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L SET LOCAL parallel_setup_cost = 1;
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql constant.numeric.sql
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L SELECT ...
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L COMMIT;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.LUW.sql
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L or choose to store vectors inline:
T 34 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L ALTER TABLE items ALTER COLUMN embedding SET STORAGE PLAIN;
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.alter.sql keyword.other.create.sql
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.alter.sql
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.alter.sql keyword.other.table.sql
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.alter.sql
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L #### Why are there less results for a query after adding an HNSW index?
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 71 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L Results are limited by the size of the dynamic candidate list (`hnsw.ef_search`), which is 40 by default. There may be even less results due to dead tuples or filtering conditions in the query. Enabling [iterative index scans](#iterative-index-scans) can help address this.
T 63 text.html.markdown meta.paragraph.markdown
T 64 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 78 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 79 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 203 text.html.markdown meta.paragraph.markdown
T 204 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 225 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 226 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 227 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 249 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 250 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 273 text.html.markdown meta.paragraph.markdown
L 
L Also, note that `NULL` vectors are not indexed (as well as zero vectors for cosine distance).
T 16 text.html.markdown meta.paragraph.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 21 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 22 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 93 text.html.markdown meta.paragraph.markdown
L 
L #### Why are there less results for a query after adding an IVFFlat index?
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 74 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L The index was likely created with too little data for the number of lists. Drop the index until the table has more data.
T 120 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L DROP INDEX index_name;
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.drop.sql keyword.other.create.sql
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.drop.sql
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql meta.drop.sql keyword.other.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Results can also be limited by the number of probes (`ivfflat.probes`). Enabling [iterative index scans](#iterative-index-scans) can address this.
T 53 text.html.markdown meta.paragraph.markdown
T 54 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 68 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 69 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 81 text.html.markdown meta.paragraph.markdown
T 82 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 103 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 104 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 105 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 127 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 128 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 146 text.html.markdown meta.paragraph.markdown
L 
L Also, note that `NULL` vectors are not indexed (as well as zero vectors for cosine distance).
T 16 text.html.markdown meta.paragraph.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 21 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 22 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 93 text.html.markdown meta.paragraph.markdown
L 
L ## Reference
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 12 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L - [Vector](#vector-type)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L - [Halfvec](#halfvec-type)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 25 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 26 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L - [Bit](#bit-type)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 6 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 7 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L - [Sparsevec](#sparsevec-type)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 13 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 30 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L ### Vector Type
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 15 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Each vector takes `4 * dimensions + 8` bytes of storage. Each element is a single-precision floating-point number (like the `real` type in Postgres), and all elements must be finite (no `NaN`, `Infinity` or `-Infinity`). Vectors can have up to 16,000 dimensions.
T 18 text.html.markdown meta.paragraph.markdown
T 19 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 37 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 38 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 124 text.html.markdown meta.paragraph.markdown
T 125 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 129 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 130 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 186 text.html.markdown meta.paragraph.markdown
T 187 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 190 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 191 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 193 text.html.markdown meta.paragraph.markdown
T 194 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 202 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 203 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 207 text.html.markdown meta.paragraph.markdown
T 208 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 217 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 218 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 262 text.html.markdown meta.paragraph.markdown
L 
L ### Vector Operators
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 20 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Operator | Description | Added
T 30 text.html.markdown meta.paragraph.markdown
L --- | --- | ---
T 15 text.html.markdown meta.paragraph.markdown
L \+ | element-wise addition |
T 2 text.html.markdown meta.paragraph.markdown constant.character.escape.markdown
T 28 text.html.markdown meta.paragraph.markdown
L \- | element-wise subtraction |
T 2 text.html.markdown meta.paragraph.markdown constant.character.escape.markdown
T 31 text.html.markdown meta.paragraph.markdown
L \* | element-wise multiplication | 0.5.0
T 2 text.html.markdown meta.paragraph.markdown constant.character.escape.markdown
T 40 text.html.markdown meta.paragraph.markdown
L \|\| | concatenate | 0.7.0
T 26 text.html.markdown meta.paragraph.markdown
L <-> | Euclidean distance |
T 1 text.html.markdown meta.paragraph.markdown meta.other.valid-bracket.markdown
T 26 text.html.markdown meta.paragraph.markdown
L <#> | negative inner product |
T 1 text.html.markdown meta.paragraph.markdown meta.other.valid-bracket.markdown
T 30 text.html.markdown meta.paragraph.markdown
L <=> | cosine distance |
T 1 text.html.markdown meta.paragraph.markdown meta.other.valid-bracket.markdown
T 23 text.html.markdown meta.paragraph.markdown
L <+> | taxicab distance | 0.7.0
T 1 text.html.markdown meta.paragraph.markdown meta.other.valid-bracket.markdown
T 30 text.html.markdown meta.paragraph.markdown
L 
L ### Vector Functions
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 20 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Function | Description | Added
T 30 text.html.markdown meta.paragraph.markdown
L --- | --- | ---
T 15 text.html.markdown meta.paragraph.markdown
L binary_quantize(vector) → bit | binary quantize | 0.7.0
T 57 text.html.markdown meta.paragraph.markdown
L cosine_distance(vector, vector) → double precision | cosine distance |
T 72 text.html.markdown meta.paragraph.markdown
L inner_product(vector, vector) → double precision | inner product |
T 68 text.html.markdown meta.paragraph.markdown
L l1_distance(vector, vector) → double precision | taxicab distance | 0.5.0
T 75 text.html.markdown meta.paragraph.markdown
L l2_distance(vector, vector) → double precision | Euclidean distance |
T 71 text.html.markdown meta.paragraph.markdown
L l2_normalize(vector) → vector | normalize with Euclidean norm | 0.7.0
T 71 text.html.markdown meta.paragraph.markdown
L subvector(vector, integer, integer) → vector | subvector | 0.7.0
T 66 text.html.markdown meta.paragraph.markdown
L vector_dims(vector) → integer | number of dimensions |
T 56 text.html.markdown meta.paragraph.markdown
L vector_norm(vector) → double precision | Euclidean norm |
T 59 text.html.markdown meta.paragraph.markdown
L 
L ### Vector Aggregate Functions
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 30 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Function | Description | Added
T 30 text.html.markdown meta.paragraph.markdown
L --- | --- | ---
T 15 text.html.markdown meta.paragraph.markdown
L avg(vector) → vector | average |
T 34 text.html.markdown meta.paragraph.markdown
L sum(vector) → vector | sum | 0.5.0
T 36 text.html.markdown meta.paragraph.markdown
L 
L ### Halfvec Type
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 16 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Each half vector takes `2 * dimensions + 8` bytes of storage. Each element is a half-precision floating-point number, and all elements must be finite (no `NaN`, `Infinity` or `-Infinity`). Half vectors can have up to 16,000 dimensions.
T 23 text.html.markdown meta.paragraph.markdown
T 24 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 42 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 43 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 154 text.html.markdown meta.paragraph.markdown
T 155 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 158 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 159 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 161 text.html.markdown meta.paragraph.markdown
T 162 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 170 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 171 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 175 text.html.markdown meta.paragraph.markdown
T 176 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 185 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 186 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 235 text.html.markdown meta.paragraph.markdown
L 
L ### Halfvec Operators
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 21 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Operator | Description | Added
T 30 text.html.markdown meta.paragraph.markdown
L --- | --- | ---
T 15 text.html.markdown meta.paragraph.markdown
L \+ | element-wise addition | 0.7.0
T 2 text.html.markdown meta.paragraph.markdown constant.character.escape.markdown
T 34 text.html.markdown meta.paragraph.markdown
L \- | element-wise subtraction | 0.7.0
T 2 text.html.markdown meta.paragraph.markdown constant.character.escape.markdown
T 37 text.html.markdown meta.paragraph.markdown
L \* | element-wise multiplication | 0.7.0
T 2 text.html.markdown meta.paragraph.markdown constant.character.escape.markdown
T 40 text.html.markdown meta.paragraph.markdown
L \|\| | concatenate | 0.7.0
T 26 text.html.markdown meta.paragraph.markdown
L <-> | Euclidean distance | 0.7.0
T 1 text.html.markdown meta.paragraph.markdown meta.other.valid-bracket.markdown
T 32 text.html.markdown meta.paragraph.markdown
L <#> | negative inner product | 0.7.0
T 1 text.html.markdown meta.paragraph.markdown meta.other.valid-bracket.markdown
T 36 text.html.markdown meta.paragraph.markdown
L <=> | cosine distance | 0.7.0
T 1 text.html.markdown meta.paragraph.markdown meta.other.valid-bracket.markdown
T 29 text.html.markdown meta.paragraph.markdown
L <+> | taxicab distance | 0.7.0
T 1 text.html.markdown meta.paragraph.markdown meta.other.valid-bracket.markdown
T 30 text.html.markdown meta.paragraph.markdown
L 
L ### Halfvec Functions
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 21 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Function | Description | Added
T 30 text.html.markdown meta.paragraph.markdown
L --- | --- | ---
T 15 text.html.markdown meta.paragraph.markdown
L binary_quantize(halfvec) → bit | binary quantize | 0.7.0
T 58 text.html.markdown meta.paragraph.markdown
L cosine_distance(halfvec, halfvec) → double precision | cosine distance | 0.7.0
T 80 text.html.markdown meta.paragraph.markdown
L inner_product(halfvec, halfvec) → double precision | inner product | 0.7.0
T 76 text.html.markdown meta.paragraph.markdown
L l1_distance(halfvec, halfvec) → double precision | taxicab distance | 0.7.0
T 77 text.html.markdown meta.paragraph.markdown
L l2_distance(halfvec, halfvec) → double precision | Euclidean distance | 0.7.0
T 79 text.html.markdown meta.paragraph.markdown
L l2_norm(halfvec) → double precision | Euclidean norm | 0.7.0
T 62 text.html.markdown meta.paragraph.markdown
L l2_normalize(halfvec) → halfvec | normalize with Euclidean norm | 0.7.0
T 73 text.html.markdown meta.paragraph.markdown
L subvector(halfvec, integer, integer) → halfvec | subvector | 0.7.0
T 68 text.html.markdown meta.paragraph.markdown
L vector_dims(halfvec) → integer | number of dimensions | 0.7.0
T 63 text.html.markdown meta.paragraph.markdown
L 
L ### Halfvec Aggregate Functions
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 31 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Function | Description | Added
T 30 text.html.markdown meta.paragraph.markdown
L --- | --- | ---
T 15 text.html.markdown meta.paragraph.markdown
L avg(halfvec) → halfvec | average | 0.7.0
T 42 text.html.markdown meta.paragraph.markdown
L sum(halfvec) → halfvec | sum | 0.7.0
T 38 text.html.markdown meta.paragraph.markdown
L 
L ### Bit Type
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 12 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Each bit vector takes `dimensions / 8 + 8` bytes of storage. See the [Postgres docs](https://www.postgresql.org/docs/current/datatype-bit.html) for more info.
T 22 text.html.markdown meta.paragraph.markdown
T 23 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 41 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 42 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 69 text.html.markdown meta.paragraph.markdown
T 70 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 83 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 84 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 85 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 142 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 143 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 158 text.html.markdown meta.paragraph.markdown
L 
L ### Bit Operators
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 17 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Operator | Description | Added
T 30 text.html.markdown meta.paragraph.markdown
L --- | --- | ---
T 15 text.html.markdown meta.paragraph.markdown
L <~> | Hamming distance | 0.7.0
T 1 text.html.markdown meta.paragraph.markdown meta.other.valid-bracket.markdown
T 30 text.html.markdown meta.paragraph.markdown
L <%> | Jaccard distance | 0.7.0
T 1 text.html.markdown meta.paragraph.markdown meta.other.valid-bracket.markdown
T 30 text.html.markdown meta.paragraph.markdown
L 
L ### Bit Functions
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 17 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Function | Description | Added
T 30 text.html.markdown meta.paragraph.markdown
L --- | --- | ---
T 15 text.html.markdown meta.paragraph.markdown
L hamming_distance(bit, bit) → double precision | Hamming distance | 0.7.0
T 74 text.html.markdown meta.paragraph.markdown
L jaccard_distance(bit, bit) → double precision | Jaccard distance | 0.7.0
T 74 text.html.markdown meta.paragraph.markdown
L 
L ### Sparsevec Type
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 18 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Each sparse vector takes `8 * non-zero elements + 16` bytes of storage. Each element is a single-precision floating-point number, and all elements must be finite (no `NaN`, `Infinity` or `-Infinity`). Sparse vectors can have up to 16,000 non-zero elements.
T 25 text.html.markdown meta.paragraph.markdown
T 26 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 52 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 53 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 166 text.html.markdown meta.paragraph.markdown
T 167 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 170 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 171 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 173 text.html.markdown meta.paragraph.markdown
T 174 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 182 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 183 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 187 text.html.markdown meta.paragraph.markdown
T 188 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 197 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 198 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 256 text.html.markdown meta.paragraph.markdown
L 
L ### Sparsevec Operators
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 23 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Operator | Description | Added
T 30 text.html.markdown meta.paragraph.markdown
L --- | --- | ---
T 15 text.html.markdown meta.paragraph.markdown
L <-> | Euclidean distance | 0.7.0
T 1 text.html.markdown meta.paragraph.markdown meta.other.valid-bracket.markdown
T 32 text.html.markdown meta.paragraph.markdown
L <#> | negative inner product | 0.7.0
T 1 text.html.markdown meta.paragraph.markdown meta.other.valid-bracket.markdown
T 36 text.html.markdown meta.paragraph.markdown
L <=> | cosine distance | 0.7.0
T 1 text.html.markdown meta.paragraph.markdown meta.other.valid-bracket.markdown
T 29 text.html.markdown meta.paragraph.markdown
L <+> | taxicab distance | 0.7.0
T 1 text.html.markdown meta.paragraph.markdown meta.other.valid-bracket.markdown
T 30 text.html.markdown meta.paragraph.markdown
L 
L ### Sparsevec Functions
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 23 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Function | Description | Added
T 30 text.html.markdown meta.paragraph.markdown
L --- | --- | ---
T 15 text.html.markdown meta.paragraph.markdown
L cosine_distance(sparsevec, sparsevec) → double precision | cosine distance | 0.7.0
T 84 text.html.markdown meta.paragraph.markdown
L inner_product(sparsevec, sparsevec) → double precision | inner product | 0.7.0
T 80 text.html.markdown meta.paragraph.markdown
L l1_distance(sparsevec, sparsevec) → double precision | taxicab distance | 0.7.0
T 81 text.html.markdown meta.paragraph.markdown
L l2_distance(sparsevec, sparsevec) → double precision | Euclidean distance | 0.7.0
T 83 text.html.markdown meta.paragraph.markdown
L l2_norm(sparsevec) → double precision | Euclidean norm | 0.7.0
T 64 text.html.markdown meta.paragraph.markdown
L l2_normalize(sparsevec) → sparsevec | normalize with Euclidean norm | 0.7.0
T 77 text.html.markdown meta.paragraph.markdown
L 
L ## Installation Notes - Linux and Mac
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 37 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L ### Postgres Location
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 21 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L If your machine has multiple Postgres installations, specify the path to [pg_config](https://www.postgresql.org/docs/current/app-pgconfig.html) with:
T 73 text.html.markdown meta.paragraph.markdown
T 74 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 83 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 84 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 85 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 142 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 143 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 149 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L export PG_CONFIG=/Library/PostgreSQL/18/bin/pg_config
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.expression.assignment.modified.shell storage.modifier.export.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.expression.assignment.modified.shell
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.expression.assignment.modified.shell variable.other.assignment.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.expression.assignment.modified.shell keyword.operator.assignment.shell
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.expression.assignment.modified.shell
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.expression.assignment.modified.shell variable.other.assignment.shell
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.expression.assignment.modified.shell
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.expression.assignment.modified.shell variable.other.assignment.shell
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.expression.assignment.modified.shell
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.expression.assignment.modified.shell variable.other.assignment.shell
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.expression.assignment.modified.shell
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.expression.assignment.modified.shell variable.other.assignment.shell
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.expression.assignment.modified.shell
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.expression.assignment.modified.shell variable.other.assignment.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Then re-run the installation instructions (run `make clean` before `make` if needed). If `sudo` is needed for `make install`, use:
T 47 text.html.markdown meta.paragraph.markdown
T 48 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 58 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 59 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 67 text.html.markdown meta.paragraph.markdown
T 68 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 72 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 73 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 89 text.html.markdown meta.paragraph.markdown
T 90 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 94 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 95 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 110 text.html.markdown meta.paragraph.markdown
T 111 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 123 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 124 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 130 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L sudo --preserve-env=PG_CONFIG make install
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L A few common paths on Mac are:
T 30 text.html.markdown meta.paragraph.markdown
L 
L - EDB installer - `/Library/PostgreSQL/18/bin/pg_config`
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 55 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 56 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L - Homebrew (arm64) - `/opt/homebrew/opt/postgresql@18/bin/pg_config`
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 67 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 68 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L - Homebrew (x86-64) - `/usr/local/opt/postgresql@18/bin/pg_config`
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 65 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 66 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L Note: Replace `18` with your Postgres server version
T 14 text.html.markdown meta.paragraph.markdown
T 15 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 18 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 52 text.html.markdown meta.paragraph.markdown
L 
L ### Missing Header
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 18 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L If compilation fails with `fatal error: postgres.h: No such file or directory`, make sure Postgres development files are installed on the server.
T 26 text.html.markdown meta.paragraph.markdown
T 27 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 77 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 78 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 145 text.html.markdown meta.paragraph.markdown
L 
L For Ubuntu and Debian, use:
T 27 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L sudo apt install postgresql-server-dev-18
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Note: Replace `18` with your Postgres server version
T 14 text.html.markdown meta.paragraph.markdown
T 15 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 18 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 52 text.html.markdown meta.paragraph.markdown
L 
L ### Missing SDK
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 15 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L If compilation fails and the output includes `warning: no such sysroot directory` on Mac, your Postgres installation points to a path that no longer exists.
T 45 text.html.markdown meta.paragraph.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 80 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 81 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 156 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L pg_config --cppflags
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Reinstall Postgres to fix this.
T 31 text.html.markdown meta.paragraph.markdown
L 
L ### Portability
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 15 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L By default, pgvector compiles with `-march=native` on some platforms for best performance. However, this can lead to `Illegal instruction` errors if trying to run the compiled extension on a different machine.
T 35 text.html.markdown meta.paragraph.markdown
T 36 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 49 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 50 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 117 text.html.markdown meta.paragraph.markdown
T 118 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 137 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 138 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 209 text.html.markdown meta.paragraph.markdown
L 
L To compile for portability, use:
T 32 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L make OPTFLAGS=""
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.quoted.double.shell punctuation.definition.string.begin.shell
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.quoted.double.shell punctuation.definition.string.end.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Installation Notes - Windows
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 31 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L ### Missing Header
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 18 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L If compilation fails with `Cannot open include file: 'postgres.h': No such file or directory`, make sure `PGROOT` is correct.
T 26 text.html.markdown meta.paragraph.markdown
T 27 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 92 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 93 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 105 text.html.markdown meta.paragraph.markdown
T 106 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 112 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 113 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 125 text.html.markdown meta.paragraph.markdown
L 
L ### Mismatched Architecture
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 27 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L If compilation fails with `error C2196: case value '4' already used`, make sure you’re using the `x64 Native Tools Command Prompt`. Then run `nmake /F Makefile.win clean` and re-run the installation instructions.
T 26 text.html.markdown meta.paragraph.markdown
T 27 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 67 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 68 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 99 text.html.markdown meta.paragraph.markdown
T 100 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 131 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 132 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 143 text.html.markdown meta.paragraph.markdown
T 144 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 171 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 172 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 214 text.html.markdown meta.paragraph.markdown
L 
L ### Missing Symbol
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 18 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L If linking fails with `unresolved external symbol float_to_shortest_decimal_bufn` with Postgres 17.0-17.2, upgrade to Postgres 17.3+.
T 22 text.html.markdown meta.paragraph.markdown
T 23 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 80 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 81 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 133 text.html.markdown meta.paragraph.markdown
L 
L ### Permissions
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 15 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L If installation fails with `Access is denied`, re-run the installation instructions as an administrator.
T 27 text.html.markdown meta.paragraph.markdown
T 28 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 44 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 45 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 104 text.html.markdown meta.paragraph.markdown
L 
L ## Additional Installation Methods
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 34 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L ### Docker
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 10 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Get the [Docker image](https://hub.docker.com/r/pgvector/pgvector) with:
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 65 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 66 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 72 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L docker pull pgvector/pgvector:pg18-trixie
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L This adds pgvector to the [Postgres image](https://hub.docker.com/_/postgres) (replace `18` with your Postgres server version, and run it the same way).
T 26 text.html.markdown meta.paragraph.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 41 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 42 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 43 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 76 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 77 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 87 text.html.markdown meta.paragraph.markdown
T 88 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 90 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 91 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 152 text.html.markdown meta.paragraph.markdown
L 
L Supported tags are:
T 19 text.html.markdown meta.paragraph.markdown
L 
L - `pg18-trixie`, `0.8.7-pg18-trixie`
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L - `pg18-bookworm`, `0.8.7-pg18-bookworm`, `pg18`, `0.8.7-pg18`
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 39 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 40 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 47 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 48 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 50 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 51 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 62 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L - `pg17-trixie`, `0.8.7-pg17-trixie`
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L - `pg17-bookworm`, `0.8.7-pg17-bookworm`, `pg17`, `0.8.7-pg17`
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 39 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 40 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 47 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 48 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 50 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 51 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 62 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L - `pg16-trixie`, `0.8.7-pg16-trixie`
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L - `pg16-bookworm`, `0.8.7-pg16-bookworm`, `pg16`, `0.8.7-pg16`
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 39 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 40 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 47 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 48 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 50 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 51 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 62 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L - `pg15-trixie`, `0.8.7-pg15-trixie`
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L - `pg15-bookworm`, `0.8.7-pg15-bookworm`, `pg15`, `0.8.7-pg15`
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 39 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 40 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 47 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 48 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 50 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 51 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 62 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L - `pg14-trixie`, `0.8.7-pg14-trixie`
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L - `pg14-bookworm`, `0.8.7-pg14-bookworm`, `pg14`, `0.8.7-pg14`
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 39 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 40 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 47 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 48 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 50 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 51 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 62 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L - `pg13-trixie`, `0.8.7-pg13-trixie`
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L - `pg13-bookworm`, `0.8.7-pg13-bookworm`, `pg13`, `0.8.7-pg13`
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 39 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 40 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 47 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 48 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 50 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 51 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 62 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L You can also build the image manually:
T 38 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L git clone --branch v0.8.7 https://github.com/pgvector/pgvector.git
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L cd pgvector
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell support.function.builtin.shell
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L docker build --pull --build-arg PG_MAJOR=18 -t myuser/pgvector .
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell constant.numeric.shell constant.numeric.integer.shell
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L If you increase `maintenance_work_mem`, make sure `--shm-size` is at least that size to avoid an error with parallel HNSW index builds.
T 16 text.html.markdown meta.paragraph.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 37 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 38 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 50 text.html.markdown meta.paragraph.markdown
T 51 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 62 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 135 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L docker run --shm-size=1g ...
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Homebrew
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 12 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L With Homebrew Postgres, you can use:
T 36 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L brew install pgvector
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Note: This only adds it to the `postgresql@18` and `postgresql@17` formulas
T 31 text.html.markdown meta.paragraph.markdown
T 32 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 45 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 51 text.html.markdown meta.paragraph.markdown
T 52 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 65 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 66 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 75 text.html.markdown meta.paragraph.markdown
L 
L ### PGXN
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 8 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Install from the [PostgreSQL Extension Network](https://pgxn.org/dist/vector) with:
T 17 text.html.markdown meta.paragraph.markdown
T 18 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 46 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 47 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 48 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 76 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 77 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 83 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L pgxn install vector
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### APT
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 7 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Debian and Ubuntu packages are available from the [PostgreSQL APT Repository](https://wiki.postgresql.org/wiki/Apt). Follow the [setup instructions](https://wiki.postgresql.org/wiki/Apt#Quickstart) and run:
T 50 text.html.markdown meta.paragraph.markdown
T 51 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 76 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 77 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 78 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 114 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 115 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 128 text.html.markdown meta.paragraph.markdown
T 129 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 147 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 148 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 149 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 196 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 197 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 206 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L sudo apt install postgresql-18-pgvector
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Note: Replace `18` with your Postgres server version
T 14 text.html.markdown meta.paragraph.markdown
T 15 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 18 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 52 text.html.markdown meta.paragraph.markdown
L 
L ### Yum
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 7 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L RPM packages are available from the [PostgreSQL Yum Repository](https://yum.postgresql.org/). Follow the [setup instructions](https://www.postgresql.org/download/linux/redhat/) for your distribution and run:
T 36 text.html.markdown meta.paragraph.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 62 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 63 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 64 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 91 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 92 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 105 text.html.markdown meta.paragraph.markdown
T 106 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 124 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 125 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 126 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 175 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 176 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 207 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L sudo yum install pgvector_18
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L # or
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell punctuation.definition.comment.shell
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell
L sudo dnf install pgvector_18
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Note: Replace `18` with your Postgres server version
T 14 text.html.markdown meta.paragraph.markdown
T 15 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 18 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 52 text.html.markdown meta.paragraph.markdown
L 
L ### pkg
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 7 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Install the FreeBSD package with:
T 33 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L pkg install postgresql18-pgvector
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L or the port with:
T 17 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L cd /usr/ports/databases/pgvector
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell support.function.builtin.shell
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L make install
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### APK
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 7 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Install the Alpine package with:
T 32 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L apk add postgresql-pgvector
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### conda-forge
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 15 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L With Conda Postgres, install from [conda-forge](https://anaconda.org/conda-forge/pgvector) with:
T 34 text.html.markdown meta.paragraph.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 46 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 47 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 48 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 89 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 90 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 96 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L conda install -c conda-forge pgvector
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L This method is [community-maintained](https://github.com/conda-forge/pgvector-feedstock) by [@mmcauliffe](https://github.com/mmcauliffe)
T 15 text.html.markdown meta.paragraph.markdown
T 16 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 38 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 87 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 88 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 92 text.html.markdown meta.paragraph.markdown
T 93 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 104 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 105 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 106 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 135 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 136 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L ### Postgres.app
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 16 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Download the [latest release](https://postgresapp.com/downloads.html) with Postgres 15+.
T 13 text.html.markdown meta.paragraph.markdown
T 14 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 68 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 69 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 88 text.html.markdown meta.paragraph.markdown
L 
L ## Hosted Postgres
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 18 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L pgvector is available on [these providers](https://github.com/pgvector/pgvector/issues/54).
T 25 text.html.markdown meta.paragraph.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 41 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 42 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 43 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 89 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 90 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 91 text.html.markdown meta.paragraph.markdown
L 
L ## Upgrading
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 12 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L [Install](#installation) the latest version (use the same method as the original installation). Then in each database you want to upgrade, run:
T 1 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 8 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 9 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 24 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 143 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L ALTER EXTENSION vector UPDATE;
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.sql
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L You can check the version in the current database with:
T 55 text.html.markdown meta.paragraph.markdown
L 
L ```sql
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L SELECT extversion FROM pg_extension WHERE extname = 'vector';
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.other.DML.sql
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql keyword.operator.comparison.sql
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.begin.sql
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql string.quoted.single.sql punctuation.definition.string.end.sql
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.sql
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Thanks
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 9 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Thanks to:
T 10 text.html.markdown meta.paragraph.markdown
L 
L - [PASE: PostgreSQL Ultra-High-Dimensional Approximate Nearest Neighbor Search Extension](https://dl.acm.org/doi/pdf/10.1145/3318464.3386131)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 88 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 89 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 90 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 140 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 141 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L - [Faiss: A Library for Efficient Similarity Search and Clustering of Dense Vectors](https://github.com/facebookresearch/faiss)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 83 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 84 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 85 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 126 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 127 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L - [Using the Triangle Inequality to Accelerate k-means](https://cdn.aaai.org/ICML/2003/ICML03-022.pdf)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 54 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 55 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 56 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 101 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 102 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L - [k-means++: The Advantage of Careful Seeding](https://theory.stanford.edu/~sergei/papers/kMeansPP-soda.pdf)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 46 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 47 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 48 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 108 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 109 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L - [Concept Decompositions for Large Sparse Text Data using Clustering](https://www.cs.utexas.edu/users/inderjit/public_papers/concept_mlj.pdf)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 69 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 70 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 71 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 141 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 142 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L - [Efficient and Robust Approximate Nearest Neighbor Search using Hierarchical Navigable Small World Graphs](https://arxiv.org/pdf/1603.09320)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 107 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 108 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 109 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 141 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 142 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L ## History
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 10 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L View the [changelog](https://github.com/pgvector/pgvector/blob/master/CHANGELOG.md)
T 9 text.html.markdown meta.paragraph.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 82 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 83 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L ## Contributing
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 15 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Everyone is encouraged to help improve this project. Here are a few ways you can help:
T 86 text.html.markdown meta.paragraph.markdown
L 
L - [Report bugs](https://github.com/pgvector/pgvector/issues)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 59 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 60 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L - Fix bugs and [submit pull requests](https://github.com/pgvector/pgvector/pulls)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 37 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 38 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 80 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 81 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L - Write, clarify, or fix documentation
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 38 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Suggest or add new features
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L To get started with development:
T 32 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L git clone https://github.com/pgvector/pgvector.git
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L cd pgvector
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell support.function.builtin.shell
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L make
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
L make install
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L To run all tests:
T 17 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L make installcheck        # regression tests
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell punctuation.definition.comment.shell
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell
L make prove_installcheck  # TAP tests
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell punctuation.definition.comment.shell
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L To run single tests:
T 20 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L make installcheck REGRESS=functions                            # regression test
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell punctuation.definition.comment.shell
T 80 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell
L make prove_installcheck PROVE_TESTS=test/t/001_ivfflat_wal.pl  # TAP test
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell punctuation.definition.comment.shell
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L To enable assertions:
T 21 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L make clean && PG_CFLAGS="-DUSE_ASSERT_CHECKING" make && make install
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript punctuation.separator.statement.and.shell
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell variable.other.assignment.shell
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell keyword.operator.assignment.shell
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.quoted.double.shell punctuation.definition.string.begin.shell
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.quoted.double.shell
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.quoted.double.shell punctuation.definition.string.end.shell
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.statement.command.env.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.statement.command.env.shell
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript punctuation.separator.statement.and.shell
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L To enable benchmarking:
T 23 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L make clean && PG_CFLAGS="-DHNSW_BENCH -DIVFFLAT_BENCH" make && make install
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript punctuation.separator.statement.and.shell
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell variable.other.assignment.shell
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell keyword.operator.assignment.shell
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.quoted.double.shell punctuation.definition.string.begin.shell
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.quoted.double.shell
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.quoted.double.shell punctuation.definition.string.end.shell
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.statement.command.env.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.statement.command.env.shell
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript punctuation.separator.statement.and.shell
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L To show memory usage:
T 21 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L make clean && PG_CFLAGS="-DHNSW_MEMORY -DIVFFLAT_MEMORY" make && make install
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript punctuation.separator.statement.and.shell
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell variable.other.assignment.shell
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell keyword.operator.assignment.shell
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.quoted.double.shell punctuation.definition.string.begin.shell
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.quoted.double.shell
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.quoted.double.shell punctuation.definition.string.end.shell
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.statement.command.env.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.statement.command.env.shell
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript punctuation.separator.statement.and.shell
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 77 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L To get k-means metrics:
T 23 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L make clean && PG_CFLAGS="-DIVFFLAT_KMEANS_DEBUG" make && make install
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript punctuation.separator.statement.and.shell
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell variable.other.assignment.shell
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell keyword.operator.assignment.shell
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.quoted.double.shell punctuation.definition.string.begin.shell
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.quoted.double.shell
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.quoted.double.shell punctuation.definition.string.end.shell
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.statement.command.env.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.statement.command.env.shell
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript punctuation.separator.statement.and.shell
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Resources for contributors
T 26 text.html.markdown meta.paragraph.markdown
L 
L - [Extension Building Infrastructure](https://www.postgresql.org/docs/current/extend-pgxs.html)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 37 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 38 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 94 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 95 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L - [Index Access Method Interface Definition](https://www.postgresql.org/docs/current/indexam.html)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 44 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 45 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 97 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 98 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L - [Generic WAL Records](https://www.postgresql.org/docs/current/generic-wal.html)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 80 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 81 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
