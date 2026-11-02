L # Next.js Docker Example - Standalone Mode
T 1 text.html.markdown markup.heading.markdown heading.1.markdown punctuation.definition.heading.markdown
T 2 text.html.markdown markup.heading.markdown heading.1.markdown
T 42 text.html.markdown markup.heading.markdown heading.1.markdown entity.name.section.markdown
L 
L A production-ready example demonstrating how to Dockerize Next.js applications using **standalone mode**. This example showcases best practices for containerizing Next.js apps with Docker.
T 85 text.html.markdown meta.paragraph.markdown
T 87 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 102 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 104 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 188 text.html.markdown meta.paragraph.markdown
L 
L ## Features
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 11 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L - Multi-stage Docker build for optimal image size
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 49 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Next.js standalone mode for minimal production builds
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 55 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Security best practices (non-root user)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 41 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Slim Linux base image for optimal compatibility and smaller size
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 66 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - BuildKit cache mounts for faster builds
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 41 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Production-ready configuration
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 32 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ## Prerequisites
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 16 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L - [Docker](https://docs.docker.com/get-docker/)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 46 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 47 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L - [Node.js 20+](https://nodejs.org/)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L ## Quick Start with Docker
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 26 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L ### Using Docker Compose
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 24 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L The `compose.yml` includes both Node.js and Bun configurations. Run one service at a time to avoid port conflicts.
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 114 text.html.markdown meta.paragraph.markdown
L 
L **Node.js:**
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 10 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 12 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
L 
L ```bash
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L # Run with Node.js
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell punctuation.definition.comment.shell
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell
L docker compose up nextjs-standalone --build
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L **Bun:**
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 6 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 8 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
L 
L ```bash
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L # OR run with Bun
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell punctuation.definition.comment.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell
L docker compose up nextjs-standalone-with-bun --build
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L **Stop the application:**
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 23 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 25 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
L 
L ```bash
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L docker compose down
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Using Docker Build
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 22 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L **Node.js:**
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 10 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 12 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
L 
L ```bash
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L # Build the image
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell punctuation.definition.comment.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell
L docker build -t nextjs-standalone-image .
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L 
L # Run the container
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell punctuation.definition.comment.shell
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell
L docker run -p 3000:3000 nextjs-standalone-image
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L **Bun:**
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 6 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 8 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
L 
L ```bash
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L # Build the image
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell punctuation.definition.comment.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell
L docker build -f Dockerfile.bun -t nextjs-standalone-bun-image .
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L 
L # Run the container
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell punctuation.definition.comment.shell
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell
L docker run -p 3000:3000 nextjs-standalone-bun-image
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L **Open your browser:** Navigate to [http://localhost:3000](http://localhost:3000)
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 20 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 22 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 35 text.html.markdown meta.paragraph.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 57 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 58 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 59 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 80 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 81 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L ### In existing projects
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 24 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L To add Docker support to your existing Next.js project:
T 55 text.html.markdown meta.paragraph.markdown
L 
L 1. Copy the [`Dockerfile`](https://github.com/vercel/next.js/blob/canary/examples/with-docker/Dockerfile) (or [`Dockerfile.bun`](https://github.com/vercel/next.js/blob/canary/examples/with-docker/Dockerfile.bun) for Bun) to your project root.
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 12 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 13 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 14 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 24 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 25 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 26 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 27 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 104 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 105 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 110 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 111 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 112 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 126 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 127 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 128 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 129 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 210 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 211 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 242 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 2. Copy the [`.dockerignore`](https://github.com/vercel/next.js/blob/canary/examples/with-docker/.dockerignore) to your project root.
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 12 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 13 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 14 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 27 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 28 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 29 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 30 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 110 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 111 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 133 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 3. Add the following to your `next.config.js` (or `next.config.ts`):
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 29 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 30 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 44 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 45 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 50 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 51 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 65 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 66 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 68 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 
L ```js
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L // next.config.js
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.javascript comment.line.double-slash.js punctuation.definition.comment.js
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.javascript comment.line.double-slash.js
L module.exports = {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.javascript support.type.object.module.js
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.javascript punctuation.accessor.js
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.javascript support.type.object.module.js
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.javascript
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.javascript keyword.operator.assignment.js
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.javascript meta.objectliteral.js
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.javascript meta.objectliteral.js punctuation.definition.block.js
L   output: "standalone",
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.javascript meta.objectliteral.js
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.javascript meta.objectliteral.js meta.object.member.js meta.object-literal.key.js
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.javascript meta.objectliteral.js meta.object.member.js meta.object-literal.key.js punctuation.separator.key-value.js
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.javascript meta.objectliteral.js meta.object.member.js
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.javascript meta.objectliteral.js meta.object.member.js string.quoted.double.js punctuation.definition.string.begin.js
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.javascript meta.objectliteral.js meta.object.member.js string.quoted.double.js
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.javascript meta.objectliteral.js meta.object.member.js string.quoted.double.js punctuation.definition.string.end.js
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.javascript meta.objectliteral.js punctuation.separator.comma.js
L };
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.javascript meta.objectliteral.js punctuation.definition.block.js
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.javascript punctuation.terminator.statement.js
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L This will build the project as a standalone app inside the Docker image.
T 72 text.html.markdown meta.paragraph.markdown
L 
L ## Project Structure
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 20 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L nextjs-docker/
T 14 text.html.markdown markup.fenced_code.block.markdown
L ├── app/                    # Next.js App Router directory
T 64 text.html.markdown markup.fenced_code.block.markdown
L │   ├── layout.tsx          # Root layout with metadata
T 63 text.html.markdown markup.fenced_code.block.markdown
L │   ├── page.tsx            # Home page with example content
T 68 text.html.markdown markup.fenced_code.block.markdown
L │   └── globals.css         # Global styles with Tailwind CSS v4
T 72 text.html.markdown markup.fenced_code.block.markdown
L ├── public/                 # Static assets
T 49 text.html.markdown markup.fenced_code.block.markdown
L │   └── next.svg            # Next.js logo
T 50 text.html.markdown markup.fenced_code.block.markdown
L ├── Dockerfile              # Multi-stage Docker configuration (Node.js)
T 78 text.html.markdown markup.fenced_code.block.markdown
L ├── Dockerfile.bun          # Multi-stage Docker configuration (Bun)
T 74 text.html.markdown markup.fenced_code.block.markdown
L ├── compose.yml             # Docker Compose configuration (Node.js & Bun services)
T 89 text.html.markdown markup.fenced_code.block.markdown
L ├── next.config.ts          # Next.js configuration (standalone mode)
T 75 text.html.markdown markup.fenced_code.block.markdown
L ├── postcss.config.js       # PostCSS configuration for Tailwind CSS
T 74 text.html.markdown markup.fenced_code.block.markdown
L ├── tsconfig.json           # TypeScript configuration
T 60 text.html.markdown markup.fenced_code.block.markdown
L ├── package.json            # Dependencies and scripts
T 60 text.html.markdown markup.fenced_code.block.markdown
L ├── pnpm-workspace.yaml     # pnpm dependency build approvals
T 67 text.html.markdown markup.fenced_code.block.markdown
L └── README.md               # This file
T 45 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Configuration
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 16 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L ### Next.js Standalone Mode
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 27 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L The `next.config.ts` file is configured with `output: "standalone"`:
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 20 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 45 text.html.markdown meta.paragraph.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 66 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 67 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 68 text.html.markdown meta.paragraph.markdown
L 
L ```typescript
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 13 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L import type { NextConfig } from "next";
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts keyword.control.import.ts
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts keyword.control.type.ts
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts meta.block.ts punctuation.definition.block.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts meta.block.ts
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts meta.block.ts variable.other.readwrite.alias.ts
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts meta.block.ts
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts meta.block.ts punctuation.definition.block.ts
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts keyword.control.from.ts
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts string.quoted.double.ts punctuation.definition.string.begin.ts
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts string.quoted.double.ts
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.import.ts string.quoted.double.ts punctuation.definition.string.end.ts
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L 
L const nextConfig: NextConfig = {
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts storage.type.ts
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts meta.definition.variable.ts variable.other.constant.ts
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts meta.type.annotation.ts keyword.operator.type.annotation.ts
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts meta.type.annotation.ts
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts meta.type.annotation.ts entity.name.type.ts
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.var-single-variable.expr.ts meta.type.annotation.ts
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts keyword.operator.assignment.ts
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.definition.block.ts
L   output: "standalone",
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts punctuation.separator.key-value.ts
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts punctuation.definition.string.begin.ts
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts punctuation.definition.string.end.ts
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.separator.comma.ts
L };
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.definition.block.ts
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L 
L export default nextConfig;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.export.default.ts keyword.control.export.ts
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.export.default.ts
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.export.default.ts keyword.control.default.ts
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.export.default.ts
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.export.default.ts variable.other.readwrite.ts
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript punctuation.terminator.statement.ts
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L The standalone output mode creates a minimal, self-contained production build optimized for containerized deployments. When enabled, Next.js generates a `.next/standalone` directory containing only the essential files needed to run your application, excluding unnecessary dependencies and files. This results in significantly smaller Docker images and faster container startup times.
T 153 text.html.markdown meta.paragraph.markdown
T 154 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 170 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 171 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 383 text.html.markdown meta.paragraph.markdown
L 
L Learn more about [Next.js standalone output](https://nextjs.org/docs/pages/api-reference/next-config-js/output#standalone) in the official documentation.
T 17 text.html.markdown meta.paragraph.markdown
T 18 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 43 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 44 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 45 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 121 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 122 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 153 text.html.markdown meta.paragraph.markdown
L 
L ### Dockerfile Highlights (Node.js)
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 35 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L - **Multi-stage build**: Separates dependency installation (`dependencies`), build (`builder`), and runtime (`runner`) stages
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 60 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 61 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 73 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 74 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 84 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 85 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 92 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 93 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 109 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 110 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 116 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 117 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 125 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **Slim Linux**: Uses `slim` image tag for optimal compatibility and smaller image size
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 88 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **BuildKit cache mounts**: Speeds up builds by caching package manager stores (`/root/.npm`, `/usr/local/share/.cache/yarn`, `/root/.local/share/pnpm/store`). See the Dockerfile for an optional `.next/cache` mount to speed up rebuilds.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 25 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 81 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 82 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 92 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 93 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 95 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 96 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 124 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 125 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 127 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 128 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 157 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 158 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 196 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 197 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 208 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 209 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 237 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **Non-root user**: Runs as `node` user for security
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 30 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 34 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 53 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **Optimized layers**: Leverages Docker layer caching effectively
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 66 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **Standalone output**: Copies only the necessary files from `.next/standalone` and `.next/static`
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 62 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 63 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 79 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 80 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 85 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 86 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 98 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 99 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L - **Writable `.next` directory**: The `.next` directory is created and owned by the `node` user so the server can write prerender cache and optimized images at runtime
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 13 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 30 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 32 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 38 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 39 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 44 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 45 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 84 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 85 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 89 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 90 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 167 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **Node.js version maintenance**: Uses Node.js 24.13.0-slim (latest LTS at time of writing). Update the `NODE_VERSION` ARG to the latest LTS version for security updates.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 31 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 33 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 105 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 106 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 118 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 119 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 171 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ### Dockerfile.bun Highlights (Bun)
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 35 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L - **Multi-stage build**: Same three-stage pattern optimized for Bun
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 67 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **Official Bun image**: Uses `oven/bun:1` for optimal Bun performance
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 31 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 32 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 71 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **Non-root user**: Runs as built-in `bun` user for security
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 38 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 39 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **Frozen lockfile**: Uses `bun.lock` for reproducible builds
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 37 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 38 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 62 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **Standalone output**: Same optimized output as the Node.js version, with writable `.next` directory for runtime cache
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 85 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 86 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 91 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 92 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 120 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L **Why Node.js slim image tag?**: The slim variant provides optimal compatibility with npm packages and native dependencies while maintaining a smaller image size (~226MB). Slim uses glibc (standard Linux), ensuring better compatibility than Alpine's musl libc, which can cause issues with some npm packages. This makes it ideal for public examples where reliability and compatibility are priorities.
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 29 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 31 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 399 text.html.markdown meta.paragraph.markdown
L 
L **When to use Alpine?**: Consider using `node:24.11.1-alpine` instead if:
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 21 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 23 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 40 text.html.markdown meta.paragraph.markdown
T 41 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 60 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 61 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 73 text.html.markdown meta.paragraph.markdown
L 
L - **Image size is critical**: Alpine images are typically ~100MB smaller than slim variants (~110MB base vs ~226MB)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 26 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 115 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **Your dependencies are compatible**: Your npm packages don't require native binaries that depend on glibc
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 38 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 108 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **You've tested thoroughly**: You've verified all your dependencies work correctly with musl libc
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 30 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 99 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **Security-focused deployments**: Alpine's minimal attack surface can be beneficial for security-sensitive applications
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 32 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 34 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 121 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L To switch to Alpine, simply change the `NODE_VERSION` ARG in the Dockerfile to `24.11.1-alpine`.
T 39 text.html.markdown meta.paragraph.markdown
T 40 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 52 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 53 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 79 text.html.markdown meta.paragraph.markdown
T 80 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 94 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 95 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 96 text.html.markdown meta.paragraph.markdown
L 
L > [!IMPORTANT]
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 13 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 14 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
L > **Node.js Version Maintenance**: This Dockerfile uses Node.js 24.13.0-slim, which was the latest LTS version at the time of writing. To ensure security and stay up-to-date, regularly check and update the `NODE_VERSION` ARG in the Dockerfile to the latest Node.js LTS version. Check the latest version at [Nodejs official website](https://nodejs.org/) and browse available Node.js images on [Docker Hub](https://hub.docker.com/_/node).
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 4 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 31 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.bold.markdown
T 33 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 206 text.html.markdown markup.quote.markdown meta.paragraph.markdown
T 207 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 219 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 220 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 306 text.html.markdown markup.quote.markdown meta.paragraph.markdown
T 307 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 330 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 331 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 332 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 351 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 352 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 392 text.html.markdown markup.quote.markdown meta.paragraph.markdown
T 393 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 403 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 404 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 405 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 434 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 435 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 436 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L 
L ## Environment Variables
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 24 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L The [`.dockerignore`](./.dockerignore) in this example **excludes `.env`**, so a local development file — which usually holds real credentials — is never copied into the build context or the final image.
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 6 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 38 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 55 text.html.markdown meta.paragraph.markdown
T 57 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 66 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 67 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 71 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown
T 72 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 74 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 207 text.html.markdown meta.paragraph.markdown
L 
L The consequence is worth knowing up front: a value you rely on from `.env` is `undefined` inside the container, even though the same code works with `next build && next start` locally. Use one of the following instead.
T 68 text.html.markdown meta.paragraph.markdown
T 69 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 73 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 74 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 78 text.html.markdown meta.paragraph.markdown
T 79 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 88 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 89 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 149 text.html.markdown meta.paragraph.markdown
T 150 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 174 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 175 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 218 text.html.markdown meta.paragraph.markdown
L 
L ### Secrets and server-only values: pass them at run time
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 57 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Values read on the server at request time (Route Handlers, dynamically rendered Server Components, Server Actions) are read from the environment when the request happens, so they need nothing at build time:
T 206 text.html.markdown meta.paragraph.markdown
L 
L ```bash
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L docker run -p 3000:3000 -e MY_SECRET=value nextjs-standalone-image
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L or in [`compose.yml`](./compose.yml):
T 6 text.html.markdown meta.paragraph.markdown
T 7 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 8 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 37 text.html.markdown meta.paragraph.markdown
L 
L ```yaml
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L services:
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
L   nextjs-standalone:
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
L     environment:
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
L       MY_SECRET: value
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml
L     # or, to read a file that is not committed:
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.whitespace.comment.leading.yaml
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml comment.line.number-sign.yaml punctuation.definition.comment.yaml
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml comment.line.number-sign.yaml
L     # env_file:
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.whitespace.comment.leading.yaml
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml comment.line.number-sign.yaml punctuation.definition.comment.yaml
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml comment.line.number-sign.yaml
L     #   - .env.production.local
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.whitespace.comment.leading.yaml
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml comment.line.number-sign.yaml punctuation.definition.comment.yaml
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml comment.line.number-sign.yaml
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Prefer this wherever it works: it keeps one image promotable across environments instead of baking values into a per-environment build.
T 135 text.html.markdown meta.paragraph.markdown
L 
L ### Non-secret build-time configuration: use `.env.production`
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 45 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
T 46 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown
T 62 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L `.env.production` is intentionally **not** ignored, so it is available to `next build` and loaded by the server at run time. Use it only for values that are safe to publish:
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown meta.paragraph.markdown
T 37 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 40 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 42 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 74 text.html.markdown meta.paragraph.markdown
T 75 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 85 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 86 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 173 text.html.markdown meta.paragraph.markdown
L 
L ```bash
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L # .env.production
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell punctuation.definition.comment.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell
L NEXT_PUBLIC_SITE_URL=https://example.com
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell variable.other.assignment.shell
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell keyword.operator.assignment.shell
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Public values needed in the client bundle: use a build argument
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 67 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L `NEXT_PUBLIC_*` values referenced from Client Components are inlined into the JavaScript sent to the browser, so they have to be present while `next build` runs. Add them to the builder stage:
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 14 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 15 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 143 text.html.markdown meta.paragraph.markdown
T 144 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 154 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 155 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 192 text.html.markdown meta.paragraph.markdown
L 
L ```dockerfile
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 13 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L ARG NEXT_PUBLIC_SITE_URL
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dockerfile keyword.other.special-method.dockerfile
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dockerfile
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dockerfile
L ENV NEXT_PUBLIC_SITE_URL=$NEXT_PUBLIC_SITE_URL
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dockerfile keyword.other.special-method.dockerfile
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dockerfile
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dockerfile
L # ... before the build step
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dockerfile comment.line.number-sign.dockerfile punctuation.definition.comment.dockerfile
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dockerfile comment.line.number-sign.dockerfile
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ```bash
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L docker build \
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell constant.character.escape.line-continuation.shell
L   --build-arg NEXT_PUBLIC_SITE_URL=https://example.com \
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell
L   -t nextjs-standalone-image .
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L > [!IMPORTANT]
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 13 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 14 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
L > Never pass secrets as build arguments. They are recoverable from the image history, and `NEXT_PUBLIC_*` values are sent to the browser by definition.
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 90 text.html.markdown markup.quote.markdown meta.paragraph.markdown
T 91 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 104 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 105 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 151 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L 
L > [!IMPORTANT]
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 13 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 14 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
L > Any env file that **is** present in the build context is also copied into `.next/standalone` by `output: "standalone"`, and this Dockerfile copies that directory wholesale into the runner stage. The file therefore ships inside the image and is readable by anyone who can pull it. Keep credentials out of committed env files and pass them at run time.
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 20 text.html.markdown markup.quote.markdown meta.paragraph.markdown
T 22 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 24 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.bold.markdown
T 26 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 76 text.html.markdown markup.quote.markdown meta.paragraph.markdown
T 77 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 93 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 94 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 98 text.html.markdown markup.quote.markdown meta.paragraph.markdown
T 99 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 119 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 120 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 352 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L 
L To build a separate image per environment instead, see [`with-docker-multi-env`](../with-docker-multi-env).
T 55 text.html.markdown meta.paragraph.markdown
T 56 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 57 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 78 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 79 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 80 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 81 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 105 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 106 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 107 text.html.markdown meta.paragraph.markdown
L 
L ## Deployment
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 13 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L This example can be deployed to any container-based platform:
T 61 text.html.markdown meta.paragraph.markdown
L 
L - Google Cloud Run
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - AWS ECS/Fargate
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Azure Container Instances
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - DigitalOcean App Platform
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Any Kubernetes cluster
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ### Deploying to Google Cloud Run
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 33 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L 1. Install the [Google Cloud SDK](https://cloud.google.com/sdk/docs/install) so you can use `gcloud` on the command line.
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 15 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 16 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 32 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 33 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 34 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 75 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 76 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 92 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 93 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 99 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 100 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 121 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 2. Run `gcloud auth login` to log in to your account.
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 7 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 8 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 25 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 26 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 53 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 3. [Create a new project](https://cloud.google.com/run/docs/quickstarts/build-and-deploy) in Google Cloud Run (e.g. `nextjs-docker`). Ensure billing is turned on.
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 4 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 24 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 25 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 26 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 88 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 89 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 116 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 117 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 130 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 131 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 162 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 4. Build your container image using Cloud Build:
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 48 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L    ```bash
T 3 text.html.markdown markup.list.numbered.markdown
T 6 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 10 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L    gcloud builds submit --tag gcr.io/PROJECT-ID/nextjs-docker --project PROJECT-ID
T 3 text.html.markdown markup.list.numbered.markdown
T 82 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript
L    ```
T 3 text.html.markdown markup.list.numbered.markdown
T 6 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L    This will also enable Cloud Build for your project.
T 3 text.html.markdown markup.list.numbered.markdown
T 54 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 5. Deploy to Cloud Run:
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 23 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 
L    ```bash
T 3 text.html.markdown markup.list.numbered.markdown
T 6 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 10 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L    gcloud run deploy --image gcr.io/PROJECT-ID/nextjs-docker --project PROJECT-ID --platform managed --allow-unauthenticated
T 3 text.html.markdown markup.list.numbered.markdown
T 124 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript
L    ```
T 3 text.html.markdown markup.list.numbered.markdown
T 6 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L    - You will be prompted for the service name: press Enter to accept the default name, `nextjs-docker`.
T 3 text.html.markdown markup.list.numbered.markdown
T 4 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 5 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown
T 88 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 89 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 102 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 103 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 104 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L    - You will be prompted for [region](https://cloud.google.com/run/docs/quickstarts/build-and-deploy#follow-cloud-run): select the region of your choice, for example `us-central1`.
T 3 text.html.markdown markup.list.numbered.markdown
T 4 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 5 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown
T 30 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 31 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 37 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 38 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 39 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 118 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 119 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 167 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 168 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 179 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 180 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 181 text.html.markdown markup.list.numbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ## Learn More
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 13 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L - [Next.js Documentation](https://nextjs.org/docs) - Comprehensive Next.js documentation
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 25 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 26 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 49 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 50 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 88 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - [Next.js Templates](https://vercel.com/templates?framework=next.js) - Browse and deploy Next.js templates
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 68 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 69 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 107 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - [Next.js Examples](https://github.com/vercel/next.js/tree/canary/examples) - Discover boilerplate example projects
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 75 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 76 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 116 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - [Deploy to Vercel](https://vercel.com/new) - Instantly deploy your Next.js site
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 44 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 81 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - [Learn Docker](https://docs.docker.com/get-started/) - Get started with Docker fundamentals, containerization, and deployment
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 53 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 54 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 127 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - [Docker Documentation](https://docs.docker.com/) - Comprehensive Docker documentation and reference guides
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 25 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 49 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 50 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 108 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - [React.js Docker Guide](https://docs.docker.com/language/nodejs/) - Official Docker guide for React.js applications following best practices for containerization
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 25 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 26 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 66 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 67 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 163 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
