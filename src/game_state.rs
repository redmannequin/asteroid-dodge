use ggez::{event, graphics, timer, Context, GameResult};

const TARGET_FPS: u32 = 60;

pub struct GameState {
    player: Player,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            player: Player::new(),
        }
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, TARGET_FPS) {
            self.player.update();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        self.player.draw(ctx)?;
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }
}

struct Player {
    pos: u32,
}

impl Player {
    fn new() -> Self {
        Player { pos: 0 }
    }

    fn update(&mut self) {
        self.pos += 1;
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(self.pos as f32, 50.0, 20.0, 20.0),
            [1.0, 0.5, 0.0, 1.0].into(),
        )?;
        graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        Ok(())
    }
}
