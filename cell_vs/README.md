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

Here are the results of my test (todo: need retest!):

||Cell|RefCell|UnsafeCell|
|-|-|-|-|
|opt-level=0|38,538,874/s|39,428,751/s|53,776,682/s|
|opt-level=1|2.729ms|1.993ms|
|opt-level=2|2.084ms|1.832ms|
|opt-level=3|2.065ms|1.802ms|
|opt-level="s"|3.118ms|1.985ms|
|opt-level="z"|3.642ms|2.122ms|

As you can see, using `UnsafeCell` is always the fastest.

When safety is guaranteed, it is a good choice to use `UnsafeCell` instead of relying on the compiler to "optimize" `Cell` into `UnsafeCell`.
