//! `rx <pattern> <text> <from>`: one search, for debugging.
use textmate_proto::regex::Regex;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let re = Regex::new(&a[1]).expect("pattern");
    let text = format!("{}\n", a[2]);
    let mut steps = 1 << 40;
    println!("{:?}", re.find_at(&text, a[3].parse().unwrap(), None, true, &mut steps));
}
