use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

enum Direction {
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
}

#[wasm_bindgen]
impl World {
    pub fn new(world_size: usize, snake_idx: usize, direction: usize) -> World {
        let snake_direction: Direction;

        match direction {
            0 => snake_direction = Direction::Up,
            1 => snake_direction = Direction::Right,
            2 => snake_direction = Direction::Down,
            3 => snake_direction = Direction::Left,
            _ => panic!("Option not covered")
        }

        return World {
            size: world_size,
            snake: Snake::new(snake_idx, snake_direction)
        };
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn get_snake_head(&self) -> usize {
        return self.snake.body[0].0;
    }

    pub fn update(&mut self) {
        let signed_size: isize = self.size as isize;
        let signed_snake_idx: isize = self.snake.body[0].0 as isize;

        let get_next_idx = |rows_to_move: isize, cols_to_move: isize|
            (
                (signed_size * ((signed_snake_idx / signed_size) + rows_to_move))
                .rem_euclid(signed_size.pow(2))
                + (signed_snake_idx + cols_to_move).rem_euclid(signed_size)
            ) as usize;

        match self.snake.direction {
            Direction::Up => self.snake.body[0].0 = get_next_idx(-1, 0),
            Direction::Right => self.snake.body[0].0 = get_next_idx(0, 1) ,
            Direction::Down => self.snake.body[0].0 = get_next_idx(1, 0),
            Direction::Left => self.snake.body[0].0 = get_next_idx(0, -1),
        }
    }
}



// cargo install wasm-pack --force --target x86_64-unknown-linux-musl
//  wasm-pack build --target web