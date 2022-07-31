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
