# WASI Sqilte

## dev

```bash
cargo expand > binding.rs
```


## build

```bash
cargo build --target=wasm32-wasip2
```

## test

```
wasmtime run --invoke 'hello("wasi")' ./target/wasm32-wasip2/debug/wasi_sqlite.wasm
wasmtime run --invoke 'init(1, 4)' ./target/wasm32-wasip2/debug/wasi_sqlite.wasm
```