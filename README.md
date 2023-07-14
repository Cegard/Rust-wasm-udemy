# Rust-wasm-udemy
Repository which goes through the course Rust &amp; WebAssembly with JS (TS) - The Practical Guide.

You can go through the course's lessons via the commit history (Most of them specify the lesson number).

## Requirements
- Rust
- cargo
- npm
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

## Steps for running

- Install project dependencies for rust `cargo build`.
- Compile the Rust code into WASM `wasm-pack build --target web`.
- Install npm dependencies `npm i` from the root folder of the project and from the `www` folder.
- From the `www` folder run `npm build`.
- From the root folder of the project run `npm start`.
- Visit [localhost:3000](http://localhost:3000).
