use bevy::prelude::*;
use std::{
    fmt::{self, Display, Formatter},
    ops::{Add, Sub},
};

use crate::settings::MAP_RES;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl Coordinates {
    #[inline]
    #[must_use]
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl From<(u16, u16)> for Coordinates {
    #[inline]
    fn from((x, y): (u16, u16)) -> Self {
        Self { x, y }
    }
}

impl Add for Coordinates {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: (self.x + rhs.x) % MAP_RES[0] as u16,
            y: (self.y + rhs.y) % MAP_RES[1] as u16,
        }
    }
}

impl Add<(i8, i8)> for Coordinates {
    type Output = Self;

    #[inline]
    fn add(self, (x, y): (i8, i8)) -> Self::Output {
        let x = (((self.x as i16) + i16::from(x)) as u16) % MAP_RES[0] as u16;
        let y = (((self.y as i16) + i16::from(y)) as u16) % MAP_RES[1] as u16;
        Self { x, y }
    }
}

impl Sub for Coordinates {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x % MAP_RES[0] as u16,
            y: self.y - rhs.y % MAP_RES[1] as u16,
        }
    }
}

impl Display for Coordinates {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
