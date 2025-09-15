## Cell vs UnsafeCell

The `externref_alloc` function in wasm-bindgen uses `Cell` internally.
Since `externref_alloc` must be thread local, it is OK to use `UnsafeCell` here.

When using `Cell` and `UnsafeCell` to complete the simple operation, and opt-level is set to 3,
the generated code is exactly the same. https://godbolt.org/z/d8nv1cvoW

However, in wasm, in order to minimize the size, opt-level is often set to "s" or "z",
and the generated code is different. https://godbolt.org/z/hc6KT6n38

Here we will test the performance differences under different opt-levels (Ultra 7-265 + 64G + Chrome):

```shell
# exec in other terminal
python3 -m http.server 8000

# change opt-level first

# use `Cell` and open browser to test
cargo build --target wasm32-unknown-unknown --release

# use `UnsafeCell` and open browser to test
cargo build --target wasm32-unknown-unknown --release --features unsafe_cell
```

Here are the results of my test:

||Cell|UnsafeCell|
|-|-|-|
|opt-level=0|18.145ms|12.758ms|
|opt-level=1|2.867ms|2.623ms|
|opt-level=2|2.503ms|2.398ms|
|opt-level=3|2.508ms|2.402ms|
|opt-level="s"|3.429ms|2.639ms|
|opt-level="z"|4.161ms|3.075ms|

As you can see, using `UnsafeCell` is always the fastest.

When safety is guaranteed, it is a good choice to use `UnsafeCell` instead of relying on the compiler to "optimize" `Cell` into `UnsafeCell`.
