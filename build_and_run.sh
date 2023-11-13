set -e
deno run -A src/dynamically_created.js
deno run -A https://raw.githubusercontent.com/jonasfrey/code_autoextender/main/code_autoextender.js
deno run -A https://raw.githubusercontent.com/jonasfrey/code_autoextender/main/code_autoextender.js src/dynamically_created.rs src/dynamically_created_autoextended.rs
cargo fmt -- src/main_autoextended.rs
cargo build
wasm-pack build --target=web
# deno run -A main.js