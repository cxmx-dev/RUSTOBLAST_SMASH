// use crate::constants::*; // Unused now
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Projectile {
    #[serde(with = "crate::serde_helper")]
    pub position: Vec2,
    #[serde(with = "crate::serde_helper")]
    pub velocity: Vec2,
    pub active: bool,
    pub age: f32, // For animation
    pub design: crate::assets::ShapeDesign,
    pub is_sidekick: bool,
}

impl Projectile {
    pub fn new(pos: Vec2, is_sidekick: bool) -> Self {
        // Load asset manually (similar to Player)
        let design = crate::assets::load_shape_file("bullets.json");

        // Sidekick bullet: Yellow, Faster, custom shape override later?
        // Using same asset for now but we can colorize it differently.
        let velocity = if is_sidekick {
            vec2(0.0, -1200.0) // Very fast
        } else {
            vec2(0.0, -800.0)
        };

        Self {
            position: pos,
            velocity,
            active: true,
            age: 0.0,
            design,
            is_sidekick, // Helper field needed? Or just render color.
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        self.age += dt;

        if self.position.y < -50.0 {
            // Buffer for larger size
            self.active = false;
        }
    }

    pub fn draw(&self) {
        if self.active {
            // Pulsating Effect: Dim to Bright 1x every 0.5 sec (2 Hz)
            // Frequency = 2 Hz
            // sin(t * 2 * PI * 2)
            let cycle = (self.age * std::f32::consts::PI * 4.0).sin();
            let brightness = 0.3 + 0.7 * (cycle + 1.0) / 2.0;

            // Draw using Design Parts manually to apply color override
            use crate::assets::ShapePart;
            let scale = 1.0;
            let pos = self.position;

            for part in &self.design.parts {
                match part {
                    ShapePart::Line {
                        start,
                        end,
                        thickness,
                        color,
                        ..
                    } => {
                        let base_color = if self.is_sidekick {
                            Color::new(1.0, 1.0, 0.0, 1.0) // YELLOW override
                        } else {
                            let c = *color;
                            Color::new(c[0], c[1], c[2], c[3])
                        };

                        let final_color = Color::new(
                            base_color.r * brightness,
                            base_color.g * brightness,
                            base_color.b * brightness,
                            base_color.a,
                        );
                        let p1 = vec2(start[0], start[1]) * scale + pos;
                        let p2 = vec2(end[0], end[1]) * scale + pos;
                        draw_line(p1.x, p1.y, p2.x, p2.y, *thickness * scale, final_color);
                    }
                    ShapePart::Circle {
                        center,
                        radius,
                        color,
                        ..
                    } => {
                        let base_color = if self.is_sidekick {
                            Color::new(1.0, 1.0, 0.0, 1.0) // YELLOW override
                        } else {
                            let c = *color;
                            Color::new(c[0], c[1], c[2], c[3])
                        };

                        let final_color = Color::new(
                            base_color.r * brightness,
                            base_color.g * brightness,
                            base_color.b * brightness,
                            base_color.a,
                        );

                        let c = vec2(center[0], center[1]) * scale + pos;
                        draw_circle(c.x, c.y, *radius * scale, final_color);
                    }
                    _ => {}
                }
            }
        }
    }
}
