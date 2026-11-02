L # Next.js Docker Example
T 1 text.html.markdown markup.heading.markdown heading.1.markdown punctuation.definition.heading.markdown
T 2 text.html.markdown markup.heading.markdown heading.1.markdown
T 24 text.html.markdown markup.heading.markdown heading.1.markdown entity.name.section.markdown
L 
L A production-ready example demonstrating how to Dockerize Next.js applications using **static export** mode. This example showcases two different approaches for serving static Next.js sites: **Nginx** and **serve** package.
T 85 text.html.markdown meta.paragraph.markdown
T 87 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 100 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 102 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 191 text.html.markdown meta.paragraph.markdown
T 193 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 198 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 200 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 205 text.html.markdown meta.paragraph.markdown
T 207 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 212 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 214 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 223 text.html.markdown meta.paragraph.markdown
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
L - Static export: Fully static HTML/CSS/JavaScript site
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 54 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Two serving options: Nginx (production-grade) and serve (simple Node.js server)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 81 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Security best practices (non-root user)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 41 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Slim/Alpine Linux base images for optimal compatibility and smaller size
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 74 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - BuildKit cache mounts for faster builds
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 41 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Production-ready configuration with optimized Nginx settings
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 62 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Docker Compose support for easy deployment
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 44 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
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
L - [Node.js 20+](https://nodejs.org/) (for local development)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 60 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ## Quick Start with Docker
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 26 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L ### Option 1: Using Nginx (Recommended for Production)
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 54 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L **Nginx** is ideal when you need:
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 7 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 33 text.html.markdown meta.paragraph.markdown
L 
L - Production-grade web server
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Maximum performance and efficiency
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Advanced caching and compression
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 34 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Smaller Docker images (~50MB)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 31 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Industry-standard web server
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 30 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L #### Using Docker Compose
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 25 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L ```bash
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L docker compose up nextjs-static-export --build
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L **Access:** [http://localhost:8080](http://localhost:8080)
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 11 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 12 text.html.markdown meta.paragraph.markdown
T 13 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 34 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 57 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 58 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L #### Using Docker Build
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 23 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L ```bash
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L docker build -t nextjs-static-export .
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L docker run -p 8080:8080 nextjs-static-export
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L **Access:** [http://localhost:8080](http://localhost:8080)
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 11 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 12 text.html.markdown meta.paragraph.markdown
T 13 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 34 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 57 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 58 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L ### Option 2: Using serve Package
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 33 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L **serve** is ideal when you need:
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 7 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 33 text.html.markdown meta.paragraph.markdown
L 
L - Simple Node.js-based static file server
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 41 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Quick development/testing deployments
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 39 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Familiar Node.js ecosystem
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Easy customization
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L #### Using Docker Compose
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 25 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L ```bash
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L # OR run with serve npm package
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell punctuation.definition.comment.shell
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell
L docker compose up nextjs-static-export-with-serve --build
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L **Access:** [http://localhost:3000](http://localhost:3000)
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 11 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 12 text.html.markdown meta.paragraph.markdown
T 13 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 34 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 57 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 58 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L #### Using Docker Build
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 23 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L ```bash
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L docker build -t nextjs-static-export-serve -f Dockerfile.serve .
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L docker run -p 3000:3000 nextjs-static-export-serve
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L **Access:** [http://localhost:3000](http://localhost:3000)
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 11 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 12 text.html.markdown meta.paragraph.markdown
T 13 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 34 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 57 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 58 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
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
L │   └── next.svg           # Next.js logo
T 49 text.html.markdown markup.fenced_code.block.markdown
L ├── Dockerfile              # Nginx-based Dockerfile (port 8080)
T 70 text.html.markdown markup.fenced_code.block.markdown
L ├── Dockerfile.serve        # serve-based Dockerfile (port 3000)
T 70 text.html.markdown markup.fenced_code.block.markdown
L ├── compose.yml             # Docker Compose with both services
T 69 text.html.markdown markup.fenced_code.block.markdown
L ├── nginx.conf              # Nginx configuration for static export
T 73 text.html.markdown markup.fenced_code.block.markdown
L ├── next.config.ts          # Next.js configuration (static export mode)
T 78 text.html.markdown markup.fenced_code.block.markdown
L ├── postcss.config.js       # PostCSS configuration for Tailwind CSS
T 74 text.html.markdown markup.fenced_code.block.markdown
L ├── tsconfig.json           # TypeScript configuration
T 60 text.html.markdown markup.fenced_code.block.markdown
L ├── package.json            # Dependencies and scripts
T 60 text.html.markdown markup.fenced_code.block.markdown
L ├── pnpm-workspace.yaml     # pnpm dependency build approvals
T 67 text.html.markdown markup.fenced_code.block.markdown
L └── README.md              # This file
T 44 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Configuration
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 16 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L ### Next.js Static Export Mode
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 30 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L The `next.config.ts` file is configured with `output: "export"`:
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 20 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 45 text.html.markdown meta.paragraph.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 62 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 63 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 64 text.html.markdown meta.paragraph.markdown
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
L   output: "export",
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts punctuation.separator.key-value.ts
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts punctuation.definition.string.begin.ts
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts string.quoted.double.ts punctuation.definition.string.end.ts
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.separator.comma.ts
L   images: {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts punctuation.separator.key-value.ts
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.objectliteral.ts punctuation.definition.block.ts
L     unoptimized: true, // Required for static export
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.objectliteral.ts
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.objectliteral.ts meta.object.member.ts meta.object-literal.key.ts punctuation.separator.key-value.ts
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.objectliteral.ts meta.object.member.ts
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.objectliteral.ts meta.object.member.ts constant.language.boolean.true.ts
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.objectliteral.ts punctuation.separator.comma.ts
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.objectliteral.ts
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.objectliteral.ts comment.line.double-slash.ts punctuation.definition.comment.ts
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.objectliteral.ts comment.line.double-slash.ts
L   },
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.objectliteral.ts
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts meta.object.member.ts meta.objectliteral.ts punctuation.definition.block.ts
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.typescript meta.var.expr.ts meta.objectliteral.ts punctuation.separator.comma.ts
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
L **Static export mode** generates a fully static HTML/CSS/JavaScript site that can be served by any static hosting service or web server. When enabled, Next.js generates an `out` directory containing all static files. This results in:
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 20 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 22 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 172 text.html.markdown meta.paragraph.markdown
T 173 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 176 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 177 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 233 text.html.markdown meta.paragraph.markdown
L 
L - Smaller Docker image (~80MB with Nginx, ~350MB with serve)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 60 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Faster deployments (no Node.js runtime needed for Nginx)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 58 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Better CDN compatibility (pure static files)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 46 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Lower resource usage (Nginx uses minimal memory)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 50 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L **Note:** Static export has limitations:
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 7 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 40 text.html.markdown meta.paragraph.markdown
L 
L - No server-side rendering (SSR)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 32 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - No API routes
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Images must be unoptimized or use external image optimization
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 63 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L Learn more about [Next.js static export](https://nextjs.org/docs/app/api-reference/next-config-js/output#static-export) in the official documentation.
T 17 text.html.markdown meta.paragraph.markdown
T 18 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 39 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 40 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 41 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 118 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 119 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 150 text.html.markdown meta.paragraph.markdown
L 
L ### Dockerfile Highlights
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 25 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L #### Nginx-based (`Dockerfile`)
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 18 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
T 19 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 29 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown markup.inline.raw.string.markdown
T 30 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 31 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
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
L - **Nginx server**: Uses `nginxinc/nginx-unprivileged:alpine3.22` for serving static files (~50MB final image)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 25 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 26 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 64 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 65 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 110 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **BuildKit cache mounts**: Speeds up builds by caching package manager stores and Next.js build cache
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 25 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 103 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **Non-root user**: Runs as `nginx` user for security
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 30 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 54 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **Production Nginx config**: Optimized with gzip compression, caching headers, and security best practices
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 108 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **Port**: 8080
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L #### serve-based (`Dockerfile.serve`)
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 18 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
T 19 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown markup.inline.raw.string.markdown
T 36 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 37 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
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
L - **Node.js runtime**: Uses `node:24.13.0-slim` for running serve package
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 46 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 47 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 73 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **serve package**: Uses `serve@14.2.5` for serving static files
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 26 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 39 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 40 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 65 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **BuildKit cache mounts**: Speeds up builds by caching package manager stores and Next.js build cache
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 25 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 103 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
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
L - **SPA mode**: Configured with single-page application support
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 63 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **Port**: 3000
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L **Node.js version maintenance**: Uses Node.js 24.13.0-slim (latest LTS at time of writing). Update the `NODE_VERSION` ARG to the latest LTS version for security updates.
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 29 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 31 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 103 text.html.markdown meta.paragraph.markdown
T 104 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 116 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 117 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 169 text.html.markdown meta.paragraph.markdown
L 
L **Nginx image maintenance**: Uses `nginxinc/nginx-unprivileged:alpine3.22`. Update the `NGINXINC_IMAGE_TAG` ARG to the latest version for security updates.
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 25 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 27 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 34 text.html.markdown meta.paragraph.markdown
T 35 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 73 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 74 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 87 text.html.markdown meta.paragraph.markdown
T 88 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 106 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 107 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 155 text.html.markdown meta.paragraph.markdown
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
L > **Version Maintenance**:
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 4 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 23 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.bold.markdown
T 25 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 26 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L >
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
L > - **Node.js**: This Dockerfile uses Node.js 24.13.0-slim, which was the latest LTS version at the time of writing. To ensure security and stay up-to-date, regularly check and update the `NODE_VERSION` ARG in the Dockerfile to the latest Node.js LTS version. Check the latest version at [Node.js official website](https://nodejs.org/) and browse available Node.js images on [Docker Hub](https://hub.docker.com/_/node).
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 4 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown
T 6 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 13 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 15 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 188 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 189 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 201 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 202 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 288 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 289 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 313 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 314 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 315 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 334 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 335 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 375 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 376 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 386 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 387 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 388 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 417 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 418 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 419 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L > - **Nginx**: The Nginx Dockerfile uses `nginxinc/nginx-unprivileged:alpine3.22`. Regularly check and update the `NGINXINC_IMAGE_TAG` ARG to the latest version. Browse available Nginx images on [Docker Hub](https://hub.docker.com/r/nginxinc/nginx-unprivileged).
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 4 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown
T 6 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 11 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 13 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 41 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 42 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 80 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 81 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 114 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 115 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 133 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 134 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 195 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 196 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 206 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 207 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 208 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 260 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 261 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 262 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L > - **serve package**: The serve Dockerfile uses `serve@14.2.5`. Update to the latest version as needed for bug fixes and features.
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 4 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown
T 6 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 19 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 21 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 49 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 50 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 62 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 63 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 131 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ### Package Manager Support
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 27 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Both Dockerfiles support multiple package managers:
T 51 text.html.markdown meta.paragraph.markdown
L 
L - **npm** (via `package-lock.json`)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 7 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 33 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 34 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **yarn** (via `yarn.lock`)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 26 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - **pnpm** (via `pnpm-lock.yaml`)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 31 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 32 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 33 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L The Dockerfiles automatically detect which lockfile is present and use the appropriate package manager.
T 103 text.html.markdown meta.paragraph.markdown
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
L With `output: "export"` there is **no server at run time**: Nginx (or `serve`) only hands out the prebuilt files in `out/`. `docker run -e …` therefore has no effect on the application. Values used by the exported application must be present while `next build` runs. Any value inlined or rendered into `out/` is served to every visitor.
T 5 text.html.markdown meta.paragraph.markdown
T 6 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 22 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 23 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 33 text.html.markdown meta.paragraph.markdown
T 35 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 56 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 58 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 70 text.html.markdown meta.paragraph.markdown
T 71 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 76 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 77 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 116 text.html.markdown meta.paragraph.markdown
T 117 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 121 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 122 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 124 text.html.markdown meta.paragraph.markdown
T 125 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 142 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 143 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 250 text.html.markdown meta.paragraph.markdown
T 251 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 261 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 262 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 304 text.html.markdown meta.paragraph.markdown
T 305 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 309 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 310 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 338 text.html.markdown meta.paragraph.markdown
L 
L ### Non-secret configuration: use `.env.production`
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 34 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
T 35 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 50 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown
T 51 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L `.env.production` is intentionally **not** ignored, so `next build` picks it up:
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown meta.paragraph.markdown
T 37 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 40 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 42 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 55 text.html.markdown meta.paragraph.markdown
T 56 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 66 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 67 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 80 text.html.markdown meta.paragraph.markdown
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
L ### Per-environment values: use a build argument
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 48 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L To build the same source for several environments, pass the value into the build:
T 81 text.html.markdown meta.paragraph.markdown
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
L   -t nextjs-export-nginx .
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L > [!IMPORTANT]
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 13 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 14 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
L > A static export cannot keep a secret. `NEXT_PUBLIC_*` values are inlined verbatim into the exported JavaScript, and values without the prefix are read while pages are prerendered, so anything derived from them can end up in the exported HTML. Keep API keys and other credentials out of the build entirely and call them from a separate backend.
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 40 text.html.markdown markup.quote.markdown meta.paragraph.markdown
T 41 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 54 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 55 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 345 text.html.markdown markup.quote.markdown meta.paragraph.markdown
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
L - Vercel (for static export)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ### Choosing Between Nginx and serve
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 36 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L **Use Nginx (`Dockerfile`)** when:
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 13 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 14 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 24 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown
T 25 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 26 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 28 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 34 text.html.markdown meta.paragraph.markdown
L 
L - Deploying to production
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 25 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Maximum performance is required
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 33 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - You need advanced caching and compression
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Image size is a concern (~50MB vs ~300MB)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - You want industry-standard web server
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 39 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L **Use serve (`Dockerfile.serve`)** when:
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 13 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 14 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 30 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown
T 31 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 32 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 34 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 40 text.html.markdown meta.paragraph.markdown
L 
L - Quick development/testing deployments
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 39 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - You prefer Node.js ecosystem
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 30 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - You need easy customization
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L - Image size is not a concern
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
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
