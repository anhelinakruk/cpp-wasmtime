# WASM Runner

Test project for running WebAssembly modules with Wasmtime.

## Prerequisites

1. Install Rust and Cargo
2. Install Emscripten SDK:

```bash
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install latest
./emsdk activate latest
source ./emsdk_env.sh
```

## Building WASM module

Create a C++ file with a simple function:

```bash
# Compile C++ to WASM
emcc simple.cpp -o simple.wasm -s STANDALONE_WASM=1 -s EXPORTED_FUNCTIONS="['_add']" -Wl,--no-entry

# Convert WASM to WAT for inspection (optional)
wasm2wat simple.wasm -o simple.wat
```

Place the generated `simple.wat` file in the `src/` directory.

## Running

```bash
cargo run
```
