# falsefox crate
A crate experiment that simply returns information about the author, falsefox.

[Documentation via docs.rs](https://docs.rs/falsefox/0.1.3/falsefox/index.html)

Example:
```rust
use falsefox;
fn main () {
    println!("{:?}", falsefox::get_github());
    // https://github.com/false-fox
}
```

