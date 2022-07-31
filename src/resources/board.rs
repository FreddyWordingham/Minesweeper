//! Game board.

use bevy::{prelude::*, utils::HashMap};

use crate::{components::Coordinates, resources::TileMap};

/// Complete world board.
#[derive(Debug)]
pub struct Board {
    pub entity: Entity,
    pub tile_map: TileMap,
    pub covered_tiles: HashMap<Coordinates, Entity>,
    pub marked_tiles: Vec<Coordinates>,
}

impl Board {
    #[must_use]
    #[inline]
    pub fn adjacent_covered_tiles(&self, coord: Coordinates) -> Vec<Entity> {
        TileMap::safe_square_at(coord)
            .filter_map(|c| self.covered_tiles.get(&c))
            .copied()
            .collect()
    }

    #[must_use]
    #[inline]
    pub fn is_completed(&self) -> bool {
        self.tile_map.bomb_count() as usize == self.covered_tiles.len()
    }
}
