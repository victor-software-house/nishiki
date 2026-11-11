//! Q1: how fast nishiki lexes a document, per language and tier
//! (openspec bench-methodology, design decisions 1 and 5).
//!
//! Each benchmark lexes every file of one language and tier, line by line.
//! Each file starts from a fresh highlighter, as an editor opening the file
//! does. The throughput is the files' total bytes over their total time, so a
//! small file weighs no more than its size. The small tier shows what
//! creating a highlighter costs. `new/<language>` times that alone.
//!
//! `mise run bench lex/python` runs only the benchmarks whose name holds
//! the filter.

use std::hint::black_box;
use std::time::Duration;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use nishiki::{Highlighter, Language, State, Token};
use nishiki_bench::{Document, Tier, corpus};

/// One language's files of one tier, split into lines before timing.
struct Group<'a> {
    language: Language,
    tier: Tier,
    bytes: u64,
    documents: Vec<Vec<&'a str>>,
}

fn groups(documents: &[Document]) -> Vec<Group<'_>> {
    let mut groups: Vec<Group<'_>> = Vec::new();
    for document in documents {
        let bytes = u64::try_from(document.bytes()).unwrap_or(u64::MAX);
        match groups
            .iter_mut()
            .find(|group| group.language == document.language && group.tier == document.tier)
        {
            Some(group) => {
                group.bytes = group.bytes.saturating_add(bytes);
                group.documents.push(document.lines());
            }
            None => groups.push(Group {
                language: document.language,
                tier: document.tier,
                bytes,
                documents: vec![document.lines()],
            }),
        }
    }
    groups
}

/// Lexes each document from a fresh highlighter; returns the tokens made.
fn lex(language: Language, documents: &[Vec<&str>], tokens: &mut Vec<Token>) -> usize {
    let mut count = 0;
    for lines in documents {
        let mut highlighter = Highlighter::new(language);
        let mut state = State::default();
        for &line in lines {
            tokens.clear();
            let report = highlighter.line(line, &mut state, tokens);
            black_box(&report);
            count += tokens.len();
        }
    }
    count
}

fn throughput(c: &mut Criterion) {
    let documents = match corpus() {
        Ok(documents) => documents,
        Err(error) => panic!("cannot read bench/corpus: {error}"),
    };
    let mut tokens = Vec::new();
    for group in groups(&documents) {
        let mut bench = c.benchmark_group(format!("lex/{}", group.language.id()));
        bench.throughput(Throughput::Bytes(group.bytes));
        bench.bench_function(group.tier.name(), |b| {
            b.iter(|| lex(group.language, &group.documents, &mut tokens));
        });
        bench.finish();
    }
    let mut languages: Vec<Language> = documents.iter().map(|document| document.language).collect();
    languages.dedup();
    let mut bench = c.benchmark_group("new");
    for language in languages {
        bench.bench_function(language.id(), |b| {
            b.iter(|| Highlighter::new(black_box(language)));
        });
    }
    bench.finish();
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
    targets = throughput
}
criterion_main!(benches);
