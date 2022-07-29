use ndarray::Array2;
use rand::Rng;

use crate::resources::Tile;

/// Base tile map.
#[derive(Debug, Clone)]
pub struct TileMap {
    /// 2D array of tiles.
    tiles: Array2<Tile>,
}

impl TileMap {
    #[inline]
    #[must_use]
    pub fn new(map_size: [usize; 2]) -> Self {
        Self {
            tiles: Array2::from_elem(map_size, Tile::Empty),
        }
    }

    #[inline]
    #[must_use]
    pub fn width(&self) -> usize {
        self.tiles.shape()[0]
    }

    #[inline]
    #[must_use]
    pub fn height(&self) -> usize {
        self.tiles.shape()[1]
    }

    #[inline]
    #[must_use]
    pub fn bomb_count(&self) -> u16 {
        self.tiles
            .iter()
            .filter(|t| match **t {
                Tile::Bomb => true,
                Tile::BombNeighbor(_) | Tile::Empty => false,
            })
            .count()
            .try_into()
            .expect("Bomb count overflow!")
    }

    #[inline]
    pub fn add_bombs(&mut self, bomb_count: u16) {
        debug_assert!(self.tiles.len() >= (bomb_count + self.bomb_count()).into());

        let mut rng = rand::thread_rng();
        for _ in 0..bomb_count {
            loop {
                let x = rng.gen_range(0..self.tiles.shape()[0]);
                let y = rng.gen_range(0..self.tiles.shape()[1]);
                if self.tiles[(x, y)] != Tile::Bomb {
                    self.tiles[(x, y)] = Tile::Bomb;
                    break;
                }
            }
        }
    }
}
