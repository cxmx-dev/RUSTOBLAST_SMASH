use crate::assets::{self, ShapeDesign};
use crate::constants::UFO_BOMB_COOLDOWN;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EnemyType {
    BigRock,
    SmallRock,
    BigSpinner,
    SmallSpinner,
    GuidedMissile,
    UFO,
    UFOBomb,
    PolySnake,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Enemy {
    #[serde(with = "crate::serde_helper")]
    pub position: Vec2,
    #[serde(with = "crate::serde_helper")]
    pub velocity: Vec2,
    pub enemy_type: EnemyType,
    pub radius: f32, // Simplified collision for now
    pub active: bool,
    pub rotation: f32,
    #[serde(skip)]
    pub rot_speed: f32,
    pub design: ShapeDesign,
    #[serde(skip, default)]
    pub bomb_cooldown: f32,
    #[serde(skip, default)]
    pub on_ground: bool,
    #[serde(skip, default)]
    pub just_landed: bool,
}

/// Shared playfield floor (same Y as the player). Guided missiles stay here;
/// rocks/spinners/bombs/poly set `just_landed` the frame they reach it.
pub fn playfield_ground_y() -> f32 {
    screen_height() - 150.0
}

impl Enemy {
    fn load_design(filename: &str) -> ShapeDesign {
        assets::load_shape_file(filename)
    }

    pub fn new_big_rock(pos: Vec2) -> Self {
        Self {
            position: pos,
            velocity: vec2(0.0, rand::gen_range(100.0, 200.0)), // Slow falling
            enemy_type: EnemyType::BigRock,
            radius: 90.0,
            active: true,
            rotation: rand::gen_range(0.0, 360.0),
            rot_speed: rand::gen_range(30.0, 90.0)
                * if rand::gen_range(0, 2) == 0 {
                    1.0
                } else {
                    -1.0
                },
            design: Self::load_design("big_ass.json"),
            bomb_cooldown: 0.0,
            on_ground: false,
            just_landed: false,
        }
    }

    pub fn new_small_rock(pos: Vec2, velocity: Vec2) -> Self {
        Self {
            position: pos,
            velocity,
            enemy_type: EnemyType::SmallRock,
            radius: 45.0,
            active: true,
            rotation: rand::gen_range(0.0, 360.0),
            rot_speed: rand::gen_range(60.0, 180.0)
                * if rand::gen_range(0, 2) == 0 {
                    1.0
                } else {
                    -1.0
                },
            design: Self::load_design("small_ass.json"),
            bomb_cooldown: 0.0,
            on_ground: false,
            just_landed: false,
        }
    }

    pub fn new_big_spinner(pos: Vec2) -> Self {
        Self {
            position: pos,
            velocity: vec2(0.0, rand::gen_range(150.0, 250.0)), // Slower than small spinner
            enemy_type: EnemyType::BigSpinner,
            radius: 240.0, // Tripled size (was 80)
            active: true,
            rotation: 0.0,
            rot_speed: 120.0,
            design: Self::load_design("big_spinner.json"),
            bomb_cooldown: 0.0,
            on_ground: false,
            just_landed: false,
        }
    }

    pub fn new_small_spinner(pos: Vec2) -> Self {
        Self {
            position: pos,
            velocity: vec2(0.0, rand::gen_range(300.0, 500.0)), // Very Fast
            enemy_type: EnemyType::SmallSpinner,
            radius: 80.0, // Matches old Big Spinner size (was 40)
            active: true,
            rotation: 0.0,
            rot_speed: -360.0, // Fast spin
            design: Self::load_design("small_spinner.json"),
            bomb_cooldown: 0.0,
            on_ground: false,
            just_landed: false,
        }
    }

    pub fn new_ufo(pos: Vec2) -> Self {
        Self {
            position: pos,
            velocity: vec2(100.0, 0.0), // Moves right by default
            enemy_type: EnemyType::UFO,
            radius: 50.0,
            active: true,
            rotation: 0.0,
            rot_speed: 0.0,
            design: Self::load_design("ufo.json"),
            bomb_cooldown: UFO_BOMB_COOLDOWN,
            on_ground: false,
            just_landed: false,
        }
    }

    pub fn new_ufo_bomb(pos: Vec2) -> Self {
        Self {
            position: pos,
            velocity: vec2(0.0, 150.0), // Falls slowly
            enemy_type: EnemyType::UFOBomb,
            radius: 15.0,
            active: true,
            rotation: 0.0,
            rot_speed: 100.0, // Rotates
            design: Self::load_design("ufo_bomb.json"),
            bomb_cooldown: 0.0,
            on_ground: false,
            just_landed: false,
        }
    }

    pub fn new_guided_missile(pos: Vec2) -> Self {
        Self {
            position: pos,
            velocity: vec2(0.0, 150.0), // Base speed
            enemy_type: EnemyType::GuidedMissile,
            radius: 40.0,
            active: true,
            rotation: 0.0,
            rot_speed: 5.0, // Turning speed
            design: Self::load_design("guided_missile.json"),
            bomb_cooldown: 0.0,
            on_ground: false,
            just_landed: false,
        }
    }

    pub fn new_poly_snake(pos: Vec2) -> Self {
        Self {
            position: pos,
            velocity: vec2(300.0, 0.0), // Move Right initially
            enemy_type: EnemyType::PolySnake,
            radius: 40.0,
            active: true,
            rotation: 0.0,
            rot_speed: 0.0, // Used as "Target Y" state storage
            design: Self::load_design("poly.json"),
            bomb_cooldown: 0.0,
            on_ground: false,
            just_landed: false,
        }
    }

    fn lands_on_playfield(&self) -> bool {
        matches!(
            self.enemy_type,
            EnemyType::BigRock
                | EnemyType::SmallRock
                | EnemyType::BigSpinner
                | EnemyType::SmallSpinner
                | EnemyType::UFOBomb
                | EnemyType::PolySnake
        )
    }

    pub fn update(&mut self, dt: f32, player_pos: Vec2) {
        self.just_landed = false;

        if let EnemyType::GuidedMissile = self.enemy_type {
            let ground_y = playfield_ground_y();

            if self.position.y < ground_y {
                // Falling phase
                self.velocity.x = 0.0;
                self.velocity.y = 400.0; // Fast fall
            } else {
                self.on_ground = true;
                self.position.y = ground_y;
                self.velocity.y = 0.0;

                if self.velocity.x == 0.0 {
                    let dx = player_pos.x - self.position.x;
                    self.velocity.x = dx.signum() * 200.0; // Fast tracking on ground
                    // Fallback if exactly on top (unlikely but safe)
                    if self.velocity.x == 0.0 {
                        self.velocity.x = 200.0;
                    }
                }

                // Point towards movement direction
                // "rotate horizontally left or right toward the player"
                // 0 degrees is UP (usually).
                // Rotation is CCW: 90 is Left, -90 is Right.
                // Smooth Interaction: Rotate towards target angle
                // Target: -90 for Right, 90 for Left.
                let target_rotation = if self.velocity.x > 0.0 { -90.0 } else { 90.0 };

                // Interpolate (Move towards target)
                // Speed: 300 degrees per second
                let rot_speed = 300.0 * dt;

                if self.rotation < target_rotation {
                    self.rotation += rot_speed;
                    if self.rotation > target_rotation {
                        self.rotation = target_rotation;
                    }
                } else if self.rotation > target_rotation {
                    self.rotation -= rot_speed;
                    if self.rotation < target_rotation {
                        self.rotation = target_rotation;
                    }
                }
            }
        } else if let EnemyType::UFO = self.enemy_type {
            // UFO Logic: Horizontal movement, bounce or wrap?
            // "Traverse screen" usually means enter one side leave other.
            // But for simple implementation let's bounce for now to stay threat, or wrap?
            // Actually, research said "traverse". Let's bounce to keep it visible.
            if self.position.x < 50.0 || self.position.x > screen_width() - 50.0 {
                self.velocity.x *= -1.0;
            }
        } else if let EnemyType::UFOBomb = self.enemy_type {
            // Just fall
        } else if let EnemyType::PolySnake = self.enemy_type {
            // "Snake" Logic: Right -> Down -> Left -> Down -> Repeat

            // Check if we are moving Vertically (Down)
            if self.velocity.y > 0.0 {
                // Vertical Phase
                // Check if we reached target Y (stored in rot_speed)
                if self.position.y >= self.rot_speed {
                    // Reached target
                    self.position.y = self.rot_speed; // Snap
                    self.velocity.y = 0.0;
                    self.rot_speed = 0.0; // Clear state

                    // Decide next horizontal direction
                    // If we are on the Right side (approx > width/2), go Left.
                    // If we are on the Left side, go Right.
                    if self.position.x > screen_width() / 2.0 {
                        self.velocity.x = -300.0;
                    } else {
                        self.velocity.x = 300.0;
                    }
                }
            } else {
                // Horizontal Phase
                let margin = 50.0;
                let right_bound = screen_width() - margin;
                let left_bound = margin;

                if self.velocity.x > 0.0 {
                    // Moving Right
                    if self.position.x >= right_bound {
                        self.position.x = right_bound; // Snap
                        self.velocity.x = 0.0;
                        self.velocity.y = 300.0; // Start moving Down
                        self.rot_speed = self.position.y + 100.0; // Set Target Y (Drop 100px)
                    }
                } else if self.velocity.x < 0.0 {
                    // Moving Left
                    if self.position.x <= left_bound {
                        self.position.x = left_bound; // Snap
                        self.velocity.x = 0.0;
                        self.velocity.y = 300.0; // Start moving Down
                        self.rot_speed = self.position.y + 100.0; // Set Target Y
                    }
                }
            }
            // Spin based on horizontal direction
            let spin_speed = 360.0 * dt; // 1 full rotation per second
            if self.velocity.x > 0.0 {
                self.rotation += spin_speed; // Clockwise
            } else {
                self.rotation -= spin_speed; // Counter-Clockwise
            }
        }

        self.position += self.velocity * dt;
        if !matches!(self.enemy_type, EnemyType::PolySnake) {
            self.rotation += self.rot_speed * dt;
        }

        if self.lands_on_playfield() && !self.on_ground && self.position.y >= playfield_ground_y() {
            self.position.y = playfield_ground_y();
            self.velocity = vec2(0.0, 0.0);
            self.on_ground = true;
            self.just_landed = true;
            self.active = false;
        }
    }

    pub fn draw(&self) {
        // Use Asset Engine logic
        // Scale: 3.0

        // Special case for Guided Missile pulsing REMOVED to restore full asset rendering
        // Falling back to standard assets::draw_design below

        let scale = match self.enemy_type {
            EnemyType::BigSpinner => 9.0, // 3x standard scale (200% larger)
            _ => 3.0,
        };

        assets::draw_design(
            &self.design,
            self.position,
            self.rotation,
            scale,
            get_time() as f32,
        );
    }
}
