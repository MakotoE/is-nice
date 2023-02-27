#![warn(clippy::all)]
use is_nice::is_nice;

fn convert_to_string(b: bool) -> &'static str {
    if b {
        "nice"
    } else {
        "not nice"
    }
}

fn main() {
    let str = std::env::args().nth(1).unwrap_or(String::new());

    if str == "--help" {
        println!("Usage: is-nice STRING");
        return;
    }

    println!("{}", convert_to_string(is_nice(&str)));
}
