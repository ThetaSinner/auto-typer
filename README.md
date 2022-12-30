# auto-typer

A utility for automatically typing text.

### Building the project

#### CLI only (Window and Linux)

To build the CLI only you can just build the Rust portion of the project

```shell
cargo build --release
```

#### MacOS app

For any API changes you need to re-generate the `cbinding/autotyper.h` header by doing

```shell
cd cbinding
cargo install --force cbindgen
cbindgen --config cbindgen.toml --crate cbinding --output autotyper.h
cd ..
cargo build --release
```

Otherwise you can just build the static C library

```shell
cargo build --release
```

Then you can build the app by opening `auto-typer-macos` in XCode and following these steps

1. `Product` -> `Archive`
2. `Distribute App` -> `Copy App` -> `Next`
3. Choose a directory to copy to
4. Copy the app from the output directory to the `Applications` directory
5. `System Settings` -> `Privacy and Security` -> `Accessibility` -> `+`
6. Find the auto-typer app in `Applications`

The application is then ready to use.

_Note_ the application cannot be used properly in a debug build through XCode because you need accessibility permitted for the app. 
You can use all features other than the keyboard shortcut.

### Testing the `cbinding` library

```shell
cd cbinding
cargo build --release --lib
clang -framework CoreFoundation -framework CoreGraphics -L ../target/release -lautotyper test.c -o test.out
./test.out
```

Press `Ctrl+C` to tidy up your prompt after the program finishes typing.
