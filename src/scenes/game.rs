use super::entities::{
    enemy::{Enemy, EnemyType},
    player::Player,
    powerup::{PowerUp, PowerUpType},
    projectile::Projectile,
};
use crate::assets::{self, ShapeDesign, ShapePart};
use crate::constants::*;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

fn default_volume() -> f32 {
    1.0
}

pub enum GameAction {
    None,
    Pause,
    Options,
    GameOver,
}

#[derive(Serialize, Deserialize, Clone)]
struct Star {
    // ... existing struct Star ...
    #[serde(with = "crate::serde_helper")]
    pos: Vec2,
    speed: f32,
    size: f32,
    phase: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Starfield {
    stars: Vec<Star>,
    #[serde(skip)]
    height_map: Vec<f32>, // Optimization: Cached max Y for each X bucket
    #[serde(skip)]
    map_min_x: f32,
    #[serde(skip)]
    map_step: f32,
}

impl Starfield {
    pub fn new() -> Self {
        let mut stars = Vec::new();
        for _ in 0..1500 {
            // Increased density further for larger area
            stars.push(Self::generate_star());
        }
        Self {
            stars,
            height_map: Vec::new(),
            map_min_x: -4000.0,
            map_step: 10.0,
        }
    }

    fn generate_star() -> Star {
        // Generate across wide X range for scrolling
        let x = rand::gen_range(-2500.0, 2500.0);
        // Generate Y from "lowest valley" to "top of sky"
        // Mountains go down to ~ -366 (scaled).
        // We need to generate lower than -150.0. Let's use -450.0.
        let y = rand::gen_range(-450.0, 2500.0);

        Star {
            pos: vec2(x, y),
            speed: rand::gen_range(20.0, 100.0), // Horizontal speed
            size: rand::gen_range(1.0, 2.5),
            phase: rand::gen_range(0.0, std::f32::consts::PI * 2.0),
        }
    }

    pub fn update_profile(&mut self, mountain_design: &ShapeDesign) {
        let scale = 3.0f32;
        let min_x = -4000.0f32;
        let max_x = 4000.0f32;
        let step = 10.0f32;
        let buckets = ((max_x - min_x) / step).ceil() as usize + 1;

        // Initialize map with a very low value
        self.height_map = vec![-9999.0; buckets];
        self.map_min_x = min_x;
        self.map_step = step;

        // Extract segments once
        let mut segments = Vec::new(); // (p1, p2)
        for part in &mountain_design.parts {
            if let ShapePart::Line { start, end, .. } = part {
                let p1 = vec2(start[0] * scale, start[1] * scale);
                let p2 = vec2(end[0] * scale, end[1] * scale);
                segments.push((p1, p2));
            }
        }

        // Rasterize segments into height map
        // For each bucket x, find max Y intersection
        for i in 0..buckets {
            let x = min_x + i as f32 * step;
            let mut max_y = -9999.0;

            for (p1, p2) in &segments {
                let (seg_min, seg_max) = if p1.x < p2.x {
                    (p1.x, p2.x)
                } else {
                    (p2.x, p1.x)
                };
                if x >= seg_min && x <= seg_max {
                    // Interpolate
                    let t = if (p2.x - p1.x).abs() < 0.001 {
                        0.0
                    } else {
                        (x - p1.x) / (p2.x - p1.x)
                    };
                    let y = p1.y + t * (p2.y - p1.y);
                    if y > max_y {
                        max_y = y;
                    }
                }
            }
            self.height_map[i] = max_y;
        }
    }

    pub fn update(&mut self, dt: f32, total_score: i32) {
        // Direction changes every 10,000 points
        // 0-9999: Right (1.0)
        // 10000-19999: Left (-1.0)
        // etc.
        let direction = if (total_score / 10000) % 2 == 0 {
            1.0
        } else {
            -1.0
        };

        for star in &mut self.stars {
            // Horizontal movement
            star.pos.x += star.speed * direction * dt * 2.0; // *2.0 for visible motion
            star.phase += dt * 2.0;

            // Wrap around X
            if star.pos.x > 2500.0 {
                star.pos.x = -2500.0;
                // Randomize Y slightly on wrap to avoid patterns
                star.pos.y = rand::gen_range(-450.0, 2500.0);
            } else if star.pos.x < -2500.0 {
                star.pos.x = 2500.0;
                star.pos.y = rand::gen_range(-450.0, 2500.0);
            }
        }
    }

    pub fn draw(&self) {
        if self.height_map.is_empty() {
            return;
        }

        for star in &self.stars {
            // Fast lookup
            let bucket = ((star.pos.x - self.map_min_x) / self.map_step).floor() as isize;
            let mut is_visible = true;

            if bucket >= 0 && (bucket as usize) < self.height_map.len() {
                let mountain_y = self.height_map[bucket as usize];
                if mountain_y > -9000.0 && star.pos.y < mountain_y {
                    is_visible = false;
                }
            }
            // If outside range (which shouldn't happen much given +/-2500 vs +/-4000), default visible

            if is_visible {
                let alpha = 0.5 + 0.5 * star.phase.sin().abs();
                let color = Color::new(1.0, 1.0, 1.0, alpha);
                draw_cube(
                    vec3(star.pos.x, star.pos.y, -10.0),
                    vec3(star.size, star.size, 0.1),
                    None,
                    color,
                );
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Debris {
    #[serde(with = "crate::serde_helper")]
    pub position: Vec2,
    #[serde(with = "crate::serde_helper")]
    pub velocity: Vec2,
    pub rotation: f32,
    pub rot_speed: f32,
    pub part: ShapePart,
    pub lifetime: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub score: i32,
    pub lives: i32,
    pub player: Player,
    pub projectiles: Vec<Projectile>,
    pub enemies: Vec<Enemy>,
    #[serde(skip, default)]
    pub powerups: Vec<PowerUp>,
    #[serde(skip, default)]
    pub debris: Vec<Debris>,
    #[serde(default)]
    pub player_dead: bool,
    #[serde(default)]
    pub dead_timer: f32,
    #[serde(skip, default)]
    pub boom_design: ShapeDesign,
    pub spawn_timer: f64,
    #[serde(default = "default_volume")]
    pub sfx_volume: f32,
    #[serde(default = "default_volume")]
    pub music_volume: f32,
    #[serde(default = "default_volume")]
    pub master_volume: f32,
    #[serde(skip)]
    pub shoot_sound: Option<macroquad::audio::Sound>, // Sound handle
    #[serde(skip)]
    pub shoot_laser_sound: Option<macroquad::audio::Sound>,
    #[serde(skip)]
    pub ufo_sound: Option<macroquad::audio::Sound>, // Sound handle
    #[serde(skip)]
    pub guided_missile_sound: Option<macroquad::audio::Sound>, // Sound handle
    #[serde(skip)]
    pub big_spinner_sound: Option<macroquad::audio::Sound>, // Sound handle
    #[serde(skip)]
    pub small_spinner_sound: Option<macroquad::audio::Sound>, // Sound handle
    #[serde(skip)]
    pub multiplier_sound: Option<macroquad::audio::Sound>, // Sound handle
    #[serde(skip)]
    pub rock_explode_sound: Option<macroquad::audio::Sound>, // Sound handle
    #[serde(skip)]
    pub spinner_explode_sound: Option<macroquad::audio::Sound>, // Sound handle
    #[serde(skip)]
    pub big_spinner_explode_sound: Option<macroquad::audio::Sound>, // Sound handle
    #[serde(skip)]
    pub guided_missile_ground_sound: Option<macroquad::audio::Sound>, // Sound handle
    #[serde(skip)]
    pub load_notification_timer: f32,
    #[serde(skip)]
    pub background: Option<ShapeDesign>,
    #[serde(skip)]
    pub ground: Option<ShapeDesign>,
    #[serde(skip)]
    pub starfield: Option<Starfield>,
    #[serde(skip)]
    pub mod_poly_wave_remaining: i32,
    #[serde(skip)]
    pub poly_wave_timer: f64,
    #[serde(skip)]
    pub player_die_sound: Option<macroquad::audio::Sound>, // Sound handle
    #[serde(skip)]
    pub ufo_sound_active: bool,
    #[serde(skip)]
    pub missile_sound_active: bool,
    #[serde(skip)]
    pub big_spinner_sound_active: bool,
    #[serde(skip)]
    pub small_spinner_sound_active: bool,
}

impl GameState {
    pub fn new(
        shoot_sound: Option<macroquad::audio::Sound>,
        shoot_laser_sound: Option<macroquad::audio::Sound>,
        ufo_sound: Option<macroquad::audio::Sound>,
        guided_missile_sound: Option<macroquad::audio::Sound>,
        guided_missile_ground_sound: Option<macroquad::audio::Sound>,
        big_spinner_sound: Option<macroquad::audio::Sound>,
        small_spinner_sound: Option<macroquad::audio::Sound>,
        multiplier_sound: Option<macroquad::audio::Sound>,
        rock_explode_sound: Option<macroquad::audio::Sound>,
        spinner_explode_sound: Option<macroquad::audio::Sound>,
        big_spinner_explode_sound: Option<macroquad::audio::Sound>,
        player_die_sound: Option<macroquad::audio::Sound>,
        master_volume: f32,
        sfx_volume: f32,
        music_volume: f32,
    ) -> Self {
        let background = assets::load_shape_path("Macroquad-Serde Asset Engine/Assets/mountains.json");
        let ground = assets::load_shape_path("Macroquad-Serde Asset Engine/Assets/ground.json");
        let boom_design = assets::load_shape_file("player_boom.json");

        let mut starfield = Starfield::new();
        // Initialize profile
        if let Some(bg) = &background {
            starfield.update_profile(bg);
        }

        let start_score = std::env::var("RUSTOBLAST_START_SCORE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        Self {
            score: start_score,
            lives: 10,
            player: Player::new(),
            projectiles: Vec::new(),
            enemies: Vec::new(),
            powerups: Vec::new(),
            spawn_timer: 0.0,
            load_notification_timer: 0.0,
            background,
            ground,
            starfield: Some(starfield),
            shoot_sound,
            shoot_laser_sound,
            ufo_sound,
            guided_missile_sound,
            guided_missile_ground_sound,
            big_spinner_sound,
            small_spinner_sound,
            multiplier_sound,
            rock_explode_sound,
            spinner_explode_sound,
            big_spinner_explode_sound,
            player_die_sound,
            master_volume,
            sfx_volume,
            music_volume,
            mod_poly_wave_remaining: 0,
            poly_wave_timer: 0.0,
            debris: Vec::new(),
            player_dead: false,
            dead_timer: 0.0,
            boom_design,
            ufo_sound_active: false,
            missile_sound_active: false,
            big_spinner_sound_active: false,
            small_spinner_sound_active: false,
        }
    }

    pub fn save(&mut self) {
        match serde_json::to_string(self) {
            Ok(serialized) => {
                let path = "SAVED GAMES - HIGHSCORES/savegame.json";
                if let Some(parent) = std::path::Path::new(path).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }

                match File::create(path) {
                    Ok(mut file) => {
                        if let Err(e) = file.write_all(serialized.as_bytes()) {
                            println!("Failed to write save file: {}", e);
                        } else {
                            println!("Game Saved! Score: {}, Lives: {}", self.score, self.lives);
                        }
                    }
                    Err(e) => println!("Failed to create save file: {}", e),
                }
            }
            Err(e) => println!("Failed to serialize game state: {}", e),
        }
    }

    pub fn load(
        shoot_sound: Option<macroquad::audio::Sound>,
        shoot_laser_sound: Option<macroquad::audio::Sound>,
        ufo_sound: Option<macroquad::audio::Sound>,
        guided_missile_sound: Option<macroquad::audio::Sound>,
        guided_missile_ground_sound: Option<macroquad::audio::Sound>,
        big_spinner_sound: Option<macroquad::audio::Sound>,
        small_spinner_sound: Option<macroquad::audio::Sound>,
        multiplier_sound: Option<macroquad::audio::Sound>,
        rock_explode_sound: Option<macroquad::audio::Sound>,
        spinner_explode_sound: Option<macroquad::audio::Sound>,
        big_spinner_explode_sound: Option<macroquad::audio::Sound>,
        player_die_sound: Option<macroquad::audio::Sound>,
        master_volume: f32,
        sfx_volume: f32,
        music_volume: f32,
    ) -> Option<Self> {
        match File::open("SAVED GAMES - HIGHSCORES/savegame.json") {
            Ok(mut file) => {
                let mut contents = String::new();
                if let Err(e) = file.read_to_string(&mut contents) {
                    println!("Failed to read save file: {}", e);
                    return None;
                }
                match serde_json::from_str::<GameState>(&contents) {
                    Ok(mut state) => {
                        println!(
                            "Game Loaded! Score: {}, Lives: {}",
                            state.score, state.lives
                        );

                        state.background = assets::load_shape_path(
                            "Macroquad-Serde Asset Engine/Assets/mountains.json",
                        );
                        state.ground = assets::load_shape_path(
                            "Macroquad-Serde Asset Engine/Assets/ground.json",
                        );

                        let mut starfield = Starfield::new();
                        if let Some(bg) = &state.background {
                            starfield.update_profile(bg);
                        }
                        state.starfield = Some(starfield);

                        state.shoot_sound = shoot_sound; // Inject sound
                        state.shoot_laser_sound = shoot_laser_sound;
                        state.ufo_sound = ufo_sound;
                        state.guided_missile_sound = guided_missile_sound;
                        state.guided_missile_ground_sound = guided_missile_ground_sound;
                        state.big_spinner_sound = big_spinner_sound;
                        state.small_spinner_sound = small_spinner_sound;

                        state.multiplier_sound = multiplier_sound;
                        state.rock_explode_sound = rock_explode_sound;
                        state.spinner_explode_sound = spinner_explode_sound;
                        state.big_spinner_explode_sound = big_spinner_explode_sound;
                        state.player_die_sound = player_die_sound;

                        state.master_volume = master_volume; // Overwrite/Inject volume
                        state.sfx_volume = sfx_volume; // Overwrite/Inject volume
                        state.music_volume = music_volume; // Overwrite/Inject volume
                        state.load_notification_timer = 2.0;
                        state.mod_poly_wave_remaining = 0; // Reset on load
                        state.poly_wave_timer = 0.0;

                        // Reload non-serialized assets for player
                        state.player.load_assets();

                        // Reload boom asset
                        state.boom_design = assets::load_shape_file("player_boom.json");
                        state.debris = Vec::new();
                        state.player_dead = false; // Reset death state on load to avoid softlock if saved while dead?
                        state.dead_timer = 0.0;

                        Some(state)
                    }
                    Err(e) => {
                        println!("Failed to deserialize save file: {}", e);
                        None
                    }
                }
            }
            Err(e) => {
                println!("Failed to open save file: {}", e);
                None
            }
        }
    }

    pub fn get_multiplier_and_bg(&self) -> (i32, Color) {
        if self.score > 100_000 {
            (6, BG_COLOR_6X)
        } else if self.score > 50_000 {
            (5, BG_COLOR_5X)
            // ... truncated (no change) ...
        } else if self.score > 20_000 {
            (4, BG_COLOR_4X)
        } else if self.score > 5_000 {
            (3, BG_COLOR_3X)
        } else if self.score > 1_000 {
            (2, BG_COLOR_2X)
        } else {
            (1, BG_COLOR_1X)
        }
    }

    pub fn spawn_debris(&mut self, pos: Vec2) {
        // Player ship parts? Or general debris?
        // Using player design
        let design = &self.player.design;

        for part in &design.parts {
            // Create a debris piece for each line/circle
            // Random velocity
            let vel = vec2(
                rand::gen_range(-200.0, 200.0),
                rand::gen_range(-200.0, 200.0),
            );
            // Slight rotation
            let rot_speed = rand::gen_range(-5.0, 5.0);

            self.debris.push(Debris {
                position: pos,
                velocity: vel,
                rotation: 0.0,
                rot_speed,
                part: part.clone(),
                lifetime: 2.0,
            });
        }
    }

    pub fn update(&mut self) -> GameAction {
        if is_key_pressed(KeyCode::P) {
            return GameAction::Pause;
        }
        if is_key_pressed(KeyCode::Escape) {
            return GameAction::Options;
        }

        let dt = get_frame_time();

        let (prev_mult, _) = self.get_multiplier_and_bg();

        // Death State Update
        if self.player_dead {
            self.dead_timer -= dt;

            // Update Debris
            for d in &mut self.debris {
                d.position += d.velocity * dt;
                d.rotation += d.rot_speed * dt;
                d.lifetime -= dt;
            }
            self.debris.retain(|d| d.lifetime > 0.0);

            // Respawn check
            if self.dead_timer <= 0.0 {
                self.player_dead = false;
                self.debris.clear();
                self.player.position = vec2(screen_width() / 2.0, screen_height() - 150.0);
                self.player.invincible_timer = 3.0; // 3 Seconds of Invincibility

                // Restore lives logic was handled BEFORE entering state
                if self.lives <= 0 {
                    return GameAction::GameOver;
                }
            }

            // Still run enemy/projectile logic in background?
            // Request says "keep enemies as they are... do not reset".
            // Implies they should continue moving or just pause?
            // "keep enemies as they are" usually means persistence.
            // "fly away in all directions... screen flash... until player respawns".
            // I will continue running their updates so they remain dynamic threats.

            // ... Continue to enemies update below ...
        } else {
            // Only update player if alive
            if self.load_notification_timer > 0.0 {
                self.load_notification_timer -= dt;
            }
            self.player.update(dt);

            // Fire logic only if alive
            let current_time = get_time();
            let is_firing = is_key_down(KeyCode::Space) || is_mouse_button_down(MouseButton::Left);

            if is_firing && current_time - self.player.last_shot_time > self.player.fire_rate {
                let spawn_pos = self.player.position + vec2(0.0, -50.0);
                self.projectiles.push(Projectile::new(spawn_pos, false)); // false = player bullet
                self.player.trigger_flash();

                if let Some(sound) = &self.shoot_sound {
                    let params = macroquad::audio::PlaySoundParams {
                        looped: false,
                        volume: self.sfx_volume * self.master_volume,
                    };
                    macroquad::audio::play_sound(sound, params);
                }

                self.player.last_shot_time = current_time;
            }

            // Update Sidekicks & Collect Shots
            let mut sidekick_shots: Vec<Vec2> = Vec::new();
            for sk in &mut self.player.sidekicks {
                if !sk.active {
                    continue;
                }
                sk.update(self.player.position, dt);

                // Sidekick Shooting (Auto-fire)
                if current_time - sk.last_shot_time > sk.fire_rate {
                    let spawn_pos = sk.position + vec2(0.0, -20.0);
                    sidekick_shots.push(spawn_pos);
                    sk.last_shot_time = current_time;
                }
            }

            // Spawn collected shots
            for pos in sidekick_shots {
                self.projectiles.push(Projectile::new(pos, true)); // true = sidekick bullet
            }

            if !is_firing {
                if let Some(sound) = &self.shoot_sound {
                    macroquad::audio::stop_sound(sound);
                }
            }
        } // End if !player_dead

        // Update Starfield
        if let Some(stars) = &mut self.starfield {
            stars.update(dt, self.score);
        }

        // Common Updates (Enemies, Projectiles)

        self.spawn_timer += dt as f64;
        let (multiplier, _) = self.get_multiplier_and_bg();

        // Audio logic moved to main loop
        let spawn_threshold = crate::spawn::spawn_interval(multiplier);

        if self.spawn_timer > spawn_threshold {
            self.spawn_timer = 0.0;

            if self.mod_poly_wave_remaining > 0 {
                self.enemies.push(Enemy::new_poly_snake(vec2(50.0, 50.0)));
                self.mod_poly_wave_remaining -= 1;
                self.spawn_timer = spawn_threshold - 0.2;
            } else {
                match crate::spawn::roll_spawn(
                    self.score,
                    rand::gen_range(0.0, 1.0),
                    rand::gen_range(0.0, 1.0),
                ) {
                    crate::spawn::SpawnKind::PolyWave => {
                        self.mod_poly_wave_remaining = POLY_WAVE_COUNT;
                    }
                    crate::spawn::SpawnKind::Ufo => {
                        let spawn_x = rand::gen_range(50.0, screen_width() - 50.0);
                        self.enemies.push(Enemy::new_ufo(vec2(spawn_x, 50.0)));
                    }
                    kind => {
                        let spawn_x = rand::gen_range(50.0, screen_width() - 50.0);
                        let pos = vec2(spawn_x, -50.0);
                        match kind {
                            crate::spawn::SpawnKind::GuidedMissile => {
                                self.enemies.push(Enemy::new_guided_missile(pos));
                            }
                            crate::spawn::SpawnKind::SmallSpinner => {
                                self.enemies.push(Enemy::new_small_spinner(pos));
                            }
                            crate::spawn::SpawnKind::BigSpinner => {
                                self.enemies.push(Enemy::new_big_spinner(pos));
                            }
                            _ => {
                                self.enemies.push(Enemy::new_big_rock(pos));
                            }
                        }
                    }
                }
            }
        }

        for p in &mut self.projectiles {
            p.update(dt);
        }

        let mut spinner_landed = false;
        let mut spawned_enemies = Vec::new();

        let mut landed_penalties = 0;

        for e in &mut self.enemies {
            e.update(dt, self.player.position);

            if let EnemyType::UFO = e.enemy_type {
                e.bomb_cooldown -= dt;
                if e.bomb_cooldown <= 0.0 {
                    spawned_enemies.push(Enemy::new_ufo_bomb(e.position));
                    e.bomb_cooldown = UFO_BOMB_COOLDOWN;
                }
            }

            if e.just_landed {
                match e.enemy_type {
                    EnemyType::BigSpinner | EnemyType::SmallSpinner | EnemyType::PolySnake => {
                        spinner_landed = true;
                    }
                    EnemyType::BigRock => {
                        landed_penalties += 5 * multiplier;
                    }
                    EnemyType::SmallRock => {
                        landed_penalties += 10 * multiplier;
                    }
                    _ => {}
                }
            }
        }

        self.score = (self.score - landed_penalties).max(0);
        self.enemies.extend(spawned_enemies);

        if spinner_landed {
            // Check Shield
            if self.player.is_shielded {
                // Shield saves you from death by landing
                // Do nothing.
            } else if self.lives > 0 {
                self.lives -= 1;
                // self.enemies.clear(); // Keep enemies
                self.projectiles.clear();

                // Trigger Death State
                self.player_dead = true;
                self.dead_timer = 2.0;
                self.spawn_debris(self.player.position);

                // Trigger Death State (If lives == 0 check was here? No this is existing logic)
                // Wait, logic is: Hit -> checks -> lives -= 1 -> debris.
                // Or "Trigger Death State" block around 920.
                // Let's check view of 760.
                self.stop_all_sounds();
                if let Some(sound) = &self.player_die_sound {
                    let params = macroquad::audio::PlaySoundParams {
                        looped: false,
                        volume: self.sfx_volume * self.master_volume * 1.6,
                    };
                    macroquad::audio::play_sound(sound, params);
                }
                // self.player.position = ... (Handled on respawn)
            } else {
                return GameAction::GameOver;
            }
        }

        let mut new_enemies = Vec::new();
        let mut score_add = 0;

        for e in &mut self.enemies {
            if !e.active {
                continue;
            }
            for p in &mut self.projectiles {
                if !p.active {
                    continue;
                }

                let dist = e.position.distance(p.position);
                if dist < e.radius + 5.0 {
                    p.active = false;
                    e.active = false;

                    // Powerup Drop (12.5% chance)
                    if rand::gen_range(0.0, 1.0) < 0.125 {
                        let p_type = if rand::gen_range(0, 2) == 0 {
                            PowerUpType::Shield
                        } else {
                            PowerUpType::Sidekick
                        };
                        self.powerups.push(PowerUp::new(e.position, p_type));
                    }

                    match e.enemy_type {
                        EnemyType::BigRock => {
                            if let Some(sound) = &self.rock_explode_sound {
                                let params = macroquad::audio::PlaySoundParams {
                                    looped: false,
                                    volume: self.sfx_volume * self.master_volume * 1.6,
                                };
                                macroquad::audio::play_sound(sound, params);
                            }
                            score_add += 10 * multiplier;
                            new_enemies.push(Enemy::new_small_rock(e.position, vec2(-50.0, 150.0)));
                            new_enemies.push(Enemy::new_small_rock(e.position, vec2(50.0, 150.0)));
                        }
                        EnemyType::SmallRock => {
                            if let Some(sound) = &self.rock_explode_sound {
                                let params = macroquad::audio::PlaySoundParams {
                                    looped: false,
                                    volume: self.sfx_volume * self.master_volume * 1.6,
                                };
                                macroquad::audio::play_sound(sound, params);
                            }
                            score_add += 20 * multiplier
                        }
                        EnemyType::BigSpinner => {
                            if let Some(sound) = &self.big_spinner_explode_sound {
                                let params = macroquad::audio::PlaySoundParams {
                                    looped: false,
                                    volume: self.sfx_volume * self.master_volume * 0.5,
                                };
                                macroquad::audio::play_sound(sound, params);
                            }
                            score_add += 40 * multiplier
                        }
                        EnemyType::SmallSpinner => {
                            if let Some(sound) = &self.spinner_explode_sound {
                                let params = macroquad::audio::PlaySoundParams {
                                    looped: false,
                                    volume: self.sfx_volume * self.master_volume * 0.5,
                                };
                                macroquad::audio::play_sound(sound, params);
                            }
                            score_add += 80 * multiplier
                        }
                        EnemyType::GuidedMissile => score_add += 50 * multiplier,
                        EnemyType::UFO => score_add += 100 * multiplier,
                        EnemyType::UFOBomb => score_add += 10 * multiplier,
                        EnemyType::PolySnake => score_add += 150 * multiplier,
                    }
                }
            }
        }
        self.score = (self.score + score_add).max(0);
        self.enemies.extend(new_enemies);

        let mut player_hit = false;
        let mut sidekick_explosion_positions = Vec::new();
        let mut wipe_sidekicks = false;

        if !self.player_dead {
            for e in &self.enemies {
                if !e.active {
                    continue;
                }
                let dist = self.player.position.distance(e.position);
                if dist < (self.player.width / 3.0) + e.radius {
                    // Check spinner invincibility
                    let is_spinner = matches!(
                        e.enemy_type,
                        EnemyType::BigSpinner | EnemyType::SmallSpinner
                    );
                    if is_spinner && self.player.spinner_invincible_timer > 0.0 {
                        // Safe
                    } else {
                        player_hit = true;
                        break;
                    }
                }

                // Check Sidekick Collisions
                for sk in &mut self.player.sidekicks {
                    if !sk.active {
                        continue;
                    }
                    let sk_dist = sk.position.distance(e.position);
                    // Sidekick radius ~20.0 (from struct)
                    if sk_dist < (sk.radius + e.radius) {
                        // Specific Logic based on Enemy Type
                        match e.enemy_type {
                            EnemyType::GuidedMissile
                            | EnemyType::SmallRock
                            | EnemyType::BigRock
                            | EnemyType::UFO
                            | EnemyType::UFOBomb
                            | EnemyType::PolySnake => {
                                sk.active = false;
                                sidekick_explosion_positions.push(sk.position);
                            }
                            EnemyType::BigSpinner | EnemyType::SmallSpinner => {
                                wipe_sidekicks = true;
                                sk.active = false;
                                sidekick_explosion_positions.push(sk.position);
                            }
                        }
                    }
                }
            }
        }

        for e in &self.enemies {
            if e.just_landed
                && matches!(
                    e.enemy_type,
                    EnemyType::BigSpinner | EnemyType::SmallSpinner
                )
            {
                wipe_sidekicks = true;
            }
        }

        // Re-iterate collision for spinners to set wipe flag if we couldn't inside the loop
        // Actually, inside the collision loop above:
        // if collision with spinner -> wipe_sidekicks = true.

        if wipe_sidekicks {
            // Collect all sidekick positions for debris before clearing
            for sk in &self.player.sidekicks {
                if sk.active {
                    sidekick_explosion_positions.push(sk.position);
                }
            }
            self.player.sidekicks.clear();
        }

        // Spawn debris for exploded sidekicks
        for pos in sidekick_explosion_positions {
            // self.spawn_debris(pos); // Call corrected
            // We need to make sure we can call this with mutable borrow.
            // We are outside loop now.
            // But wait, earlier lint said spawn_debris_at not found. I need to use spawn_debris.
            self.spawn_debris(pos);
        }

        // Handle Exploded Sidekicks (Cleanup & Debris)
        // We need to know which ones just died.
        // Actually, let's just use `retain` and spawn debris for removed ones?
        // `retain` doesn't give us the removed items easily in stable Rust (drain_filter is nightly).
        // Manual implementation:
        let mut exploded_positions = Vec::new();
        let mut i = 0;
        while i < self.player.sidekicks.len() {
            if !self.player.sidekicks[i].active {
                exploded_positions.push(self.player.sidekicks[i].position);
                self.player.sidekicks.remove(i);
            } else {
                i += 1;
            }
        }

        for pos in exploded_positions {
            self.spawn_debris(pos);
        }

        if player_hit {
            // Invincibility Check
            if self.player.invincible_timer > 0.0 {
                // Ignore hit
            } else if self.player.is_shielded {
                // Shield absorbs hit completely (Invincible for duration)
                // Do nothing.
            } else if self.lives > 0 {
                self.lives -= 1;
                // self.enemies.clear(); // Keep enemies
                self.projectiles.clear();

                // Trigger Death State
                self.player_dead = true;
                self.dead_timer = 2.0;
                self.spawn_debris(self.player.position);

                // Prioritize Death Sound
                self.stop_all_sounds();
                if let Some(sound) = &self.player_die_sound {
                    let params = macroquad::audio::PlaySoundParams {
                        looped: false,
                        volume: self.sfx_volume * self.master_volume * 1.6,
                    };
                    macroquad::audio::play_sound(sound, params);
                }
            } else {
                return GameAction::GameOver;
            }
        }

        self.projectiles.retain(|p| p.active);
        self.enemies.retain(|e| e.active);

        // Old Audio Logic Removed

        let (current_mult, _) = self.get_multiplier_and_bg();

        // Audio Trackers
        let mut ufo_count = 0;
        let mut missile_count = 0;
        let mut big_spinner_count = 0;
        let mut small_spinner_count = 0;

        for e in &self.enemies {
            if !e.active {
                continue;
            }
            match e.enemy_type {
                EnemyType::UFO => ufo_count += 1,
                EnemyType::GuidedMissile => missile_count += 1,
                EnemyType::BigSpinner => big_spinner_count += 1,
                EnemyType::SmallSpinner => small_spinner_count += 1,
                _ => {}
            }
        }

        // Garbage Collect Audio & Manage Global Loops
        // Rules:
        // - If count > 0 && !active -> Start Loop, active=true
        // - If count == 0 && active -> Stop Loop, active=false

        // UFO
        if ufo_count > 0 {
            if !self.ufo_sound_active {
                if let Some(sound) = &self.ufo_sound {
                    let params = macroquad::audio::PlaySoundParams {
                        looped: true,
                        volume: self.sfx_volume * self.master_volume,
                    };
                    macroquad::audio::play_sound(sound, params);
                    self.ufo_sound_active = true;
                }
            }
        } else {
            if self.ufo_sound_active {
                if let Some(sound) = &self.ufo_sound {
                    macroquad::audio::stop_sound(sound);
                }
                self.ufo_sound_active = false;
            }
        }

        // Guided Missile
        if missile_count > 0 {
            if !self.missile_sound_active {
                if let Some(sound) = &self.guided_missile_sound {
                    let params = macroquad::audio::PlaySoundParams {
                        looped: true,
                        volume: self.sfx_volume * self.master_volume,
                    };
                    macroquad::audio::play_sound(sound, params);
                    self.missile_sound_active = true;
                }
            }
        } else {
            if self.missile_sound_active {
                if let Some(sound) = &self.guided_missile_sound {
                    macroquad::audio::stop_sound(sound);
                }
                self.missile_sound_active = false;
            }
        }

        // Big Spinner (Volume 0.5x)
        if big_spinner_count > 0 {
            if !self.big_spinner_sound_active {
                if let Some(sound) = &self.big_spinner_sound {
                    let params = macroquad::audio::PlaySoundParams {
                        looped: true,
                        volume: self.sfx_volume * self.master_volume * 0.25,
                    };
                    macroquad::audio::play_sound(sound, params);
                    self.big_spinner_sound_active = true;
                }
            }
        } else {
            if self.big_spinner_sound_active {
                if let Some(sound) = &self.big_spinner_sound {
                    macroquad::audio::stop_sound(sound);
                }
                self.big_spinner_sound_active = false;
            }
        }

        // Small Spinner (Volume 0.5x)
        if small_spinner_count > 0 {
            if !self.small_spinner_sound_active {
                if let Some(sound) = &self.small_spinner_sound {
                    let params = macroquad::audio::PlaySoundParams {
                        looped: true,
                        volume: self.sfx_volume * self.master_volume * 0.25,
                    };
                    macroquad::audio::play_sound(sound, params);
                    self.small_spinner_sound_active = true;
                }
            }
        } else {
            if self.small_spinner_sound_active {
                if let Some(sound) = &self.small_spinner_sound {
                    macroquad::audio::stop_sound(sound);
                }
                self.small_spinner_sound_active = false;
            }
        }

        if current_mult != prev_mult {
            if let Some(sound) = &self.multiplier_sound {
                let params = macroquad::audio::PlaySoundParams {
                    looped: false,
                    volume: self.sfx_volume * self.master_volume,
                };
                macroquad::audio::play_sound(sound, params);
            }
        }

        // Only Trigger Game Over if lives are gone AND we are not in the 'dead' animation state
        // Actually, if lives == 0, we can wait until dead_timer finishes or invalid state?
        // Let's just return GameOver if lives == 0 and we are done with debris?
        // Or simpler: If lives == 0, we set player_dead. When dead_timer expires, if lives == 0 -> GameOver.

        if self.lives == 0 && !self.player_dead {
            return GameAction::GameOver;
        }

        // --- Powerups ---
        for p in &mut self.powerups {
            p.update();
        }
        self.powerups.retain(|p| p.active);

        // Player Collision with Powerups
        if !self.player_dead {
            for p in &mut self.powerups {
                if !p.active {
                    continue;
                }

                let dist = self.player.position.distance(p.position);
                if dist < self.player.width / 2.0 + p.radius {
                    // Collect!
                    p.active = false;
                    match p.power_type {
                        PowerUpType::Shield => {
                            if self.player.shields < self.player.max_shields {
                                self.player.shields += 1;
                            }
                        }
                        PowerUpType::Sidekick => {
                            self.player.add_sidekick();
                        }
                    }

                    // Play Pickup Sound if available
                    // Add later if requested, reusing shield sound currently?
                }
            }
        }

        // Shield Activation Input
        if is_key_pressed(KeyCode::E) {
            self.player.activate_shield();
        }

        return GameAction::None;
    }

    pub fn stop_all_sounds(&mut self) {
        if let Some(sound) = &self.shoot_sound {
            macroquad::audio::stop_sound(sound);
        }
        if let Some(sound) = &self.ufo_sound {
            macroquad::audio::stop_sound(sound);
        }

        if let Some(sound) = &self.guided_missile_sound {
            macroquad::audio::stop_sound(sound);
        }

        if let Some(sound) = &self.guided_missile_ground_sound {
            macroquad::audio::stop_sound(sound);
        }

        if let Some(sound) = &self.big_spinner_sound {
            macroquad::audio::stop_sound(sound);
        }

        if let Some(sound) = &self.small_spinner_sound {
            macroquad::audio::stop_sound(sound);
        }
    }

    pub fn draw(&self) {
        let (multiplier, bg_color) = self.get_multiplier_and_bg();

        // Background
        let mut final_bg_color = bg_color;

        // Death Flash Logic
        if self.player_dead {
            // Flash 2x per sec = 0.5s period.
            // On for 0.25s, off for 0.25s?
            // "flash red and whatever color... 2x per sec"
            // Use time or timer.
            // dead_timer goes from 2.0 down to 0.0.
            if self.dead_timer % 0.5 < 0.25 {
                final_bg_color = RED;
            }
        }

        clear_background(final_bg_color);

        if self.background.is_some() || self.ground.is_some() {
            let time = get_time() as f32;
            let cam_dist = 2500.0f32;
            let cam_pos = vec3(0.0, 0.0, cam_dist);

            let camera = Camera3D {
                position: cam_pos,
                target: vec3(0.0, 100.0, 0.0),
                up: vec3(0.0, 1.0, 0.0),
                ..Default::default()
            };

            set_camera(&camera);

            // Draw Starfield BEHIND mountains (first)
            if let Some(stars) = &self.starfield {
                stars.draw();
            }

            if let Some(bg) = &self.background {
                assets::draw_design_3d(bg, 3.0, time);
            }

            set_default_camera();

            if let Some(gnd) = &self.ground {
                let ground_pos = vec2(screen_width() / 2.0, screen_height() - 32.0);
                assets::draw_design(gnd, ground_pos, 0.0, 1.0, time);
            }
        }

        if !self.player_dead {
            self.player.draw();
            for sk in &self.player.sidekicks {
                sk.draw();
            }
        } else {
            // Draw Debris
            let scale = 3.0; // Assuming same scale
            for d in &self.debris {
                // We need to construct a temp ShapeDesign or just draw the part?
                // assets::draw_design takes a full design.
                // We can expose a helper to draw a part or just construct a temp wrapper.
                // Or just use draw_line manually based on the part type since Debris holds ShapePart.

                // Easier: Create a temporary ShapeDesign with 1 part.
                let temp_design = ShapeDesign {
                    parts: vec![d.part.clone()],
                };
                assets::draw_design(&temp_design, d.position, d.rotation, scale, 0.0);
            }
        }

        for p in &self.projectiles {
            p.draw();
        }

        for e in &self.enemies {
            e.draw();
        }

        draw_text(&format!("SCORE: {}", self.score), 20.0, 50.0, 40.0, WHITE);
        draw_text(
            &format!("LIVES: {}", self.lives),
            screen_width() - 200.0,
            50.0,
            40.0,
            WHITE,
        );

        // Shield HUD
        // "little orange lined on black rectangular hud meter... shows the shield amount as a half circle icon... near the score area"
        // Score is at 20.0, 50.0.
        // Let's put Shield HUD below it at Y=120.0.
        let hud_x = 20.0;
        let hud_y = 100.0;
        let hud_w = 200.0;
        let hud_h = 40.0;

        // Background
        draw_rectangle(hud_x, hud_y, hud_w, hud_h, BLACK);
        draw_rectangle_lines(hud_x, hud_y, hud_w, hud_h, 2.0, ORANGE);

        // Icons
        // Max 6. Spacing based on width.
        let icon_spacing = 30.0;
        for i in 0..self.player.max_shields {
            let cx = hud_x + 20.0 + i as f32 * icon_spacing;
            let cy = hud_y + 25.0; // Center Y
            let radius = 10.0;

            // Draw empty slot (maybe dim?)
            // draw_circle_lines(cx, cy, radius, 1.0, GRAY);

            if i < self.player.shields {
                // Draw "yellow half circle icon (top half)"
                // macroquad doesn't have draw_arc easily, but we can use poly or verify "top half".
                // We can just draw a circle and clip it? No clip rect.
                // Draw a circle then draw a rect over bottom half?
                // Or draw lines.
                // Or use `draw_poly` logic manually.

                // Let's draw a yellow semi-circle using `draw_poly`?
                // Or just a full circle for simplicity first? Request says "top half of the circle".
                // Implementation: Draw top half manually.

                // Implementation: Draw top half manually.

                // Center fan
                let color = YELLOW;

                // We want angles from PI (180) to 2*PI (360) for top half? No, screen coords. 0 is right.
                // Top half in screen space (Y down) is actually from PI to 0? No.
                // Standard circle:
                // 0 is Right (1, 0)
                // PI/2 is Down (0, 1)
                // PI is Left (-1, 0)
                // 3PI/2 is Up (0, -1)

                // So top half is PI to 2PI (or 0).
                // Actually 3PI/2 is up. So we want PI to 2PI? No.
                // Let's verify.
                // We want Y < Cy.
                // Angles where sin(theta) < 0.
                // sin(PI) = 0. sin(3PI/2) = -1. sin(2PI) = 0.
                // So PI to 2PI.

                // Draw a filled semi-circle
                // We can fake it with a circle and a black rect covering the bottom half.
                // Since background is black, this works perfectly!
                draw_circle(cx, cy, radius, color);
                draw_rectangle(cx - radius, cy, radius * 2.0, radius + 1.0, BLACK); // Cover bottom
            } else {
                // Empty slot marker
                draw_circle(cx, cy, 2.0, DARKGRAY);
            }
        }

        // Draw Powerups
        for p in &self.powerups {
            p.draw();
        }

        draw_text(&format!("{}X", multiplier), 20.0, 90.0, 30.0, GREEN);

        let font_size = 20.0;
        let mut msg: Option<&str> = None;
        if self.load_notification_timer > 0.0 {
            msg = Some("GAME LOADED");
        }

        if let Some(text) = msg {
            let params = TextParams {
                font_size: font_size as u16,
                font: None,
                color: GREEN,
                ..Default::default()
            };

            let dims = measure_text(text, None, font_size as u16, 1.0);
            draw_text_ex(text, screen_width() - dims.width - 20.0, 30.0, params);
        }
    }
}
