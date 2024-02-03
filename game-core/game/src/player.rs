use tsify::declare;
use wasm_interop::wasm_interop;

use crate::tile::Item;

#[cfg_attr(feature = "wasm", declare)]
pub type PlayerId = u8;

#[wasm_interop]
#[derive(Clone)]
pub struct Player {
    id: PlayerId,
    position: Position,
    start_position: Position,
    collected: Vec<Item>,
    to_collect: Vec<Item>,
}

impl Player {
    pub fn new(id: PlayerId, start_position: Position, to_collect: Vec<Item>) -> Self {
        Self {
            id,
            position: start_position,
            start_position,
            collected: Vec::new(),
            to_collect,
        }
    }

    pub fn set_to_collect(&mut self, to_collect: Vec<Item>) {
        self.to_collect = to_collect;
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn get_next_to_collect(&self) -> Option<Item> {
        self.to_collect.last().cloned()
    }

    pub fn try_collect_item(&mut self, item: Item) {
        if self.get_next_to_collect() == Some(item) {
            self.collected.push(self.to_collect.pop().unwrap());
        }
    }
}

#[wasm_interop]
#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
