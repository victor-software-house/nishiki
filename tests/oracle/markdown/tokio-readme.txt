L # Tokio
T 1 text.html.markdown markup.heading.markdown heading.1.markdown punctuation.definition.heading.markdown
T 2 text.html.markdown markup.heading.markdown heading.1.markdown
T 7 text.html.markdown markup.heading.markdown heading.1.markdown entity.name.section.markdown
L 
L A runtime for writing reliable, asynchronous, and slim applications with
T 72 text.html.markdown meta.paragraph.markdown
L the Rust programming language. It is:
T 37 text.html.markdown meta.paragraph.markdown
L 
L * **Fast**: Tokio's zero-cost abstractions give you bare-metal
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 62 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   performance.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * **Reliable**: Tokio leverages Rust's ownership, type system, and
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 66 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   concurrency model to reduce bugs and ensure thread safety.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 60 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * **Scalable**: Tokio has a minimal footprint, and handles backpressure
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 71 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   and cancellation naturally.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L [![Crates.io][crates-badge]][crates-url]
T 1 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 39 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 40 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
L [![MIT licensed][mit-badge]][mit-url]
T 1 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
L [![Build Status][actions-badge]][actions-url]
T 1 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 31 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 32 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 33 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 44 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 45 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
L [![Discord chat][discord-badge]][discord-url]
T 1 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 31 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 32 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 33 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 44 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 45 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
L 
L [crates-badge]: https://img.shields.io/crates/v/tokio.svg
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 13 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 14 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 15 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 16 text.html.markdown meta.link.reference.def.markdown
T 57 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [crates-url]: https://crates.io/crates/tokio
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 11 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 12 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 13 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 14 text.html.markdown meta.link.reference.def.markdown
T 44 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 10 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 11 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 12 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 13 text.html.markdown meta.link.reference.def.markdown
T 62 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [mit-url]: https://github.com/tokio-rs/tokio/blob/master/LICENSE
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 8 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 9 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 10 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 11 text.html.markdown meta.link.reference.def.markdown
T 64 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [actions-badge]: https://github.com/tokio-rs/tokio/workflows/CI/badge.svg
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 14 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 15 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 16 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 17 text.html.markdown meta.link.reference.def.markdown
T 73 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [actions-url]: https://github.com/tokio-rs/tokio/actions?query=workflow%3ACI+branch%3Amaster
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 12 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 13 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 14 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 15 text.html.markdown meta.link.reference.def.markdown
T 92 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [discord-badge]: https://img.shields.io/discord/500028886025895936.svg?logo=discord&style=flat-square
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 14 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 15 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 16 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 17 text.html.markdown meta.link.reference.def.markdown
T 101 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [discord-url]: https://discord.gg/tokio
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 12 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 13 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 14 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 15 text.html.markdown meta.link.reference.def.markdown
T 39 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L 
L [Website](https://tokio.rs) |
T 1 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 8 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 9 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 29 text.html.markdown meta.paragraph.markdown
L [Guides](https://tokio.rs/tokio/tutorial) |
T 1 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 7 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 8 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 9 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 40 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 41 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 43 text.html.markdown meta.paragraph.markdown
L [API Docs](https://docs.rs/tokio/latest/tokio) |
T 1 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 9 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 11 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 45 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 46 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 48 text.html.markdown meta.paragraph.markdown
L [Chat](https://discord.gg/tokio)
T 1 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 6 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 7 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 31 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 32 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L ## Overview
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 11 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Tokio is an event-driven, non-blocking I/O platform for writing
T 63 text.html.markdown meta.paragraph.markdown
L asynchronous applications with the Rust programming language. At a high
T 71 text.html.markdown meta.paragraph.markdown
L level, it provides a few major components:
T 42 text.html.markdown meta.paragraph.markdown
L 
L * A multithreaded, work-stealing based task [scheduler].
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 44 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 45 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 54 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 55 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 56 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * A reactor backed by the operating system's event queue (epoll, kqueue,
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 72 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   IOCP, etc.).
T 2 text.html.markdown markup.list.unnumbered.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Asynchronous [TCP and UDP][net] sockets.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 32 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 33 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L These components provide the runtime components necessary for building
T 70 text.html.markdown meta.paragraph.markdown
L an asynchronous application.
T 28 text.html.markdown meta.paragraph.markdown
L 
L [net]: https://docs.rs/tokio/latest/tokio/net/index.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 4 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 5 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 6 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 7 text.html.markdown meta.link.reference.def.markdown
T 56 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [scheduler]: https://docs.rs/tokio/latest/tokio/runtime/index.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 10 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 11 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 12 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 13 text.html.markdown meta.link.reference.def.markdown
T 66 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L 
L ## Example
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 10 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L A basic TCP echo server with Tokio.
T 35 text.html.markdown meta.paragraph.markdown
L 
L Make sure you enable the full features of the tokio crate on Cargo.toml:
T 72 text.html.markdown meta.paragraph.markdown
L 
L ```toml
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language
L [dependencies]
T 14 text.html.markdown markup.fenced_code.block.markdown
L tokio = { version = "1.54.1", features = ["full"] }
T 51 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L Then, on your main.rs:
T 22 text.html.markdown meta.paragraph.markdown
L 
L ```rust,no_run
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
T 14 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.attributes.markdown
L use tokio::net::TcpListener;
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust keyword.other.rust
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust entity.name.namespace.rust
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust keyword.operator.namespace.rust
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust entity.name.namespace.rust
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust keyword.operator.namespace.rust
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust entity.name.type.rust
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust punctuation.semi.rust
L use tokio::io::{AsyncReadExt, AsyncWriteExt};
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust keyword.other.rust
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust entity.name.namespace.rust
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust keyword.operator.namespace.rust
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust entity.name.namespace.rust
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust keyword.operator.namespace.rust
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust punctuation.brackets.curly.rust
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust entity.name.type.rust
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust punctuation.comma.rust
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust entity.name.type.rust
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust punctuation.brackets.curly.rust
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.use.rust punctuation.semi.rust
L 
L #[tokio::main]
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.attribute.rust punctuation.definition.attribute.rust
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.attribute.rust punctuation.brackets.attribute.rust
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.attribute.rust
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.attribute.rust keyword.operator.namespace.rust
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.attribute.rust
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.attribute.rust punctuation.brackets.attribute.rust
L async fn main() -> Result<(), Box<dyn std::error::Error>> {
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.other.rust
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust keyword.other.fn.rust
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust entity.name.function.rust
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust punctuation.brackets.round.rust
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust punctuation.brackets.round.rust
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust keyword.operator.arrow.skinny.rust
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust entity.name.type.rust
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust punctuation.brackets.angle.rust
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust punctuation.brackets.round.rust
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust punctuation.brackets.round.rust
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust punctuation.comma.rust
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust entity.name.type.rust
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust punctuation.brackets.angle.rust
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust keyword.other.rust
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust variable.other.rust
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust keyword.operator.namespace.rust
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust variable.other.rust
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust keyword.operator.namespace.rust
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust entity.name.type.rust
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust punctuation.brackets.angle.rust
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust punctuation.brackets.angle.rust
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.definition.rust punctuation.brackets.curly.rust
L     let listener = TcpListener::bind("127.0.0.1:8080").await?;
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.other.rust storage.type.rust
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust variable.other.rust
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.operator.assignment.equal.rust
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust entity.name.type.rust
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.operator.namespace.rust
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust entity.name.function.rust
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust punctuation.brackets.round.rust
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust string.quoted.double.rust punctuation.definition.string.rust
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust string.quoted.double.rust
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust string.quoted.double.rust punctuation.definition.string.rust
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust punctuation.brackets.round.rust
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.operator.access.dot.rust
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.control.rust
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.operator.question.rust
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.semi.rust
L 
L     loop {
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.control.rust
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.curly.rust
L         let (mut socket, _) = listener.accept().await?;
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.other.rust storage.type.rust
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.round.rust
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust storage.modifier.mut.rust
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust variable.other.rust
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.comma.rust
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust variable.other.rust
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.round.rust
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.operator.assignment.equal.rust
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust variable.other.rust
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.operator.access.dot.rust
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust entity.name.function.rust
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust punctuation.brackets.round.rust
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust punctuation.brackets.round.rust
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.operator.access.dot.rust
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.control.rust
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.operator.question.rust
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.semi.rust
L 
L         tokio::spawn(async move {
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust entity.name.namespace.rust
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.operator.namespace.rust
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust entity.name.function.rust
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust punctuation.brackets.round.rust
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust keyword.other.rust
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust keyword.other.rust
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust punctuation.brackets.curly.rust
L             let mut buf = [0; 1024];
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust keyword.other.rust storage.type.rust
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust storage.modifier.mut.rust
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust variable.other.rust
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust keyword.operator.assignment.equal.rust
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust punctuation.brackets.square.rust
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust constant.numeric.decimal.rust
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust punctuation.semi.rust
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust constant.numeric.decimal.rust
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust punctuation.brackets.square.rust
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust punctuation.semi.rust
L 
L             // In a loop, read data from the socket and write the data back.
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust comment.line.double-slash.rust punctuation.definition.comment.rust
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust comment.line.double-slash.rust
L             loop {
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust keyword.control.rust
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust punctuation.brackets.curly.rust
L                 let n = match socket.read(&mut buf).await {
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust keyword.other.rust storage.type.rust
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust variable.other.rust
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust keyword.operator.assignment.equal.rust
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust keyword.control.rust
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust variable.other.rust
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust keyword.operator.access.dot.rust
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust meta.function.call.rust entity.name.function.rust
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust meta.function.call.rust punctuation.brackets.round.rust
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust meta.function.call.rust keyword.operator.borrow.and.rust
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust meta.function.call.rust storage.modifier.mut.rust
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust meta.function.call.rust
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust meta.function.call.rust variable.other.rust
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust meta.function.call.rust punctuation.brackets.round.rust
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust keyword.operator.access.dot.rust
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust keyword.control.rust
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust punctuation.brackets.curly.rust
L                     // socket closed
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust comment.line.double-slash.rust punctuation.definition.comment.rust
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust comment.line.double-slash.rust
L                     Ok(0) => return,
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust entity.name.type.result.rust
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust punctuation.brackets.round.rust
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust constant.numeric.decimal.rust
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust punctuation.brackets.round.rust
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.operator.arrow.fat.rust
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.control.rust
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.comma.rust
L                     Ok(n) => n,
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust entity.name.type.result.rust
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.round.rust
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust variable.other.rust
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.round.rust
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.operator.arrow.fat.rust
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust variable.other.rust
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.comma.rust
L                     Err(e) => {
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust entity.name.type.result.rust
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.round.rust
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust variable.other.rust
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.round.rust
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.operator.arrow.fat.rust
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.curly.rust
L                         eprintln!("failed to read from socket; err = {:?}", e);
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.macro.rust entity.name.function.macro.rust
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.round.rust
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust string.quoted.double.rust punctuation.definition.string.rust
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust string.quoted.double.rust
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust string.quoted.double.rust meta.interpolation.rust punctuation.definition.interpolation.rust
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust string.quoted.double.rust meta.interpolation.rust
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust string.quoted.double.rust meta.interpolation.rust punctuation.definition.interpolation.rust
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust string.quoted.double.rust punctuation.definition.string.rust
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.comma.rust
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 77 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust variable.other.rust
T 78 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.round.rust
T 79 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.semi.rust
L                         return;
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.control.rust
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.semi.rust
L                     }
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.curly.rust
L                 };
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.curly.rust
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.semi.rust
L 
L                 // Write the data back
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust comment.line.double-slash.rust punctuation.definition.comment.rust
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust comment.line.double-slash.rust
L                 if let Err(e) = socket.write_all(&buf[0..n]).await {
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.control.rust
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.other.rust storage.type.rust
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust entity.name.type.result.rust
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.round.rust
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust variable.other.rust
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.round.rust
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.operator.assignment.equal.rust
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust variable.other.rust
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.operator.access.dot.rust
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust entity.name.function.rust
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust punctuation.brackets.round.rust
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust keyword.operator.borrow.and.rust
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust variable.other.rust
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust punctuation.brackets.square.rust
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust constant.numeric.decimal.rust
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust keyword.operator.range.rust
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust variable.other.rust
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust punctuation.brackets.square.rust
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.function.call.rust punctuation.brackets.round.rust
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.operator.access.dot.rust
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.control.rust
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.curly.rust
L                     eprintln!("failed to write to socket; err = {:?}", e);
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust meta.macro.rust entity.name.function.macro.rust
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.round.rust
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust string.quoted.double.rust punctuation.definition.string.rust
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust string.quoted.double.rust
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust string.quoted.double.rust meta.interpolation.rust punctuation.definition.interpolation.rust
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust string.quoted.double.rust meta.interpolation.rust
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust string.quoted.double.rust meta.interpolation.rust punctuation.definition.interpolation.rust
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust string.quoted.double.rust punctuation.definition.string.rust
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.comma.rust
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust variable.other.rust
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.round.rust
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.semi.rust
L                     return;
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust keyword.control.rust
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.semi.rust
L                 }
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.curly.rust
L             }
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.curly.rust
L         });
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.curly.rust
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.round.rust
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.semi.rust
L     }
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.curly.rust
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.rust punctuation.brackets.curly.rust
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L More examples can be found [here][examples]. For a larger "real world" example, see the
T 27 text.html.markdown meta.paragraph.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 32 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 33 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 34 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 42 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 43 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 87 text.html.markdown meta.paragraph.markdown
L [mini-redis] repository.
T 1 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 11 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 12 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 24 text.html.markdown meta.paragraph.markdown
L 
L [examples]: https://github.com/tokio-rs/tokio/tree/master/examples
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 9 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 10 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 11 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 12 text.html.markdown meta.link.reference.def.markdown
T 66 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [mini-redis]: https://github.com/tokio-rs/mini-redis/
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 11 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 12 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 13 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 14 text.html.markdown meta.link.reference.def.markdown
T 53 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L 
L To see a list of the available feature flags that can be enabled, check our
T 75 text.html.markdown meta.paragraph.markdown
L [docs][feature-flag-docs].
T 1 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 6 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 7 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 24 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 26 text.html.markdown meta.paragraph.markdown
L 
L ## Getting Help
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 15 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L First, see if the answer to your question can be found in the [Guides] or the
T 62 text.html.markdown meta.paragraph.markdown
T 63 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 69 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 70 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 77 text.html.markdown meta.paragraph.markdown
L [API documentation]. If the answer is not there, there is an active community in
T 80 text.html.markdown meta.paragraph.markdown
L the [Tokio Discord server][chat]. We would be happy to try to answer your
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 31 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 32 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 73 text.html.markdown meta.paragraph.markdown
L question. You can also ask your question on [the discussions page][discussions].
T 44 text.html.markdown meta.paragraph.markdown
T 45 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 65 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 66 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 67 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 78 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 79 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 80 text.html.markdown meta.paragraph.markdown
L 
L [Guides]: https://tokio.rs/tokio/tutorial
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 7 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 8 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 9 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 10 text.html.markdown meta.link.reference.def.markdown
T 41 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [API documentation]: https://docs.rs/tokio/latest/tokio
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 18 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 19 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 20 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 21 text.html.markdown meta.link.reference.def.markdown
T 55 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [chat]: https://discord.gg/tokio
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 5 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 6 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 7 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 8 text.html.markdown meta.link.reference.def.markdown
T 32 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [discussions]: https://github.com/tokio-rs/tokio/discussions
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 12 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 13 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 14 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 15 text.html.markdown meta.link.reference.def.markdown
T 60 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [feature-flag-docs]: https://docs.rs/tokio/#feature-flags
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 18 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 19 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 20 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 21 text.html.markdown meta.link.reference.def.markdown
T 57 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L 
L ## Contributing
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 15 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L :balloon: Thanks for your help improving the project! We are so happy to have
T 77 text.html.markdown meta.paragraph.markdown
L you! We have a [contributing guide][guide] to help you get involved in the Tokio
T 15 text.html.markdown meta.paragraph.markdown
T 16 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 34 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 41 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 42 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 80 text.html.markdown meta.paragraph.markdown
L project.
T 8 text.html.markdown meta.paragraph.markdown
L 
L [guide]: https://github.com/tokio-rs/tokio/blob/master/docs/contributing/README.md
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 6 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 7 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 8 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 9 text.html.markdown meta.link.reference.def.markdown
T 82 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L 
L ## Related Projects
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 19 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L In addition to the crates in this repository, the Tokio project also maintains
T 78 text.html.markdown meta.paragraph.markdown
L several other libraries, including:
T 35 text.html.markdown meta.paragraph.markdown
L 
L * [`toasty`]: An async ORM for Rust supporting both SQL and NoSQL databases.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 76 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * [`axum`]: A web application framework that focuses on ergonomics and modularity.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 82 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * [`hyper`]: A fast and correct HTTP/1.1 and HTTP/2 implementation for Rust.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 76 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * [`tonic`]: A gRPC over HTTP/2 implementation focused on high performance, interoperability, and flexibility.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 110 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * [`warp`]: A super-easy, composable, web server framework for warp speeds.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 75 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * [`tower`]: A library of modular and reusable components for building robust networking clients and servers.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 109 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * [`tracing`] (formerly `tokio-trace`): A framework for application-level tracing and async-aware diagnostics.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 13 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 25 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 37 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 110 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * [`mio`]: A low-level, cross-platform abstraction over OS I/O APIs that powers `tokio`.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 80 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 81 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 86 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 87 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 88 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * [`bytes`]: Utilities for working with bytes, including efficient byte buffers.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 80 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * [`loom`]: A testing tool for concurrent Rust code.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 52 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L [`toasty`]: https://github.com/tokio-rs/toasty
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 9 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 10 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 11 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 12 text.html.markdown meta.link.reference.def.markdown
T 46 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [`axum`]: https://github.com/tokio-rs/axum
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 7 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 8 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 9 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 10 text.html.markdown meta.link.reference.def.markdown
T 42 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [`warp`]: https://github.com/seanmonstar/warp
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 7 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 8 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 9 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 10 text.html.markdown meta.link.reference.def.markdown
T 45 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [`hyper`]: https://github.com/hyperium/hyper
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 8 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 9 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 10 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 11 text.html.markdown meta.link.reference.def.markdown
T 44 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [`tonic`]: https://github.com/hyperium/tonic
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 8 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 9 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 10 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 11 text.html.markdown meta.link.reference.def.markdown
T 44 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [`tower`]: https://github.com/tower-rs/tower
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 8 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 9 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 10 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 11 text.html.markdown meta.link.reference.def.markdown
T 44 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [`loom`]: https://github.com/tokio-rs/loom
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 7 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 8 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 9 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 10 text.html.markdown meta.link.reference.def.markdown
T 42 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [`tracing`]: https://github.com/tokio-rs/tracing
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 10 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 11 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 12 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 13 text.html.markdown meta.link.reference.def.markdown
T 48 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [`mio`]: https://github.com/tokio-rs/mio
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 6 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 7 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 8 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 9 text.html.markdown meta.link.reference.def.markdown
T 40 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [`bytes`]: https://github.com/tokio-rs/bytes
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 8 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 9 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 10 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 11 text.html.markdown meta.link.reference.def.markdown
T 44 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L 
L ## Changelog
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 12 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L The Tokio repository contains multiple crates. Each crate has its own changelog.
T 80 text.html.markdown meta.paragraph.markdown
L 
L  * `tokio` - [view changelog](https://github.com/tokio-rs/tokio/blob/master/tokio/CHANGELOG.md)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 13 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 30 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 94 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 95 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L  * `tokio-util` - [view changelog](https://github.com/tokio-rs/tokio/blob/master/tokio-util/CHANGELOG.md)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 33 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 34 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 104 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 105 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L  * `tokio-stream` - [view changelog](https://github.com/tokio-rs/tokio/blob/master/tokio-stream/CHANGELOG.md)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 37 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 108 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 109 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L  * `tokio-macros` - [view changelog](https://github.com/tokio-rs/tokio/blob/master/tokio-macros/CHANGELOG.md)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 37 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 108 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 109 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L  * `tokio-test` - [view changelog](https://github.com/tokio-rs/tokio/blob/master/tokio-test/CHANGELOG.md)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 33 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 34 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 104 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 105 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L ## Supported Rust Versions
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 26 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L <!--
T 4 text.html.markdown comment.block.html punctuation.definition.comment.html
L When updating this, also update:
T 32 text.html.markdown comment.block.html
L - .github/workflows/ci.yml
T 26 text.html.markdown comment.block.html
L - CONTRIBUTING.md
T 17 text.html.markdown comment.block.html
L - README.md
T 11 text.html.markdown comment.block.html
L - tokio/README.md
T 17 text.html.markdown comment.block.html
L - tokio/Cargo.toml
T 18 text.html.markdown comment.block.html
L - tokio-util/Cargo.toml
T 23 text.html.markdown comment.block.html
L - tokio-test/Cargo.toml
T 23 text.html.markdown comment.block.html
L - tokio-stream/Cargo.toml
T 25 text.html.markdown comment.block.html
L -->
T 3 text.html.markdown comment.block.html punctuation.definition.comment.html
L 
L Tokio will keep a rolling MSRV (minimum supported rust version) policy of **at
T 78 text.html.markdown meta.paragraph.markdown
L least** 6 months. When increasing the MSRV, the new Rust version must have been
T 79 text.html.markdown meta.paragraph.markdown
L released at least six months ago. The current MSRV is 1.85.
T 59 text.html.markdown meta.paragraph.markdown
L 
L Note that the MSRV is not increased automatically, and only as part of a minor
T 78 text.html.markdown meta.paragraph.markdown
L release. The MSRV history for past minor releases can be found below:
T 69 text.html.markdown meta.paragraph.markdown
L 
L  * 1.54 to now  - Rust 1.85
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L  * 1.48 to 1.53 - Rust 1.71
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L  * 1.39 to 1.47 - Rust 1.70
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L  * 1.30 to 1.38 - Rust 1.63
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L  * 1.27 to 1.29 - Rust 1.56
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L  * 1.17 to 1.26 - Rust 1.49
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L  * 1.15 to 1.16 - Rust 1.46
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L  * 1.0 to 1.14 - Rust 1.45
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 26 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L Note that although we try to avoid the situation where a dependency transitively
T 80 text.html.markdown meta.paragraph.markdown
L increases the MSRV of Tokio, we do not guarantee that this does not happen.
T 75 text.html.markdown meta.paragraph.markdown
L However, every minor release will have some set of versions of dependencies that
T 80 text.html.markdown meta.paragraph.markdown
L works with the MSRV of that minor release.
T 42 text.html.markdown meta.paragraph.markdown
L 
L ## Release schedule
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 19 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Tokio doesn't follow a fixed release schedule, but we typically make one minor
T 78 text.html.markdown meta.paragraph.markdown
L release each month. We make patch releases for bugfixes as necessary.
T 69 text.html.markdown meta.paragraph.markdown
L 
L ## Bug patching policy
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 22 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L For the purposes of making patch releases with bugfixes, we have designated
T 75 text.html.markdown meta.paragraph.markdown
L certain minor releases as LTS (long term support) releases. Whenever a bug
T 74 text.html.markdown meta.paragraph.markdown
L warrants a patch release with a fix for the bug, it will be backported and
T 74 text.html.markdown meta.paragraph.markdown
L released as a new patch release for each LTS minor version. Our current LTS
T 75 text.html.markdown meta.paragraph.markdown
L releases are:
T 13 text.html.markdown meta.paragraph.markdown
L 
L  * `1.51.x` - LTS release until March 2027. (MSRV 1.71)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 55 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L  * `1.53.x` - LTS release until September 2027. (MSRV 1.71)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 59 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L Each LTS release will continue to receive backported fixes for at least a year.
T 79 text.html.markdown meta.paragraph.markdown
L If you wish to use a fixed minor release in your project, we recommend that you
T 79 text.html.markdown meta.paragraph.markdown
L use an LTS release.
T 19 text.html.markdown meta.paragraph.markdown
L 
L To use a fixed minor version, you can specify the version with a tilde. For
T 75 text.html.markdown meta.paragraph.markdown
L example, to specify that you wish to use the newest `1.51.x` patch release, you
T 52 text.html.markdown meta.paragraph.markdown
T 53 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 59 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 60 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 79 text.html.markdown meta.paragraph.markdown
L can use the following dependency specification:
T 47 text.html.markdown meta.paragraph.markdown
L ```text
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language
L tokio = { version = "~1.51", features = [...] }
T 47 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Previous LTS releases
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 25 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L  * `1.8.x` - LTS release until February 2022.
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 45 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L  * `1.14.x` - LTS release until June 2022.
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L  * `1.18.x` - LTS release until June 2023.
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L  * `1.20.x` - LTS release until September 2023.
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 47 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L  * `1.25.x` - LTS release until March 2024.
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L  * `1.32.x` - LTS release until September 2024.
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 47 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L  * `1.36.x` - LTS release until March 2025.
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L  * `1.38.x` - LTS release until July 2025.
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L  * `1.43.x` - LTS release until March 2026.
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L  * `1.47.x` - LTS release until September 2026.
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 47 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ## License
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 10 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L This project is licensed under the [MIT license].
T 49 text.html.markdown meta.paragraph.markdown
L 
L [MIT license]: https://github.com/tokio-rs/tokio/blob/master/LICENSE
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 12 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 13 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 14 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 15 text.html.markdown meta.link.reference.def.markdown
T 68 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L 
L ### Contribution
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 16 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Unless you explicitly state otherwise, any contribution intentionally submitted
T 79 text.html.markdown meta.paragraph.markdown
L for inclusion in Tokio by you shall be licensed as MIT, without any additional
T 78 text.html.markdown meta.paragraph.markdown
L terms or conditions.
T 20 text.html.markdown meta.paragraph.markdown
L 
