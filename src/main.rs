#![allow(dead_code)]
use piston_window::*;
// mod game;
// mod drawing;
// use game::Game;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 600.0;
fn main() {
    let mut window: PistonWindow = WindowSettings::new("Space Shooter", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let (width, height) = (640.0, 480.0);
    let mut game = Game::new(width, height);
}
