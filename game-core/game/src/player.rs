use std::collections::BTreeMap;

use ts_interop::ts_interop;

use crate::{board::Board, tile::Item};

pub type PlayerId = usize;

#[ts_interop]
pub struct Players {
    /// Stores the players in the order of their turn.
    players: BTreeMap<PlayerId, Player>,
}

#[ts_interop]
pub struct Player {
    id: PlayerId,
    position: Position,
    start_position: Position,
    collected: Vec<Item>,
    to_collect: Vec<Item>,
}

#[ts_interop]
#[derive(Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Players {
    pub fn new(ids: Vec<PlayerId>, items_per_player: usize, board: &Board) -> Self {
        todo!()
    }

    pub fn remove_player(&mut self, player_id: PlayerId) {
        todo!()
    }

    pub fn move_player(&mut self, player_id: PlayerId, position: Position) {
        todo!()
    }

    pub fn try_collect_item(&mut self, player_id: PlayerId, item: Item) -> Result<(), Item> {
        todo!()
    }
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

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn get_next_to_collect(&self) -> Option<Item> {
        self.to_collect.last().copied()
    }

    pub fn try_collect_item(&mut self, item: Item) -> Result<(), Item> {
        todo!()
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
