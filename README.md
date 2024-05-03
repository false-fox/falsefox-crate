# falsefox crate
A crate experiment that simply returns information about the author, falsefox. 
Fair warning: this is just to help me learn cargo and the crates system, i'm not intending for a completely polished and professionally made experience, but please create an issue if you want to help me out and learn best practices :)

[Documentation via docs.rs](https://docs.rs/falsefox/0.1.3/falsefox/index.html)

## Example
```rust
use falsefox;
fn main () {
    println!("{:?}", falsefox::get_github());
    // https://github.com/false-fox
}
```

