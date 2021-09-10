mod game_state;

// third party imports
use ggez::{event, GameResult};

// local imports
use game_state::GameState;

const SCREEN_HEIGHT: f32 = 600.0;
const SCREEN_WIDTH: f32 = 800.0;

fn main() -> GameResult {
    let (ctx, events_loop) = ggez::ContextBuilder::new("asteroid-dodge", "redmannequin")
        .window_setup(ggez::conf::WindowSetup::default().title("AsteroidDodge"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;

    let game_state = GameState::new();
    event::run(ctx, events_loop, game_state)
}
