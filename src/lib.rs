use ggez::graphics::Rect;

// --- Constants ---
pub const TILE_SIZE: f32 = 8.0; // Size of a single tile in pixels.
pub const MAZE_OFFSET_Y: f32 = TILE_SIZE * 5.0; // Vertical offset for the maze to make space for UI elements.
pub const WALL_CODE_OFFSET: u8 = 100; // Offset added to wall mask to distinguish wall types in `display_map`.

// --- Helper Functions ---

// Checks if a tile at given map coordinates is a wall. Handles out-of-bounds coordinates by treating them as walls.
pub fn is_wall_at(x: isize, y: isize, map: &Vec<Vec<u8>>) -> bool {
    if y < 0 || y >= map.len() as isize || x < 0 || x >= map[0].len() as isize {
        return true; // Outside map boundaries are considered walls.
    }
    map[y as usize][x as usize] == 1 // Check if the tile value is '1' (wall).
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
