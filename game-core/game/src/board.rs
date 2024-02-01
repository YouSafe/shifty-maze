use crate::tile::{FreeTile, Tile};

pub struct Board {
    pub tiles: Vec<Tile>,
    pub side_length: usize,
    pub free_tile: FreeTile,
}

impl Board {
    pub fn new(number_of_items: u8, side_length: usize) -> Self {
        let tiles = Vec::with_capacity(side_length * side_length);
        // TODO: Procedurally generate the board.
        let free_tile = FreeTile {
            tile: todo!(),
            side_with_index: None,
        };
        Self {
            tiles,
            side_length,
            free_tile,
        }
    }
}
