use piston_window::Key;
use piston_window::*;

const SNAKE_COLOR: [f32; 4] = [0.905, 0.298, 0.235, 1.0];
const SNAKE_SIZE: f64 = 30.0;

pub struct Snake {
    pub blocks: Vec<Block>,
    pub is_life: bool,
}

impl Snake {
    pub fn new(x: f64, y: f64) -> Snake {
        Snake {
            blocks: vec![
                Block {
                    x: x,
                    y: y,
                    size: SNAKE_SIZE,
                    direction: Direction::Down,
                },
                Block {
                    x: x,
                    y: y - 30.0,
                    size: SNAKE_SIZE,
                    direction: Direction::Down,
                },
                Block {
                    x: x,
                    y: y - 60.0,
                    size: SNAKE_SIZE,
                    direction: Direction::Down,
                },
            ],
            is_life: true,
        }
    }

    pub fn down(&mut self, block_index: &usize) -> f64 {
        self.blocks[*block_index].y = self.blocks[*block_index].y + SNAKE_SIZE;
        0.0
    }

    pub fn up(&mut self, block_index: &usize) -> f64 {
        self.blocks[*block_index].y = self.blocks[*block_index].y - SNAKE_SIZE;
        0.0
    }

    pub fn left(&mut self, block_index: &usize) -> f64 {
        self.blocks[*block_index].x = self.blocks[*block_index].x - SNAKE_SIZE;
        0.0
    }

    pub fn right(&mut self, block_index: &usize) -> f64 {
        self.blocks[*block_index].x = self.blocks[*block_index].x + SNAKE_SIZE;
        0.0
    }

    pub fn snake_move(&mut self) {
        let mut next_move: Direction = Direction::Right;
        let mut buff_next_move: Direction = Direction::Right;
        
        for i in 0..self.blocks.len() {
            match &self.blocks[i].direction {
                Direction::Right => self.right(&i),
                Direction::Left => self.left(&i),
                Direction::Up => self.up(&i),
                Direction::Down => self.down(&i),
            };

            if i == 0 {
                match &self.blocks[i].direction {
                    Direction::Right => next_move = Direction::Right,
                    Direction::Left => next_move = Direction::Left,
                    Direction::Up => next_move = Direction::Up,
                    Direction::Down => next_move = Direction::Down,
                };
            } else {
                match &self.blocks[i].direction {
                    Direction::Right => buff_next_move = Direction::Right,
                    Direction::Left => buff_next_move = Direction::Left,
                    Direction::Up => buff_next_move = Direction::Up,
                    Direction::Down => buff_next_move = Direction::Down,
                };

                match next_move {
                    Direction::Right => self.blocks[i].direction = Direction::Right,
                    Direction::Left => self.blocks[i].direction = Direction::Left,
                    Direction::Up => self.blocks[i].direction = Direction::Up,
                    Direction::Down => self.blocks[i].direction = Direction::Down,
                }

                next_move = buff_next_move;
            }
        }

        self.is_life = !self.check_collision();
    }

    pub fn check_collision(&mut self) -> bool {
        let mut is_collision = false;

        let x = self.blocks[0].x;
        let y = self.blocks[0].y;

        if(x < 0.0 || x > 600.0 || y < 0.0 || y > 600.0) {
            is_collision = true;
        }

        for i in 1..self.blocks.len() {
            if self.blocks[i].x == x && self.blocks[i].y == y {
                is_collision = true;
                break;
            } 
        }


        is_collision
    }

    pub fn handle_key(&mut self, k: Key) {
        match k {
            piston_window::Key::A => self.blocks[0].direction = Direction::Left,
            piston_window::Key::D => self.blocks[0].direction = Direction::Right,
            piston_window::Key::W => self.blocks[0].direction = Direction::Up,
            piston_window::Key::S => self.blocks[0].direction = Direction::Down,
            _ => (),
        }
    }

    pub fn draw<G: Graphics>(&self, t: math::Matrix2d,  g: &mut G) {
        for i in &self.blocks {
            rectangle(SNAKE_COLOR, [i.x, i.y, i.size, i.size], t, g);
        }
    }

    pub fn check_eat(&mut self, x: f64, y: f64) -> bool {
        let mut is_check = false;

        if(self.blocks[0].x == x && self.blocks[0].y == y) {
            let mut x = self.blocks[self.blocks.len() - 1].x;
            let mut y = self.blocks[self.blocks.len() - 1].y;
            let direction = self.blocks[self.blocks.len() - 1].direction.clone();

            match direction {
                Direction::Up => y += SNAKE_SIZE,
                Direction::Down => y -= SNAKE_SIZE,
                Direction::Left => x += SNAKE_SIZE,
                Direction::Right => x -= SNAKE_SIZE,
            }

            &self.blocks.push(Block {
                x: x,
                y: y,
                size: SNAKE_SIZE,
                direction: direction,
            });

            is_check = true;
        }

        is_check
    }
}

pub struct Block {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub direction: Direction,
}

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}
