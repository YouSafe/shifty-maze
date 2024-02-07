use std::collections::BTreeMap;

use ts_interop::ts_interop;

use crate::tile::Item;

#[cfg_attr(feature = "wasm", tsify::declare)]
pub type PlayerId = usize;

#[ts_interop]
#[derive(Clone)]
pub struct Players {
    /// Stores the players in the order of their turn.
    players: BTreeMap<PlayerId, Player>,
    player_turn: PlayerId,
}

#[ts_interop]
#[derive(Clone)]
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
