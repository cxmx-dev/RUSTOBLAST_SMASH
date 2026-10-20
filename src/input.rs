use macroquad::prelude::*;

pub fn is_confirm_pressed() -> bool {
    is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter)
}

pub fn is_nav_up_pressed() -> bool {
    is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Kp8)
}

pub fn is_nav_down_pressed() -> bool {
    is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Kp2)
}

pub fn is_nav_left_pressed() -> bool {
    is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Kp4)
}

pub fn is_nav_right_pressed() -> bool {
    is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Kp6)
}

// For continuous movement (player control)
pub fn is_move_left_down() -> bool {
    is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) || is_key_down(KeyCode::Kp4)
}

pub fn is_move_right_down() -> bool {
    is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) || is_key_down(KeyCode::Kp6)
}
