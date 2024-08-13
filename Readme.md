To reproduce the bug run:

```
$ cargo clippy
warning: this let-binding has unit value
  --> marshalable/src/lib.rs:15:5
   |
15 |     _b: (),
   |     ^^ help: omit the `let` binding: `_b;`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#let_unit_value
   = note: `#[warn(clippy::let_unit_value)]` on by default
warning: `marshalable` (lib) generated 1 warning (run `cargo clippy --fix --lib -p marshalable` to apply 1 suggestion)
$ cargo clippy --fix --lib -p marshalable
```

Reproducable on:

```
$ rustc -Vv
rustc 1.80.1 (3f5fd8dd4 2024-08-06)
binary: rustc
commit-hash: 3f5fd8dd41153bc5fdca9427e9e05be2c767ba23
commit-date: 2024-08-06
host: x86_64-unknown-linux-gnu
release: 1.80.1
LLVM version: 18.1.7
```
