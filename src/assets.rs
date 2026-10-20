use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

const ASSET_DIR: &str = "Macroquad-Serde Asset Engine/Assets";

/// Load a shape JSON by filename under the asset engine folder.
/// Tries CWD then parent (when launched from `src/`). No machine-absolute paths.
pub fn load_shape_file(filename: &str) -> ShapeDesign {
    load_shape_path(&format!("{ASSET_DIR}/{filename}")).unwrap_or_default()
}

pub fn load_shape_path(path: &str) -> Option<ShapeDesign> {
    for candidate in [path.to_string(), format!("../{path}")] {
        if let Ok(content) = fs::read_to_string(&candidate) {
            match serde_json::from_str(&content) {
                Ok(design) => return Some(design),
                Err(e) => {
                    println!("Failed to parse {candidate}: {e}");
                    return None;
                }
            }
        }
    }
    println!("Failed to load asset: {path} (also tried ../{path})");
    None
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShapeDesign {
    pub parts: Vec<ShapePart>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Animation {
    pub speed: f32,
    pub amplitude: f32, // Degrees
    pub offset: f32,    // Phase shift
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ShapePart {
    Line {
        start: Vec<f32>,
        end: Vec<f32>,
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
#[allow(dead_code)]
fn vec3_from_vec(v: &Vec<f32>, default_z: f32) -> Vec3 {
    let x = *v.get(0).unwrap_or(&0.0);
    let y = *v.get(1).unwrap_or(&0.0);
    let z = *v.get(2).unwrap_or(&default_z);
    vec3(x, y, z)
}

fn color_from_arr(arr: [f32; 4]) -> Color {
    Color::new(arr[0], arr[1], arr[2], arr[3])
}

// Helper for safe 2D vector access
fn safe_vec2(v: &Vec<f32>, default_val: f32) -> Vec2 {
    let x = *v.get(0).unwrap_or(&default_val);
    let y = *v.get(1).unwrap_or(&default_val);
    vec2(x, y)
}

pub fn draw_design(design: &ShapeDesign, position: Vec2, rotation: f32, scale: f32, time: f32) {
    let rot_rad = rotation.to_radians();
    let cos_r = rot_rad.cos();
    let sin_r = rot_rad.sin();

    // Helper to rotate a point around (0,0)
    let rotate_point =
        |p: Vec2| -> Vec2 { vec2(p.x * cos_r - p.y * sin_r, p.x * sin_r + p.y * cos_r) };

    for part in &design.parts {
        match part {
            ShapePart::Line {
                start,
                end,
                thickness,
                color,
                offset_start,
                offset_end,
                animation,
                z: _,
            } => {
                let p_start_base = safe_vec2(start, 0.0);
                let p_offset_start = safe_vec2(offset_start, 0.0);
                let p_end_base = safe_vec2(end, 0.0);
                let p_offset_end = safe_vec2(offset_end, 0.0);

                let p_start_local = p_start_base + p_offset_start;
                let mut p_end_local = p_end_base + p_offset_end;

                if let Some(anim) = animation {
                    let angle_offset = (time * anim.speed + anim.offset).sin() * anim.amplitude;
                    let delta = p_end_local - p_start_local;
                    let current_angle = delta.y.atan2(delta.x);
                    let new_angle = current_angle + angle_offset.to_radians();
                    let len = delta.length();

                    p_end_local.x = p_start_local.x + new_angle.cos() * len;
                    p_end_local.y = p_start_local.y + new_angle.sin() * len;
                }

                // Rotate points
                let p1 = rotate_point(p_start_local) * scale + position;
                let p2 = rotate_point(p_end_local) * scale + position;

                draw_line(
                    p1.x,
                    p1.y,
                    p2.x,
                    p2.y,
                    *thickness * (scale * 0.5),
                    color_from_arr(*color),
                );
            }
            ShapePart::Triangle {
                v1,
                v2,
                v3,
                color,
                z: _,
            } => {
                let p1 = rotate_point(safe_vec2(v1, 0.0)) * scale + position;
                let p2 = rotate_point(safe_vec2(v2, 0.0)) * scale + position;
                let p3 = rotate_point(safe_vec2(v3, 0.0)) * scale + position;
                let c = color_from_arr(*color);

                let t = 2.0;
                draw_line(p1.x, p1.y, p2.x, p2.y, t, c);
                draw_line(p2.x, p2.y, p3.x, p3.y, t, c);
                draw_line(p3.x, p3.y, p1.x, p1.y, t, c);
            }
            ShapePart::PolyLine {
                center,
                sides,
                radius,
                rotation: shape_rot,
                thickness,
                color,
                z: _,
            } => {
                let c_off = safe_vec2(center, 0.0);
                let r = *radius; // Apply scale later
                let shape_rot_rad = shape_rot.to_radians(); // Internal shape rotation
                let col = color_from_arr(*color);

                for i in 0..*sides {
                    let angle1 =
                        shape_rot_rad + i as f32 * std::f32::consts::PI * 2.0 / *sides as f32;
                    let angle2 =
                        shape_rot_rad + (i + 1) as f32 * std::f32::consts::PI * 2.0 / *sides as f32;

                    let p1_local = c_off + vec2(angle1.cos() * r, angle1.sin() * r);
                    let p2_local = c_off + vec2(angle2.cos() * r, angle2.sin() * r);

                    let p1 = rotate_point(p1_local) * scale + position;
                    let p2 = rotate_point(p2_local) * scale + position;

                    draw_line(p1.x, p1.y, p2.x, p2.y, *thickness, col);
                }
            }
            ShapePart::Poly {
                center,
                sides,
                radius,
                rotation: shape_rot,
                color,
                z: _,
            } => {
                let c_off = safe_vec2(center, 0.0);
                let r = *radius;
                let shape_rot_rad = shape_rot.to_radians();
                let col = color_from_arr(*color);

                for i in 0..*sides {
                    let angle1 =
                        shape_rot_rad + i as f32 * std::f32::consts::PI * 2.0 / *sides as f32;
                    let angle2 =
                        shape_rot_rad + (i + 1) as f32 * std::f32::consts::PI * 2.0 / *sides as f32;

                    let p1_local = c_off + vec2(angle1.cos() * r, angle1.sin() * r);
                    let p2_local = c_off + vec2(angle2.cos() * r, angle2.sin() * r);

                    let p1 = rotate_point(p1_local) * scale + position;
                    let p2 = rotate_point(p2_local) * scale + position;

                    draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, col);
                }
            }
            ShapePart::Circle {
                center,
                radius,
                color,
                z: _,
            } => {
                let c_local = safe_vec2(center, 0.0);
                let c = rotate_point(c_local) * scale + position;
                // Circle rotation doesn't change outline, so just position
                let r = *radius * scale;
                let col = color_from_arr(*color);
                draw_circle_lines(c.x, c.y, r, 2.0, col);
            }
            ShapePart::Cylinder {
                start,
                end,
                radius,
                sides: _,
                color,
                offset_start,
                offset_end,
                animation,
                z: _,
            } => {
                let p_start_base = safe_vec2(start, 0.0);
                let p_offset_start = safe_vec2(offset_start, 0.0);
                let p_end_base = safe_vec2(end, 0.0);
                let p_offset_end = safe_vec2(offset_end, 0.0);

                let p_start_local = p_start_base + p_offset_start;
                let mut p_end_local = p_end_base + p_offset_end;

                if let Some(anim) = animation {
                    let angle_offset = (time * anim.speed + anim.offset).sin() * anim.amplitude;
                    let delta = p_end_local - p_start_local;
                    let current_angle = delta.y.atan2(delta.x);
                    let new_angle = current_angle + angle_offset.to_radians();
                    let len = delta.length();
                    p_end_local.x = p_start_local.x + new_angle.cos() * len;
                    p_end_local.y = p_start_local.y + new_angle.sin() * len;
                }

                // Draw as WIREFRAME (Outline only)
                // We need to draw the "box" around it logic in 2D locally then rotate
                // p_start_local and p_end_local define the spine.
                // We need width = radius * 2.
                // Perpendicular vector for width.
                let spine = p_end_local - p_start_local;
                let spine_len = spine.length();
                if spine_len > 0.001 {
                    let spine_norm = spine / spine_len;
                    let perp = vec2(-spine_norm.y, spine_norm.x);
                    let r = *radius;

                    // 4 Corners relative to (0,0) before global transform
                    let c1 = p_start_local + perp * r;
                    let c2 = p_start_local - perp * r;
                    let c3 = p_end_local + perp * r;
                    let c4 = p_end_local - perp * r;

                    // Transform to world
                    let w1 = rotate_point(c1) * scale + position;
                    let w2 = rotate_point(c2) * scale + position;
                    let w3 = rotate_point(c3) * scale + position;
                    let w4 = rotate_point(c4) * scale + position;

                    let col = color_from_arr(*color);
                    let t = 2.0; // Thickness of outline

                    // Draw Rectangle Outline
                    draw_line(w1.x, w1.y, w2.x, w2.y, t, col); // Bottom Cap
                    draw_line(w3.x, w3.y, w4.x, w4.y, t, col); // Top Cap
                    draw_line(w1.x, w1.y, w3.x, w3.y, t, col); // Side 1
                    draw_line(w2.x, w2.y, w4.x, w4.y, t, col); // Side 2
                }
            }
        }
    }
}

pub fn draw_design_3d(design: &ShapeDesign, scale: f32, time: f32) {
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
