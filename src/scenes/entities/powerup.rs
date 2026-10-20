use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum PowerUpType {
    Shield,
    Sidekick,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PowerUp {
    #[serde(with = "crate::serde_helper")]
    pub position: Vec2,
    pub power_type: PowerUpType,
    pub active: bool,
    pub radius: f32,
    pub timer: f32, // For animation
}

impl PowerUp {
    pub fn new(position: Vec2, power_type: PowerUpType) -> Self {
        Self {
            position,
            power_type,
            active: true,
            radius: 15.0,
            timer: 0.0,
        }
    }

    pub fn update(&mut self) {
        // Drift slowly downwards
        self.position.y += 1.25;
        self.timer += get_frame_time();

        if self.position.y > screen_height() + 50.0 {
            self.active = false;
        }
    }

    pub fn draw(&self) {
        match self.power_type {
            PowerUpType::Shield => {
                // "Yellow orbs that glow from dim yellow to bright white .4x per sec in an animated gradient style"
                // .4x per sec = Period of 2.5 seconds? Or 0.4 Hz? User likely means 0.4 seconds period or speed.
                // Let's assume 0.4 Hz -> 2.5s cycle. Or maybe rapid flicker? "Dim yellow to bright white"

                let speed = 2.5; // (1.0 / 0.4)
                let t = (self.timer * std::f32::consts::PI * 2.0 / speed).sin() * 0.5 + 0.5; // 0.0 to 1.0

                // Interpolate Color: Dim Yellow -> Bright White
                // Dim Yellow: (0.8, 0.8, 0.0, 0.8)
                // Bright White: (1.0, 1.0, 1.0, 1.0)

                let r = 0.8 + (0.2 * t);
                let g = 0.8 + (0.2 * t);
                let b = 0.0 + (1.0 * t); // 0 to 1 makes it white
                let a = 0.8 + (0.2 * t); // somewhat opaque

                let color = Color::new(r, g, b, a);

                // Draw Orb
                draw_circle(self.position.x, self.position.y, self.radius, color);

                // Animated gradient style? Maybe a core?
                draw_circle(self.position.x, self.position.y, self.radius * 0.6, WHITE);
            }
            PowerUpType::Sidekick => {
                // Magenta, dim to bright 3x per sec (3 Hz)
                // Center Blue Triangle
                let speed = 3.0; // 3 Hz
                let t = (self.timer * std::f32::consts::PI * 2.0 * speed).sin() * 0.5 + 0.5; // 0.0 to 1.0

                // Magenta: (1.0, 0.0, 1.0)
                // Dim: 0.5 brightness? Bright: 1.0
                let brightness = 0.5 + (0.5 * t);
                let color = Color::new(1.0 * brightness, 0.0, 1.0 * brightness, 1.0);

                // Draw Magenta Orb/Base
                draw_circle(self.position.x, self.position.y, self.radius, color);

                // Draw Blue Triangle in center
                // Triangle size: radius * 0.6?
                let tri_size = self.radius * 0.6;
                let center = self.position;

                // Upward pointing triangle
                let p1 = center + vec2(0.0, -tri_size);
                // Bottom left
                let p2 = center + vec2(-tri_size * 0.866, tri_size * 0.5);
                // Bottom right
                let p3 = center + vec2(tri_size * 0.866, tri_size * 0.5);

                draw_triangle(p1, p2, p3, BLUE);
            }
        }
    }
}
