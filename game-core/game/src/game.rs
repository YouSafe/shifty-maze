use std::collections::BTreeSet;

use crate::{
    board::{Board, NewBoardError, ShiftTileError},
    player::{MoveError, PlayerId, Players, Position},
    tile::{Rotation, SideIndex},
};
use ts_interop::ts_interop;

#[ts_interop]
#[derive(Clone)]
pub struct Game {
    board: Board,
    players: Players,
    phase: GamePhase,
    winner: Option<PlayerId>,
}

#[ts_interop]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GamePhase {
    MoveTiles,
    MovePlayer,
}

#[ts_interop]
pub struct GameStartSettings {
    players: BTreeSet<PlayerId>,
    side_length: usize,
    items_per_player: usize,
}

#[derive(Debug)]
pub enum NewGameError {
    BoardError(NewBoardError),
    PlayerError,
}

pub type ActionResult<E> = Result<(), GameError<E>>;

pub enum GameError<T> {
    GameOver,
    StateError,
    ActionError(T),
}

impl Game {
    pub fn new(settings: GameStartSettings) -> Result<Self, NewGameError> {
        let board = Board::new(settings.side_length)?;
        let players = Players::new(settings.players, settings.items_per_player, &board)
            .ok_or(NewGameError::PlayerError)?;

        Ok(Self {
            board,
            players,
            phase: GamePhase::MoveTiles,
            winner: None,
        })
    }

    pub fn currently_reachable(&self) -> Option<Vec<Position>> {
        if self.winner.is_some() {
            return None;
        }

        Some(
            self.board
                .get_reachable(self.players.current_player().get_position()),
        )
    }

    pub fn rotate_free_tile(&mut self, rotation: Rotation) -> bool {
        if self.winner.is_some() {
            false
        } else {
            self.board.rotate_free_tile(rotation);
            true
        }
    }

    pub fn shift_tiles(&mut self, side_index: SideIndex) -> ActionResult<ShiftTileError> {
        if self.winner.is_some() {
            return Err(GameError::GameOver);
        }

        if self.phase != GamePhase::MoveTiles {
            return Err(GameError::StateError);
        }

        let changes = self.board.shift_tiles(side_index)?;
        for player in self.players.iter_mut() {
            if let Some(new_pos) = changes.get(&player.get_position()) {
                player.set_position(*new_pos);
            }
        }
        self.phase = GamePhase::MovePlayer;

        Ok(())
    }

    pub fn remove_player(&mut self, player_id: PlayerId) -> ActionResult<()> {
        if self.winner.is_some() {
            return Err(GameError::GameOver);
        }

        self.winner = self.players.remove_player(player_id)?;

        Ok(())
    }

    pub fn move_player(
        &mut self,
        player_id: PlayerId,
        position: Position,
    ) -> Result<Vec<Position>, GameError<MoveError>> {
        if self.winner.is_some() {
            return Err(GameError::GameOver);
        }

        if self.phase != GamePhase::MovePlayer {
            return Err(GameError::StateError);
        }

        let result = self.players.move_player(player_id, position, &self.board)?;
        self.phase = GamePhase::MoveTiles;
        self.winner = result.winner;

        Ok(result.path)
    }
}

impl GameStartSettings {
    pub fn new(players: BTreeSet<PlayerId>, side_length: usize, items_per_player: usize) -> Self {
        Self {
            players,
            side_length,
            items_per_player,
        }
    }
}

impl From<NewBoardError> for NewGameError {
    fn from(value: NewBoardError) -> Self {
        NewGameError::BoardError(value)
    }
}

impl<E> From<E> for GameError<E> {
    fn from(value: E) -> Self {
        GameError::ActionError(value)
    }
}
