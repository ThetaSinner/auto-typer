# auto-typer

A utility for automatically typing text.

### Building

```sh
cd cbinding
cargo install --force cbindgen
cbindgen --config cbindgen.toml --crate cbinding --output autotyper.h
cd ..
cargo build --release
```

### Testing the `cbinding` library

```sh
cd cbinding
cargo build --release --lib
clang -framework CoreFoundation -framework CoreGraphics -L ../target/release -lautotyper test.c -o test.out
./test.out
```

Press `Ctrl+C` to tidy up your prompt after the program finishes typing.
