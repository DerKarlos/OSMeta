## Build for android

On windows hosts, install `llvm` (contains `clang`) from https://github.com/llvm/llvm-project/releases/

```
cargo install --git https://github.com/NiklasEi/xbuild
```

run `x doctor` to see what else you need to install for android, then run

```
x build --release --platform android --format apk --arch arm64
```
