use ndarray::Array2;

/// Base tile map.
#[derive(Debug, Clone)]
pub struct TileMap {
    map: Array2<Tile>,
}

impl TileMap {
    pub fn new(size: [usize; 2]) -> Self {
        TileMap {
            map: Array2::from_elem(size, Tile::Empty),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Bomb,
    BombNeighbor(u8),
    Empty,
}
