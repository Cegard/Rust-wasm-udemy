use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn greet(name: &str) {
    println!("Hello {}", name);
}

// cargo install wasm-pack --force --target x86_64-unknown-linux-musl
//  wasm-pack build --target web