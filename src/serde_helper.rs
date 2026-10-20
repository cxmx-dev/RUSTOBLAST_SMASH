use macroquad::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// Internal surrogate struct
#[derive(Serialize, Deserialize)]
struct SerdeVec2 {
    x: f32,
    y: f32,
}

impl From<Vec2> for SerdeVec2 {
    fn from(v: Vec2) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl Into<Vec2> for SerdeVec2 {
    fn into(self) -> Vec2 {
        vec2(self.x, self.y)
    }
}

pub fn serialize<S>(val: &Vec2, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let surrogate = SerdeVec2::from(*val);
    surrogate.serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec2, D::Error>
where
    D: Deserializer<'de>,
{
    let surrogate = SerdeVec2::deserialize(deserializer)?;
    Ok(surrogate.into())
}
