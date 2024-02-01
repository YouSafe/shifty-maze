use std::num::NonZeroU8;

pub struct Tile {
    pub id: u32,
    pub variant: TileVariant,
    pub rotation: Rotation,
    pub item: Option<Item>,
}

pub enum TileVariant {
    LShape,
    TShape,
    IShape,
}

pub enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Item(pub NonZeroU8);

pub struct FreeTile {
    pub tile: Tile,
    pub side_with_index: Option<(Side, usize)>,
}

/// The side of the board where the free tile is located.
/// The index always goes from left to right or from top to bottom.
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}
