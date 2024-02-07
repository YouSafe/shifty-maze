use std::num::NonZeroUsize;

use ts_interop::ts_interop;

#[ts_interop]
#[derive(Clone, Copy)]
pub struct Tile {
    id: usize,
    variant: TileVariant,
    rotation: Rotation,
    item: Option<Item>,
}

#[ts_interop]
#[derive(Clone, Copy)]
pub enum TileVariant {
    /// 0°   is └
    /// 90°  is ┌
    /// 180° is ┐
    /// 270° is ┘
    LShape,
    /// 0°   is ┬
    /// 90°  is ┤
    /// 180° is ┴
    /// 270° is ├
    TShape,
    /// 0°   is │
    /// 90°  is ─
    /// 180° is │
    /// 270° is ─
    IShape,
}

#[ts_interop]
#[derive(Clone, Copy)]
pub enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

#[ts_interop]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Item(#[cfg_attr(feature = "wasm", tsify(type = "number"))] NonZeroUsize);

#[ts_interop]
#[derive(Clone, Copy)]
pub struct FreeTile {
    tile: Tile,
    side_with_index: Option<SideIndex>,
}

/// The index always goes from left to right or from top to bottom.
#[ts_interop]
#[derive(Clone, Copy)]
pub struct SideIndex {
    side: Side,
    index: usize,
}

/// The side of the board where the free tile is located.
#[ts_interop]
#[derive(Clone, Copy)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}
