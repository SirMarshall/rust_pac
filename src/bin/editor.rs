use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, DrawParam, Image, Rect};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, ContextBuilder, GameResult};
use rust_pack::{is_wall_at, MAZE_OFFSET_Y, TILE_SIZE, WALL_CODE_OFFSET};
use std::env;
use std::path;

// --- Editor Structures ---

struct EditorState {
    wall_images: Vec<Image>,
    sprite_sheet: Image,
    map: Vec<Vec<u8>>, // The map being edited
    display_map: Vec<Vec<u8>>, // The calculated display map
    small_dot_rect: Rect,
    big_dot_rect: Rect,
    current_tool: u8, // 1: Wall, 2: Small Dot, 3: Big Dot, 0: Empty
}

impl EditorState {
    fn new(ctx: &mut Context) -> GameResult<EditorState> {
        // Load wall images
        let mut wall_images = Vec::with_capacity(16);
        for i in 0..16 {
            let path = format!("/walls/wall_{}.png", i);
            wall_images.push(Image::from_path(ctx, &path)?);
        }

        let sprite_sheet = Image::from_path(ctx, "/player.png")?;

        let sprite_width_normalized = 1.0 / 6.0;
        let sprite_height_normalized = 1.0;

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

        // Initialize with default 28x31 map (all walls or empty?)
        // Let's initialize with empty map (0) and borders (1).
        let rows = 31;
        let cols = 28;
        let mut map = vec![vec![0u8; cols]; rows];

        // Optional: Pre-fill borders with walls
        for y in 0..rows {
            for x in 0..cols {
                if x == 0 || x == cols - 1 || y == 0 || y == rows - 1 {
                    map[y][x] = 1;
                }
            }
        }

        // Also update display_map initially
        let mut display_map = map.clone();
        update_display_map(&map, &mut display_map);

        Ok(EditorState {
            wall_images,
            sprite_sheet,
            map,
            display_map,
            small_dot_rect,
            big_dot_rect,
            current_tool: 1, // Default to Wall
        })
    }
}

// Helper to update display map based on wall neighbors
fn update_display_map(map: &Vec<Vec<u8>>, display_map: &mut Vec<Vec<u8>>) {
    for (y, row) in map.iter().enumerate() {
        for (x, &tile_value) in row.iter().enumerate() {
            if tile_value == 1 {
                let mut wall_mask: u8 = 0;
                if is_wall_at(x as isize, y as isize - 1, map) {
                    wall_mask += 1;
                } // North
                if is_wall_at(x as isize, y as isize + 1, map) {
                    wall_mask += 2;
                } // South
                if is_wall_at(x as isize - 1, y as isize, map) {
                    wall_mask += 4;
                } // West
                if is_wall_at(x as isize + 1, y as isize, map) {
                    wall_mask += 8;
                } // East
                display_map[y][x] = wall_mask + WALL_CODE_OFFSET;
            } else {
                display_map[y][x] = tile_value;
            }
        }
    }
}

impl EventHandler for EditorState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mouse_ctx = &ctx.mouse;
        if mouse_ctx.button_pressed(ggez::input::mouse::MouseButton::Left) {
            let pos = mouse_ctx.position();
            let map_x = (pos.x / TILE_SIZE) as isize;
            let map_y = ((pos.y - MAZE_OFFSET_Y) / TILE_SIZE) as isize;

            if map_y >= 0
                && map_y < self.map.len() as isize
                && map_x >= 0
                && map_x < self.map[0].len() as isize
            {
                let x = map_x as usize;
                let y = map_y as usize;
                if self.map[y][x] != self.current_tool {
                    self.map[y][x] = self.current_tool;
                    // Recalculate display map.
                    // Optimally, only update neighbors, but full update is fast enough for editor.
                    update_display_map(&self.map, &mut self.display_map);
                }
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

        // Draw cursor highlight
        let mouse_pos = ctx.mouse.position();
         let map_x = (mouse_pos.x / TILE_SIZE).floor();
         let map_y = ((mouse_pos.y - MAZE_OFFSET_Y) / TILE_SIZE).floor();

         if map_y >= 0.0 && map_y < self.map.len() as f32 && map_x >= 0.0 && map_x < self.map[0].len() as f32 {
             let dest_x = map_x * TILE_SIZE;
             let dest_y = (map_y * TILE_SIZE) + MAZE_OFFSET_Y;

             let rect = graphics::Mesh::new_rectangle(
                 ctx,
                 graphics::DrawMode::stroke(1.0),
                 Rect::new(dest_x, dest_y, TILE_SIZE, TILE_SIZE),
                 Color::RED,
             )?;
             canvas.draw(&rect, DrawParam::default());
         }

        canvas.finish(ctx)?;
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
                KeyCode::Key1 => self.current_tool = 1,
                KeyCode::Key2 => self.current_tool = 2,
                KeyCode::Key3 => self.current_tool = 3,
                KeyCode::Key0 => self.current_tool = 0,
                KeyCode::P | KeyCode::E => {
                    println!("vec![");
                    for row in &self.map {
                        print!("    vec![");
                        for (i, val) in row.iter().enumerate() {
                            print!("{}", val);
                            if i < row.len() - 1 {
                                print!(", ");
                            }
                        }
                        println!("],");
                    }
                    println!("];");
                    println!("Exported map to stdout.");
                }
                _ => {}
            }
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ContextBuilder::new("rust_pac_editor", "Sir Marshall")
        .window_setup(ggez::conf::WindowSetup::default().title("RUST PAC EDITOR"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(672.0, 864.0))
        .add_resource_path(resource_dir)
        .build()?;

    let editor_state = EditorState::new(&mut ctx)?;

    event::run(ctx, event_loop, editor_state)
}
