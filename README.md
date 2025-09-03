git clone https://github.com/emscripten-core/emsdk.git
./emsdk install latest
./emsdk activate latest

emcc simple.cpp -o simple.wasm -s STANDALONE_WASM=1 -s EXPORTED_FUNCTIONS="['_add']" -Wl,--no-entry
wasm2wat simple.wasm -o simple.wat

cargo run
