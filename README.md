[![Latest version](https://img.shields.io/crates/v/is-nice.svg)](https://crates.io/crates/is-nice) [![Documentation](https://docs.rs/is-thirteen/badge.svg)](https://docs.rs/is-nice/) ![Not fake](https://img.shields.io/badge/ci-passing-brightgreen?logo=github) [![Nice](https://img.shields.io/badge/coverage-69%25-yellow)](https://youtu.be/dQw4w9WgXcQ)

# is-nice

`is_nice()` tells you if a string is [nice](https://www.urbandictionary.com/define.php?term=nice) or not. Written in Rust 🦀, memory safe 🛟, and blazingly fast 🚀! 

To use as a library: `cargo add is-nice`

```rust
use is_nice::is_nice;

fn main() {
    println!("{}", is_nice("69")); // true
}

```

You can also get the CLI version with `cargo install is-nice`.

```
is-nice 68
not nice

is-nice 69
nice

is-nice LXIX
nice
```

Also check out [is-thirteen](https://github.com/MakotoE/is-thirteen).

# WASM support

No.