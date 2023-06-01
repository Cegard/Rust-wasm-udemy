use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/src/helpers.ts")]
extern "C" {
    fn now() -> usize;
}

pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

struct CellsIdxsCalculator {
    calc_cells_idxs: Box<dyn Fn(isize) -> usize + 'static>,
}

#[wasm_bindgen]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[wasm_bindgen]
pub struct World {
    length: isize,
    size: isize,
    snake: Snake,
    next_cell_idx: Option<usize>,
    reward_idx: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(
        world_length: usize,
        snake_idx: usize,
        direction: Direction,
        snake_length: usize,
    ) -> World {
        let mut body: Vec<SnakeCell> = vec![SnakeCell(snake_idx)];
        let idx_calculator = World::get_idxs_calculator(snake_idx, snake_length, world_length);
        let size = world_length.pow(2);

        for i in 1..(snake_length as isize) {
            body.push(SnakeCell((idx_calculator.calc_cells_idxs)(i)));
        }

        World {
            length: world_length as isize,
            size: size as isize,
            next_cell_idx: None,
            snake: Snake { body, direction },
            reward_idx: now() % size,
        }
    }

    pub fn get_reward_idx(&self) -> usize {
        self.reward_idx
    }

    pub fn length(&self) -> usize {
        self.length as usize
    }

    pub fn get_snake_head(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        let next_cell = self.move_snake(&direction);

        if self.snake.body.len() < 2 || next_cell != self.snake.body[1].0 {
            self.snake.direction = direction;
            self.next_cell_idx = Some(next_cell);
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
        let prev_idx = self.snake.body[0].0;

        match self.next_cell_idx {
            Some(idx) => {
                self.snake.body[0].0 = idx;
                self.next_cell_idx = None;
            }
            None => {
                self.snake.body[0].0 = self.move_snake(&self.snake.direction);
            }
        }

        self.move_snake_body(prev_idx);
    }

    fn move_snake(&self, direction: &Direction) -> usize {
        let snake_head_idx = self.snake.body[0].0 as isize;

        match direction {
            Direction::Up => self.calc_next_idx(
                (snake_head_idx - self.length) > -1,
                snake_head_idx,
                -self.length,
                self.size - self.length,
            ),

            Direction::Right => self.calc_next_idx(
                (snake_head_idx + 1) < snake_head_idx / self.length * self.length + self.length,
                snake_head_idx,
                1,
                1 - self.length,
            ),

            Direction::Down => self.calc_next_idx(
                (snake_head_idx + self.length) < self.size,
                snake_head_idx,
                self.length,
                self.length - self.size,
            ),

            Direction::Left => self.calc_next_idx(
                (snake_head_idx - 1) > snake_head_idx / self.length * self.length - 1,
                snake_head_idx,
                -1,
                self.length - 1,
            ),
        }
    }

    fn calc_next_idx(
        &self,
        is_inside: bool,
        snake_head_idx: isize,
        cells_to_move: isize,
        reset_value: isize,
    ) -> usize {
        if is_inside {
            (snake_head_idx + cells_to_move) as usize
        } else {
            (snake_head_idx + reset_value) as usize
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
        world_length: usize,
    ) -> CellsIdxsCalculator {
        let i_snake_idx = snake_idx as isize;

        if snake_length <= (snake_idx % world_length) + 1 {
            CellsIdxsCalculator {
                calc_cells_idxs: Box::new(move |i: isize| (i_snake_idx - i) as usize),
            }
        } else {
            let row = snake_idx / world_length * world_length;
            let i_world_length = world_length as isize;

            CellsIdxsCalculator {
                calc_cells_idxs: Box::new(move |i: isize| {
                    row + (i_snake_idx - i).rem_euclid(i_world_length) as usize
                }),
            }
        }
    }
}

// cargo install wasm-pack --force --target x86_64-unknown-linux-musl
//  wasm-pack build --target web
