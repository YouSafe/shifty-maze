use std::num::NonZeroU8;
use wasm_interop::wasm_interop;

#[derive(Clone)]
#[wasm_interop]
pub struct Tile {
    id: u32,
    variant: TileVariant,
    rotation: Rotation,
    item: Option<Item>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
pub enum TileVariant {
    LShape,
    TShape,
    IShape,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
pub enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[wasm_interop]
pub struct Item(#[cfg_attr(feature = "wasm", tsify(type = "number"))] NonZeroU8);

#[derive(Clone)]
#[wasm_interop]
pub struct FreeTile {
    tile: Tile,
    side_with_index: Option<(Side, usize)>,
}

/// The side of the board where the free tile is located.
/// The index always goes from left to right or from top to bottom.
#[derive(Clone)]
#[wasm_interop]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}
