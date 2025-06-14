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

// At the top of your file, right after the `use` statements...
const TILE_SIZE: f32 = 8.0;
// We leave 3 tiles of space (24 pixels) at the top for score.
const MAZE_OFFSET_Y: f32 = TILE_SIZE * 5.0;

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
    let (mut ctx, event_loop) = ContextBuilder::new("rust_pac", "Sir Marshall")
        .window_setup(ggez::conf::WindowSetup::default().title("RUST PAC v1.0"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(672.0, 864.0))
        .add_resource_path(resource_dir)
        .build()?;

    //Initialize state variable
    let game_state = GameState::new(&mut ctx)?;

    //Start event handling
    event::run(ctx, event_loop, game_state)
}

//Struct to handle game's state

struct GameState {
    // Primary spritesheet
    sprite_sheet: Image,
    // Maze map
    level_map: Vec<Vec<u8>>,
    // --- NEW: Our tile templates! ---
    wall_rect: Rect,
    small_dot_rect: Rect,
    big_dot_rect: Rect,
}


//Definition of GameState functions
impl GameState {
    // Constructor
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let image = Image::from_path(ctx, "/spritesheet.png")?;

        // --- NEW: Define all our tile templates once ---
        // This is much more efficient than creating them every frame.
        let wall_rect = Rect::new(0.0, 0.0, 1.0 / 3.0, 1.0);
        let small_dot_rect = Rect::new(1.0 / 3.0, 0.0, 1.0 / 3.0, 1.0);
        let big_dot_rect = Rect::new(2.0 / 3.0, 0.0, 1.0 / 3.0, 1.0);

        // --- NEW: The full-size maze blueprint! ---
        // Our screen is 224 pixels wide -> 28 tiles.
        // We leave 3 tiles top and 3 bottom for score/lives, so 288/8 - 6 = 30 tiles high.
        let level_map = vec![
            // Row 0 - Top border
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            // Row 1
            vec![1, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 1],
            // Row 2
            vec![1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1],
            // Row 3
            vec![1, 2, 1, 0, 0, 1, 2, 1, 0, 0, 0, 1, 2, 1, 1, 2, 1, 0, 0, 0, 1, 2, 1, 0, 0, 1, 2, 1],
            // Row 4
            vec![1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1],
            // Row 5
            vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
            // ... for now, we just repeat empty rows to fill space ...
            vec![1, 2, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 2, 1],
            vec![1, 2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 1],
            vec![1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1],
            // A few empty rows in middle
            vec![1, 0, 0, 0, 0, 1, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 1, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 1, 2, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 2, 1, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 2, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 2, 1, 1, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0], // Tunnel
            vec![1, 1, 1, 1, 1, 1, 2, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 2, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 1, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 1, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 1, 2, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 2, 1, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 2, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 2, 1, 1, 1, 1, 1, 1],
            // Add many more rows here to fill out the 30-tile height...
            // For now, this is a good start. We will just copy-paste some rows.
            vec![1, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 1],
            vec![1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1],
            vec![1, 2, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 2, 1],
            vec![1, 1, 1, 2, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 2, 1, 1, 1],
            vec![1, 2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 1],
            vec![1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1],
            vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
            // Row 29 - Bottom border
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];


        Ok(GameState {
            sprite_sheet: image,
            level_map,
            wall_rect,
            small_dot_rect,
            big_dot_rect,
        })
    }
}
//How to handle events from GameState
impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_screen_coordinates(Rect::new(0.0, 0.0, 224.0, 288.0));
        let sampler = graphics::Sampler::nearest_clamp();
        canvas.set_sampler(sampler);

        // --- UPGRADED DRAWING LOGIC ---
        for (y, row) in self.level_map.iter().enumerate() {
            for (x, &tile_type) in row.iter().enumerate() {
                // We calculate the destination for any potential tile
                let dest_x = x as f32 * TILE_SIZE;
                // We add our Y offset to push the maze down!
                let dest_y = (y as f32 * TILE_SIZE) + MAZE_OFFSET_Y;
                let dest_point = Vec2::new(dest_x, dest_y);

                // We use a `match` statement to decide which tile to draw.
                // Is much cleaner than many `if/else if` statements.
                match tile_type {
                    1 => { // Draw a Wall
                        let params = DrawParam::new().dest(dest_point).src(self.wall_rect);
                        canvas.draw(&self.sprite_sheet, params);
                    }
                    2 => { // Draw a Small Dot
                        let params = DrawParam::new().dest(dest_point).src(self.small_dot_rect);
                        canvas.draw(&self.sprite_sheet, params);
                    }
                    3 => { // Draw a Big Dot
                        let params = DrawParam::new().dest(dest_point).src(self.big_dot_rect);
                        canvas.draw(&self.sprite_sheet, params);
                    }
                    // For `0` (empty) or any other number, we do nothing!
                    _ => {}
                }
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}