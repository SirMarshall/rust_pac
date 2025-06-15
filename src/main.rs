use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, DrawParam, Image, Rect};
use ggez::{Context, ContextBuilder, GameResult};
use std::env;
use std::path;
const TILE_SIZE: f32 = 8.0;
const MAZE_OFFSET_Y: f32 = TILE_SIZE * 5.0;
const WALL_CODE_OFFSET: u8 = 100;
/// Main entry point of the game.
/// Initializes the game context, loads resources, and runs the game loop.
fn main() -> GameResult {
    // Determine the resource directory based on CARGO_MANIFEST_DIR or default to "./resources".
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    // Create a new game context and event loop.
    let (mut ctx, event_loop) = ContextBuilder::new("rust_pac", "Sir Marshall")
        .window_setup(ggez::conf::WindowSetup::default().title("RUST PAC v1.0"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(672.0, 864.0))
        .add_resource_path(resource_dir)
        .build()?;

    // Initialize the game state.
    let game_state = GameState::new(&mut ctx)?;

    // Run the game loop.
    event::run(ctx, event_loop, game_state)
}
/// Represents the main game state, holding all necessary assets and game data.
struct GameState {
    /// Stores images for different wall configurations (16 variations).
    wall_images: Vec<Image>,
    /// A single sprite sheet containing various game elements like dots.
    sprite_sheet: Image,
    /// The map displayed on screen, with wall codes adjusted for rendering.
    display_map: Vec<Vec<u8>>,
    /// Source rectangle for drawing small dots from the sprite sheet.
    small_dot_rect: Rect,
    /// Source rectangle for drawing big dots (power pellets) from the sprite sheet.
    big_dot_rect: Rect,
}
impl GameState {
/// Creates a new `GameState` instance, loading all game assets and initializing the map.
fn new(ctx: &mut Context) -> GameResult<GameState> {
    // Load 16 different wall images based on their connection masks.
    let mut wall_images = Vec::with_capacity(16);
    for i in 0..16 {
        let path = format!("/walls/wall_{}.png", i);
        wall_images.push(Image::from_path(ctx, &path)?);
    }

    // Load the main sprite sheet for dots and other elements.
    let sprite_sheet = Image::from_path(ctx, "/spritesheet.png")?;

    // Define source rectangles on the sprite sheet for small and big dots.
    let small_dot_rect = Rect::new(1.0 / 3.0, 0.0, 1.0 / 3.0, 1.0);
    let big_dot_rect = Rect::new(2.0 / 3.0, 0.0, 1.0 / 3.0, 1.0);

    // Define the initial level map.
    // 0: Empty space
    // 1: Wall (will be replaced by specific wall image based on neighbors)
    // 2: Small dot
    // 3: Big dot (power pellet)
    let level_map = vec![
        vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ],
        vec![
            1, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 1,
        ],
        vec![
            1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1,
        ],
        vec![
            1, 2, 1, 0, 0, 1, 2, 1, 0, 0, 0, 1, 2, 1, 1, 2, 1, 0, 0, 0, 1, 2, 1, 0, 0, 1, 2, 1,
        ],
        vec![
            1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1,
        ],
        vec![
            1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1,
        ],
        vec![
            1, 2, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 2, 1,
        ],
        vec![
            1, 2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 1,
        ],
        vec![
            1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1,
        ],
        vec![
            1, 0, 0, 0, 0, 1, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 1, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 0, 0, 0, 0, 1, 2, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 2, 1, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 1, 1, 1, 1, 1, 2, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 2, 1, 1, 1, 1, 1, 1,
        ],
        vec![
            0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0,
        ],
        vec![
            1, 1, 1, 1, 1, 1, 2, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 2, 1, 1, 1, 1, 1, 1,
        ],
        vec![
            1, 0, 0, 0, 0, 1, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 1, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 0, 0, 0, 0, 1, 2, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 2, 1, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 1, 1, 1, 1, 1, 2, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 2, 1, 1, 1, 1, 1, 1,
        ],
        vec![
            1, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 1,
        ],
        vec![
            1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1,
        ],
        vec![
            1, 2, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 2, 1,
        ],
        vec![
            1, 1, 1, 2, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 2, 1, 1, 1,
        ],
        vec![
            1, 2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 1,
        ],
        vec![
            1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1,
        ],
        vec![
            1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1,
        ],
        vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ],
    ];
    let mut display_map = level_map.clone();
    for (y, row) in level_map.iter().enumerate() {
        for (x, &tile_value) in row.iter().enumerate() {
            if tile_value == 1 {
                let mut wall_mask: u8 = 0;
                if is_wall_at(x as isize, y as isize - 1, &level_map) {
                    wall_mask += 1;
                } // Up
                if is_wall_at(x as isize, y as isize + 1, &level_map) {
                    wall_mask += 2;
                } // Down
                if is_wall_at(x as isize - 1, y as isize, &level_map) {
                    wall_mask += 4;
                } // Left
                if is_wall_at(x as isize + 1, y as isize, &level_map) {
                    wall_mask += 8;
                } // Right
                display_map[y][x] = wall_mask + WALL_CODE_OFFSET;
            }
        }
    }
    Ok(GameState {
        wall_images,
        sprite_sheet,
        display_map,
        small_dot_rect,
        big_dot_rect,
    })
}
}
/// Checks if a wall tile exists at the given coordinates within the map.
/// Handles out-of-bounds access by returning false.
fn is_wall_at(x: isize, y: isize, map: &Vec<Vec<u8>>) -> bool {
    if y < 0 || y >= map.len() as isize {
        return false;
    }
    let row = &map[y as usize];
    if x < 0 || x >= row.len() as isize {
        return false;
    }
    map[y as usize][x as usize] == 1
}
impl EventHandler for GameState {
    /// The `update` method is called every game tick.
    /// Currently, it does nothing as there's no game logic implemented yet.
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// The `draw` method is called to render the game state to the screen.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Create a new canvas for drawing, clearing the screen to black.
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // Set the screen coordinates for drawing, defining the visible game area.
        canvas.set_screen_coordinates(Rect::new(0.0, 0.0, 224.0, 288.0));

        // Use nearest-neighbor sampling for pixel-perfect rendering of sprites.
        let sampler = graphics::Sampler::nearest_clamp();
        canvas.set_sampler(sampler);

        // Iterate through the display map and draw each tile.
        for (y, row) in self.display_map.iter().enumerate() {
            for (x, &tile_type) in row.iter().enumerate() {
                // Calculate the destination point for the current tile.
                let dest_x = x as f32 * TILE_SIZE;
                let dest_y = (y as f32 * TILE_SIZE) + MAZE_OFFSET_Y;
                let dest_point = Vec2::new(dest_x, dest_y);

                // Draw the appropriate tile based on its type.
                match tile_type {
                    // Wall tiles: codes from WALL_CODE_OFFSET to 115 (100 + 15).
                    // The specific wall image is determined by the wall_mask.
                    code @ WALL_CODE_OFFSET..=115 => {
                        let wall_index = (code - WALL_CODE_OFFSET) as usize;
                        let wall_image = &self.wall_images[wall_index];
                        canvas.draw(wall_image, DrawParam::new().dest(dest_point));
                    }
                    // Small dot.
                    2 => {
                        let params = DrawParam::new().dest(dest_point).src(self.small_dot_rect);
                        canvas.draw(&self.sprite_sheet, params);
                    }
                    // Big dot (power pellet).
                    3 => {
                        let params = DrawParam::new().dest(dest_point).src(self.big_dot_rect);
                        canvas.draw(&self.sprite_sheet, params);
                    }
                    // Ignore other tile types (e.g., 0 for empty space).
                    _ => {}
                }
            }
        }
        // Present the drawn canvas to the screen.
        canvas.finish(ctx)?;
        Ok(())
    }
}
