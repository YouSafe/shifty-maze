use tsify::declare;
use wasm_interop::wasm_interop;

use crate::tile::Item;

#[cfg_attr(feature = "wasm", declare)]
pub type PlayerId = u8;

#[derive(Clone)]
#[wasm_interop]
pub struct Player {
    id: PlayerId,
    position: Position,
    start_position: Position,
    collected: Vec<Item>,
    to_collect: Vec<Item>,
}

#[derive(Clone)]
#[wasm_interop]
pub struct Position {
    x: usize,
    y: usize,
}
