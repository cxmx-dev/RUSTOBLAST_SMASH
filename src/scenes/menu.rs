use crate::constants::*;
use macroquad::prelude::*;

pub enum MainMenuAction {
    None,
    NewGame,
    Options,
    LoadGame,
    Quit,
}

// ... imports unchanged

pub struct MainMenuState {
    selected_index: usize,
}

impl MainMenuState {
    pub fn new() -> Self {
        Self { selected_index: 0 }
    }

    pub fn update(&mut self) -> MainMenuAction {
        let button_count = 4; // NEW GAME, OPTIONS, LOAD GAME, QUIT

        // Keyboard Navigation
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
                0 => return MainMenuAction::NewGame,
                1 => return MainMenuAction::Options,
                2 => return MainMenuAction::LoadGame,
                3 => return MainMenuAction::Quit,
                _ => {}
            }
        }

        // Keyboard Shortcuts (Keep existing)
        if is_key_pressed(KeyCode::N) {
            return MainMenuAction::NewGame;
        }
        if is_key_pressed(KeyCode::O) {
            return MainMenuAction::Options;
        }
        if is_key_pressed(KeyCode::L) {
            return MainMenuAction::LoadGame;
        }
        if is_key_pressed(KeyCode::Q) {
            return MainMenuAction::Quit;
        }

        // Mouse Interaction
        let mouse_pos = mouse_position();
        let center_x = screen_width() / 2.0;
        let btn_y_start = 500.0;
        let spacing = 80.0;
        let buttons = ["NEW GAME", "OPTIONS", "LOAD GAME", "QUIT"];

        if is_mouse_button_pressed(MouseButton::Left) {
            for (i, btn_text) in buttons.iter().enumerate() {
                let y_pos = btn_y_start + (i as f32 * spacing);
                let dims = measure_text(btn_text, None, 50, 1.0);
                let x_pos = center_x - dims.width / 2.0;

                if mouse_pos.0 >= x_pos - 10.0
                    && mouse_pos.0 <= x_pos + dims.width + 10.0
                    && mouse_pos.1 >= y_pos - 40.0
                    && mouse_pos.1 <= y_pos + 10.0
                {
                    match i {
                        0 => return MainMenuAction::NewGame,
                        1 => return MainMenuAction::Options,
                        2 => return MainMenuAction::LoadGame,
                        3 => return MainMenuAction::Quit,
                        _ => {}
                    }
                }
            }
        }

        // Update selected index on mouse hover
        for (i, btn_text) in buttons.iter().enumerate() {
            let y_pos = btn_y_start + (i as f32 * spacing);
            let dims = measure_text(btn_text, None, 50, 1.0);
            let x_pos = center_x - dims.width / 2.0;

            if mouse_pos.0 >= x_pos - 10.0
                && mouse_pos.0 <= x_pos + dims.width + 10.0
                && mouse_pos.1 >= y_pos - 40.0
                && mouse_pos.1 <= y_pos + 10.0
            {
                self.selected_index = i;
            }
        }

        MainMenuAction::None
    }

    pub fn draw(&self) {
        let center_x = screen_width() / 2.0;
        let _center_y = screen_height() / 2.0;

        // Title: RUSTOBLAST
        let text1 = "RUSTOBLAST";
        let text2 = "SMASH";

        let dims1 = measure_text(text1, None, 100, 1.0);
        let dims2 = measure_text(text2, None, 100, 1.0);

        // Draw Title with Glow efffect (shadow)
        draw_text(
            text1,
            center_x - dims1.width / 2.0 + 4.0,
            200.0 + 4.0,
            100.0,
            Color::new(0.0, 0.5, 0.5, 0.5),
        );
        draw_text(
            text1,
            center_x - dims1.width / 2.0,
            200.0,
            100.0,
            PALETTE_NEON_BLUE,
        );

        draw_text(
            text2,
            center_x - dims2.width / 2.0 + 4.0,
            310.0 + 4.0,
            100.0,
            Color::new(0.5, 0.0, 0.4, 0.5),
        );
        draw_text(
            text2,
            center_x - dims2.width / 2.0,
            310.0,
            100.0,
            PALETTE_NEON_PURPLE,
        );

        // Buttons
        let btn_y_start = 500.0;
        let spacing = 80.0;
        let buttons = ["NEW GAME", "OPTIONS", "LOAD GAME", "QUIT"];

        for (i, btn_text) in buttons.iter().enumerate() {
            let y_pos = btn_y_start + (i as f32 * spacing);
            let dims = measure_text(btn_text, None, 50, 1.0);
            let x_pos = center_x - dims.width / 2.0;

            let color = if i == self.selected_index {
                PALETTE_NEON_RED
            } else {
                WHITE
            };

            // Draw selection arrow if selected
            if i == self.selected_index {
                draw_text(">", x_pos - 40.0, y_pos, 50.0, PALETTE_NEON_RED);
            }

            draw_text(btn_text, x_pos, y_pos, 50.0, color);
        }

        // Keybind Hints
        draw_text(
            "[N] New Game  [O] Options  [L] Load  [Q] Quit",
            center_x - 300.0,
            screen_height() - 50.0,
            30.0,
            DARKGRAY,
        );
    }
}
