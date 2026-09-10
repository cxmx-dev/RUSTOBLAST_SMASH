use crate::assets::{self, ShapeDesign};
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Sidekick {
    #[serde(with = "crate::serde_helper")]
    pub position: Vec2,
    #[serde(with = "crate::serde_helper")]
    pub offset: Vec2, // Offset from player center
    pub last_shot_time: f64,
    pub fire_rate: f64,
    pub active: bool,
    pub design: ShapeDesign,
    pub radius: f32,
}

impl Sidekick {
    pub fn new(offset: Vec2) -> Self {
        let mut sk = Self {
            position: vec2(0.0, 0.0), // Will be updated immediately
            offset,
            last_shot_time: 0.0,
            fire_rate: 0.1, // Fast fire rate (same as player default or faster)
            active: true,
            design: ShapeDesign::default(),
            radius: 20.0, // Approximation
        };
        sk.load_assets();
        sk
    }

    fn load_assets(&mut self) {
        self.design = assets::load_shape_file("player-extra-ship.json");
    }

    pub fn update(&mut self, player_pos: Vec2, _dt: f32) {
        // Strict follow for now (can add smoothing later if requested)
        // Offset is relative to player center.
        self.position = player_pos + self.offset;
    }

    pub fn draw(&self) {
        if self.active {
            // Scale 2.0? Small sidekick.
            assets::draw_design(&self.design, self.position, 0.0, 2.0, get_time() as f32);
        }
    }
}
