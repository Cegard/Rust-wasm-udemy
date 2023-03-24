use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(format!("Hello {}!", name).as_str());
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

// cargo install wasm-pack --force --target x86_64-unknown-linux-musl
//  wasm-pack build --target web