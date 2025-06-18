// Import necessary modules from the ggez game framework and standard library.
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, DrawParam, Image, Rect};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, ContextBuilder, GameResult};
use std::env;
use std::path;

// Constants for game configuration.
const TILE_SIZE: f32 = 8.0; // Size of a single tile in the game grid.
const MAZE_OFFSET_Y: f32 = TILE_SIZE * 5.0; // Vertical offset for the maze display.
const WALL_CODE_OFFSET: u8 = 100; // Offset added to wall mask to get the tile code for display.
const PLAYER_SPEED: f32 = 40.0; // Speed of the player in pixels per second.

/// Represents the direction a character can move.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
    Stopped, // Indicates no movement.
}

/// The main entry point of the game.
/// Initializes ggez context, loads resources, and runs the game loop.
fn main() -> GameResult {
    // Determine the resource directory based on the CARGO_MANIFEST_DIR environment variable.
    // This helps in locating resources correctly whether running from cargo or a compiled binary.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        // Fallback for when CARGO_MANIFEST_DIR is not set (e.g., when running directly).
        path::PathBuf::from("./resources")
    };

    // Create a new game context and event loop.
    // Configures window title, dimensions, and resource path.
    let (mut ctx, event_loop) = ContextBuilder::new("rust_pac", "Sir Marshall")
        .window_setup(ggez::conf::WindowSetup::default().title("RUST PAC v1.0"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(672.0, 864.0))
        .add_resource_path(resource_dir)
        .build()?;

    // Create the initial game state.
    let game_state = GameState::new(&mut ctx)?;
    // Run the game loop.
    event::run(ctx, event_loop, game_state)
}

/// Represents the player character in the game.
struct Player {
    pos: Vec2, // Current position of the player in world coordinates.
    direction: Direction, // Current movement direction of the player.
    sprite_rects: Vec<Rect>, // Collection of Rects defining the player's animation frames on the sprite sheet.
}

/// The main game state, holding all game-related data.
struct GameState {
    wall_images: Vec<Image>, // Vector of images for different wall configurations.
    sprite_sheet: Image, // Sprite sheet containing player and dot sprites.
    display_map: Vec<Vec<u8>>, // Map used for rendering, includes wall masks.
    level_map: Vec<Vec<u8>>, // Original map defining walls (1), dots (2), power pellets (3), and empty spaces (0).
    small_dot_rect: Rect, // Source rectangle for the small dot sprite on the sprite sheet.
    big_dot_rect: Rect, // Source rectangle for the big dot (power pellet) sprite on the sprite sheet.
    player: Player, // The player character.
}

impl GameState {
    /// Creates a new `GameState` instance, loading all necessary assets and initializing game elements.
    ///
    /// # Arguments
    /// * `ctx` - A mutable reference to the `ggez::Context`.
    ///
    /// # Returns
    /// A `GameResult` containing the initialized `GameState` or an error.
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        // Load wall images. There are 16 different wall sprites based on their neighbors.
        let mut wall_images = Vec::with_capacity(16);
        for i in 0..16 {
            let path = format!("/walls/wall_{}.png", i);
            wall_images.push(Image::from_path(ctx, &path)?);
        }

        // Load the main sprite sheet containing player and dot sprites.
        let sprite_sheet = Image::from_path(ctx, "/player.png")?;

        // Calculate normalized sprite dimensions based on a 6-column sprite sheet.
        let sprite_width_normalized = 1.0 / 6.0;
        let sprite_height_normalized = 1.0;

        // Define the source rectangles for the player's animation frames.
        // The player has 4 frames for animation, located in the first 4 columns.
        let player_rects = (0..4)
            .map(|i| {
                Rect::new(
                    i as f32 * sprite_width_normalized, // X position on sprite sheet.
                    0.0, // Y position on sprite sheet (top of the sheet).
                    sprite_width_normalized, // Width of the sprite.
                    sprite_height_normalized, // Height of the sprite.
                )
            })
            .collect();

        // Define the source rectangle for the small dot sprite.
        // It's located in the 5th column (index 4).
        let small_dot_rect = Rect::new(
            4.0 * sprite_width_normalized,
            0.0,
            sprite_width_normalized,
            sprite_height_normalized,
        );
        // Define the source rectangle for the big dot (power pellet) sprite.
        // It's located in the 6th column (index 5).
        let big_dot_rect = Rect::new(
            5.0 * sprite_width_normalized,
            0.0,
            sprite_width_normalized,
            sprite_height_normalized,
        );

        // Define the game level map.
        // 1: Wall, 2: Small Dot, 3: Big Dot (Power Pellet), 0: Empty Space
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
                    }
                    if is_wall_at(x as isize, y as isize + 1, &level_map) {
                        wall_mask += 2;
                    }
                    if is_wall_at(x as isize - 1, y as isize, &level_map) {
                        wall_mask += 4;
                    }
                    if is_wall_at(x as isize + 1, y as isize, &level_map) {
                        wall_mask += 8;
                    }
                    display_map[y][x] = wall_mask + WALL_CODE_OFFSET;
                }
            }
        }

        let start_pos = Vec2::new(13.5 * TILE_SIZE, (23.0 * TILE_SIZE) + MAZE_OFFSET_Y);

        let player = Player {
            pos: start_pos,
            direction: Direction::Stopped,
            sprite_rects: player_rects,
        };

        Ok(GameState {
            wall_images,
            sprite_sheet,
            display_map,
            level_map,
            small_dot_rect,
            big_dot_rect,
            player,
        })
    }
}

fn is_wall_at(x: isize, y: isize, map: &Vec<Vec<u8>>) -> bool {
    if y < 0 || y >= map.len() as isize || x < 0 || x >= map[0].len() as isize {
        return true;
    }
    map[y as usize][x as usize] == 1
}

fn is_tile_walkable(pixel_x: f32, pixel_y: f32, map: &Vec<Vec<u8>>) -> bool {
    let map_y = ((pixel_y - MAZE_OFFSET_Y) / TILE_SIZE) as isize;
    let map_x = (pixel_x / TILE_SIZE) as isize;
    !is_wall_at(map_x, map_y, map)
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();
        let mut next_pos = self.player.pos;

        match self.player.direction {
            Direction::North => next_pos.y -= PLAYER_SPEED * dt,
            Direction::South => next_pos.y += PLAYER_SPEED * dt,
            Direction::West => next_pos.x -= PLAYER_SPEED * dt,
            Direction::East => next_pos.x += PLAYER_SPEED * dt,
            Direction::Stopped => {}
        }

        if is_tile_walkable(next_pos.x, next_pos.y, &self.level_map) {
            self.player.pos = next_pos;
        } else {
            self.player.direction = Direction::Stopped;
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> GameResult {
        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::Up => self.player.direction = Direction::North,
                KeyCode::Down => self.player.direction = Direction::South,
                KeyCode::Left => self.player.direction = Direction::West,
                KeyCode::Right => self.player.direction = Direction::East,
                _ => {}
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_screen_coordinates(Rect::new(0.0, 0.0, 224.0, 288.0));
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        for (y, row) in self.display_map.iter().enumerate() {
            for (x, &tile_type) in row.iter().enumerate() {
                let dest_x = x as f32 * TILE_SIZE;
                let dest_y = (y as f32 * TILE_SIZE) + MAZE_OFFSET_Y;
                let dest_point = Vec2::new(dest_x, dest_y);

                match tile_type {
                    code @ WALL_CODE_OFFSET..=115 => {
                        let wall_index = (code - WALL_CODE_OFFSET) as usize;
                        canvas.draw(&self.wall_images[wall_index], dest_point);
                    }
                    2 => {
                        let params = DrawParam::new().dest(dest_point).src(self.small_dot_rect);
                        canvas.draw(&self.sprite_sheet, params);
                    }
                    3 => {
                        let params = DrawParam::new().dest(dest_point).src(self.big_dot_rect);
                        canvas.draw(&self.sprite_sheet, params);
                    }
                    _ => {}
                }
            }
        }

        let player_sprite_rect = match self.player.direction {
            Direction::North => self.player.sprite_rects[0],
            Direction::East => self.player.sprite_rects[1],
            Direction::South => self.player.sprite_rects[2],
            Direction::West => self.player.sprite_rects[3],
            Direction::Stopped => self.player.sprite_rects[2],
        };

        let player_dest = self.player.pos - Vec2::new(TILE_SIZE / 2.0, TILE_SIZE / 2.0);

        let params = DrawParam::new().dest(player_dest).src(player_sprite_rect);
        canvas.draw(&self.sprite_sheet, params);

        canvas.finish(ctx)?;
        Ok(())
    }
}
