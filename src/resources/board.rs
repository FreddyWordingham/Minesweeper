//! Game board.

use bevy::prelude::*;

/// Complete world board.
#[derive(Debug)]
pub struct Board {
    pub entity: Entity,
}

impl Board {
    /// Generate a new Board.
    #[inline]
    #[must_use]
    pub fn new(entity: Entity) -> Self {
        Board { entity }
    }
}
