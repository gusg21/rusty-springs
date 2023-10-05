mod actor;

use ggez::{
    self, event,
    glam::{vec2, Vec2},
    graphics::{self, Color},
    Context, ContextBuilder, GameResult,
};

struct AppState {
    actors: actor::Actor
}

impl AppState {
    /// Create a new Game State container
    fn create(ctx: &mut Context) -> GameResult<AppState> {
        return Ok(AppState { 
            actors: crate::actor::Actor {} 
        });
    }
}

impl event::EventHandler<ggez::GameError> for AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        return Ok(());
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        

        canvas.finish(ctx)?;

        return Ok(());
    }
}

pub fn main() -> GameResult {
    let context_builder = ContextBuilder::new("app", "gusg21");
    let (mut context, event_loop) = context_builder.build()?;
    let state = AppState::create(&mut context)?;
    event::run(context, event_loop, state);
}
