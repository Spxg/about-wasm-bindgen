## Cell Vs

The `externref_alloc` function in wasm-bindgen uses `Cell` internally.
Since `externref_alloc` must be thread local, it is OK to use `UnsafeCell` here.

When using `Cell` and `UnsafeCell` to complete the simple operation, and opt-level is set to 3,
the generated code is exactly the same. https://godbolt.org/z/d8nv1cvoW

However, in wasm, in order to minimize the size, opt-level is often set to "s" or "z",
and the generated code is different. https://godbolt.org/z/hc6KT6n38

Here we will test the performance differences under different opt-levels:

```shell
OPT=0 ./run.sh
OPT=1 ./run.sh
OPT=2 ./run.sh
OPT=3 ./run.sh
OPT=s ./run.sh
OPT=z ./run.sh
```

Here are the results of my test:

||Cell|RefCell|UnsafeCell|
|-|-|-|-|
|opt-level=0|22,113,356 ops/sec ±0.23%|24,878,384 ops/sec ±0.44%|35,012,090 ops/sec ±0.41%|
|opt-level=1|70,824,816 ops/sec ±0.45%|72,064,993 ops/sec ±0.41%|76,436,707 ops/sec ±0.35%|
|opt-level=2|68,412,049 ops/sec ±0.28%|81,423,239 ops/sec ±0.60%|85,974,886 ops/sec ±0.37%|
|opt-level=3|70,689,223 ops/sec ±0.55%|83,553,134 ops/sec ±0.60%|84,443,275 ops/sec ±0.49%|
|opt-level="s"|57,638,158 ops/sec ±0.43%|73,181,241 ops/sec ±0.48%|80,080,127 ops/sec ±0.36%|
|opt-level="z"|58,498,860 ops/sec ±0.28%|74,129,376 ops/sec ±0.25%|75,057,844 ops/sec ±0.20%|

wasm-bindgen's externref bench (see `externref_bench`):

||Cell|UnsafeCell|improve|
|-|-|-|-|
|opt-level=0|4,542,025 ops/sec ±0.37%|6,452,675 ops/sec ±0.41%|42%|
|opt-level=1|17,899,143 ops/sec ±0.44%|20,226,752 ops/sec ±0.17%|13%|
|opt-level=2|18,842,926 ops/sec ±0.28%|22,055,264 ops/sec ±0.38%|17%|
|opt-level=3|19,284,668 ops/sec ±0.44%|21,858,031 ops/sec ±0.51%|13%|
|opt-level="s"|15,597,994 ops/sec ±0.44%|20,452,012 ops/sec ±0.43%|31%|
|opt-level="z"|14,377,145 ops/sec ±0.24%|18,637,259 ops/sec ±0.18%|30%|

As you can see, using `UnsafeCell` is always the fastest.

When safety is guaranteed, it is a good choice to use `UnsafeCell` instead of relying on the compiler to "optimize" `Cell` into `UnsafeCell`.
