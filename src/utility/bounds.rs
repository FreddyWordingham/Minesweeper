use bevy::prelude::Vec2;

#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    pub mins: Vec2,
    pub maxs: Vec2,
}

impl Bounds {
    #[inline]
    #[must_use]
    pub const fn new(mins: Vec2, maxs: Vec2) -> Self {
        Self { mins, maxs }
    }
}
