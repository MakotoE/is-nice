#![warn(clippy::all)]
use is_nice::is_nice;

fn main() {
    let str = std::env::args().nth(1).unwrap_or(String::new());

    if str == "--help" {
        println!("Usage: is-nice STRING");
        return;
    }

    println!("{}", is_nice(&str));
}
