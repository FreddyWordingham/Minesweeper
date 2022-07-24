use bevy::prelude::Vec2;

#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    pub mins: Vec2,
    pub maxs: Vec2,
}
