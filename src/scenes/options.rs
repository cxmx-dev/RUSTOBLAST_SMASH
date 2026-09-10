use crate::constants::*;
use crate::scenes::game::GameState;
use macroquad::prelude::*;

pub enum OptionsAction {
    None,
    Back(f32, f32, f32, Option<GameState>), // Master, SFX, Music, Preserved GameState
}

pub struct OptionsState {
    selected_index: usize,
    master_volume: f32, // 0.0 to 1.0
    sfx_volume: f32,    // 0.0 to 1.0
    music_volume: f32,  // 0.0 to 1.0

    // Key Repeat State
    key_hold_timer: f32,

    // Preserved Game State
    pub game_state: Option<GameState>,
}

impl OptionsState {
    pub fn new(
        current_master: f32,
        current_sfx: f32,
        current_music: f32,
        game_state: Option<GameState>,
    ) -> Self {
        Self {
            selected_index: 0,
            master_volume: current_master,
            sfx_volume: current_sfx,
            music_volume: current_music,
            key_hold_timer: 0.0,
            game_state,
        }
    }

    pub fn update(&mut self) -> OptionsAction {
        let items_len = 4; // Master, SFX, Music, Back

        if crate::input::is_nav_down_pressed() {
            self.selected_index = (self.selected_index + 1) % items_len;
        }
        if crate::input::is_nav_up_pressed() {
            if self.selected_index == 0 {
                self.selected_index = items_len - 1;
            } else {
                self.selected_index -= 1;
            }
        }

        let dt = get_frame_time();

        // Handle Key Repeat Logic
        let mut adjust_val = 0.0f32;

        // Check Left Key
        if crate::input::is_move_left_down() {
            // Need to map "Key" for repeat logic... simplify: use Left as proxy for "Nav Left"
            // Actually, helper uses `is_key_down`. Let's just assume any left intent.
            // But we need `last_key` equivalent.
            // Let's Simplify: just use one timer.

            // If timer > 0, we are repeating.
            // How to detect "New Press"?
            // Helper `is_nav_left_pressed()` is distinct from `is_move_left_down()`.
            // But repeat logic relies on `is_key_down`.

            // Refactored Logic:
            if crate::input::is_nav_left_pressed() {
                adjust_val = -0.01;
                self.key_hold_timer = 0.0;
            } else if crate::input::is_move_left_down() {
                self.key_hold_timer += dt;
                if self.key_hold_timer > 0.25 {
                    adjust_val = -0.01;
                }
            }
        }
        // Check Right Key
        else if crate::input::is_move_right_down() {
            if crate::input::is_nav_right_pressed() {
                adjust_val = 0.01;
                self.key_hold_timer = 0.0;
            } else {
                // holding
                self.key_hold_timer += dt;
                if self.key_hold_timer > 0.25 {
                    adjust_val = 0.01;
                }
            }
        } else {
            self.key_hold_timer = 0.0;
        }

        // Mouse Interaction
        let mouse_pos = mouse_position();
        let center_x = screen_width() / 2.0;

        // Format Strings & Layout (Must match Draw to align hitboxes)
        let master_pct = (self.master_volume * 100.0).round() as i32;
        let master_str = format!("Master Volume: {}%", master_pct);
        let sfx_pct = (self.sfx_volume * 100.0).round() as i32;
        let sfx_str = format!("Sound Effects: {}%", sfx_pct);
        let music_pct = (self.music_volume * 100.0).round() as i32;
        let music_str = format!("Music: {}%", music_pct);

        let items = [
            (master_str.as_str(), 250.0, true),
            (sfx_str.as_str(), 350.0, true),
            (music_str.as_str(), 450.0, true),
            ("[ESC] BACK", screen_height() - 150.0, false),
        ];

        let mut mouse_adjust = 0.0f32;

        for (i, (text, y_pos, is_slider)) in items.iter().enumerate() {
            let dims = measure_text(text, None, 40, 1.0);
            let x_pos = center_x - dims.width / 2.0;

            // Hover Row Detection
            // Broad vertical detection, narrower horizontal
            if mouse_pos.1 >= y_pos - 30.0 && mouse_pos.1 <= y_pos + 10.0 {
                // If reasonably close horizontally
                if mouse_pos.0 >= center_x - 300.0 && mouse_pos.0 <= center_x + 300.0 {
                    self.selected_index = i;
                }
            }

            // Interaction if selected
            if self.selected_index == i {
                if *is_slider {
                    // Check Arrow Buttons
                    // < : x_pos + width + 20
                    // > : x_pos + width + 50
                    let arrow_y = *y_pos;
                    let left_arrow_x = x_pos + dims.width + 20.0;
                    let right_arrow_x = x_pos + dims.width + 50.0;

                    // Check Left Arrow
                    if mouse_pos.0 >= left_arrow_x - 5.0
                        && mouse_pos.0 <= left_arrow_x + 25.0
                        && mouse_pos.1 >= arrow_y - 25.0
                        && mouse_pos.1 <= arrow_y + 5.0
                    {
                        if is_mouse_button_down(MouseButton::Left) {
                            if self.key_hold_timer == 0.0 {
                                mouse_adjust = -0.01;
                            }
                            self.key_hold_timer += dt;
                            if self.key_hold_timer > 0.25 {
                                mouse_adjust = -0.01;
                            }
                        }
                    }
                    // Check Right Arrow
                    else if mouse_pos.0 >= right_arrow_x - 5.0
                        && mouse_pos.0 <= right_arrow_x + 25.0
                        && mouse_pos.1 >= arrow_y - 25.0
                        && mouse_pos.1 <= arrow_y + 5.0
                    {
                        if is_mouse_button_down(MouseButton::Left) {
                            if self.key_hold_timer == 0.0 {
                                mouse_adjust = 0.01;
                            }
                            self.key_hold_timer += dt;
                            if self.key_hold_timer > 0.25 {
                                mouse_adjust = 0.01;
                            }
                        }
                    }
                } else {
                    // Back Button Click
                    // Hitbox over text
                    if mouse_pos.0 >= x_pos
                        && mouse_pos.0 <= x_pos + dims.width
                        && mouse_pos.1 >= *y_pos - 30.0
                        && mouse_pos.1 <= *y_pos + 10.0
                    {
                        if is_mouse_button_pressed(MouseButton::Left) {
                            // Trigger Back
                            // Return action immediately below
                            // Hack: simulate 'Escape' specific check or just return here?
                            // We can just return the action right here.
                            return OptionsAction::Back(
                                self.master_volume,
                                self.sfx_volume,
                                self.music_volume,
                                self.game_state.take(),
                            );
                        }
                    }
                }
            }
        }

        if mouse_adjust != 0.0 {
            adjust_val = mouse_adjust; // Override key adjust if mouse input
        }

        // Apply Adjustment
        if adjust_val != 0.0 {
            match self.selected_index {
                0 => {
                    // Master
                    self.master_volume = (self.master_volume + adjust_val).clamp(0.0, 1.0);
                }
                1 => {
                    // SFX
                    self.sfx_volume = (self.sfx_volume + adjust_val).clamp(0.0, 1.0);
                }
                2 => {
                    // Music
                    self.music_volume = (self.music_volume + adjust_val).clamp(0.0, 1.0);
                }
                _ => {}
            }
        }

        if crate::input::is_confirm_pressed() || is_key_pressed(KeyCode::Space) {
            if self.selected_index == 3 {
                // We must take the game_state to pass it back
                return OptionsAction::Back(
                    self.master_volume,
                    self.sfx_volume,
                    self.music_volume,
                    self.game_state.take(),
                );
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            return OptionsAction::Back(
                self.master_volume,
                self.sfx_volume,
                self.music_volume,
                self.game_state.take(),
            );
        }
        OptionsAction::None
    }

    pub fn draw(&self) {
        clear_background(PALETTE_BLACK);

        let center_x = screen_width() / 2.0;

        draw_text(
            "OPTIONS",
            center_x - 100.0,
            100.0,
            60.0,
            PALETTE_NEON_PURPLE,
        );

        // Format Strings
        let master_pct = (self.master_volume * 100.0).round() as i32;
        let master_str = format!("Master Volume: {}%", master_pct);

        let sfx_pct = (self.sfx_volume * 100.0).round() as i32;
        let sfx_str = format!("Sound Effects: {}%", sfx_pct);

        let music_pct = (self.music_volume * 100.0).round() as i32;
        let music_str = format!("Music: {}%", music_pct);

        // Items: Text, Y-Pos, Is-Slider
        // Adjusted spacing to avoid overlap with Title (Y=100)
        let items = [
            (master_str.as_str(), 250.0, true),
            (sfx_str.as_str(), 350.0, true),
            (music_str.as_str(), 450.0, true),
            ("[ESC] BACK", screen_height() - 150.0, false),
        ];

        for (i, (text, y_pos, is_slider)) in items.iter().enumerate() {
            let color = if i == self.selected_index {
                PALETTE_NEON_RED
            } else {
                WHITE
            };

            let dims = measure_text(text, None, 40, 1.0);
            let x_pos = center_x - dims.width / 2.0;

            if i == self.selected_index {
                draw_text(">", x_pos - 40.0, *y_pos, 40.0, PALETTE_NEON_RED);

                if *is_slider {
                    draw_text(
                        "<",
                        x_pos + dims.width + 20.0,
                        *y_pos,
                        40.0,
                        PALETTE_NEON_RED,
                    );
                    draw_text(
                        ">",
                        x_pos + dims.width + 50.0,
                        *y_pos,
                        40.0,
                        PALETTE_NEON_RED,
                    );
                }
            }

            draw_text(text, x_pos, *y_pos, 40.0, color);
        }
    }
}
