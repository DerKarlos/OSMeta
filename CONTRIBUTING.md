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

## Build for windows (on Mac/Linux)

brew install mingw-w64
rustup target add x86_64-pc-windows-gnu
cargo clippy --fix --allow-dirty --target x86_64-pc-windows-gnu
cargo fmt
cargo check --target x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu
error: linking with `x86_64-w64-mingw32-gcc` failed: exit status: 1
cargo run --target x86_64-pc-windows-gnu   (realy?)
