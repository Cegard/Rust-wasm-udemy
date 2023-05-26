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
    fn new(spawn_idx: usize, direction: Direction, length: usize, world_length: usize) -> Snake {
        let mut body = Vec::<SnakeCell>::new();
        let row = spawn_idx/world_length * world_length;
        let i_spawn_idx = spawn_idx as isize;
        let i_world_length = world_length as isize;

        for i in 0..(length as isize).max(1) {
            body.push(SnakeCell(row + (i_spawn_idx - i).rem_euclid(i_world_length) as usize));
        }

        return Snake {
            body,
            direction
        };
    }
}

#[wasm_bindgen]
impl World {
    pub fn new(world_length: usize, snake_idx: usize, direction: Direction, snake_length: usize) -> World {
        
        return World {
            length: world_length as isize,
            snake: Snake::new(snake_idx, direction, snake_length, world_length)
        };
    }

    pub fn length(&self) -> usize {
        
        return self.length as usize;
    }

    pub fn get_snake_head(&self) -> usize {

        return self.snake.body[0].0;
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        self.snake.direction = direction;
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
        let snake_head_idx = self.snake.body[0].0 as isize;
        let vert_thresholds = [-1, last_idx];
        let hor_thresholds = [
            snake_head_idx/self.length * self.length - 1,
            snake_head_idx/self.length * self.length + self.length
        ];

        self.snake.body[0].0 = match self.snake.direction {
            Direction::Up => if (snake_head_idx - self.length) > vert_thresholds[0]
                            { (snake_head_idx - self.length) as usize }
                            else { (last_idx - self.length + snake_head_idx) as usize }
            Direction::Right => if (snake_head_idx + 1) < hor_thresholds[1]
                                { self.snake.body[0].0 + 1 }
                                else { (snake_head_idx + 1 - self.length) as usize }
            Direction::Down => if (snake_head_idx + self.length) < vert_thresholds[1]
                            { (snake_head_idx + self.length) as usize }
                            else { (snake_head_idx + self.length - last_idx) as usize }
            Direction::Left => if (snake_head_idx - 1) > hor_thresholds[0]
                            { self.snake.body[0].0 - 1 }
                            else { (snake_head_idx + self.length - 1) as usize }
        };

        self.move_snake_body(snake_head_idx as usize);
    }

    fn move_snake_body(&mut self, prev_head_idx: usize) {
        let snake_length = self.snake.body.len();
        let mut prev_idx = prev_head_idx;

        for i in 1..snake_length {
            let temp_idx = self.snake.body[i].0;
            self.snake.body[i].0 = prev_idx;
            prev_idx = temp_idx;
        }
    }
}



// cargo install wasm-pack --force --target x86_64-unknown-linux-musl
//  wasm-pack build --target web