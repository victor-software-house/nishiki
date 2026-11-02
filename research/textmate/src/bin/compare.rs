//! Scores the prototype against the oracle fixtures and times it.
//!
//! Usage: `compare [--bench N] [language ...]`, run from `research/textmate`.
//! Fidelity is the share of non-whitespace bytes whose scope stack equals
//! Shiki's.

use std::path::Path;
use std::time::Instant;

use textmate_proto::grammar::Grammars;
use textmate_proto::tokenizer::{Engine, scope_names};

const LANGS: &[(&str, &str)] = &[
    ("rust", "source.rust"),
    ("shellscript", "source.shell"),
    ("typescript", "source.ts"),
    ("markdown", "text.html.markdown"),
];

struct Doc {
    lines: Vec<(String, Vec<(usize, Vec<String>)>)>,
}

fn parse(path: &Path) -> Doc {
    let text = std::fs::read_to_string(path).expect("fixture");
    let mut lines: Vec<(String, Vec<(usize, Vec<String>)>)> = Vec::new();
    for row in text.lines() {
        if let Some(rest) = row.strip_prefix("L ") {
            lines.push((rest.to_owned(), Vec::new()));
        } else if row == "L" {
            lines.push((String::new(), Vec::new()));
        } else if let Some(rest) = row.strip_prefix("T ") {
            let mut parts = rest.split(' ');
            let end: usize = parts.next().and_then(|n| n.parse().ok()).expect("end");
            let scopes = parts.map(str::to_owned).collect();
            if let Some(last) = lines.last_mut() {
                last.1.push((end, scopes));
            }
        }
    }
    Doc { lines }
}

fn run(engine: &Engine, root: &str, doc: &Doc, mut sink: impl FnMut(usize, &str, &[(usize, Vec<String>)])) {
    let mut state = engine.start(root);
    for (i, (line, _)) in doc.lines.iter().enumerate() {
        let tokens = engine.tokenize_line(line, &mut state);
        let mine: Vec<(usize, Vec<String>)> = tokens
            .iter()
            .map(|t| (t.end, scope_names(&t.scopes).iter().map(|s| s.to_string()).collect()))
            .collect();
        sink(i, line, &mine);
    }
}

fn scope_at(tokens: &[(usize, Vec<String>)], byte: usize) -> Option<&Vec<String>> {
    tokens.iter().find(|t| t.0 > byte).map(|t| &t.1)
}

fn work() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let bench = args.iter().position(|a| a == "--bench").and_then(|i| args.get(i + 1)).and_then(|n| n.parse::<usize>().ok());
    let only: Vec<&str> = args.iter().map(String::as_str).filter(|a| LANGS.iter().any(|l| l.0 == *a)).collect();
    let started = Instant::now();
    let grammars = Grammars::load(
        Path::new("grammars"),
        &["source.rust", "source.shell", "source.ts", "text.html.markdown"],
    );
    let engine = Engine::new(grammars);
    println!("load+link {:?}; errors {}", started.elapsed(), engine.grammars.errors.len());
    for &(lang, root) in LANGS {
        if !only.is_empty() && !only.contains(&lang) {
            continue;
        }
        let dir = Path::new("../../tests/oracle").join(lang);
        let mut files: Vec<_> = std::fs::read_dir(&dir).expect("dir").filter_map(Result::ok).map(|e| e.path()).filter(|p| p.extension().is_some_and(|e| e == "txt")).collect();
        files.sort();
        for file in files {
            let doc = parse(&file);
            let (mut total, mut good, mut first_bad) = (0usize, 0usize, None);
            run(&engine, root, &doc, |i, line, mine| {
                let theirs = &doc.lines[i].1;
                for (b, c) in line.bytes().enumerate() {
                    if c.is_ascii_whitespace() {
                        continue;
                    }
                    total += 1;
                    if scope_at(mine, b) == scope_at(theirs, b) {
                        good += 1;
                    } else if first_bad.is_none() {
                        first_bad = Some((i + 1, b, line.to_owned(), scope_at(mine, b).cloned(), scope_at(theirs, b).cloned()));
                    }
                }
            });
            let name = file.file_stem().and_then(|s| s.to_str()).unwrap_or("?");
            let bytes: usize = doc.lines.iter().map(|l| l.0.len() + 1).sum();
            let mut line = format!("{lang:12} {name:28} {good}/{total} {:.2}%", 100.0 * good as f64 / total.max(1) as f64);
            if let Some(n) = bench {
                let mut times = Vec::new();
                for _ in 0..n {
                    let t = Instant::now();
                    run(&engine, root, &doc, |_, _, _| {});
                    times.push(t.elapsed().as_secs_f64());
                }
                times.sort_by(f64::total_cmp);
                let med = times[times.len() / 2];
                line += &format!("  {:.2} MiB/s", bytes as f64 / med / 1048576.0);
            }
            println!("{line}");
            if let Some((l, b, text, mine, theirs)) = first_bad {
                println!("    first diff line {l} byte {b}: {text:?}\n      mine   {mine:?}\n      shiki  {theirs:?}");
            }
        }
    }
    println!("matcher steps {}", engine.steps.borrow());
}

fn main() {
    // The matcher recurses per character, so give it room.
    std::thread::Builder::new().stack_size(1 << 30).spawn(work).expect("thread").join().expect("run");
}
