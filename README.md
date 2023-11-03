## Building & Running
Make sure you have installed the `wasm32-unknown-unknown` target using rustup:
```
rustup target add wasm32-unknown-unknown
```

Then, to build a cart.wasm file, run:

```
cargo build --release
```

To start and load
```
tic80 --fs . --cmd 'load game.wasmp & import binary target/wasm32-unknown-unknown/release/cart.wasm'
```

Once the TIC-80 is loaded you can just reimport the wasm file after it was compiled with
```
import binary target/wasm32-unknown-unknown/release/cart.wasm
```

When saving, make sure to save to `game.wasmp`

# Notes from the original template

## wasm-opt
It is highly recommended that you run `wasm-opt` on the output `cart.wasm` file, especially if using the usual unoptimised builds. To do so, make sure `wasm-opt` is installed, then run:
```
wasm-opt -Os target/wasm32-unknown-unknown/release/cart.wasm -o cart.wasm
```
This will create a new, smaller `cart.wasm` file in the working directory.

## Important Note
Don't access TIC-80's I/O memory by dereferencing raw pointers. The optimiser will ruin attempts to do this, because Rust has no equivalent to C's `volatile` for direct access. Instead, use [`std::ptr::read_volatile`](https://doc.rust-lang.org/std/ptr/fn.read_volatile.html) and [`std::ptr::write_volatile`](https://doc.rust-lang.org/std/ptr/fn.write_volatile.html), or just use the standard TIC-80 `peek`/`poke`.