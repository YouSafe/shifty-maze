use std::num::NonZeroU8;
use wasm_interop::wasm_interop;

#[wasm_interop]
#[derive(Clone, Copy)]
pub struct Tile {
    id: u32,
    variant: TileVariant,
    rotation: Rotation,
    item: Option<Item>,
}

#[wasm_interop]
#[derive(Clone, Copy)]
pub enum TileVariant {
    LShape,
    TShape,
    IShape,
}

#[wasm_interop]
#[derive(Clone, Copy)]
pub enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

#[wasm_interop]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Item(pub NonZeroU8);

#[wasm_interop]
pub struct FreeTile {
    tile: Tile,
    side_with_index: Option<(Side, usize)>,
}

/// The side of the board where the free tile is located.
/// The index always goes from left to right or from top to bottom.
#[wasm_interop]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

impl Tile {
    pub fn new(id: usize, variant: TileVariant, rotation: Rotation, item: Option<Item>) -> Self {
        Self {
            id,
            variant,
            rotation,
            item,
        }
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
