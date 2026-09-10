use crate::assets::{self, ShapeDesign};
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    #[serde(with = "crate::serde_helper")]
    pub position: Vec2,
    pub speed: f32,
    pub width: f32,
    pub height: f32,
    pub last_shot_time: f64,
    pub fire_rate: f64, // Seconds between shots
    pub design: ShapeDesign,
    #[serde(skip, default)]
    pub flash_design: ShapeDesign,
    #[serde(skip)]
    pub flash_timer: f32,
    #[serde(skip)]
    pub shields: i32,
    #[serde(skip)]
    pub max_shields: i32,
    #[serde(skip)]
    pub is_shielded: bool,
    #[serde(skip)]
    pub shield_active_timer: f32, // Duration of shield
    #[serde(skip)]
    pub invincible_timer: f32,
    #[serde(skip)]
    pub spinner_invincible_timer: f32, // Specific to Spinners (10s)
    #[serde(skip, default)]
    pub sidekicks: Vec<crate::scenes::entities::sidekick::Sidekick>,
}

impl Player {
    pub fn new() -> Self {
        let mut player = Self {
            position: vec2(screen_width() / 2.0, screen_height() - 150.0),
            speed: 500.0, // Pixels per second
            width: 120.0,
            height: 120.0,
            last_shot_time: 0.0,
            fire_rate: 0.1, // 10 shots per second (was 0.2)
            design: ShapeDesign::default(),
            flash_design: ShapeDesign::default(),
            flash_timer: 0.0,
            shields: 0,
            max_shields: 6,
            is_shielded: false,
            shield_active_timer: 0.0,
            invincible_timer: 3.0,
            spinner_invincible_timer: 10.0,
            sidekicks: Vec::new(),
        };
        player.load_assets();
        player
    }

    pub fn load_assets(&mut self) {
        self.design = assets::load_shape_file("player_ship.json");
        self.flash_design = assets::load_shape_file("ship_flash.json");
    }

    pub fn trigger_flash(&mut self) {
        self.flash_timer = 0.05; // 50ms flash duration
    }

    pub fn update(&mut self, dt: f32) {
        // Speed Boost
        let current_speed = if is_mouse_button_down(MouseButton::Right) {
            self.speed * 3.0
        } else {
            self.speed
        };

        // Movement
        if crate::input::is_move_left_down() {
            self.position.x -= current_speed * dt;
        }
        if crate::input::is_move_right_down() {
            self.position.x += current_speed * dt;
        }

        // Hyperspace
        if is_key_pressed(KeyCode::LeftShift) || is_key_pressed(KeyCode::Q) {
            self.position.x = rand::gen_range(self.width / 2.0, screen_width() - self.width / 2.0);
        }

        // Clamp to screen
        self.position.x = self
            .position
            .x
            .clamp(self.width / 2.0, screen_width() - self.width / 2.0);

        if self.flash_timer > 0.0 {
            self.flash_timer -= dt;
        }

        if self.invincible_timer > 0.0 {
            self.invincible_timer -= dt;
        }
        if self.spinner_invincible_timer > 0.0 {
            self.spinner_invincible_timer -= dt;
        }

        if self.is_shielded {
            self.shield_active_timer -= dt;
            if self.shield_active_timer <= 0.0 {
                self.is_shielded = false;
            }
        }
    }

    pub fn activate_shield(&mut self) {
        if self.shields > 0 && !self.is_shielded {
            self.shields -= 1;
            self.is_shielded = true;
            self.shield_active_timer = 5.0; // 5 seconds duration
        }
    }

    pub fn add_sidekick(&mut self) {
        if self.sidekicks.len() >= 6 {
            return;
        }

        // Logic: Alternating Left / Right
        // 0 -> Left 1
        // 1 -> Right 1
        // 2 -> Left 2
        // etc.
        let count = self.sidekicks.len();
        let is_left = count % 2 == 0;
        let pair_index = (count / 2) as f32; // 0, 1, 2

        // Spacing parameters
        let x_spacing = 60.0;

        let x_offset = if is_left {
            -(self.width / 2.0 + 40.0 + (pair_index * x_spacing))
        } else {
            self.width / 2.0 + 40.0 + (pair_index * x_spacing)
        };

        // Horizontal Line formation: same Y as player, but shifted down slightly to match visual baseline
        // Player center is 0.0 relative offset.
        let y_offset = 20.0;

        let offset = vec2(x_offset, y_offset);
        let mut sk = crate::scenes::entities::sidekick::Sidekick::new(offset);
        // Force update position immediately so it doesn't spawn at 0,0
        sk.update(self.position, 0.0);
        self.sidekicks.push(sk);
    }

    pub fn draw(&self) {
        // Invincibility Flicker (3Hz)
        // If invincible, only draw if phase is "on"
        let mut draw_ship = true;
        if self.invincible_timer > 0.0 {
            let time = get_time() as f32;
            // 3Hz = 3 times per second.
            // Sine wave > 0.0 is roughly 50% duty cycle.
            if (time * std::f32::consts::PI * 2.0 * 3.0).sin() < 0.0 {
                draw_ship = false;
            }
        }

        if draw_ship {
            assets::draw_design(&self.design, self.position, 0.0, 3.0, get_time() as f32);
        }

        if self.flash_timer > 0.0 {
            // Antenna tip offset relative to center at scale 3.0
            // Original tip: (0, -20.0) -> scaled (0, -60.0)
            let flash_pos = self.position + vec2(0.0, -60.0);
            assets::draw_design(&self.flash_design, flash_pos, 0.0, 3.0, get_time() as f32);
        }

        // Shield Visuals
        if self.is_shielded {
            // Dynamic Blink Logic
            // > 1.6s: Solid (b_draw = true)
            // <= 1.6s: Blinking with increasing frequency
            let b_draw = if self.shield_active_timer > 1.6 {
                true
            } else {
                // Timer goes from 1.6 -> 0.0
                // We want frequency to increase as timer decreases.
                // Try quadratic phase: sin(Constant * (1.6 - timer)^2)
                let time_remaining = 1.6 - self.shield_active_timer; // 0.0 -> 1.6
                // Coefficient 40.0 tuned for nice end speed (~20Hz final)
                (40.0 * time_remaining.powi(2)).sin() > 0.0
            };

            if b_draw {
                // "opaque animated cyan ... to turquoise animated gradient... top to bottom bubble"

                // Base Bubble
                let bubble_radius = self.width / 2.0 + 20.0;

                // We want a gradient effect. We can draw multiple circles or lines.
                // Or we can just pulsate color?
                // "Top to bottom bubble like structure" -> implies lines being drawn?
                // Let's draw horizontal lines clipping to a circle definition to simulate scanning bubbles.

                let num_lines = 20;
                let time = get_time() as f32; // Restore time variable
                for i in 0..num_lines {
                    let y_norm = i as f32 / num_lines as f32; // 0.0 to 1.0 from top to bottom

                    // Animation offset
                    let phase = (time * 5.0 + y_norm * 10.0).sin();
                    // Gradient: Cyan (0, 255, 255) -> Turquoise (64, 224, 208) -> White

                    // Mix based on phase
                    let color_val = (phase * 0.5 + 0.5).clamp(0.0, 1.0);

                    let r = 0.0 + color_val * 1.0;
                    let g = 1.0; // Cyan/Turquoise high G
                    let b = 1.0; // Cyan high B
                    let a = 0.4 + (phase * 0.2); // Opaque ish

                    let color = Color::new(r, g, b, a);

                    // Circle Width at this Y
                    let y_offset = (y_norm - 0.5) * 2.0 * bubble_radius;
                    let x_width_sq = bubble_radius * bubble_radius - y_offset * y_offset;
                    if x_width_sq > 0.0 {
                        let x_width = x_width_sq.sqrt();
                        let start = self.position + vec2(-x_width, y_offset);
                        let end = self.position + vec2(x_width, y_offset);
                        draw_line(start.x, start.y, end.x, end.y, 3.0, color);
                    }
                }

                // Outline
                draw_circle_lines(
                    self.position.x,
                    self.position.y,
                    bubble_radius,
                    2.0,
                    Color::new(0.0, 1.0, 1.0, 1.0),
                );
            }
        }
    }
}
