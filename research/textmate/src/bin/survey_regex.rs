//! Compiles every regular expression of the grammars under `grammars/` and
//! reports the ones the engine rejects.

use std::fs;

use serde_json::Value;
use textmate_proto::regex::Regex;

fn walk(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::Array(items) => items.iter().for_each(|v| walk(v, out)),
        Value::Object(map) => {
            for (key, v) in map {
                match (key.as_str(), v) {
                    ("match" | "begin" | "end" | "while", Value::String(s)) => out.push(s.clone()),
                    _ => walk(v, out),
                }
            }
        }
        _ => {}
    }
}

fn main() {
    let mut files: Vec<_> = fs::read_dir("grammars")
        .expect("run tools/dump-grammars.ts first")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect();
    files.sort();
    for file in files {
        let grammar: Value = serde_json::from_str(&fs::read_to_string(&file).unwrap()).unwrap();
        let mut patterns = Vec::new();
        walk(&grammar, &mut patterns);
        patterns.sort();
        patterns.dedup();
        let mut failed = 0;
        for pattern in &patterns {
            // Back-references in an end pattern are text substituted at run time.
            let probe = pattern.replace("\\1", "x").replace("\\2", "x").replace("\\3", "x");
            if let Err(e) = Regex::new(&probe) {
                failed += 1;
                println!("  FAIL {}: {pattern}", e.0);
            }
        }
        println!("{}: {} of {} patterns compile", file.display(), patterns.len() - failed, patterns.len());
    }
}
