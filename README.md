# embed-c

`embed-c` is a crate that allows you to embed C code inside Rust code files. The C code is
translated into Rust code at compile time using [C2Rust](https://github.com/immunant/c2rust),
which means that it is fully interoperable with Rust. C code can call Rust code, and vice-versa.

## Install
The library is not yet on crates.io. Clone the repository somewhere and add it as a dependency
to your `Cargo.toml`:
```toml
[dependencies]
embed-c = { path = "./embed-c", version = "0.1" }
```

**NOTE:** this crate is designed to work for the `nightly-2019-12-05` version of Rust.

## Basic usage
```rust
use embed_c::embed_c;

embed_c! {
    int add(int x, int y) {
        return x + y;
    }
}

fn main() {
    let x = unsafe { add(1, 2) };
    println!("{}", x);
}
```

See more examples in [src/lib.rs](src/lib.rs).

## Limitations
Many

## Motivation
N/A

## License
This project is licensed under either of
* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  https://opensource.org/licenses/MIT)
  at your option.