use crate::constants::*;
use macroquad::prelude::*;

pub enum PauseAction {
    None,
    Resume,
    Save,
    Options,
    Load,
    MainMenu,
    Quit,
}

use crate::scenes::game::GameState;

pub struct PauseState {
    pub game_state: Option<GameState>, // Option for easy take()
    selected_index: usize,
    pub notification_timer: f32, // For "Details Saved" message
    pub _pause_music: Option<macroquad::audio::Sound>, // Placeholder for future Pause Menu Song
}

impl PauseState {
    pub fn new(game_state: GameState) -> Self {
        Self {
            game_state: Some(game_state),
            selected_index: 0,
            notification_timer: 0.0,
            _pause_music: None, // Placeholder
        }
    }

    pub fn update(&mut self) -> PauseAction {
        let dt = get_frame_time();
        if self.notification_timer > 0.0 {
            self.notification_timer -= dt;
        }

        let button_count = 6; // RESUME, SAVE, OPTIONS, LOAD, MAIN MENU, QUIT

        // Keyboard Navigation
        if crate::input::is_nav_down_pressed() {
            self.selected_index = (self.selected_index + 1) % button_count;
        }
        if crate::input::is_nav_up_pressed() {
            if self.selected_index == 0 {
                self.selected_index = button_count - 1;
            } else {
                self.selected_index -= 1;
            }
        }
        if crate::input::is_confirm_pressed() || is_key_pressed(KeyCode::Space) {
            match self.selected_index {
                0 => return PauseAction::Resume,
                1 => {
                    if let Some(state) = &mut self.game_state {
                        state.save();
                        self.notification_timer = 2.0;
                    }
                    return PauseAction::None; // Stay in pause menu
                }
                2 => return PauseAction::Options,
                3 => return PauseAction::Load,
                4 => return PauseAction::MainMenu,
                5 => return PauseAction::Quit,
                _ => {}
            }
        }

        if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape) {
            return PauseAction::Resume;
        }

        // Mouse Interaction
        let mouse_pos = mouse_position();
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;
        let btn_y_start = center_y - 80.0; // Moved up slightly to fit 6 buttons
        let spacing = 65.0; // Slightly tighter spacing
        let buttons = [
            PauseAction::Resume,
            PauseAction::Save,
            PauseAction::Options,
            PauseAction::Load,
            PauseAction::MainMenu,
            PauseAction::Quit,
        ];
        let btn_labels = [
            "RESUME",
            "SAVE",
            "OPTIONS",
            "LOAD",
            "QUIT TO MAIN MENU",
            "QUIT",
        ];

        if is_mouse_button_pressed(MouseButton::Left) {
            for (i, _action) in buttons.iter().enumerate() {
                let y_pos = btn_y_start + (i as f32 * spacing);
                let dims = measure_text(btn_labels[i], None, 40, 1.0);
                let x_pos = center_x - dims.width / 2.0;

                if mouse_pos.0 >= x_pos
                    && mouse_pos.0 <= x_pos + dims.width
                    && mouse_pos.1 >= y_pos - 30.0
                    && mouse_pos.1 <= y_pos + 10.0
                {
                    match i {
                        0 => return PauseAction::Resume,
                        1 => {
                            if let Some(state) = &mut self.game_state {
                                state.save();
                                self.notification_timer = 2.0;
                            }
                            return PauseAction::None;
                        }
                        2 => return PauseAction::Options,
                        3 => return PauseAction::Load,
                        4 => return PauseAction::MainMenu,
                        5 => return PauseAction::Quit,
                        _ => {}
                    }
                }
            }
        }

        // Update selected index on mouse hover
        for (i, _action) in buttons.iter().enumerate() {
            let y_pos = btn_y_start + (i as f32 * spacing);
            let dims = measure_text(btn_labels[i], None, 40, 1.0);
            let x_pos = center_x - dims.width / 2.0;

            if mouse_pos.0 >= x_pos
                && mouse_pos.0 <= x_pos + dims.width
                && mouse_pos.1 >= y_pos - 30.0
                && mouse_pos.1 <= y_pos + 10.0
            {
                self.selected_index = i;
            }
        }

        PauseAction::None
    }

    pub fn draw(&self) {
        // Semi-transparent overlay
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.7),
        );

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        draw_text(
            "PAUSED",
            center_x - 100.0,
            center_y - 200.0,
            80.0,
            PALETTE_NEON_BLUE,
        );

        // Save Notification
        if self.notification_timer > 0.0 {
            let msg = "GAME SAVED";
            let dims = measure_text(msg, None, 30, 1.0);
            draw_text(
                msg,
                center_x - dims.width / 2.0,
                center_y - 120.0,
                30.0,
                GREEN,
            );
        }

        // Buttons
        let buttons = [
            "RESUME",
            "SAVE",
            "OPTIONS",
            "LOAD",
            "QUIT TO MAIN MENU",
            "QUIT",
        ];
        let btn_y_start = center_y - 80.0;
        let spacing = 65.0;

        for (i, btn_text) in buttons.iter().enumerate() {
            let y_pos = btn_y_start + (i as f32 * spacing);
            let dims = measure_text(btn_text, None, 40, 1.0);
            let x_pos = center_x - dims.width / 2.0;

            let color = if i == self.selected_index {
                PALETTE_NEON_RED
            } else {
                WHITE
            };

            if i == self.selected_index {
                draw_text(">", x_pos - 30.0, y_pos, 40.0, PALETTE_NEON_RED);
            }

            draw_text(btn_text, x_pos, y_pos, 40.0, color);
        }
    }
}
