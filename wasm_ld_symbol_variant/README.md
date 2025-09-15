## wasm-ld symbol variant

I recently encountered an interesting [wasm-bindgen issue](https://github.com/wasm-bindgen/wasm-bindgen/issues/4390#issuecomment-2719191785). 
It was eventually confirmed that it was not a problem with wasm-bindgen, but rather the special handling of 
symbol variants with the same symbol name but different signatures by wasm-ld.

Below are the steps to reproduce and the analysis process.

### Rust Minimal Reproducible Case

I simplified the use case from the comments and it can now be summarized as:

```shell
cargo build --target wasm32-unknown-unknown -p foo
# init synbol exists:
# 00000021 D __stack_pointer
# 0000006a T init
llvm-nm target/wasm32-unknown-unknown/debug/foo.wasm

cargo build --target wasm32-unknown-unknown -p foo --features control
# init synbol now missing:
# 00000021 D __stack_pointer
# 0000006d T control
llvm-nm target/wasm32-unknown-unknown/debug/foo.wasm
```

We turn on verbose mode for `wasm-ld` and enable log output for rustc, and we can find some clues:

```shell
cargo clean
RUSTFLAGS="-Clink-args=--verbose" RUSTC_LOG="rustc_codegen_ssa::back::link=info" cargo build --target wasm32-unknown-unknown -p foo --features control
```

You can see that wasm-ld found two different definitions but the same exported symbol:

```
rust-lld: warning: function signature mismatch: init
>>> defined as () -> i32 in /home/xxx/repo/wasm-ld-symbol-variant/rust/target/wasm32-unknown-unknown/debug/deps/libbar-3463390d170e59bf.rlib(bar-3463390d170e59bf.4h8dic9h55qzf0ewdg
p7sixku.0jd5z4y.rcgu.o)
>>> defined as () -> void in /home/xxx/repo/wasm-ld-symbol-variant/rust/target/wasm32-unknown-unknown/debug/deps/foo.e11rv2ujxd69ayq4l3bd7a8ou.1nu84mc.rcgu.o
```

But if the control feature is disabled, wasm-ld will not detect this problem. Why?

Rust does so many things, it is necessary to construct a minimal reproducible use case in C language for ease of analysis.

### C Minimal Reproducible Case

By executing the following command, we successfully let wasm-ld find the symbol variants:

```shell
clang -target wasm32-unknown-unknown foo.c -c
clang -target wasm32-unknown-unknown bar.c -c
# wasm-ld: warning: function signature mismatch: init
# >>> defined as () -> i32 in bar.o
# >>> defined as () -> void in foo.o
wasm-ld foo.o bar.o --no-entry --verbose --export=init --export=control -o out.wasm
# init synbol missing:
# 0000001a D __stack_pointer
# 0000003a T control
llvm-nm out.wasm
```

By default, wasm-ld removes unused code. Let's use `--no-gc-sections` to see what happens.

```shell
wasm-ld foo.o bar.o --no-entry --verbose --export=init --export=control -o out.wasm --no-gc-sections
# 00000022 D __stack_pointer
# 00000042 t __wasm_call_ctors
# 00000059 T control
# 00000049 t init
# 0000004d t init
# 00000045 t signature_mismatch:init
llvm-nm out.wasm
```

Aha, the symbol is back, but why is it of type "t" instead of the expected "T"?

In fact, this is how wasm-ld handles symbol variants. They are no longer exported,
and a new `signature_mismatch:t` function (unreachable body) is added (probably to remind users).

The same behavior can be observed in rust case using `--no-gc-sections`:

```shell
RUSTFLAGS="-Clink-args=--no-gc-sections" cargo build --target wasm32-unknown-unknown -p foo --features control
# 0004a716 T control
# 00000827 t init
# 0004a70a t init
# 00049aa3 t memcmp
# 00000823 t signature_mismatch:init
llvm-nm target/wasm32-unknown-unknown/debug/foo.wasm
```

What happens if there is no `control` symbol? Will the `init` symbol be retained? Let's try the following:

```shell
clang -target wasm32-unknown-unknown bar_without_control.c -c
wasm-ld foo.o bar_without_control.o --no-entry --verbose --export=init -o out.wasm --no-gc-sections
# 00000021 D __stack_pointer
# 00000037 t __wasm_call_ctors
# 0000003e t init
# 00000042 t init
# 0000003a t signature_mismatch:init
llvm-nm out.wasm
```

The symbol variants is still found by wasm-ld, so why is the symbol of the rust case retained under the same conditions?

After some time reading the wasm-ld source code, I found the reason.

### Summarizes the following behaviors of wasm-ld

* `symMap`, which symbol variants can be found. ([code](https://github.com/llvm/llvm-project/blob/21cca5ea9d13ff791a7982a2a8edb4a56ef4674e/lld/wasm/SymbolTable.cpp#L109))

* `LazySymbol`, which does not resolve symbols immediately, but waits until needed, such as `--export=xxx`. ([code](https://github.com/llvm/llvm-project/blob/21cca5ea9d13ff791a7982a2a8edb4a56ef4674e/lld/wasm/Driver.cpp#L1371))

* ObjFile is considered lazy if it is a relocatable object file/bitcode file in an ar archive or between --start-lib and --end-lib. ([code](https://github.com/llvm/llvm-project/blob/21cca5ea9d13ff791a7982a2a8edb4a56ef4674e/lld/wasm/InputFiles.h#L70))
  
* When inserting a defined symbol, variant checking is triggered only if the symbol already exists and is not a lazy symbol. ([code](https://github.com/llvm/llvm-project/blob/21cca5ea9d13ff791a7982a2a8edb4a56ef4674e/lld/wasm/SymbolTable.cpp#L445))

* Insert a lazy symbol does not trigger variant checking. ([code](https://github.com/llvm/llvm-project/blob/21cca5ea9d13ff791a7982a2a8edb4a56ef4674e/lld/wasm/SymbolTable.cpp#L874))

### Now we can reproduce our problem based on these behaviors

```shell
wasm-ld foo.o bar.o --no-entry --export=init -o out.wasm
# Nothing is left because neither foo.o nor bar.o is a lazy ObjFile,
# so the symbol variant is found and gc:
# 00000010 D __stack_pointer
llvm-nm out.wasm 
```

```shell
wasm-ld foo.o --start-lib bar.o --end-lib --no-entry --export=init -o out.wasm
# We make foo.o or bar.o as ObjFile, which does not trigger the variant check,
# so the init symbol is back (just like in the rust use case):
# 0000001a D __stack_pointer
# 00000037 T init
llvm-nm out.wasm 
```

```shell
wasm-ld foo.o --start-lib bar.o --end-lib --no-entry --export=init --export=control -o out.wasm
# We use --export to trigger variant checking (just like in the rust use case):
# 0000001a D __stack_pointer
# 0000003a T control
llvm-nm out.wasm
```

### Summarize

This is not actually a bug in wasm-ld, I consider it undefined behavior, as the rust documentation says: https://doc.rust-lang.org/reference/abi.html#r-abi.export_name.unsafe
