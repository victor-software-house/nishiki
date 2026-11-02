L <p align="center">
T 18 text.html.markdown meta.paragraph.markdown
L   <img src="logo.svg" width="200px" align="center" alt="Zod logo" />
T 2 text.html.markdown meta.paragraph.markdown
T 68 text.html.markdown meta.paragraph.markdown
L   <h1 align="center">Zod</h1>
T 2 text.html.markdown meta.paragraph.markdown
T 29 text.html.markdown meta.paragraph.markdown
L   <p align="center">
T 2 text.html.markdown meta.paragraph.markdown
T 20 text.html.markdown meta.paragraph.markdown
L     TypeScript-first schema validation with static type inference
T 4 text.html.markdown meta.paragraph.markdown
T 65 text.html.markdown meta.paragraph.markdown
L     <br/>
T 4 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown
L     by <a href="https://x.com/colinhacks">@colinhacks</a>
T 4 text.html.markdown meta.paragraph.markdown
T 57 text.html.markdown meta.paragraph.markdown
L   </p>
T 2 text.html.markdown meta.paragraph.markdown
T 6 text.html.markdown meta.paragraph.markdown
L </p>
T 4 text.html.markdown meta.paragraph.markdown
L <br/>
T 5 text.html.markdown meta.paragraph.markdown
L 
L <p align="center">
T 18 text.html.markdown meta.paragraph.markdown
L <a href="https://github.com/colinhacks/zod/actions?query=branch%3Amain"><img src="https://github.com/colinhacks/zod/actions/workflows/test.yml/badge.svg?event=push&branch=main" alt="Zod CI status" /></a>
T 163 text.html.markdown meta.paragraph.markdown
T 164 text.html.markdown meta.paragraph.markdown meta.other.valid-ampersand.markdown
T 203 text.html.markdown meta.paragraph.markdown
L <a href="https://opensource.org/licenses/MIT" rel="nofollow"><img src="https://img.shields.io/github/license/colinhacks/zod" alt="License"></a>
T 143 text.html.markdown meta.paragraph.markdown
L <a href="https://www.npmjs.com/package/zod" rel="nofollow"><img src="https://img.shields.io/npm/dw/zod.svg" alt="npm"></a>
T 122 text.html.markdown meta.paragraph.markdown
L <a href="https://discord.gg/KaSRdyX2vc" rel="nofollow"><img src="https://img.shields.io/discord/893487829802418277?label=Discord&logo=discord&logoColor=white" alt="discord server"></a>
T 128 text.html.markdown meta.paragraph.markdown
T 129 text.html.markdown meta.paragraph.markdown meta.other.valid-ampersand.markdown
T 141 text.html.markdown meta.paragraph.markdown
T 142 text.html.markdown meta.paragraph.markdown meta.other.valid-ampersand.markdown
T 184 text.html.markdown meta.paragraph.markdown
L <a href="https://github.com/colinhacks/zod" rel="nofollow"><img src="https://img.shields.io/github/stars/colinhacks/zod" alt="stars"></a>
T 137 text.html.markdown meta.paragraph.markdown
L </p>
T 4 text.html.markdown meta.paragraph.markdown
L 
L <div align="center">
T 20 text.html.markdown meta.paragraph.markdown
L   <a href="https://zod.dev/api">Docs</a>
T 2 text.html.markdown meta.paragraph.markdown
T 40 text.html.markdown meta.paragraph.markdown
L   <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
T 2 text.html.markdown meta.paragraph.markdown
T 42 text.html.markdown meta.paragraph.markdown
L   <a href="https://discord.gg/RcG33DQJdf">Discord</a>
T 2 text.html.markdown meta.paragraph.markdown
T 53 text.html.markdown meta.paragraph.markdown
L   <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
T 2 text.html.markdown meta.paragraph.markdown
T 42 text.html.markdown meta.paragraph.markdown
L   <a href="https://twitter.com/colinhacks">𝕏</a>
T 2 text.html.markdown meta.paragraph.markdown
T 51 text.html.markdown meta.paragraph.markdown
L   <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
T 2 text.html.markdown meta.paragraph.markdown
T 42 text.html.markdown meta.paragraph.markdown
L   <a href="https://bsky.app/profile/zod.dev">Bluesky</a>
T 2 text.html.markdown meta.paragraph.markdown
T 56 text.html.markdown meta.paragraph.markdown
L   <br />
T 2 text.html.markdown meta.paragraph.markdown
T 8 text.html.markdown meta.paragraph.markdown
L </div>
T 6 text.html.markdown meta.paragraph.markdown
L 
L <br/>
T 5 text.html.markdown meta.paragraph.markdown
L <br/>
T 5 text.html.markdown meta.paragraph.markdown
L 
L ### [Read the docs →](https://zod.dev/api)
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 5 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 22 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown meta.link.inline.markdown string.other.link.title.markdown
T 23 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 24 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 43 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown meta.link.inline.markdown markup.underline.link.markdown
T 44 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L <br/>
T 5 text.html.markdown meta.paragraph.markdown
L <br/>
T 5 text.html.markdown meta.paragraph.markdown
L 
L ## What is Zod?
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 15 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Zod is a TypeScript-first validation library. Define a schema and parse some data with it. You'll get back a strongly typed, validated result.
T 142 text.html.markdown meta.paragraph.markdown
L 
L ```ts
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L import * as z from "zod";
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts keyword.control.import.ts
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts constant.language.import-export-all.ts
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts keyword.control.as.ts
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts variable.other.readwrite.alias.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts keyword.control.from.ts
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts string.quoted.double.ts punctuation.definition.string.begin.ts
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts string.quoted.double.ts
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts string.quoted.double.ts punctuation.definition.string.end.ts
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L 
L const User = z.object({
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts storage.type.ts
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts meta.definition.variable.ts variable.other.constant.ts
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts keyword.operator.assignment.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts variable.other.object.ts
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts punctuation.accessor.ts
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts entity.name.function.ts
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.definition.block.ts
L   name: z.string(),
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts punctuation.separator.key-value.ts
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.function-call.ts variable.other.object.ts
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.function-call.ts punctuation.accessor.ts
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.function-call.ts entity.name.function.ts
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.brace.round.ts
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.brace.round.ts
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.separator.comma.ts
L });
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.definition.block.ts
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L 
L // some untrusted data...
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts punctuation.definition.comment.ts
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts
L const input = {
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts storage.type.ts
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts meta.definition.variable.ts variable.other.constant.ts
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts keyword.operator.assignment.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.definition.block.ts
L   /* stuff */
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts comment.block.ts punctuation.definition.comment.ts
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts comment.block.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts comment.block.ts punctuation.definition.comment.ts
L };
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.definition.block.ts
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L 
L // the parsed result is validated and type safe!
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts punctuation.definition.comment.ts
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts
L const data = User.parse(input);
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts storage.type.ts
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts meta.definition.variable.ts variable.other.constant.ts
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts keyword.operator.assignment.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts variable.other.object.ts
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts punctuation.accessor.ts
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts entity.name.function.ts
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts variable.other.readwrite.ts
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L 
L // so you can use it with confidence :)
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts punctuation.definition.comment.ts
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts
L console.log(data.name);
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.function-call.ts variable.other.object.ts
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.function-call.ts punctuation.accessor.ts
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.function-call.ts entity.name.function.ts
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.brace.round.ts
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript variable.other.object.ts
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.accessor.ts
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript variable.other.property.ts
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.brace.round.ts
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L <br/>
T 5 text.html.markdown meta.paragraph.markdown
L 
L ## Features
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 11 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L - Zero external dependencies
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Works in Node.js and all modern browsers
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Tiny: `2kb` core bundle (gzipped)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 13 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Immutable API: methods return a new instance
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 46 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Concise interface
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Works with TypeScript and plain JS
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Built-in JSON Schema conversion
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 33 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Extensive ecosystem
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L <br/>
T 5 text.html.markdown meta.paragraph.markdown
L 
L ## Installation
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 15 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language
L npm install zod
T 15 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L <br/>
T 5 text.html.markdown meta.paragraph.markdown
L 
L ## Basic usage
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 14 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Before you can do anything else, you need to define a schema. For the purposes of this guide, we'll use a simple object schema.
T 127 text.html.markdown meta.paragraph.markdown
L 
L ```ts
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L import * as z from "zod";
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts keyword.control.import.ts
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts constant.language.import-export-all.ts
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts keyword.control.as.ts
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts variable.other.readwrite.alias.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts keyword.control.from.ts
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts string.quoted.double.ts punctuation.definition.string.begin.ts
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts string.quoted.double.ts
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts string.quoted.double.ts punctuation.definition.string.end.ts
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L 
L const Player = z.object({
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts storage.type.ts
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts meta.definition.variable.ts variable.other.constant.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts keyword.operator.assignment.ts
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts variable.other.object.ts
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts punctuation.accessor.ts
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts entity.name.function.ts
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.definition.block.ts
L   username: z.string(),
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts punctuation.separator.key-value.ts
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.function-call.ts variable.other.object.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.function-call.ts punctuation.accessor.ts
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.function-call.ts entity.name.function.ts
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.brace.round.ts
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.brace.round.ts
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.separator.comma.ts
L   xp: z.number(),
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts punctuation.separator.key-value.ts
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.function-call.ts variable.other.object.ts
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.function-call.ts punctuation.accessor.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.function-call.ts entity.name.function.ts
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.brace.round.ts
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.brace.round.ts
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.separator.comma.ts
L });
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.definition.block.ts
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Parsing data
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 16 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Given any Zod schema, use `.parse` to validate an input. If it's valid, Zod returns a strongly-typed _deep clone_ of the input.
T 26 text.html.markdown meta.paragraph.markdown
T 27 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 33 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 34 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 101 text.html.markdown meta.paragraph.markdown
T 102 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 112 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 113 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 127 text.html.markdown meta.paragraph.markdown
L 
L ```ts
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L Player.parse({ username: "billie", xp: 100 });
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.function-call.ts variable.other.object.ts
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.function-call.ts punctuation.accessor.ts
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.function-call.ts entity.name.function.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.brace.round.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts punctuation.definition.block.ts
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts punctuation.separator.key-value.ts
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts punctuation.definition.string.begin.ts
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts punctuation.definition.string.end.ts
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts punctuation.separator.comma.ts
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts punctuation.separator.key-value.ts
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts constant.numeric.decimal.ts
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts punctuation.definition.block.ts
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.brace.round.ts
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L // => returns { username: "billie", xp: 100 }
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts punctuation.definition.comment.ts
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L **Note** — If your schema uses certain asynchronous APIs like `async` [refinements](https://zod.dev/api#refinements) or [transforms](https://zod.dev/api#transforms), you'll need to use the `.parseAsync()` method instead.
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 6 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 8 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 64 text.html.markdown meta.paragraph.markdown
T 65 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 70 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 71 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 72 text.html.markdown meta.paragraph.markdown
T 73 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 84 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 85 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 86 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 117 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 118 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 122 text.html.markdown meta.paragraph.markdown
T 123 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 133 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 134 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 135 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 165 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 166 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 191 text.html.markdown meta.paragraph.markdown
T 192 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 205 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 206 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 222 text.html.markdown meta.paragraph.markdown
L 
L ```ts
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L const schema = z.string().refine(async (val) => val.length <= 8);
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts storage.type.ts
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts meta.definition.variable.ts variable.other.constant.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts keyword.operator.assignment.ts
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts variable.other.object.ts
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts punctuation.accessor.ts
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts entity.name.function.ts
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts punctuation.accessor.ts
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts entity.name.function.ts
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts storage.modifier.async.ts
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts meta.parameters.ts punctuation.definition.parameters.begin.ts
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts meta.parameters.ts variable.parameter.ts
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts meta.parameters.ts punctuation.definition.parameters.end.ts
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts storage.type.function.arrow.ts
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts variable.other.object.ts
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts punctuation.accessor.ts
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts support.variable.property.ts
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts keyword.operator.relational.ts
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts constant.numeric.decimal.ts
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L 
L await schema.parseAsync("hello");
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript keyword.control.flow.ts
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.function-call.ts variable.other.object.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.function-call.ts punctuation.accessor.ts
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.function-call.ts entity.name.function.ts
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.brace.round.ts
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript string.quoted.double.ts punctuation.definition.string.begin.ts
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript string.quoted.double.ts
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript string.quoted.double.ts punctuation.definition.string.end.ts
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.brace.round.ts
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L // => "hello"
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts punctuation.definition.comment.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### AOT compilation
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 19 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L For hot validation paths, `z.compile(schema)` returns a schema clone with an ahead-of-time compiled fast path. Valid inputs take the compiled path; invalid inputs fall back to the regular parser so error reporting stays identical.
T 26 text.html.markdown meta.paragraph.markdown
T 27 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 44 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 45 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 230 text.html.markdown meta.paragraph.markdown
L 
L Across a 55-schema benchmark the median speedup is **2.4x**, and it scales with how much work the schema does per parse: a large array of objects is ~9x, a 20-key object ~9x, a nested object ~4.5x, while a bare `z.string()` gains nothing — compilation removes per-node dispatch and allocation, and a single `typeof` has none to remove.
T 51 text.html.markdown meta.paragraph.markdown
T 53 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 57 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 59 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 211 text.html.markdown meta.paragraph.markdown
T 212 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 222 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 223 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 309 text.html.markdown meta.paragraph.markdown
T 310 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 316 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 317 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 337 text.html.markdown meta.paragraph.markdown
L 
L ```ts
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L const CompiledPlayer = z.compile(Player);
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts storage.type.ts
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts meta.definition.variable.ts variable.other.constant.ts
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts keyword.operator.assignment.ts
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts variable.other.object.ts
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts punctuation.accessor.ts
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts entity.name.function.ts
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts variable.other.readwrite.ts
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L 
L CompiledPlayer.parse({ username: "billie", xp: 100 });
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.function-call.ts variable.other.object.ts
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.function-call.ts punctuation.accessor.ts
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.function-call.ts entity.name.function.ts
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.brace.round.ts
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts punctuation.definition.block.ts
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts punctuation.separator.key-value.ts
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts punctuation.definition.string.begin.ts
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts punctuation.definition.string.end.ts
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts punctuation.separator.comma.ts
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts punctuation.separator.key-value.ts
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts constant.numeric.decimal.ts
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts meta.object.member.ts
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.objectliteral.ts punctuation.definition.block.ts
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.brace.round.ts
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L To enable compilation globally for schemas constructed after import:
T 68 text.html.markdown meta.paragraph.markdown
L 
L ```ts
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L import "zod/compile"; // place before modules that define schemas
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts keyword.control.import.ts
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts string.quoted.double.ts punctuation.definition.string.begin.ts
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts string.quoted.double.ts
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts string.quoted.double.ts punctuation.definition.string.end.ts
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts punctuation.definition.comment.ts
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Things to know:
T 15 text.html.markdown meta.paragraph.markdown
L 
L - Compilation uses `new Function`. Global mode is automatically disabled when `z.config({ jitless: true })` is set (e.g. CSP environments); calling `z.compile()` directly is an explicit opt-in.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 32 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 33 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 78 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 79 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 106 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 107 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 148 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 149 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 160 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 161 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 193 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Schemas with async refinements or transforms can't be compiled, and neither can a few other constructs. That is not an error: `z.compile()` hands the schema back unchanged and it keeps using the regular parser, exactly as global mode leaves it. Pass `{ strict: true }` to throw `ZodCompileAsyncError` / `ZodCompileUnsupportedError` instead.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 128 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 129 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 140 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 141 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 252 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 253 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 269 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 270 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 280 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 281 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 301 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 302 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 305 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 306 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 332 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 333 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 342 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - On invalid input, refinements and transforms may run twice (fast path, then fallback).
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 88 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Deriving a new schema from a compiled one (`.refine()`, `.extend()`, etc.) returns an uncompiled schema — compile the final schema.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 45 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 46 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 55 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 56 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 58 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 59 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 68 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 69 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 135 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L See [`compile` docs](https://zod.dev/compile) for details.
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 6 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 13 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 14 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 44 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 45 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 58 text.html.markdown meta.paragraph.markdown
L 
L ### Handling errors
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 19 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L When validation fails, the `.parse()` method will throw a `ZodError` instance with granular information about the validation issues.
T 27 text.html.markdown meta.paragraph.markdown
T 28 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 36 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 37 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 58 text.html.markdown meta.paragraph.markdown
T 59 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 67 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 68 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 132 text.html.markdown meta.paragraph.markdown
L 
L ```ts
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L try {
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript keyword.control.trycatch.ts
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts punctuation.definition.block.ts
L   Player.parse({ username: 42, xp: "100" });
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.function-call.ts variable.other.object.ts
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.function-call.ts punctuation.accessor.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.function-call.ts entity.name.function.ts
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.brace.round.ts
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.objectliteral.ts punctuation.definition.block.ts
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.objectliteral.ts
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts punctuation.separator.key-value.ts
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.objectliteral.ts meta.object.member.ts
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.objectliteral.ts meta.object.member.ts constant.numeric.decimal.ts
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.objectliteral.ts punctuation.separator.comma.ts
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.objectliteral.ts
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts punctuation.separator.key-value.ts
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.objectliteral.ts meta.object.member.ts
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts punctuation.definition.string.begin.ts
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts punctuation.definition.string.end.ts
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.objectliteral.ts meta.object.member.ts
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.objectliteral.ts punctuation.definition.block.ts
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.brace.round.ts
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts punctuation.terminator.statement.ts
L } catch (err) {
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts punctuation.definition.block.ts
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript keyword.control.trycatch.ts
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.brace.round.ts
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript variable.other.readwrite.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.brace.round.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts punctuation.definition.block.ts
L   if (err instanceof z.ZodError) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts keyword.control.conditional.ts
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.brace.round.ts
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts variable.other.readwrite.ts
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts keyword.operator.expression.instanceof.ts
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts entity.name.type.module.ts
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts punctuation.accessor.ts
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts entity.name.type.ts
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.brace.round.ts
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts punctuation.definition.block.ts
L     err.issues;
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts variable.other.object.ts
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts punctuation.accessor.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts variable.other.property.ts
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts punctuation.terminator.statement.ts
L     /* [
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts comment.block.ts punctuation.definition.comment.ts
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts comment.block.ts
L       {
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts comment.block.ts
L         expected: 'string',
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts comment.block.ts
L         code: 'invalid_type',
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts comment.block.ts
L         path: [ 'username' ],
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts comment.block.ts
L         message: 'Invalid input: expected string, received number'
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts comment.block.ts
L       },
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts comment.block.ts
L       {
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts comment.block.ts
L         expected: 'number',
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts comment.block.ts
L         code: 'invalid_type',
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts comment.block.ts
L         path: [ 'xp' ],
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts comment.block.ts
L         message: 'Invalid input: expected number, received string'
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts comment.block.ts
L       }
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts comment.block.ts
L     ] */
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts comment.block.ts
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts comment.block.ts punctuation.definition.comment.ts
L   }
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts meta.block.ts punctuation.definition.block.ts
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts punctuation.definition.block.ts
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L To avoid a `try/catch` block, you can use the `.safeParse()` method to get back a plain result object containing either the successfully parsed data or a `ZodError`. The result type is a [discriminated union](https://www.typescriptlang.org/docs/handbook/2/narrowing.html#discriminated-unions), so you can handle both cases conveniently.
T 11 text.html.markdown meta.paragraph.markdown
T 12 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 21 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 22 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 46 text.html.markdown meta.paragraph.markdown
T 47 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 59 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 60 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 154 text.html.markdown meta.paragraph.markdown
T 155 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 163 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 164 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 187 text.html.markdown meta.paragraph.markdown
T 188 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 207 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 208 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 209 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 291 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 292 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 336 text.html.markdown meta.paragraph.markdown
L 
L ```ts
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L const result = Player.safeParse({ username: 42, xp: "100" });
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts storage.type.ts
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts meta.definition.variable.ts variable.other.constant.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts keyword.operator.assignment.ts
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts variable.other.object.ts
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts punctuation.accessor.ts
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts entity.name.function.ts
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.definition.block.ts
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts punctuation.separator.key-value.ts
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts constant.numeric.decimal.ts
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.separator.comma.ts
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts punctuation.separator.key-value.ts
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts punctuation.definition.string.begin.ts
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts punctuation.definition.string.end.ts
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.definition.block.ts
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L if (!result.success) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript keyword.control.conditional.ts
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.brace.round.ts
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript keyword.operator.logical.ts
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript variable.other.object.ts
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.accessor.ts
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript variable.other.property.ts
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.brace.round.ts
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts punctuation.definition.block.ts
L   result.error; // ZodError instance
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts variable.other.object.ts
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts punctuation.accessor.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts variable.other.property.ts
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts punctuation.terminator.statement.ts
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts comment.line.double-slash.ts punctuation.definition.comment.ts
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts comment.line.double-slash.ts
L } else {
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts punctuation.definition.block.ts
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript keyword.control.conditional.ts
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts punctuation.definition.block.ts
L   result.data; // { username: string; xp: number }
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts variable.other.object.ts
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts punctuation.accessor.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts variable.other.property.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts punctuation.terminator.statement.ts
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts comment.line.double-slash.ts punctuation.definition.comment.ts
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts comment.line.double-slash.ts
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.block.ts punctuation.definition.block.ts
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L **Note** — If your schema uses certain asynchronous APIs like `async` [refinements](https://zod.dev/api#refinements) or [transforms](https://zod.dev/api#transforms), you'll need to use the `.safeParseAsync()` method instead.
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 6 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 8 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 64 text.html.markdown meta.paragraph.markdown
T 65 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 70 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 71 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 72 text.html.markdown meta.paragraph.markdown
T 73 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 84 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 85 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 86 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 117 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 118 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 122 text.html.markdown meta.paragraph.markdown
T 123 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 133 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 134 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 135 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 165 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 166 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 191 text.html.markdown meta.paragraph.markdown
T 192 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 209 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 210 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 226 text.html.markdown meta.paragraph.markdown
L 
L ```ts
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L const schema = z.string().refine(async (val) => val.length <= 8);
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts storage.type.ts
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts meta.definition.variable.ts variable.other.constant.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts keyword.operator.assignment.ts
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts variable.other.object.ts
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts punctuation.accessor.ts
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts entity.name.function.ts
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts punctuation.accessor.ts
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts entity.name.function.ts
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts storage.modifier.async.ts
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts meta.parameters.ts punctuation.definition.parameters.begin.ts
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts meta.parameters.ts variable.parameter.ts
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts meta.parameters.ts punctuation.definition.parameters.end.ts
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts storage.type.function.arrow.ts
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts variable.other.object.ts
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts punctuation.accessor.ts
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts support.variable.property.ts
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts keyword.operator.relational.ts
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts constant.numeric.decimal.ts
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L 
L await schema.safeParseAsync("hello");
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript keyword.control.flow.ts
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.function-call.ts variable.other.object.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.function-call.ts punctuation.accessor.ts
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.function-call.ts entity.name.function.ts
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.brace.round.ts
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript string.quoted.double.ts punctuation.definition.string.begin.ts
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript string.quoted.double.ts
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript string.quoted.double.ts punctuation.definition.string.end.ts
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.brace.round.ts
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L // => { success: true; data: "hello" }
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts punctuation.definition.comment.ts
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Inferring types
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 19 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Zod infers a static type from your schema definitions. You can extract this type with the `z.infer<>` utility and use it however you like.
T 90 text.html.markdown meta.paragraph.markdown
T 91 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 100 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 101 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 138 text.html.markdown meta.paragraph.markdown
L 
L ```ts
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L const Player = z.object({
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts storage.type.ts
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts meta.definition.variable.ts variable.other.constant.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts keyword.operator.assignment.ts
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts variable.other.object.ts
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts punctuation.accessor.ts
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts entity.name.function.ts
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.definition.block.ts
L   username: z.string(),
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts punctuation.separator.key-value.ts
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.function-call.ts variable.other.object.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.function-call.ts punctuation.accessor.ts
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.function-call.ts entity.name.function.ts
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.brace.round.ts
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.brace.round.ts
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.separator.comma.ts
L   xp: z.number(),
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts punctuation.separator.key-value.ts
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.function-call.ts variable.other.object.ts
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.function-call.ts punctuation.accessor.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.function-call.ts entity.name.function.ts
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.brace.round.ts
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.brace.round.ts
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.separator.comma.ts
L });
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.definition.block.ts
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L 
L // extract the inferred type
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts punctuation.definition.comment.ts
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts
L type Player = z.infer<typeof Player>;
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts storage.type.type.ts
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts entity.name.type.alias.ts
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts keyword.operator.assignment.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts entity.name.type.module.ts
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts punctuation.accessor.ts
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts entity.name.type.ts
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts meta.type.parameters.ts punctuation.definition.typeparameters.begin.ts
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts meta.type.parameters.ts keyword.operator.expression.typeof.ts
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts meta.type.parameters.ts
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts meta.type.parameters.ts variable.other.readwrite.ts
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts meta.type.parameters.ts punctuation.definition.typeparameters.end.ts
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L 
L // use it in your code
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts punctuation.definition.comment.ts
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts
L const player: Player = { username: "billie", xp: 100 };
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts storage.type.ts
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts meta.definition.variable.ts variable.other.constant.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts meta.type.annotation.ts keyword.operator.type.annotation.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts meta.type.annotation.ts
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts meta.type.annotation.ts entity.name.type.ts
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts meta.type.annotation.ts
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts keyword.operator.assignment.ts
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.definition.block.ts
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts punctuation.separator.key-value.ts
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts punctuation.definition.string.begin.ts
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts punctuation.definition.string.end.ts
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.separator.comma.ts
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts punctuation.separator.key-value.ts
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts constant.numeric.decimal.ts
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.definition.block.ts
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L In some cases, the input & output types of a schema can diverge. For instance, the `.transform()` API can convert the input from one type to another. In these cases, you can extract the input and output types independently:
T 25 text.html.markdown meta.paragraph.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.other.valid-ampersand.markdown
T 83 text.html.markdown meta.paragraph.markdown
T 84 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 96 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 97 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 223 text.html.markdown meta.paragraph.markdown
L 
L ```ts
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L const mySchema = z.string().transform((val) => val.length);
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts storage.type.ts
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts meta.definition.variable.ts variable.other.constant.ts
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts keyword.operator.assignment.ts
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts variable.other.object.ts
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts punctuation.accessor.ts
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts entity.name.function.ts
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts punctuation.accessor.ts
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.function-call.ts entity.name.function.ts
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts meta.parameters.ts punctuation.definition.parameters.begin.ts
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts meta.parameters.ts variable.parameter.ts
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts meta.parameters.ts punctuation.definition.parameters.end.ts
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts storage.type.function.arrow.ts
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.arrow.ts
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts variable.other.object.ts
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts punctuation.accessor.ts
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts support.variable.property.ts
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.brace.round.ts
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L 
L type MySchemaIn = z.input<typeof mySchema>;
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts storage.type.type.ts
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts entity.name.type.alias.ts
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts keyword.operator.assignment.ts
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts entity.name.type.module.ts
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts punctuation.accessor.ts
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts entity.name.type.ts
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts meta.type.parameters.ts punctuation.definition.typeparameters.begin.ts
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts meta.type.parameters.ts keyword.operator.expression.typeof.ts
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts meta.type.parameters.ts
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts meta.type.parameters.ts variable.other.readwrite.ts
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts meta.type.parameters.ts punctuation.definition.typeparameters.end.ts
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L // => string
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts punctuation.definition.comment.ts
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts
L 
L type MySchemaOut = z.output<typeof mySchema>; // equivalent to z.infer<typeof mySchema>
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts storage.type.type.ts
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts entity.name.type.alias.ts
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts keyword.operator.assignment.ts
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts entity.name.type.module.ts
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts punctuation.accessor.ts
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts entity.name.type.ts
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts meta.type.parameters.ts punctuation.definition.typeparameters.begin.ts
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts meta.type.parameters.ts keyword.operator.expression.typeof.ts
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts meta.type.parameters.ts
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts meta.type.parameters.ts variable.other.readwrite.ts
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.type.declaration.ts meta.type.parameters.ts punctuation.definition.typeparameters.end.ts
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts punctuation.definition.comment.ts
T 87 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts
L // number
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts punctuation.definition.comment.ts
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript comment.line.double-slash.ts
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
