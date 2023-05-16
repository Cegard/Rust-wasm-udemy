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
        let based_16_size: i8 = self.size.try_into().unwrap();
        let get_next_idx = |tiles_to_move: i8| ((self.get_snake_head() as i8 + tiles_to_move).rem_euclid(based_16_size.pow(2))) as usize;
        println!("{}", self.snake.body[0].0);

        match self.snake.direction {
            Direction::Up => self.snake.body[0].0 = get_next_idx(-based_16_size),
            Direction::Right => self.snake.body[0].0 = get_next_idx(1) ,
            Direction::Down => self.snake.body[0].0 = get_next_idx(based_16_size),
            Direction::Left => self.snake.body[0].0 = get_next_idx(-1),
        }
    }
}



// cargo install wasm-pack --force --target x86_64-unknown-linux-musl
//  wasm-pack build --target web