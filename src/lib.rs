use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

struct SnakeSection(usize);

struct Snake {
    body: Vec<SnakeSection>
}

#[wasm_bindgen]
pub struct World {
    size: usize,
    snake: Snake
}

impl Snake {
    fn new(spawn_idx: usize) -> Snake {
        return Snake { body: vec!(SnakeSection(spawn_idx)) };
    }
}

#[wasm_bindgen]
impl World {
    pub fn new() -> World {
        return World {
            size: 8,
            snake: Snake::new(10)
        };
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn get_snake_head(&self) -> usize {
        return self.snake.body[0].0;
    }

    pub fn update(&mut self) {
        self.snake.body[0].0 = (self.get_snake_head() + 1) % self.size.pow(2);
    }
}



// cargo install wasm-pack --force --target x86_64-unknown-linux-musl
//  wasm-pack build --target web