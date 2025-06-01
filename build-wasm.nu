cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "bevy-jam-6" ./target/wasm32-unknown-unknown/release/bevy-jam-6.wasm