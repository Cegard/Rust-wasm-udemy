use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub struct World {
    size: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new() -> World {
        return World {
            size: 8
        };
    }

    pub fn size(&self) -> usize {
        return self.size;
    }
}

// cargo install wasm-pack --force --target x86_64-unknown-linux-musl
//  wasm-pack build --target web