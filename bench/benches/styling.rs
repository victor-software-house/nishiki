//! Q2: how fast nishiki colours a document (openspec bench-methodology,
//! design decisions 1 and 5), with Shiki's github-dark theme.
//!
//! - `style/<language>` times `Theme::style` alone, over the real token stream
//!   of the language's medium files, per token.
//! - `colour/<language>` times the whole path from text to colours: each line
//!   lexed, then each of its tokens styled with `Highlighter::style`, per byte.
//!   This is the figure the comparison with other highlighters uses, because
//!   every engine can produce colours.

use std::hint::black_box;
use std::time::Duration;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use nishiki::{Highlighter, Language, State, Theme};
use nishiki_bench::{Document, GITHUB_DARK, Tier, corpus};

/// A language's medium files, split into lines before timing.
struct Files<'a> {
    language: Language,
    bytes: u64,
    documents: Vec<Vec<&'a str>>,
}

fn medium_files(documents: &[Document]) -> Vec<Files<'_>> {
    let mut files: Vec<Files<'_>> = Vec::new();
    for document in documents
        .iter()
        .filter(|document| document.tier == Tier::Medium)
    {
        let bytes = u64::try_from(document.bytes()).unwrap_or(u64::MAX);
        match files
            .iter_mut()
            .find(|files| files.language == document.language)
        {
            Some(files) => {
                files.bytes = files.bytes.saturating_add(bytes);
                files.documents.push(document.lines());
            }
            None => files.push(Files {
                language: document.language,
                bytes,
                documents: vec![document.lines()],
            }),
        }
    }
    files
}

fn styling(c: &mut Criterion) {
    let documents = match corpus() {
        Ok(documents) => documents,
        Err(error) => panic!("cannot read bench/corpus: {error}"),
    };
    let theme = match Theme::from_json(GITHUB_DARK) {
        Ok(theme) => theme,
        Err(error) => panic!("github-dark does not load: {error}"),
    };
    for files in medium_files(&documents) {
        // Each document's highlighter, which owns its stacks, and its tokens.
        let mut lexed = Vec::new();
        for lines in &files.documents {
            let mut highlighter = Highlighter::new(files.language);
            let mut state = State::default();
            let mut all = Vec::new();
            let mut tokens = Vec::new();
            for &line in lines {
                tokens.clear();
                let _ = highlighter.line(line, &mut state, &mut tokens);
                all.extend_from_slice(&tokens);
            }
            lexed.push((highlighter, all));
        }
        let count: usize = lexed.iter().map(|(_, tokens)| tokens.len()).sum();

        let mut bench = c.benchmark_group(format!("style/{}", files.language.id()));
        bench.throughput(Throughput::Elements(
            u64::try_from(count).unwrap_or(u64::MAX),
        ));
        bench.bench_function("medium", |b| {
            b.iter(|| {
                for (highlighter, tokens) in &lexed {
                    for token in tokens {
                        black_box(theme.style(highlighter.scopes(token.stack())));
                    }
                }
            });
        });
        bench.finish();

        let mut bench = c.benchmark_group(format!("colour/{}", files.language.id()));
        bench.throughput(Throughput::Bytes(files.bytes));
        let mut tokens = Vec::new();
        bench.bench_function("medium", |b| {
            b.iter(|| {
                for lines in &files.documents {
                    let mut highlighter = Highlighter::new(files.language);
                    let mut state = State::default();
                    for &line in lines {
                        tokens.clear();
                        let _ = highlighter.line(line, &mut state, &mut tokens);
                        for token in &tokens {
                            black_box(highlighter.style(&theme, token.stack()));
                        }
                    }
                }
            });
        });
        bench.finish();
    }
}

fn config() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(3))
        .sample_size(50)
}

criterion_group! {
    name = benches;
    config = config();
    targets = styling
}
criterion_main!(benches);
