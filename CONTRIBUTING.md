## Build for android

On windows hosts, install `llvm` (contains `clang`) from https://github.com/llvm/llvm-project/releases/

```
cargo install --git https://github.com/NiklasEi/xbuild
```

run `x doctor` to see what else you need to install for android, then run

```
x build --release --platform android --format apk --arch arm64
```
## wasm

To run the project in the browser locally, you need to do the following setup:

```bash
rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner
cargo run --target wasm32-unknown-unknown
```
