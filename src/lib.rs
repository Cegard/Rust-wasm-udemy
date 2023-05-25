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
    size: isize,
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
    pub fn new(world_size: usize, snake_idx: usize, direction: Direction, snake_length: usize) -> World {
        
        return World {
            size: world_size as isize,
            snake: Snake::new(snake_idx, direction, snake_length)
        };
    }

    pub fn size(&self) -> usize {
        
        return self.size as usize;
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
        self.snake.set_head(SnakeCell(match self.snake.direction {
            Direction::Up => self.calc_next_cell(-1, 0, self.get_snake_head() as isize),
            Direction::Right => self.calc_next_cell(0, 1, self.get_snake_head() as isize),
            Direction::Down => self.calc_next_cell(1, 0, self.get_snake_head() as isize),
            Direction::Left => self.calc_next_cell(0, -1, self.get_snake_head() as isize),
        }));
    }

    fn calc_next_cell(&self, rows_to_move: isize, cols_to_move: isize, snake_idx: isize) -> usize {
        return (
            (self.size * ((snake_idx / self.size) + rows_to_move))
            .rem_euclid(self.size.pow(2))
            + (snake_idx + cols_to_move).rem_euclid(self.size)
        ) as usize;
    }
}



// cargo install wasm-pack --force --target x86_64-unknown-linux-musl
//  wasm-pack build --target web