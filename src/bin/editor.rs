use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, DrawParam, Image, Rect, Text};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::input::mouse::MouseButton;
use ggez::{Context, ContextBuilder, GameResult};
use rust_pack::*;
use std::env;
use std::io::{Read, Write};
use std::path;

enum EditorMode {
    Menu,
    Editing,
}

// Editor state
struct EditorState {
    wall_images: Vec<Image>,
    sprite_sheet: Image,
    display_map: Vec<Vec<u8>>,
    level_map: Vec<Vec<u8>>,
    small_dot_rect: Rect,
    big_dot_rect: Rect,
    current_tool: u8, // 0: Empty, 1: Wall, 2: Small Dot, 3: Big Dot
    filepath: String,
    mode: EditorMode,
    font: String,
    menu_options: Vec<(String, Rect)>,
}

impl EditorState {
    fn new(ctx: &mut Context, filepath: &str) -> GameResult<EditorState> {
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

        let level_map = if let Ok(mut file) = ctx.fs.open(filepath) {
            let mut content = String::new();
            if file.read_to_string(&mut content).is_ok() {
                load_level_from_string(&content)
            } else {
                create_default_map()
            }
        } else {
            create_default_map()
        };

        let font = "LiberationMono-Regular".to_string();

        let menu_options = vec![
            ("New Level".to_string(), Rect::new(0.0, 0.0, 0.0, 0.0)),
            ("Load Level".to_string(), Rect::new(0.0, 0.0, 0.0, 0.0)),
            ("Save Level".to_string(), Rect::new(0.0, 0.0, 0.0, 0.0)),
            ("Resume Editing".to_string(), Rect::new(0.0, 0.0, 0.0, 0.0)),
        ];

        let mut state = EditorState {
            wall_images,
            sprite_sheet,
            display_map: vec![],
            level_map,
            small_dot_rect,
            big_dot_rect,
            current_tool: 1,
            filepath: filepath.to_string(),
            mode: EditorMode::Menu,
            font,
            menu_options,
        };
        state.update_display_map();
        Ok(state)
    }

    fn update_display_map(&mut self) {
        self.display_map = self.level_map.clone();
        for (y, row) in self.level_map.iter().enumerate() {
            for (x, &tile_value) in row.iter().enumerate() {
                if tile_value == 1 {
                    let mut wall_mask: u8 = 0;
                    if is_wall_at(x as isize, y as isize - 1, &self.level_map) {
                        wall_mask += 1;
                    }
                    if is_wall_at(x as isize, y as isize + 1, &self.level_map) {
                        wall_mask += 2;
                    }
                    if is_wall_at(x as isize - 1, y as isize, &self.level_map) {
                        wall_mask += 4;
                    }
                    if is_wall_at(x as isize + 1, y as isize, &self.level_map) {
                        wall_mask += 8;
                    }
                    self.display_map[y][x] = wall_mask + WALL_CODE_OFFSET;
                }
            }
        }
    }

    fn save_level(&self, _ctx: &mut Context) -> GameResult {
        let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut path = path::PathBuf::from(manifest_dir);
            path.push("resources");
            path
        } else {
            path::PathBuf::from("./resources")
        };
        
        let relative_path = if self.filepath.starts_with("/") {
             &self.filepath[1..]
        } else {
             &self.filepath
        };

        let full_path = resource_dir.join(relative_path);
        
        let content = save_level_to_string(&self.level_map);
        
        if let Ok(mut file) = std::fs::File::create(&full_path) {
             let _ = file.write_all(content.as_bytes());
             println!("Saved level to {:?}", full_path);
        } else {
             eprintln!("Failed to save level to {:?}", full_path);
        }
        
        Ok(())
    }

    fn update_menu(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.mouse.button_pressed(MouseButton::Left) {
            let mouse_pos = ctx.mouse.position();
            let scaled_pos = Vec2::new(mouse_pos.x / 3.0, mouse_pos.y / 3.0);

            let mut clicked_index = None;
            for (i, (_, rect)) in self.menu_options.iter().enumerate() {
                if rect.contains(scaled_pos) {
                    clicked_index = Some(i);
                    break;
                }
            }

            if let Some(i) = clicked_index {
                match i {
                    0 => {
                        self.level_map = create_default_map();
                        self.update_display_map();
                        self.mode = EditorMode::Editing;
                    }
                    1 => {
                        if let Ok(mut file) = ctx.fs.open(&self.filepath) {
                            let mut content = String::new();
                            if file.read_to_string(&mut content).is_ok() {
                                self.level_map = load_level_from_string(&content);
                                self.update_display_map();
                            }
                        }
                        self.mode = EditorMode::Editing;
                    }
                    2 => {
                        self.save_level(ctx)?;
                    }
                    3 => {
                        self.mode = EditorMode::Editing;
                    }
                    _ => (),
                }
            }
        }
        Ok(())
    }

    fn draw_menu(&mut self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(Vec2::new(0.0, 0.0))
                .scale(Vec2::new(448.0, 320.0))
                .color(Color::new(0.1, 0.1, 0.1, 1.0)),
        );
        let mut text = Text::new("RUST-PAC EDITOR");
        text.set_font(self.font.clone()).set_scale(32.0);
        let text_dims = text.measure(ctx)?;
        let center_x = (448.0 - text_dims.x) / 2.0;
        canvas.draw(&text, Vec2::new(center_x, 40.0));

        let mut y = 100.0;
        for (i, (label, _)) in self.menu_options.clone().iter().enumerate() {
            let mut menu_item = Text::new(label.as_str());
            menu_item.set_font(self.font.clone()).set_scale(20.0);
            let dims = menu_item.measure(ctx)?;
            let center_x = (448.0 - dims.x) / 2.0;
            let dest = Vec2::new(center_x, y);

            let bounds = Rect::new(center_x - 10.0, y, dims.x + 20.0, dims.y);
            self.menu_options[i].1 = bounds;
            
            let mesh = graphics::Mesh::new_rectangle(
                ctx, 
                graphics::DrawMode::stroke(1.0), 
                bounds, 
                Color::WHITE
            )?;
            canvas.draw(&mesh, Vec2::new(0.0, 0.0));

            canvas.draw(&menu_item, dest);
            y += 30.0;
        }

        Ok(())
    }

    fn update_editor(&mut self, ctx: &mut Context) -> GameResult {
        let mouse = ctx.mouse.position();
        let scaled_mouse_x = mouse.x / 3.0;
        let scaled_mouse_y = mouse.y / 3.0;

        if ctx.mouse.button_pressed(MouseButton::Left) {
            let map_offset_x = 112.0;
            let map_x = ((scaled_mouse_x - map_offset_x) / TILE_SIZE) as isize;
            let map_y = ((scaled_mouse_y - MAZE_OFFSET_Y) / TILE_SIZE) as isize;
            
            if map_x >= 0 && map_y >= 0 {
                let x = map_x as usize;
                let y = map_y as usize;
                if y < self.level_map.len() && x < self.level_map[0].len() {
                    if self.level_map[y][x] != self.current_tool {
                        self.level_map[y][x] = self.current_tool;
                        self.update_display_map();
                    }
                }
            }
        }
        Ok(())
    }

    fn draw_editor(&mut self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        let legend_text = [
            "Controls: [1] Wall | [2] Dot | [3] Big Dot | [0] Erase",
            "[S] Save | [Esc] Menu | [C] Clear Level",
        ];
        let mut y = 5.0;
        for line in legend_text.iter() {
            let mut text = Text::new(*line);
            text.set_font(self.font.clone()).set_scale(10.0);
            canvas.draw(&text, Vec2::new(5.0, y));
            y += 12.0;
        }
        
        let mut tool_text = Text::new("Tool:");
        tool_text.set_font(self.font.clone()).set_scale(10.0);
        let tool_text_width = tool_text.measure(ctx)?.x;
        canvas.draw(&tool_text, Vec2::new(5.0, y));
        
        let color = match self.current_tool {
            1 => Color::BLUE,
            2 => Color::YELLOW,
            3 => Color::RED,
            _ => Color::WHITE,
        };
        let rect = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            Rect::new(0.0, 0.0, 10.0, 10.0), 
            color
        )?;
        canvas.draw(&rect, Vec2::new(5.0 + tool_text_width + 5.0, y));

        let map_offset_x = 112.0; // (448.0 - (28 * 8)) / 2
        let map_width = 28.0 * TILE_SIZE;
        let map_height = 31.0 * TILE_SIZE;

        let map_outline = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            Rect::new(map_offset_x - 1.0, MAZE_OFFSET_Y - 1.0, map_width + 2.0, map_height + 2.0),
            Color::WHITE
        )?;
        canvas.draw(&map_outline, Vec2::new(0.0, 0.0));

        for (y, row) in self.display_map.iter().enumerate() {
            for (x, &tile_type) in row.iter().enumerate() {
                let dest_x = (x as f32 * TILE_SIZE) + map_offset_x;
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
        
        Ok(())
    }

    fn key_down_editor(&mut self, ctx: &mut Context, input: KeyInput) -> GameResult {
        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::Key1 => self.current_tool = 1,
                KeyCode::Key2 => self.current_tool = 2,
                KeyCode::Key3 => self.current_tool = 3,
                KeyCode::Key0 | KeyCode::Grave => self.current_tool = 0,
                KeyCode::S => {
                    self.save_level(ctx)?;
                }
                KeyCode::C => {
                    self.level_map = create_default_map();
                    self.update_display_map();
                }
                KeyCode::Escape => self.mode = EditorMode::Menu,
                _ => {}
            }
        }
        Ok(())
    }
}

fn create_default_map() -> Vec<Vec<u8>> {
    vec![vec![0; 28]; 31]
}

impl EventHandler for EditorState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.mode {
            EditorMode::Menu => self.update_menu(ctx),
            EditorMode::Editing => self.update_editor(ctx),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_screen_coordinates(Rect::new(0.0, 0.0, 448.0, 320.0));
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        match self.mode {
            EditorMode::Menu => self.draw_menu(&mut canvas, ctx)?,
            EditorMode::Editing => self.draw_editor(&mut canvas, ctx)?,
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> GameResult {
        match self.mode {
            EditorMode::Menu => {
                if let Some(KeyCode::Escape) = input.keycode {
                    self.mode = EditorMode::Editing;
                }
            }
            EditorMode::Editing => self.key_down_editor(ctx, input)?,
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
        .window_mode(ggez::conf::WindowMode::default().dimensions(1344.0, 960.0))
        .add_resource_path(resource_dir)
        .build()?;

    let state = EditorState::new(&mut ctx, "/levels/level1.txt")?;

    event::run(ctx, event_loop, state)
}
