run:
    cargo build --release
    tic80 --skip --fs . --cmd 'load game.wasmp & import binary target/wasm32-unknown-unknown/release/cart.wasm & run'
