use std::env;
use std::path;

use ggez::glam::Vec2;
use ggez::graphics::{self, Color, DrawParam, Image, Rect};
use ggez::event::{self, EventHandler};

use ggez::{Context, ContextBuilder, GameResult};

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let (mut ctx, event_loop) = ContextBuilder::new("rust_pac", "Sir Marshall")
        .window_setup(ggez::conf::WindowSetup::default().title("RUST PAC v1.0"))
        .add_resource_path(resource_dir)
        .build()?;
    let game_state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, game_state)
}

struct GameState {
    sprite_sheet: Image,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let image = Image::from_path(ctx, "/spritesheet.png")?;
        Ok(GameState {
            sprite_sheet: image,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        let sampler = graphics::Sampler::nearest_clamp();
        canvas.set_sampler(sampler);
        let scale_factor = 6.0;
        let params = DrawParam::new()
            .dest(Vec2::new(100.0, 100.0))
            .scale(Vec2::new(scale_factor, scale_factor))
            .src(Rect::new(0.0, 0.0, 1.0 / 3.0, 1.0));
        canvas.draw(&self.sprite_sheet, params);
        canvas.finish(ctx)?;
        Ok(())
    }
}
