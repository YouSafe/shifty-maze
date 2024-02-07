use ts_interop::ts_interop;

use crate::tile::{FreeTile, Tile};

#[ts_interop]
#[derive(Clone)]
pub struct Board {
    tiles: Vec<Tile>,
    side_length: usize,
    free_tile: FreeTile,
}
