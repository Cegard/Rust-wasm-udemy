use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

struct SnakeSection(usize);

struct Snake {
    body: Vec<SnakeSection>,
    direction: Direction
}

#[wasm_bindgen]
pub struct World {
    size: usize,
    snake: Snake
}

impl Snake {
    fn new(spawn_idx: usize, direction: Direction) -> Snake {
        return Snake {
            body: vec!(SnakeSection(spawn_idx)),
            direction
        };
    }

    fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn get_head_idx(&self) -> usize {
        
        return self.body[0].0;
    }

    fn set_head_idx(&mut self, idx: usize) {
        self.body[0].0 = idx;
    }
}

#[wasm_bindgen]
impl World {
    pub fn new(world_size: usize, snake_idx: usize, direction: Direction) -> World {

        return World {
            size: world_size,
            snake: Snake::new(snake_idx, direction)
        };
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn get_snake_head(&self) -> usize {
        return self.snake.get_head_idx();
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        self.snake.change_direction(direction);
    }

    pub fn update(&mut self) {
        let signed_size: isize = self.size as isize;
        let signed_snake_idx: isize = self.snake.get_head_idx() as isize;

        let get_next_idx = |rows_to_move: isize, cols_to_move: isize|
            (
                (signed_size * ((signed_snake_idx / signed_size) + rows_to_move))
                .rem_euclid(signed_size.pow(2))
                + (signed_snake_idx + cols_to_move).rem_euclid(signed_size)
            ) as usize;

        let next_snake_idx = match self.snake.direction {
            Direction::Up => get_next_idx(-1, 0),
            Direction::Right => get_next_idx(0, 1) ,
            Direction::Down => get_next_idx(1, 0),
            Direction::Left => get_next_idx(0, -1),
        };

        self.snake.set_head_idx(next_snake_idx);
    }
}



// cargo install wasm-pack --force --target x86_64-unknown-linux-musl
//  wasm-pack build --target web