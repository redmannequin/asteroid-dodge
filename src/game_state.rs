use ggez::{event, graphics, Context, GameResult};

pub struct GameState {}

impl GameState {
    pub fn new() -> Self {
        GameState {}
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        graphics::present(ctx)?;
        Ok(())
    }
}
