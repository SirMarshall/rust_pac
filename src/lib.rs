use ggez::glam::Vec2;
use ggez::graphics::{self, Rect};

// --- Constants ---
pub const TILE_SIZE: f32 = 8.0; // Size of a single tile in pixels.
pub const MAZE_OFFSET_Y: f32 = TILE_SIZE * 5.0; // Vertical offset for the maze to make space for UI elements.
pub const WALL_CODE_OFFSET: u8 = 100; // Offset added to wall mask to distinguish wall types in `display_map`.
pub const PLAYER_SPEED: f32 = 40.0; // Player movement speed in pixels per second.

// Represents the cardinal directions and a stopped state for movement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Stopped,
}

// Checks if a tile at given map coordinates is a wall. Handles out-of-bounds coordinates by treating them as walls.
pub fn is_wall_at(x: isize, y: isize, map: &Vec<Vec<u8>>) -> bool {
    if y < 0 || (y as usize) >= map.len() {
        return true;
    }
    let row = &map[y as usize];
    if x < 0 || (x as usize) >= row.len() {
        return true;
    }

    row[x as usize] == 1
}

// Converts pixel coordinates to map coordinates and checks if the corresponding tile is walkable.
pub fn is_tile_walkable(pixel_x: f32, pixel_y: f32, map: &Vec<Vec<u8>>) -> bool {
    let map_y = ((pixel_y - MAZE_OFFSET_Y) / TILE_SIZE) as isize;
    let map_x = (pixel_x / TILE_SIZE) as isize;
    !is_wall_at(map_x, map_y, map) // Returns true if the tile is NOT a wall.
}

// Checks if all four corners of a given Rect are on walkable tiles, used for collision detection.
pub fn is_rect_walkable(rect: Rect, level_map: &Vec<Vec<u8>>) -> bool {
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

pub fn load_level_from_string(content: &str) -> Vec<Vec<u8>> {
    let lines: Vec<_> = content.lines().collect();
    let max_width = lines.iter().map(|s| s.len()).max().unwrap_or(0);

    lines
        .into_iter()
        .map(|line| {
            let mut row: Vec<u8> = line
                .chars()
                .map(|c| match c {
                    '#' => 1,
                    '.' => 2,
                    'o' => 3,
                    _ => 0,
                })
                .collect();
            row.resize(max_width, 0); // Pad with empty space
            row
        })
        .collect()
}

pub fn save_level_to_string(map: &Vec<Vec<u8>>) -> String {
    map.iter()
        .map(|row| {
            row.iter()
                .map(|&cell| match cell {
                    1 => '#',
                    2 => '.',
                    3 => 'o',
                    _ => ' ',
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}
