//! Game board.

use bevy::prelude::*;

use crate::resources::TileMap;

/// Complete world board.
#[derive(Debug)]
pub struct Board {
    pub entity: Entity,
    pub tile_map: TileMap,
}
