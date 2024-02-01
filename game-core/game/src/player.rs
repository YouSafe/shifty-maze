use crate::tile::Item;
use wasm_interop::wasm_interop;

pub type PlayerId = u8;

#[wasm_interop]
pub struct Player {
    id: PlayerId,
    position: Position,
    start_position: Position,
    collected: Vec<Item>,
    to_collect: Vec<Item>,
}

#[wasm_interop]
struct Position {
    x: usize,
    y: usize,
}
