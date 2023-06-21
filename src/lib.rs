use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/src/helpers.ts")]
extern "C" {
    fn randomInt(max: usize) -> usize;
}

#[derive(PartialEq)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum SnakeStatus {
    Playing,
    Failed,
    Finished,
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
    free_idxs: HashSet<usize>,
    status: Option<SnakeStatus>,
}

#[wasm_bindgen]
impl World {
    pub fn new(
        world_length: usize,
        snake_idx: usize,
        direction: Direction,
        snake_length: usize,
    ) -> World {
        let size = world_length.pow(2);
        let mut free_idxs: HashSet<usize> = HashSet::from_iter(0..size);
        let mut body = vec![SnakeCell(snake_idx)];

        free_idxs.remove(&snake_idx);

        for i in 1..snake_length {
            let body_cell = (snake_idx / world_length * world_length).rem_euclid(size)
                + (snake_idx - i).rem_euclid(world_length);
            body.push(SnakeCell(body_cell));
            free_idxs.remove(&body_cell);
        }

        World {
            length: world_length as isize,
            size: size as isize,
            next_cell_idx: None,
            snake: Snake { body, direction },
            reward_idx: free_idxs.iter().copied().collect::<Vec<usize>>()
                [randomInt(free_idxs.len())],
            free_idxs,
            status: None,
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
        let next_cell = self.calc_snake_next_position(&direction);

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
        if self.status == Some(SnakeStatus::Playing) {
            self.play_step();
        }
    }

    fn play_step(&mut self) {
        let prev_idx = self.snake.body[0].0;
        let prev_tail = self.snake.body.last().unwrap().0;

        self.move_snake_head();
        self.move_snake_body(prev_idx, prev_tail);

        if self.snake.body[0].0 == self.reward_idx {
            self.eat_reward(prev_tail);
        };

        self.status = Some(SnakeStatus::Playing);
    }

    fn move_snake_head(&mut self) {
        match self.next_cell_idx {
            Some(idx) => {
                self.snake.body[0].0 = idx;
                self.next_cell_idx = None;
            }
            None => {
                self.snake.body[0].0 = self.calc_snake_next_position(&self.snake.direction);
            }
        }
    }

    fn eat_reward(&mut self, prev_tail: usize) {
        self.snake.body.push(SnakeCell(prev_tail));
        self.free_idxs.remove(&prev_tail);

        match self.new_reward_pos() {
            Some(reward_cell) => self.reward_idx = reward_cell,
            None => self.status = Some(SnakeStatus::Finished),
        };
    }

    fn calc_snake_next_position(&self, direction: &Direction) -> usize {
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

    fn move_snake_body(&mut self, prev_head_idx: usize, prev_tail: usize) {
        use std::mem::swap;
        let snake_length = self.snake.body.len();
        let mut prev_idx = prev_head_idx;

        for i in 1..snake_length {
            swap(&mut self.snake.body[i].0, &mut prev_idx)
        }

        self.free_idxs.insert(prev_tail);
        self.free_idxs.remove(&self.snake.body[0].0);
    }

    fn new_reward_pos(&mut self) -> Option<usize> {
        if !self.free_idxs.is_empty() {
            Some(
                self.free_idxs.iter().copied().collect::<Vec<usize>>()
                    [randomInt(self.free_idxs.len())],
            )
        } else {
            None
        }
    }
}

// cargo install wasm-pack --force --target x86_64-unknown-linux-musl
//  wasm-pack build --target web
