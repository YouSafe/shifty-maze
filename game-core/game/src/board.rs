use crate::tile::{FreeTile, Tile};
use wasm_interop::wasm_interop;

#[wasm_interop]
pub struct Board {
    tiles: Vec<Tile>,
    side_length: usize,
    free_tile: FreeTile,
}

impl Board {
    pub fn new(number_of_items: u8, side_length: usize) -> Self {
        let tiles = Vec::with_capacity(side_length.pow(2));
        // TODO: Procedurally generate the board.
        let free_tile = todo!();
        Self {
            tiles,
            side_length,
            free_tile,
        }
    }
}
