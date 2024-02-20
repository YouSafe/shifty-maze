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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SideIndex {
    side: Side,
    index: usize,
}

/// The side of the board where the free tile is located.
#[ts_interop]
#[derive(Clone, Copy, PartialEq, Eq)]
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

    pub fn get_connection(&self) -> Vec<Side> {
        use Rotation as R;
        use Side::*;
        use TileVariant as V;

        match (self.variant, self.rotation) {
            (V::LShape, R::Zero) => vec![Top, Right],
            (V::LShape, R::Ninety) => vec![Bottom, Right],
            (V::LShape, R::OneEighty) => vec![Bottom, Left],
            (V::LShape, R::TwoSeventy) => vec![Top, Left],
            (V::TShape, R::Zero) => vec![Bottom, Left, Right],
            (V::TShape, R::Ninety) => vec![Top, Bottom, Left],
            (V::TShape, R::OneEighty) => vec![Top, Left, Right],
            (V::TShape, R::TwoSeventy) => vec![Top, Bottom, Right],
            (V::IShape, R::Zero | R::OneEighty) => vec![Top, Bottom],
            (V::IShape, R::Ninety | R::TwoSeventy) => vec![Left, Right],
        }
    }

    pub fn get_item(&self) -> Option<Item> {
        self.item
    }
}

impl Item {
    pub fn new(id: usize) -> Self {
        assert!(id != 0);
        // Safety: id != 0
        Self(unsafe { NonZeroUsize::new_unchecked(id) })
    }
}

impl FreeTile {
    pub fn new(tile: Tile) -> Self {
        Self {
            tile,
            side_with_index: None,
        }
    }

    pub fn get_side_index(&self) -> Option<SideIndex> {
        self.side_with_index
    }

    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.tile.rotation = rotation;
    }

    pub fn tile_mut(&mut self) -> &mut Tile {
        &mut self.tile
    }

    pub fn set_side_index(&mut self, side_index: SideIndex) {
        self.side_with_index = Some(side_index);
    }
}

impl SideIndex {
    pub fn new(side: Side, index: usize) -> Self {
        Self { side, index }
    }

    pub fn get_side(&self) -> Side {
        self.side
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn shift(&self) -> Self {
        let side = match self.side {
            Side::Top => Side::Bottom,
            Side::Right => Side::Left,
            Side::Bottom => Side::Top,
            Side::Left => Side::Right,
        };

        Self {
            side,
            index: self.index,
        }
    }
}
