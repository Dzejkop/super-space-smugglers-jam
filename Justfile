run:
    cargo build --release

    tic80 \
        --skip \
        --fs . \
        --cmd 'load game.wasmp & import binary target/wasm32-unknown-unknown/release/cart.wasm & run'

build:
    cargo build --release
    rm -rf build
    mkdir build
    just build-export win build/super-space-smugglers.exe
    just build-export linux build/super-space-smugglers-linux
    just build-export mac build/super-space-smugglers-mac
    just build-export html build/super-space-smugglers

[private]
build-export platform output:
    tic80 \
        --cli \
        --skip \
        --fs . \
        --cmd 'load game.wasmp & import binary target/wasm32-unknown-unknown/release/cart.wasm & export {{ platform }} {{ output }} alone=1 & save & exit'
