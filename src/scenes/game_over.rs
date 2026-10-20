use macroquad::prelude::*;

pub enum GameOverAction {
    None,
    Replay,
    MainMenu,
    Quit,
}

pub struct GameOverState {
    pub _score: i32,
    pub skull_rotation: f32,
    pub game_over_sound: Option<macroquad::audio::Sound>,
    pub _music_volume: f32,
    pub sfx_volume: f32,
    pub sound_played: bool,
    pub selected_option: usize, // 0: Replay, 1: MainMenu, 2: Quit
}

impl GameOverState {
    pub fn new(
        score: i32,
        game_over_sound: Option<macroquad::audio::Sound>,
        sfx_volume: f32,
        music_volume: f32,
    ) -> Self {
        Self {
            _score: score,
            skull_rotation: 0.0,
            game_over_sound,
            sfx_volume,
            _music_volume: music_volume,
            sound_played: false,
            selected_option: 0,
        }
    }

    pub fn update(&mut self) -> GameOverAction {
        // Play Sound once
        if !self.sound_played {
            if let Some(sound) = &self.game_over_sound {
                let params = macroquad::audio::PlaySoundParams {
                    looped: false,
                    volume: self.sfx_volume,
                };
                macroquad::audio::play_sound(sound, params);
            }
            self.sound_played = true;
        }

        // Skull Animation: Rotate slightly right then left real fast
        // Speed: 15.0 (fast), Range: +/- 15 degrees (~0.26 radians)
        let time = get_time() as f32;
        self.skull_rotation = (time * 15.0).sin() * 15.0f32.to_radians();

        // Keyboard Navigation
        // Keyboard Navigation
        if crate::input::is_nav_down_pressed() {
            self.selected_option = (self.selected_option + 1) % 3;
        }
        if crate::input::is_nav_up_pressed() {
            if self.selected_option == 0 {
                self.selected_option = 2;
            } else {
                self.selected_option -= 1;
            }
        }

        if crate::input::is_confirm_pressed() || is_key_pressed(KeyCode::Space) {
            match self.selected_option {
                0 => return GameOverAction::Replay,
                1 => return GameOverAction::MainMenu,
                2 => return GameOverAction::Quit,
                _ => {}
            }
        }

        // Mouse interaction (Hover updates selection, Click triggers)
        let (_mouse_x, mouse_y) = mouse_position();
        let skull_y = screen_height() * 0.3;
        let replay_y = skull_y + 200.0;
        let menu_y = replay_y + 120.0;
        let quit_y = menu_y + 200.0;

        let check_hover = |y: f32, margin_top: f32, margin_bot: f32| -> bool {
            mouse_y >= y - margin_top && mouse_y <= y + margin_bot
        };

        if check_hover(replay_y, 50.0, 20.0) {
            self.selected_option = 0;
            if is_mouse_button_pressed(MouseButton::Left) {
                return GameOverAction::Replay;
            }
        } else if check_hover(menu_y, 50.0, 20.0) {
            self.selected_option = 1;
            if is_mouse_button_pressed(MouseButton::Left) {
                return GameOverAction::MainMenu;
            }
        } else if check_hover(quit_y, 80.0, 50.0) {
            self.selected_option = 2;
            if is_mouse_button_pressed(MouseButton::Left) {
                return GameOverAction::Quit;
            }
        }

        GameOverAction::None
    }

    fn draw_skull(&self, center_x: f32, center_y: f32) {
        // Procedural Skull
        // Rotate around center
        let rot = self.skull_rotation;
        let cos_r = rot.cos();
        let sin_r = rot.sin();

        // Function to rotate a point relative to center (0,0 of skull) then translate to world
        let transform = |x: f32, y: f32| -> Vec2 {
            let rx = x * cos_r - y * sin_r;
            let ry = x * sin_r + y * cos_r;
            vec2(center_x + rx, center_y + ry)
        };

        // 1. Cranium (Top part)
        // Draw multiple circles to animate/wiggle or just one big one?
        // We'll draw a main "Circle" by using `draw_circle` but that doesn't rotate well if it's not a texture.
        // Actually, draw_circle is always axis aligned.
        // To rotate "shapes", we need to draw lines or polygons.
        // Let's use `draw_poly` maybe? Or just lines for a "sketchy" look?
        // Let's try to compose it of a few rotated rects/custom drawing.

        // Simpler approach: Draw axis-aligned parts but offset their positions based on rotation.
        // This works for "wiggling" but not true rotation of the shape itself.
        // Given the user wants "animatedly rotates slightly... real fast", the shape itself needs to rotate.
        // We can use `draw_texture_ex` if we generated a texture, but we are drawing directly.
        // We will simulate the skull with standard primitives that follow the rotation transform.

        // 30% smaller from 2.0 -> 1.4
        let scale = 1.4;

        // Jaw (Rectangle-ish)
        let jaw_w = 80.0 * scale;
        let jaw_h = 60.0 * scale;
        let jaw_y_offset = 50.0 * scale;

        let jaw_tl = transform(-jaw_w / 2.0, jaw_y_offset);
        let jaw_tr = transform(jaw_w / 2.0, jaw_y_offset);
        let jaw_br = transform(jaw_w / 2.0, jaw_y_offset + jaw_h);
        let jaw_bl = transform(-jaw_w / 2.0, jaw_y_offset + jaw_h);

        // Draw Jaw Outline
        draw_line(jaw_tl.x, jaw_tl.y, jaw_tr.x, jaw_tr.y, 5.0, WHITE);
        draw_line(jaw_tr.x, jaw_tr.y, jaw_br.x, jaw_br.y, 5.0, WHITE);
        draw_line(jaw_br.x, jaw_br.y, jaw_bl.x, jaw_bl.y, 5.0, WHITE);
        draw_line(jaw_bl.x, jaw_bl.y, jaw_tl.x, jaw_tl.y, 5.0, WHITE);

        // Cranium (Circle approximated by lines or just a filled circle if rotation is small enough it doesn't matter?)
        // The user wants it to rotate. A circle rotating looks like a circle.
        // We need "features" to show rotation. Eyes!

        let cranium_radius = 90.0 * scale;
        let cranium_center = transform(0.0, -20.0 * scale);

        draw_circle(cranium_center.x, cranium_center.y, cranium_radius, WHITE);
        // Cover bottom of circle for jaw connection?
        // Actually letting them overlap is fine for a "skull" shape.

        // Eyes (Black sockets)
        let eye_offset_x = 35.0 * scale;
        let eye_offset_y = -30.0 * scale;
        let eye_size = 25.0 * scale;

        let left_eye = transform(-eye_offset_x, eye_offset_y);
        let right_eye = transform(eye_offset_x, eye_offset_y);

        draw_circle(left_eye.x, left_eye.y, eye_size, BLACK);
        draw_circle(right_eye.x, right_eye.y, eye_size, BLACK);

        // Nose (Triangle)
        let nose_y = 10.0 * scale;
        let nose_top = transform(0.0, nose_y - 15.0);
        let nose_bl = transform(-10.0, nose_y + 10.0);
        let nose_br = transform(10.0, nose_y + 10.0);

        draw_triangle(nose_top, nose_bl, nose_br, BLACK);

        // Teeth (Lines on jaw)
        let _teeth_y = jaw_y_offset + jaw_h * 0.5;
        for i in -2..=2 {
            let offset = i as f32 * 20.0 * scale;
            let t_top = transform(offset, jaw_y_offset);
            let t_bot = transform(offset, jaw_y_offset + jaw_h);
            draw_line(t_top.x, t_top.y, t_bot.x, t_bot.y, 3.0, BLACK);
        }
    }

    pub fn draw(&self) {
        clear_background(BLACK);

        let center_x = screen_width() / 2.0;
        let skull_y = screen_height() * 0.3;

        // Draw "GAME-OVER"
        let title_text = "GAME-OVER";
        let title_dims = measure_text(title_text, None, 100, 1.0);
        draw_text(
            title_text,
            center_x - title_dims.width / 2.0,
            skull_y - 150.0,
            100.0,
            RED,
        );

        // Draw Procedural Skull
        self.draw_skull(center_x, skull_y);

        // Helper for Options
        let draw_option = |text: &str, y: f32, size: u16, selected: bool| {
            let dims = measure_text(text, None, size, 1.0);
            let start_x = center_x - dims.width / 2.0;

            let color_base = if selected { GOLD } else { DARKGRAY };
            let color_mid = if selected { YELLOW } else { GRAY };
            let color_top = if selected { WHITE } else { WHITE }; // Keep top white for pop? Or Gold?

            // Base
            draw_text(text, start_x, y, size as f32, color_base);

            if selected {
                // Add Arrow >
                draw_text(">", start_x - 60.0, y, size as f32, YELLOW);
                draw_text("<", start_x + dims.width + 20.0, y, size as f32, YELLOW);

                // Pulse/Glow effect
                draw_text(text, start_x, y - 5.0, size as f32, color_mid);
                draw_text(text, start_x, y - 10.0, size as f32, color_top);
            } else {
                // Standard Gradient
                draw_text(text, start_x, y - 5.0, size as f32, color_mid);
                draw_text(text, start_x, y - 10.0, size as f32, color_top);
            }
        };

        let replay_y = skull_y + 200.0;
        draw_option("REPLAY", replay_y, 150, self.selected_option == 0);

        let menu_y = replay_y + 120.0;
        draw_option("MAIN MENU", menu_y, 100, self.selected_option == 1);

        let quit_y = menu_y + 200.0;
        // Quit needs Red scheme?
        let quit_text = "QUIT";
        let quit_size = 250;
        let quit_dims = measure_text(quit_text, None, quit_size, 1.0);
        let quit_x = center_x - quit_dims.width / 2.0;

        if self.selected_option == 2 {
            draw_text(">", quit_x - 80.0, quit_y, quit_size as f32, RED);
            draw_text(
                "<",
                quit_x + quit_dims.width + 20.0,
                quit_y,
                quit_size as f32,
                RED,
            );
            draw_text(quit_text, quit_x, quit_y, quit_size as f32, RED);
        } else {
            draw_text(quit_text, quit_x, quit_y, quit_size as f32, MAROON);
        }
    }
}
