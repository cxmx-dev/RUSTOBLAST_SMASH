use crate::constants::*;
use macroquad::prelude::*;

pub struct WindowManager {
    pub is_fullscreen: bool,
}

impl WindowManager {
    pub fn new() -> Self {
        // We start in fullscreen based on main configuration, so track it as such.
        Self {
            is_fullscreen: true,
        }
    }

    pub fn update(&mut self) {
        // Check for Alt+Enter (Support both Alts)
        let alt_pressed = is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt);
        if alt_pressed && is_key_pressed(KeyCode::Enter) {
            self.toggle();
        }
    }

    fn toggle(&mut self) {
        self.is_fullscreen = !self.is_fullscreen;
        set_fullscreen(self.is_fullscreen);

        if !self.is_fullscreen {
            // Force resolution to 2560x1440 when windowed
            // Note: Macroquad might not resize the window automatically when leaving fullscreen
            // without a restart or specific conf, but request_new_screen_size helps.
            request_new_screen_size(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);

            // Note: We can't easily force window position to 0,0 without native bindings,
            // but usually the OS handles it.
        }
    }
}
