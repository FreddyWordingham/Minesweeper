#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Bomb,
    BombNeighbor(u8),
    Empty,
}

impl Tile {
    #[inline]
    #[must_use]
    pub const fn is_bomb(&self) -> bool {
        matches!(*self, Self::Bomb)
    }
}
