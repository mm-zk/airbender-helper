

to rebuild:

```shell
rustup target add wasm32-unknown-unknown
wasm-pack build --target web
```


RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --target web --no-opt