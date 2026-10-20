use macroquad::prelude::*;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct ShapeDesign {
    parts: Vec<ShapePart>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
struct Animation {
    speed: f32,
    amplitude: f32, // Degrees
    offset: f32,    // Phase shift
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum ShapePart {
    Line {
        start: Vec<f32>,
        end: Vec<f32>,
        #[allow(dead_code)]
        thickness: f32,
        color: [f32; 4],
        #[serde(default)]
        offset_start: Vec<f32>,
        #[serde(default)]
        offset_end: Vec<f32>,
        #[serde(default)]
        animation: Option<Animation>,
        #[serde(default)]
        z: f32,
    },
    Triangle {
        v1: Vec<f32>,
        v2: Vec<f32>,
        v3: Vec<f32>,
        color: [f32; 4],
        #[serde(default)]
        z: f32,
    },
    Poly {
        center: Vec<f32>,
        sides: u8,
        radius: f32,
        rotation: f32,
        color: [f32; 4],
        #[serde(default)]
        z: f32,
    },
    PolyLine {
        center: Vec<f32>,
        sides: u8,
        radius: f32,
        rotation: f32,
        thickness: f32,
        color: [f32; 4],
        #[serde(default)]
        z: f32,
    },
    Circle {
        center: Vec<f32>,
        radius: f32,
        color: [f32; 4],
        #[serde(default)]
        z: f32,
    },
    Cylinder {
        start: Vec<f32>,
        end: Vec<f32>,
        radius: f32,
        sides: u8,
        color: [f32; 4],
        #[serde(default)]
        offset_start: Vec<f32>,
        #[serde(default)]
        offset_end: Vec<f32>,
        #[serde(default)]
        animation: Option<Animation>,
        #[serde(default)]
        z: f32,
    },
}

// Helper to get Vec3 from dynamic vec (2D or 3D)
fn vec3_from_vec(v: &Vec<f32>, default_z: f32) -> Vec3 {
    let x = *v.get(0).unwrap_or(&0.0);
    let y = *v.get(1).unwrap_or(&0.0);
    let z = *v.get(2).unwrap_or(&default_z);
    vec3(x, y, z)
}

fn color_from_arr(arr: [f32; 4]) -> Color {
    Color::new(arr[0], arr[1], arr[2], arr[3])
}

fn draw_design(design: &ShapeDesign, scale: f32, time: f32) {
    for part in &design.parts {
        match part {
            ShapePart::Line {
                start,
                end,
                thickness: _,
                color,
                offset_start,
                offset_end,
                animation,
                z,
            } => {
                let z_base = *z;
                let p_start_local = vec3_from_vec(start, z_base) + vec3_from_vec(offset_start, 0.0);
                let mut p_end_local = vec3_from_vec(end, z_base) + vec3_from_vec(offset_end, 0.0);

                if let Some(anim) = animation {
                    // Wiggle relative to start pos (simple 2D-plane rotation for now, could be upgraded)
                    let angle_offset = (time * anim.speed + anim.offset).sin() * anim.amplitude;

                    let delta = p_end_local - p_start_local;
                    // Rotate around Z axis
                    let current_angle = delta.y.atan2(delta.x);
                    let new_angle = current_angle + angle_offset.to_radians();
                    let len_xy = vec2(delta.x, delta.y).length();

                    p_end_local.x = p_start_local.x + new_angle.cos() * len_xy;
                    p_end_local.y = p_start_local.y + new_angle.sin() * len_xy;
                    // Z remains offset
                }

                draw_line_3d(
                    p_start_local * scale,
                    p_end_local * scale,
                    color_from_arr(*color),
                );
            }
            ShapePart::Triangle {
                v1,
                v2,
                v3,
                color,
                z,
            } => {
                // Draw triangle as generic lines for wireframe 3D look
                let p1 = vec3_from_vec(v1, *z) * scale;
                let p2 = vec3_from_vec(v2, *z) * scale;
                let p3 = vec3_from_vec(v3, *z) * scale;
                let c = color_from_arr(*color);
                draw_line_3d(p1, p2, c);
                draw_line_3d(p2, p3, c);
                draw_line_3d(p3, p1, c);
            }
            ShapePart::Poly {
                center,
                sides,
                radius,
                rotation,
                color,
                z,
            } => {
                let c = vec3_from_vec(center, *z) * scale;
                let r = *radius * scale;
                let rot_rad = rotation.to_radians();
                let col = color_from_arr(*color);

                for i in 0..*sides {
                    let angle1 = rot_rad + i as f32 * std::f32::consts::PI * 2.0 / *sides as f32;
                    let angle2 =
                        rot_rad + (i + 1) as f32 * std::f32::consts::PI * 2.0 / *sides as f32;

                    let p1 = c + vec3(angle1.cos() * r, angle1.sin() * r, 0.0);
                    let p2 = c + vec3(angle2.cos() * r, angle2.sin() * r, 0.0);

                    draw_line_3d(p1, p2, col);
                }
            }
            ShapePart::PolyLine {
                center,
                sides,
                radius,
                rotation,
                thickness: _thickness,
                color,
                z,
            } => {
                // Treated same as Poly for 3D wireframe
                let c = vec3_from_vec(center, *z) * scale;
                let r = *radius * scale;
                let rot_rad = rotation.to_radians();
                let col = color_from_arr(*color);

                for i in 0..*sides {
                    let angle1 = rot_rad + i as f32 * std::f32::consts::PI * 2.0 / *sides as f32;
                    let angle2 =
                        rot_rad + (i + 1) as f32 * std::f32::consts::PI * 2.0 / *sides as f32;

                    let p1 = c + vec3(angle1.cos() * r, angle1.sin() * r, 0.0);
                    let p2 = c + vec3(angle2.cos() * r, angle2.sin() * r, 0.0);

                    draw_line_3d(p1, p2, col);
                }
            }
            ShapePart::Circle {
                center,
                radius,
                color,
                z,
            } => {
                let c = vec3_from_vec(center, *z) * scale;
                let r = *radius * scale;
                let col = color_from_arr(*color);

                // Approximate circle with 16 segments
                let segments = 16;
                for i in 0..segments {
                    let angle1 = i as f32 * std::f32::consts::PI * 2.0 / segments as f32;
                    let angle2 = (i + 1) as f32 * std::f32::consts::PI * 2.0 / segments as f32;

                    let p1 = c + vec3(angle1.cos() * r, angle1.sin() * r, 0.0);
                    let p2 = c + vec3(angle2.cos() * r, angle2.sin() * r, 0.0);
                    draw_line_3d(p1, p2, col);
                }
            }
            ShapePart::Cylinder {
                start,
                end,
                radius,
                sides,
                color,
                offset_start,
                offset_end,
                animation,
                z,
            } => {
                let z_base = *z;
                let p_start_base = vec3_from_vec(start, z_base) + vec3_from_vec(offset_start, 0.0);
                let mut p_end_base = vec3_from_vec(end, z_base) + vec3_from_vec(offset_end, 0.0);

                if let Some(anim) = animation {
                    let angle_offset = (time * anim.speed + anim.offset).sin() * anim.amplitude;
                    let delta = p_end_base - p_start_base;
                    let current_angle = delta.y.atan2(delta.x);
                    let new_angle = current_angle + angle_offset.to_radians();
                    let len_xy = vec2(delta.x, delta.y).length();
                    p_end_base.x = p_start_base.x + new_angle.cos() * len_xy;
                    p_end_base.y = p_start_base.y + new_angle.sin() * len_xy;
                }

                let p1 = p_start_base * scale;
                let p2 = p_end_base * scale;
                let r = *radius * scale;
                let col = color_from_arr(*color);

                // Compute basis constants for the ring
                let axis = (p2 - p1).normalize_or_zero();
                // Arbitrary vector needed to create orthogonal basis
                let mut arbitrary = vec3(0.0, 1.0, 0.0);
                if axis.y.abs() > 0.99 {
                    arbitrary = vec3(1.0, 0.0, 0.0);
                }
                let u = axis.cross(arbitrary).normalize_or_zero();
                let v = axis.cross(u).normalize_or_zero();

                for i in 0..*sides {
                    let angle_a = i as f32 * std::f32::consts::PI * 2.0 / *sides as f32;
                    let angle_b = (i + 1) as f32 * std::f32::consts::PI * 2.0 / *sides as f32;

                    // Start Ring Points
                    let s_a = p1 + (u * angle_a.cos() + v * angle_a.sin()) * r;
                    let s_b = p1 + (u * angle_b.cos() + v * angle_b.sin()) * r;

                    // End Ring Points
                    let e_a = p2 + (u * angle_a.cos() + v * angle_a.sin()) * r;
                    let e_b = p2 + (u * angle_b.cos() + v * angle_b.sin()) * r;

                    // Draw Start Cap Ring
                    draw_line_3d(s_a, s_b, col);
                    // Draw End Cap Ring
                    draw_line_3d(e_a, e_b, col);
                    // Draw Rib (side line)
                    draw_line_3d(s_a, e_a, col);
                }
            }
        }
    }
}

#[macroquad::main("Macroquad-Serde Asset Engine")]
async fn main() {
    let mut assets_dir = "Assets".to_string();
    // Try to find Assets directory if not in current dir
    if fs::metadata(&assets_dir).is_err() {
        if fs::metadata("../Assets").is_ok() {
            assets_dir = "../Assets".to_string();
        }
    }

    // Initial load
    let mut design: Option<ShapeDesign>; // fix unused assignment warning
    let load_design = |path: &str| -> Option<ShapeDesign> {
        if let Ok(content) = fs::read_to_string(path) {
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    };

    let mut files: Vec<String> = Vec::new();

    // Scan for json files
    if let Ok(entries) = fs::read_dir(&assets_dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".json") {
                    files.push(format!("{}/{}", assets_dir, name));
                }
            }
        }
    }
    files.sort();

    let mut current_index = 0;
    if files.is_empty() {
        println!("No JSON files found in Assets/");
        return;
    }

    let mut asset_path = files[current_index].clone();
    let mut last_modified = fs::metadata(&asset_path).and_then(|m| m.modified()).ok();

    design = load_design(&asset_path);

    // Camera State
    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;
    let mut cam_dist = 500.0f32;

    loop {
        // Handle Input
        let mut changed = false;
        if is_key_pressed(KeyCode::Right) {
            current_index = (current_index + 1) % files.len();
            changed = true;
        }
        if is_key_pressed(KeyCode::Left) {
            current_index = (current_index + files.len() - 1) % files.len();
            changed = true;
        }

        if changed {
            asset_path = files[current_index].clone();
            design = load_design(&asset_path);
            last_modified = fs::metadata(&asset_path).and_then(|m| m.modified()).ok();
        }

        // Orbit Camera Controls
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_delta = mouse_delta_position();
            cam_angle_x += mouse_delta.x * 2.0;
            cam_angle_y += mouse_delta.y * 2.0;
            // clamp y to avoid flipping
            cam_angle_y = cam_angle_y.clamp(-1.5, 1.5);
        }

        let mouse_wheel = mouse_wheel().1;
        if mouse_wheel != 0.0 {
            cam_dist -= mouse_wheel * 20.0;
            cam_dist = cam_dist.clamp(50.0, 2000.0);
        }

        // Hot Reload
        if let Ok(metadata) = fs::metadata(&asset_path) {
            if let Ok(modified) = metadata.modified() {
                if last_modified != Some(modified) {
                    last_modified = Some(modified);
                    if let Some(new_design) = load_design(&asset_path) {
                        println!("Reloaded design: {}", asset_path);
                        design = Some(new_design);
                    }
                }
            }
        }

        clear_background(BLACK);

        let time = get_time() as f32;

        // Setup Camera
        let cam_pos = vec3(
            cam_angle_x.sin() * cam_angle_y.cos() * cam_dist,
            cam_angle_y.sin() * cam_dist,
            cam_angle_x.cos() * cam_angle_y.cos() * cam_dist,
        );

        let camera = Camera3D {
            position: cam_pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        };

        set_camera(&camera);

        // Draw Origin Grid using built-in or manual lines
        draw_grid(
            20,
            50.0,
            Color::new(0.2, 0.2, 0.2, 0.5),
            Color::new(0.2, 0.2, 0.2, 0.5),
        );

        if let Some(d) = &design {
            draw_design(d, 3.0, time);
        }

        set_default_camera();

        if design.is_none() {
            draw_text(
                &format!("Failed to load: {}", asset_path),
                20.0,
                40.0,
                30.0,
                RED,
            );
        }

        draw_text("Macroquad-Serde Asset Engine", 20.0, 30.0, 30.0, SKYBLUE);
        draw_text(
            &format!(
                "File: {} ({}/{})",
                asset_path,
                current_index + 1,
                files.len()
            ),
            20.0,
            60.0,
            20.0,
            WHITE,
        );
        draw_text(
            "Left Click + Drag to Rotate. Scroll to Zoom.",
            20.0,
            80.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
