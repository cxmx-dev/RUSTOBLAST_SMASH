#![allow(dead_code)]
use macroquad::prelude::*;

pub const WINDOW_WIDTH: i32 = 2560;
pub const WINDOW_HEIGHT: i32 = 1440;
pub const WINDOW_TITLE: &str = "RUSTOBLAST_SMASH";

pub const PALETTE_BLACK: Color = Color::new(0.05, 0.05, 0.05, 1.00); // #0d0d0d
pub const PALETTE_NEON_BLUE: Color = Color::new(0.00, 1.00, 1.00, 1.00); // Cyan (used for 2x/4x or accents)
pub const PALETTE_NEON_RED: Color = Color::new(1.00, 0.00, 0.30, 1.00); // Neon Red
pub const PALETTE_NEON_PURPLE: Color = Color::new(0.70, 0.00, 1.00, 1.00); // Purple
pub const PALETTE_WHITE: Color = WHITE; // Standard White
pub const PALETTE_CURSOR_ORANGE: Color = Color::new(1.0, 0.75, 0.0, 1.0); // Orangish-Yellow
pub const CURSOR_SIZE: f32 = 30.0; // ~20% bigger than standard 24px

// Background Colors for Multipliers
pub const BG_COLOR_1X: Color = PALETTE_BLACK;
pub const BG_COLOR_2X: Color = Color::new(0.0, 0.0, 0.5, 1.0); // Dark Blue
pub const BG_COLOR_3X: Color = Color::new(0.2, 0.0, 0.4, 1.0); // Dark Purple
pub const BG_COLOR_4X: Color = Color::new(0.0, 0.4, 0.4, 1.0); // Dark Turquoise
pub const BG_COLOR_5X: Color = Color::new(0.3, 0.3, 0.3, 1.0); // Gray
pub const BG_COLOR_6X: Color = PALETTE_BLACK;

pub const UFO_UNLOCK_SCORE: i32 = 5_000;
pub const POLY_WAVE_CHANCE: f32 = 0.02;
pub const POLY_WAVE_COUNT: i32 = 13;
pub const UFO_BOMB_COOLDOWN: f32 = 1.25;

/// Cumulative spawn weights, early game (score < UFO_UNLOCK_SCORE).
pub const EARLY_GUIDED: f32 = 0.15;
pub const EARLY_SMALL_SPINNER: f32 = 0.29;
pub const EARLY_BIG_SPINNER: f32 = 0.41;
// remainder: BigRock

/// Cumulative spawn weights after UFO unlock.
pub const LATE_UFO: f32 = 0.10;
pub const LATE_GUIDED: f32 = 0.22;
pub const LATE_SMALL_SPINNER: f32 = 0.38;
pub const LATE_BIG_SPINNER: f32 = 0.52;
// remainder: BigRock
