//! Prints the tokens of the lines on stdin: `trace <scope>`.
use std::io::BufRead;
use std::path::Path;
use textmate_proto::grammar::Grammars;
use textmate_proto::tokenizer::{Engine, scope_names};

fn main() {
    let root = std::env::args().nth(1).expect("scope");
    let engine = Engine::new(Grammars::load(Path::new("grammars"), &["source.rust", "source.shell", "source.ts", "text.html.markdown"]));
    let mut state = engine.start(&root);
    let mut at = 0;
    for line in std::io::stdin().lock().lines() {
        let line = line.expect("line");
        println!("L {line}");
        for t in engine.tokenize_line(&line, &mut state) {
            let names: Vec<String> = scope_names(&t.scopes).iter().map(|s| s.to_string()).collect();
            println!("T {} {}", t.end, names.join(" "));
        }
        at += 1;
    }
    let _ = at;
    if std::env::var_os("STATE").is_some() { eprintln!("{state:?}"); }
}
