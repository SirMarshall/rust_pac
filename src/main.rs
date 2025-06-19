use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, DrawParam, Image, Rect};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, ContextBuilder, GameResult};

use std::env;
use std::path;

// --- Constants ---
const TILE_SIZE: f32 = 8.0; // Size of a single tile in pixels.
const MAZE_OFFSET_Y: f32 = TILE_SIZE * 5.0; // Vertical offset for the maze to make space for UI elements.
const WALL_CODE_OFFSET: u8 = 100; // Offset added to wall mask to distinguish wall types in `display_map`.
const PLAYER_SPEED: f32 = 40.0; // Player movement speed in pixels per second.

// Represents the cardinal directions and a stopped state for movement.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
    Stopped,
}

// Main entry point of the game.
fn main() -> GameResult {
    // Determine the resource directory for loading assets.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    // Create a new game context and event loop with specified window settings.
    let (mut ctx, event_loop) = ContextBuilder::new("rust_pac", "Sir Marshall")
        .window_setup(ggez::conf::WindowSetup::default().title("RUST PAC v1.0"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(672.0, 864.0))
        .add_resource_path(resource_dir)
        .build()?;

    // Initialize the game state.
    let game_state = GameState::new(&mut ctx)?;

    // Run the game event loop, passing control to the `GameState` handler.
    event::run(ctx, event_loop, game_state)
}

// --- Game Structures ---

// Represents the player character.
struct Player {
    pos: Vec2, // Current position of the player (center of sprite).
    direction: Direction, // Current actual movement direction.
    desired_direction: Direction, // Direction input by the player, used for turning logic.
    sprite_rects: Vec<Rect>, // Normalized UV coordinates for player animation frames.
}

// Holds all the game's state, assets, and game logic data.
struct GameState {
    wall_images: Vec<Image>, // Loaded images for different wall configurations.
    sprite_sheet: Image, // Single sprite sheet for player and dots.
    display_map: Vec<Vec<u8>>, // Map used for drawing, includes pre-calculated wall configurations.
    level_map: Vec<Vec<u8>>, // Base map for game logic (walls, dots, empty spaces).
    small_dot_rect: Rect, // UV coordinates for the small dot sprite.
    big_dot_rect: Rect, // UV coordinates for the big dot (power pellet) sprite.
    player: Player, // The player character.
}

// --- GameState Implementation ---

impl GameState {
    // Creates a new GameState, loading assets and setting up initial game elements.
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        // Load wall images (16 variations based on surrounding walls).
        let mut wall_images = Vec::with_capacity(16);
        for i in 0..16 {
            let path = format!("/walls/wall_{}.png", i);
            wall_images.push(Image::from_path(ctx, &path)?);
        }

        // Load the main sprite sheet containing player and dot sprites.
        let sprite_sheet = Image::from_path(ctx, "/player.png")?;

        // Define normalized dimensions for sprites on the sheet (sheet has 6 columns).
        let sprite_width_normalized = 1.0 / 6.0;
        let sprite_height_normalized = 1.0;

        // Extract UV coordinates for player animation frames (first 4 sprites).
        let player_rects = (0..4)
            .map(|i| {
                Rect::new(
                    i as f32 * sprite_width_normalized,
                    0.0,
                    sprite_width_normalized,
                    sprite_height_normalized,
                )
            })
            .collect();

        // Extract UV coordinates for the small and big dot sprites.
        let small_dot_rect = Rect::new(
            4.0 * sprite_width_normalized,
            0.0,
            sprite_width_normalized,
            sprite_height_normalized,
        );
        let big_dot_rect = Rect::new(
            5.0 * sprite_width_normalized,
            0.0,
            sprite_width_normalized,
            sprite_height_normalized,
        );

        // Define the base level map: 1 = Wall, 2 = Small Dot, 3 = Big Dot, 0 = Empty/Walkable path.
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

        // Create `display_map` by calculating wall masks for graphical representation.
        // A wall mask (0-15) indicates which adjacent tiles (N, S, W, E) are also walls.
        let mut display_map = level_map.clone();
        for (y, row) in level_map.iter().enumerate() {
            for (x, &tile_value) in row.iter().enumerate() {
                if tile_value == 1 {
                    let mut wall_mask: u8 = 0;
                    if is_wall_at(x as isize, y as isize - 1, &level_map) {
                        wall_mask += 1;
                    } // North
                    if is_wall_at(x as isize, y as isize + 1, &level_map) {
                        wall_mask += 2;
                    } // South
                    if is_wall_at(x as isize - 1, y as isize, &level_map) {
                        wall_mask += 4;
                    } // West
                    if is_wall_at(x as isize + 1, y as isize, &level_map) {
                        wall_mask += 8;
                    } // East
                    display_map[y][x] = wall_mask + WALL_CODE_OFFSET; // Store with offset for drawing lookup.
                }
            }
        }

        // Initialize player's starting position and state.
        let start_pos = Vec2::new(13.5 * TILE_SIZE, (23.5 * TILE_SIZE) + MAZE_OFFSET_Y);
        let player = Player {
            pos: start_pos,
            direction: Direction::Stopped,
            desired_direction: Direction::Stopped,
            sprite_rects: player_rects,
        };

        // Return the initialized GameState.
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

// --- Helper Functions ---

// Checks if a tile at given map coordinates is a wall. Handles out-of-bounds coordinates by treating them as walls.
fn is_wall_at(x: isize, y: isize, map: &Vec<Vec<u8>>) -> bool {
    if y < 0 || y >= map.len() as isize || x < 0 || x >= map[0].len() as isize {
        return true; // Outside map boundaries are considered walls.
    }
    map[y as usize][x as usize] == 1 // Check if the tile value is '1' (wall).
}

// Converts pixel coordinates to map coordinates and checks if the corresponding tile is walkable.
fn is_tile_walkable(pixel_x: f32, pixel_y: f32, map: &Vec<Vec<u8>>) -> bool {
    let map_y = ((pixel_y - MAZE_OFFSET_Y) / TILE_SIZE) as isize;
    let map_x = (pixel_x / TILE_SIZE) as isize;
    !is_wall_at(map_x, map_y, map) // Returns true if the tile is NOT a wall.
}

// Checks if all four corners of a given Rect are on walkable tiles, used for collision detection.
fn is_rect_walkable(rect: Rect, level_map: &Vec<Vec<u8>>) -> bool {
    let right_edge = rect.x + rect.w - 1.0;
    let bottom_edge = rect.y + rect.h - 1.0;

    // Check each corner: top-left, top-right, bottom-left, bottom-right.
    if !is_tile_walkable(rect.x, rect.y, level_map) {
        return false;
    }
    if !is_tile_walkable(right_edge, rect.y, level_map) {
        return false;
    }
    if !is_tile_walkable(rect.x, bottom_edge, level_map) {
        return false;
    }
    if !is_tile_walkable(right_edge, bottom_edge, level_map) {
        return false;
    }

    true // All corners are walkable.
}

// --- ggez EventHandler Implementation ---

impl EventHandler for GameState {
    // Called once per game frame to update game logic.
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32(); // Time elapsed since last frame.

        // Check if moving in the desired direction (player input) is possible.
        let desired_move_possible = {
            let mut pos_if_desired = self.player.pos;
            match self.player.desired_direction {
                Direction::North => pos_if_desired.y -= PLAYER_SPEED * dt,
                Direction::South => pos_if_desired.y += PLAYER_SPEED * dt,
                Direction::West => pos_if_desired.x -= PLAYER_SPEED * dt,
                Direction::East => pos_if_desired.x += PLAYER_SPEED * dt,
                Direction::Stopped => {}
            }
            // Create a rectangle representing the player's potential next position.
            let rect_if_desired = Rect::new(
                pos_if_desired.x - TILE_SIZE / 2.0,
                pos_if_desired.y - TILE_SIZE / 2.0,
                TILE_SIZE,
                TILE_SIZE,
            );
            is_rect_walkable(rect_if_desired, &self.level_map)
        };

        // If a desired direction is set and it's walkable, change the player's actual movement direction.
        if self.player.desired_direction != Direction::Stopped && desired_move_possible {
            self.player.direction = self.player.desired_direction;
        }

        // Calculate the player's next position based on the current movement direction.
        let mut next_pos = self.player.pos;
        match self.player.direction {
            Direction::North => next_pos.y -= PLAYER_SPEED * dt,
            Direction::South => next_pos.y += PLAYER_SPEED * dt,
            Direction::West => next_pos.x -= PLAYER_SPEED * dt,
            Direction::East => next_pos.x += PLAYER_SPEED * dt,
            Direction::Stopped => {}
        }

        // Create a rectangle representing the player's potential next position for collision check.
        let next_player_rect = Rect::new(
            next_pos.x - TILE_SIZE / 2.0,
            next_pos.y - TILE_SIZE / 2.0,
            TILE_SIZE,
            TILE_SIZE,
        );

        // If the player's next position is walkable, update their position; otherwise, stop.
        if is_rect_walkable(next_player_rect, &self.level_map) {
            self.player.pos = next_pos;
        } else {
            self.player.direction = Direction::Stopped;
        }

        Ok(())
    }

    // Handles keyboard key press events.
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> GameResult {
        if let Some(keycode) = input.keycode {
            // Set the player's desired direction based on arrow key input.
            match keycode {
                KeyCode::Up => self.player.desired_direction = Direction::North,
                KeyCode::Down => self.player.desired_direction = Direction::South,
                KeyCode::Left => self.player.desired_direction = Direction::West,
                KeyCode::Right => self.player.desired_direction = Direction::East,
                _ => {} // Ignore other key presses.
            }
        }
        Ok(())
    }

    // Called once per game frame to draw everything to the screen.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Create a new drawing canvas with a black background and set the coordinate system.
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_screen_coordinates(Rect::new(0.0, 0.0, 224.0, 288.0)); // Original Pac-Man resolution.
        canvas.set_sampler(graphics::Sampler::nearest_clamp()); // For pixel-perfect rendering.

        // Draw the maze based on the `display_map`.
        for (y, row) in self.display_map.iter().enumerate() {
            for (x, &tile_type) in row.iter().enumerate() {
                let dest_x = x as f32 * TILE_SIZE;
                let dest_y = (y as f32 * TILE_SIZE) + MAZE_OFFSET_Y; // Apply vertical offset for the maze.
                let dest_point = Vec2::new(dest_x, dest_y);

                match tile_type {
                    // Draw walls using the pre-calculated wall images based on the wall mask.
                    code @ WALL_CODE_OFFSET..=115 => {
                        let wall_index = (code - WALL_CODE_OFFSET) as usize;
                        canvas.draw(&self.wall_images[wall_index], dest_point);
                    }
                    // Draw small dots using the sprite sheet.
                    2 => {
                        let params = DrawParam::new().dest(dest_point).src(self.small_dot_rect);
                        canvas.draw(&self.sprite_sheet, params);
                    }
                    // Draw big dots (power pellets) using the sprite sheet.
                    3 => {
                        let params = DrawParam::new().dest(dest_point).src(self.big_dot_rect);
                        canvas.draw(&self.sprite_sheet, params);
                    }
                    _ => {} // Ignore empty spaces (0) for drawing.
                }
            }
        }

        // Determine the correct player sprite frame based on the current movement direction.
        let player_sprite_rect = match self.player.direction {
            Direction::North => self.player.sprite_rects[0],
            Direction::East => self.player.sprite_rects[1],
            Direction::South => self.player.sprite_rects[2],
            Direction::West => self.player.sprite_rects[3],
            Direction::Stopped => self.player.sprite_rects[2], // Default to facing South when stopped.
        };

        // Calculate player drawing position (adjust from center to top-left for sprite).
        let player_dest = self.player.pos - Vec2::new(TILE_SIZE / 2.0, TILE_SIZE / 2.0);

        // Draw the player sprite.
        let params = DrawParam::new().dest(player_dest).src(player_sprite_rect);
        canvas.draw(&self.sprite_sheet, params);

        // Present the drawn frame to the screen.
        canvas.finish(ctx)?;
        Ok(())
    }
}