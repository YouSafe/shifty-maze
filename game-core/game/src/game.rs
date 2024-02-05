use ts_interop::ts_interop;

use crate::{
    board::Board,
    player::{MoveResult, PlayerId, Players, Position},
    tile::{Rotation, SideIndex},
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

impl Game {
    pub fn new(settings: GameStartSettings) -> Self {
        let board = Board::new(settings.side_length);
        let players = Players::new(settings.players, settings.items_per_player, &board);

        Self {
            board,
            players,
            phase: GamePhase::MoveTiles,
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_players(&self) -> &Players {
        &self.players
    }

    pub fn get_phase(&self) -> GamePhase {
        self.phase
    }

    pub fn rotate_free_tile(&mut self, rotation: Rotation) {
        self.board.rotate_free_tile(rotation);
    }

    pub fn shift_tiles(&mut self, side_index: SideIndex) {
        assert!(self.phase == GamePhase::MoveTiles);
        self.board.shift_tiles(side_index);
        self.phase = GamePhase::MovePlayer;
    }

    pub fn remove_player(&mut self, player_id: PlayerId) {
        self.players.remove_player(player_id);
    }

    pub fn move_player(&mut self, player_id: PlayerId, position: Position) -> Option<PlayerId> {
        assert!(self.phase == GamePhase::MovePlayer);

        // TODO: check validity
        match self.players.move_player(player_id, position) {
            MoveResult::Won(id) => return Some(id),
            MoveResult::Moved(player) => player.try_collect_item(&self.board),
        }

        self.players.next_player_turn();
        self.phase = GamePhase::MoveTiles;
        None
    }

    pub fn into_parts(self) -> (Board, Players, GamePhase) {
        (self.board, self.players, self.phase)
    }
}

impl GameStartSettings {
    pub fn new(players: Vec<PlayerId>, side_length: usize, items_per_player: usize) -> Self {
        Self {
            players,
            side_length,
            items_per_player,
        }
    }

    pub fn get_phase(&self) -> GamePhase {
        self.phase
    }
}
