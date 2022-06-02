# dland-wasm

CLI wrapper and Wasm targets for Dland time-lock encryption.

```shell
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo install wasm-pack
wasm-pack build --target web
python -m http.server
```
