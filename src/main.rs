extern crate piston_window;
extern crate find_folder;
extern crate rand;

use piston_window::*;

mod game;
mod snake;

const BACK_COLOR: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake Fucker!", [600; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut snake = snake::Snake::new(0.0, 60.0);
    let mut game = game::Game::new();
    game.span_food();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw_food(c.transform, g);
            snake.draw(c.transform, g);

            if !snake.is_life {
                game.draw_die(c.transform, g);
            }
        });

        if let Some(Button::Keyboard(key)) = event.press_args() {
            snake.handle_key(key);

            if !snake.is_life && key == Key::Space {
                snake = snake::Snake::new(0.0, 60.0);
                game = game::Game::new();
            }
        }

        event.update(|arg| {
            game.step_time += arg.dt;

            if game.is_game_step() && snake.is_life {
                snake.snake_move();
                if snake.check_eat(game.food.x, game.food.y) {
                    game.span_food();
                }

                game.step_time = 0.0;
            }
        });
    }
}
