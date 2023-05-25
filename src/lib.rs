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

pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction
}

#[wasm_bindgen]
pub struct World {
    length: isize,
    snake: Snake
}

impl Snake {
    fn new(spawn_idx: usize, direction: Direction, length: usize) -> Snake {

        return Snake {
            body: (0..length.max(1)).map(|i| SnakeCell(spawn_idx - i)).collect(),
            direction
        };
    }

    fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn get_head_idx(&self) -> usize {
        
        return self.body[0].0;
    }

    fn set_head(&mut self, head: SnakeCell) {
        self.body[0] = head;
    }
}

#[wasm_bindgen]
impl World {
    pub fn new(world_length: usize, snake_idx: usize, direction: Direction, snake_length: usize) -> World {
        
        return World {
            length: world_length as isize,
            snake: Snake::new(snake_idx, direction, snake_length)
        };
    }

    pub fn length(&self) -> usize {
        
        return self.length as usize;
    }

    pub fn get_snake_head(&self) -> usize {

        return self.snake.get_head_idx();
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        self.snake.change_direction(direction);
    }

    pub fn get_snake_length(&self) -> usize {

        return self.snake.body.len();
    }

    // *const is a raw pointer, not a reference (&)
    // borrrowing rules don't apply here
    pub fn get_snake_cells(&self) -> *const SnakeCell {

        return self.snake.body.as_ptr(); 
    }

    pub fn step(&mut self) {
        let last_idx = self.length.pow(2);
        let snake_head_idx = self.snake.get_head_idx() as isize;
        let vert_thresholds = [-1, last_idx];
        let hor_thresholds = [
            snake_head_idx/self.length * self.length - 1,
            snake_head_idx/self.length * self.length + self.length
        ];

        self.snake.set_head(SnakeCell(match self.snake.direction {
            Direction::Up => ((snake_head_idx - self.length) > vert_thresholds[0])
                             .then_some((snake_head_idx - self.length) as usize)
                             .unwrap_or((last_idx - self.length + snake_head_idx) as usize),
            Direction::Right => ((snake_head_idx + 1) < hor_thresholds[1])
                                .then_some(self.snake.get_head_idx() + 1)
                                .unwrap_or((snake_head_idx + 1 - self.length) as usize),
            Direction::Down => ((snake_head_idx + self.length) < vert_thresholds[1])
                               .then_some((snake_head_idx + self.length) as usize)
                               .unwrap_or((snake_head_idx + self.length - last_idx) as usize),
            Direction::Left => ((snake_head_idx - 1) > hor_thresholds[0])
                               .then_some(self.snake.get_head_idx() - 1)
                               .unwrap_or((snake_head_idx + self.length - 1) as usize)
        }));
    }
}



// cargo install wasm-pack --force --target x86_64-unknown-linux-musl
//  wasm-pack build --target web