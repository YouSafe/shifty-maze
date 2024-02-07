use ts_interop::ts_interop;

use crate::{
    board::Board,
    player::{PlayerId, Players},
};

#[ts_interop]
#[derive(Clone)]
pub struct Game {
    board: Board,
    players: Players,
    phase: GamePhase,
}

#[ts_interop]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GamePhase {
    MoveTiles,
    MovePlayer,
}

#[ts_interop]
pub struct GameStartSettings {
    players: Vec<PlayerId>,
    side_length: usize,
    items_per_player: usize,
}
