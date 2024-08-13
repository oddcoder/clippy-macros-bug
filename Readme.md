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
cargo clippy --fix --lib -p marshalable
   Compiling marshalable-derive v0.1.0 (/home/oddcoder/projects/clippy-bug/marshalable-derive)
    Checking marshalable v0.1.0 (/home/oddcoder/projects/clippy-bug/marshalable)
warning: failed to automatically apply fixes suggested by rustc to crate `marshalable`

after fixes were automatically applied the compiler reported errors within these files:

  * marshalable/src/lib.rs

This likely indicates a bug in either rustc or cargo itself,
and we would appreciate a bug report! You're likely to see
a number of compiler warnings after this message which cargo
attempted to fix but failed. If you could open an issue at
https://github.com/rust-lang/rust-clippy/issues
quoting the full output of this command we'd be very appreciative!
Note that you may be able to make some more progress in the near-term
fixing code with the `--broken-code` flag

The following errors were reported:
error: expected `:`, found `;`
  --> marshalable/src/lib.rs:15:7
   |
14 | struct HasUnitField {
   |        ------------ while parsing this struct
15 |     _b;: (),
   |       ^ expected `:`

error: expected `:`
  --> marshalable/src/lib.rs:15:7
   |
15 |     _b;: (),
   |       ^

error: aborting due to 2 previous errors

Original diagnostics will follow.

warning: this let-binding has unit value
  --> marshalable/src/lib.rs:15:5
   |
15 |     _b: (),
   |     ^^ help: omit the `let` binding: `_b;`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#let_unit_value
   = note: `#[warn(clippy::let_unit_value)]` on by default

warning: `marshalable` (lib) generated 1 warning (run `cargo clippy --fix --lib -p marshalable` to apply 1 suggestion)

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
