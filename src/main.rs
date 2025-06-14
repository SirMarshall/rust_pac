//Standard library to set environment varibale for resource pathing
use std::env;
use std::path;

//Exceptional library for 2D Coordinates
use ggez::glam::Vec2;

//Standard graphical manipulation packages
use ggez::graphics::{self, Color, DrawParam, Image, Rect};

//Game loop systems
use ggez::event::{self, EventHandler};

//Primary constructor context
use ggez::{Context, ContextBuilder, GameResult};

fn main() -> GameResult {
    //Find the resources folder based on Cargo manifesto
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    //Launch game context
    let (mut ctx, event_loop) =
    ContextBuilder::new
    ("rust_pac", 
    "Sir Marshall")
        .window_setup(ggez::conf::WindowSetup::default()
        .title("RUST PAC v1.0"))
        .add_resource_path(resource_dir)
        .build()?;

    //Initialize state variable
    let game_state = GameState::new(&mut ctx)?;

    //Start event handling
    event::run(ctx, event_loop, game_state)
}

//Struct to handle game's state
struct GameState {
    sprite_sheet: Image,
}

//Definition of GameState functions
impl GameState {
    //Constructor
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        //Use primary game context passed in
        let image = Image::from_path(ctx, "/spritesheet.png")?;
        Ok(GameState {
            sprite_sheet: image,
        })
    }
}

//How to handle events from GameState
impl EventHandler for GameState {
    //Primary frame change - collision checks, movements
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    //Draw to the screen
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //Tasty blank canvas!
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        //Rendering object
        let sampler = graphics::Sampler::nearest_clamp();

        //Import config
        canvas.set_sampler(sampler);

        //Master sprite scaling
        let scale_factor = 6.0;

        //Initialize rendering objcet
        let params = DrawParam::new()
            //Where to draw
            .dest(Vec2::new(100.0, 100.0))
            //How big
            .scale(Vec2::new(scale_factor, scale_factor))
            //Which part of source image to use (left 1/3)
            .src(Rect::new(0.0, 0.0, 1.0 / 3.0, 1.0));

        //JUST DO IT!
        canvas.draw(&self.sprite_sheet, params);

        //Show it off!
        canvas.finish(ctx)?;

        //Return 0 B)
        Ok(())
    }
}
