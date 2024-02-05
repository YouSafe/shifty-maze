use crate::tile::{FreeTile, Tile};
use ts_interop::ts_interop;

#[derive(Clone)]
#[ts_interop]
pub struct Board {
    tiles: Vec<Tile>,
    side_length: usize,
    free_tile: FreeTile,
}

impl Board {
    pub fn new(side_length: usize) -> Self {
        todo!()
    }

    pub fn get_side_length(&self) -> usize {
        todo!()
    }

    pub fn get_number_of_items(&self) -> usize {
        todo!()
    }

    pub fn get_item(&self, position: Position) -> Option<Item> {
        todo!()
    }

    pub fn rotate_free_tile(&mut self, rotation: Rotation) {
        todo!()
    }

    pub fn shift_tiles(&mut self, side_index: SideIndex) -> HashMap<Position, Position> {
        todo!()
    }
}
