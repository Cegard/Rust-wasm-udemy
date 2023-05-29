use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction
}

struct CellsIdxsCalculator {
    calc_cells_idxs: Box<dyn Fn(isize) -> usize + 'static>,
}

#[wasm_bindgen]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[wasm_bindgen]
pub struct World {
    length: isize,
    snake: Snake
}

#[wasm_bindgen]
impl World {
    pub fn new(
        world_length: usize,
        snake_idx: usize,
        direction: Direction,
        snake_length: usize
    ) -> World {
        let mut body: Vec<SnakeCell> = vec![SnakeCell(snake_idx)];
        let idx_calculator = World::get_idxs_calculator(snake_idx, snake_length, world_length);

        for i in 1..(snake_length as isize) {
            body.push(SnakeCell((idx_calculator.calc_cells_idxs)(i)));
        }

        World {
            length: world_length as isize,
            snake: Snake {
                body,
                direction
            }
        }
    }

    pub fn length(&self) -> usize {
        self.length as usize
    }

    pub fn get_snake_head(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {

        if self.snake.body.len() < 2 || self.calc_next_idx(&direction) != self.snake.body[1].0 {
            self.snake.direction = direction;
        }
    }

    pub fn get_snake_length(&self) -> usize {
        self.snake.body.len()
    }

    // *const is a raw pointer, not a reference (&)
    // borrrowing rules don't apply here
    pub fn get_snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn step(&mut self) {
        let curr_idx = self.snake.body[0].0;
        self.snake.body[0] = SnakeCell(self.calc_next_idx(&self.snake.direction));
        self.move_snake_body(curr_idx);
    }

    fn calc_next_idx(&self, direction: &Direction) -> usize {
        let last_idx = self.length.pow(2);
        let snake_head_idx = self.snake.body[0].0 as isize;
        let vert_thresholds = [-1, last_idx];
        let hor_thresholds = [
            snake_head_idx/self.length * self.length - 1,
            snake_head_idx/self.length * self.length + self.length
        ];
        
        match direction {
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
        }
    }

    fn move_snake_body(&mut self, prev_head_idx: usize) {
        use std::mem::swap;
        let snake_length = self.snake.body.len();
        let mut prev_idx = prev_head_idx;

        for i in 1..snake_length {
            swap(&mut self.snake.body[i].0, &mut prev_idx)
        }
    }

    fn get_idxs_calculator(
        snake_idx: usize,
        snake_length: usize,
        world_length: usize
    ) -> CellsIdxsCalculator {
        let i_snake_idx = snake_idx as isize;

        if snake_length <= (snake_idx % world_length) + 1 {
            
            CellsIdxsCalculator {
                calc_cells_idxs: Box::new(move |i: isize| (i_snake_idx - i) as usize)
            }
        } else {
            let row = snake_idx/world_length * world_length;
            let i_world_length = world_length as isize;

            CellsIdxsCalculator {
                calc_cells_idxs: Box::new(
                    move |i: isize| row + (i_snake_idx - i).rem_euclid(i_world_length) as usize
                )
            }
        }
    }
}



// cargo install wasm-pack --force --target x86_64-unknown-linux-musl
//  wasm-pack build --target web