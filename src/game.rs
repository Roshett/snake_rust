use piston_window::*;
use rand::Rng;

const SPEED: f64 = 0.1;
const FOOD_SIZE: f64 = 30.0;
const FOOD_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct Game {
    pub step_time: f64,
    pub food: Food,
}

pub struct Food {
    pub x: f64,
    pub y: f64,
    pub size: f64,
}

impl Game {
    pub fn new() -> Game {
        let mut food = Food {x: 90.0, y: 90.0, size: FOOD_SIZE};
        Game { step_time: 0.0, food}
    }

    pub fn is_game_step(&self) -> bool {
        self.step_time > SPEED
    }

    pub fn span_food(&mut self) {
        let mut rng = rand::thread_rng();
        let x  = rng.gen_range(0, 20) * 30;
        let y  = rng.gen_range(0, 20) * 30;
        
        self.food.x = x as f64;
        self.food.y = y as f64;

        println!("{} {}", self.food.x, self.food.y);
    }

    pub fn draw_food<G: Graphics>(&self, t: math::Matrix2d,  g: &mut G) {
        rectangle(FOOD_COLOR, [self.food.x, self.food.y, self.food.size, self.food.size], t, g);
    }
}