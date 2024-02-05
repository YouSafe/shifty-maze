use ts_interop::ts_interop;

use crate::{
    board::Board,
    player::{PlayerId, Players, Position},
    tile::{Rotation, SideIndex},
};

pub struct Game {
    board: Board,
    players: Players,
    player_turn: PlayerId,
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

impl Game {
    pub fn new(settings: GameStartSettings) -> Self {
        todo!()
    }

    pub fn rotate_free_tile(&mut self, rotation: Rotation) {
        todo!()
    }

    pub fn shift_tiles(&mut self, side_index: SideIndex) {
        todo!()
    }

    pub fn remove_player(&mut self, player_id: PlayerId) {
        todo!()
    }

    pub fn move_player(&mut self, player_id: PlayerId, position: Position) {
        todo!()
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_player_turn(&self) -> PlayerId {
        self.player_turn
    }

    pub fn get_phase(&self) -> GamePhase {
        self.phase
    }
}
