emcc main.c \
-o main.js \
-s WASM=1 \
-s EXPORTED_FUNCTIONS='["_sumArray", "_malloc", "_free", "_sumFloat64Array" ]' \
-s EXTRA_EXPORTED_RUNTIME_METHODS='["ccall","cwrap","getValue","setValue"]'