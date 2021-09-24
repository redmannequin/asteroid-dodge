use ggez::event::KeyCode;
use ggez::{event, graphics, input, timer, Context, GameResult};
use mint::Point2;
use rand::prelude::*;

const SCREEN_HEIGHT: f32 = 600.0;
const SCREEN_WIDTH: f32 = 800.0;
const TARGET_FPS: u32 = 60;

pub struct GameState {
    players: Vec<Player>,
}

impl GameState {
    pub fn new() -> Self {
        const N: usize = 100;
        let mut rng = rand::thread_rng();
        let mut players = Vec::with_capacity(N);
        for _ in 0..N {
            players.push(Player::new(
                rng.gen_range(0.0..SCREEN_WIDTH),
                rng.gen_range(0.0..SCREEN_HEIGHT),
                &mut rng,
            ))
        }
        GameState { players }
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, TARGET_FPS) {
            if input::keyboard::is_key_pressed(ctx, KeyCode::Up) {
            } else if input::keyboard::is_key_pressed(ctx, KeyCode::Down) {
            }

            if input::keyboard::is_key_pressed(ctx, KeyCode::Left) {
            } else if input::keyboard::is_key_pressed(ctx, KeyCode::Right) {
            }

            for p in &mut self.players {
                p.update();
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        for p in &self.players {
            p.draw(ctx)?;
        }
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }
}

struct Player {
    x_pos: f32,
    y_pos: f32,
    x_speed: f32,
    y_speed: f32,
}

impl Player {
    fn new(x_pos: f32, y_pos: f32, rng: &mut ThreadRng) -> Self {
        Player {
            x_pos,
            y_pos,
            x_speed: rng.gen_range(-20.0..20.0),
            y_speed: rng.gen_range(-20.0..20.0),
        }
    }

    fn update(&mut self) {
        if self.x_pos > SCREEN_WIDTH || self.x_pos < 0.0 {
            self.x_speed *= -1.0;
        }
        if self.y_pos > SCREEN_HEIGHT || self.y_pos < 0.0 {
            self.y_speed *= -1.0;
        }
        self.x_pos += self.x_speed;
        self.y_pos += self.y_speed;
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2 {
                x: self.x_pos,
                y: self.y_pos,
            },
            2.0,
            0.5,
            [1.0, 0.5, 0.0, 1.0].into(),
        )?;
        graphics::draw(ctx, &circle, (Point2 { x: 0.0, y: 0.0 },))?;
        Ok(())
    }
}
