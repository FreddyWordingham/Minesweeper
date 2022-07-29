use ndarray::Array2;
use rand::Rng;

use crate::{components::Coordinates, resources::Tile};

/// Delta coordinates for all 8 square neighbors
/// [6] [7] [8]
/// [4]     [5]
/// [1] [2] [3]
const SQUARE_COORDINATES: [(i16, i16); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

/// Base tile map.
#[derive(Debug, Clone)]
pub struct TileMap {
    /// 2D array of tiles.
    pub tiles: Array2<Tile>,
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

        for y in 0..self.height() {
            for x in 0..self.width() {
                let coords = Coordinates {
                    x: x as i16,
                    y: y as i16,
                };
                if !self.is_bomb_at(coords) {
                    let count = self.bomb_count_at(coords);
                    if count > 0 {
                        self.tiles[(x, y)] = Tile::BombNeighbor(count);
                    }
                }
            }
        }
    }

    #[inline]
    pub fn safe_square_at(coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
        SQUARE_COORDINATES
            .iter()
            .copied()
            .map(move |offset| coordinates + offset)
    }

    #[inline]
    #[must_use]
    pub fn is_bomb_at(&self, coordinates: Coordinates) -> bool {
        if coordinates.x as usize >= self.width() || coordinates.y as usize >= self.height() {
            return false;
        };
        self.tiles[(coordinates.x as usize, coordinates.y as usize)].is_bomb()
    }

    #[inline]
    #[must_use]
    pub fn bomb_count_at(&self, coordinates: Coordinates) -> u8 {
        Self::safe_square_at(coordinates)
            .filter(|coord| self.is_bomb_at(*coord))
            .count() as u8
    }
}
