# embed-c

`embed-c` is a crate that allows you to embed C code inside Rust code files. The C code is
translated into Rust code at compile time using [C2Rust](https://github.com/immunant/c2rust),
which means that it is fully interoperable with Rust. C code can call Rust code, and vice-versa.

## Install
The library is not yet on crates.io. Clone the repository somewhere and set it up:
```shell
git clone https://github.com/zdimension/embed-c.git
cd embed-c
git submodule update --init c2rust
cp Cargo.lock ..
cd ..
```
and add this to your `Cargo.toml`:
```toml
[dependencies]
embed-c = { path = "./embed-c", version = "0.1" }

[patch.crates-io]
c2rust-transpile = { path = "./embed-c/c2rust/c2rust-transpile" }
```


**NOTE:** this crate is designed to work for the `nightly-2019-12-05` version of Rust, 
so put this in your `rust-toolchain.toml`:
```toml
[toolchain]
channel = "nightly-2019-12-05"
```
And change the `package.edition` setting in your `Cargo.toml` to be "2018":
```toml
[package]
edition = "2018"
```

If you get errors about the `matches!` macro, or from the `half` or `rustc_demangle` crates, copy the `Cargo.lock`
file to your project root again.

## Basic usage
```rust
#![feature(rustc_private)] 

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

The `#![feature(rustc_private)]` bit is required since the crate uses internal features while not being loaded 
from crates.io.

See more examples in [src/lib.rs](src/lib.rs).

```rust
embed_c! {
    void send(to, from, count)
        register short *to, *from;
        register count;
    {
        register n = (count + 7) / 8;
        switch (count % 8) {
        case 0: do { *to++ = *from++;
        case 7:      *to++ = *from++;
        case 6:      *to++ = *from++;
        case 5:      *to++ = *from++;
        case 4:      *to++ = *from++;
        case 3:      *to++ = *from++;
        case 2:      *to++ = *from++;
        case 1:      *to++ = *from++;
                } while (--n > 0);
        }
    }
}

fn main() {
    let mut source = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut dest = [0; 10];
    unsafe { send(dest.as_mut_ptr(), source.as_mut_ptr(), 10); };
    assert_eq!(source, dest);
}
```

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