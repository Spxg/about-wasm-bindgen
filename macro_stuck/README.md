```shell
error: cannot find attribute `foo` in this scope
 --> src/lib.rs:6:5
  |
6 |     #[pmacro]
  |     ^^^^^^^^^
  |
  = note: `foo` is an attribute that can be used by the derive macro `Attr`, you might be missing a `derive` attribute
  = note: this error originates in the attribute macro `pmacro` (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot determine resolution for the derive macro `pmacro::Attr`
 --> src/lib.rs:6:5
  |
6 |     #[pmacro]
  |     ^^^^^^^^^
  |
  = note: import resolution is stuck, try simplifying macro imports
  = note: this error originates in the attribute macro `pmacro` (in Nightly builds, run with -Z macro-backtrace for more info)
```

This problem can be solved by changing `#[derive(pmacro::Attr)]` to `#[derive(::pmacro::Attr)]`

Related issues: https://github.com/wasm-bindgen/wasm-bindgen/issues/4597
