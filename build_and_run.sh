cargo build
deno run -A https://raw.githubusercontent.com/jonasfrey/code_autoextender/main/code_autoextender.js
wasm-pack build --target=web
# deno run -A main.js