use std::num::NonZeroU8;

use ts_interop::ts_interop;

#[ts_interop]
pub struct Tile {
    id: u32,
    variant: TileVariant,
    rotation: Rotation,
    item: Option<Item>,
}

#[ts_interop]
pub enum TileVariant {
    LShape,
    TShape,
    IShape,
}

#[ts_interop]
pub enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

#[ts_interop]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Item(NonZeroU8);

#[ts_interop]
pub struct FreeTile {
    tile: Tile,
    side_with_index: Option<SideIndex>,
}

/// The side of the board where the free tile is located.
/// The index always goes from left to right or from top to bottom.
#[ts_interop]
pub enum SideIndex {
    Top(usize),
    Right(usize),
    Bottom(usize),
    Left(usize),
}

impl Tile {
    pub fn new(id: u32, variant: TileVariant, rotation: Rotation, item: Option<Item>) -> Self {
        Self {
            id,
            variant,
            rotation,
            item,
        }
    }

    pub fn get_item(&self) -> Option<Item> {
        self.item
    }
}

impl FreeTile {
    pub fn new(tile: Tile) -> Self {
        Self {
            tile,
            side_with_index: None,
        }
    }
}
