use ndarray::Array2;

use crate::resources::Tile;

/// Base tile map.
#[derive(Debug, Clone)]
pub struct TileMap {
    map: Array2<Tile>,
}

impl TileMap {
    #[inline]
    #[must_use]
    pub fn new(size: [usize; 2]) -> Self {
        Self {
            map: Array2::from_elem(size, Tile::Empty),
        }
    }
}
