use bevy::prelude::*;
use std::{
    fmt::{self, Display, Formatter},
    ops::Add,
};

use crate::settings::{MAP_RES, TILE_SIZE};

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinates {
    pub x: i16,
    pub y: i16,
}

impl Coordinates {
    #[inline]
    #[must_use]
    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

impl Coordinates {
    pub fn world_pos(self) -> Vec3 {
        Vec3::new(
            TILE_SIZE * (self.x as f32 + 0.5),
            TILE_SIZE * (self.y as f32 + 0.5),
            1.0,
        )
    }
}

impl Add<(i16, i16)> for Coordinates {
    type Output = Self;

    #[inline]
    fn add(self, (x, y): (i16, i16)) -> Self::Output {
        Self {
            x: (self.x + x).rem_euclid(MAP_RES[0]),
            y: (self.y + y).rem_euclid(MAP_RES[1]),
        }
    }
}

impl Display for Coordinates {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
