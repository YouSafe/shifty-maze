use ts_interop::ts_interop;

use crate::{
    board::{Board, NewBoardError, ShiftTileError},
    player::{MoveResult, PlayerId, Players, Position},
    tile::{FreeTile, Rotation, SideIndex},
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

#[derive(Debug)]
pub enum NewGameError {
    BoardError(NewBoardError),
    PlayerError,
}

pub enum GameError<T> {
    StateError,
    ActionError(T),
}

pub enum MovePlayerError {
    InvalidPosition,
    InvalidPlayer,
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
        })
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

    pub fn rotate_free_tile(&mut self, rotation: Rotation) -> FreeTile {
        self.board.rotate_free_tile(rotation)
    }

    pub fn shift_tiles(&mut self, side_index: SideIndex) -> Result<(), GameError<ShiftTileError>> {
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

    pub fn remove_player(&mut self, player_id: PlayerId) -> Result<Option<PlayerId>, ()> {
        self.players.remove_player(player_id)
    }

    pub fn move_player(
        &mut self,
        player_id: PlayerId,
        position: Position,
    ) -> Result<Option<PlayerId>, GameError<MovePlayerError>> {
        if self.phase != GamePhase::MovePlayer {
            return Err(GameError::StateError);
        }

        if self.board.get_tile(position).is_none() {
            return Err(MovePlayerError::InvalidPosition.into());
        }

        // TODO: check validity
        match self.players.move_player(player_id, position) {
            MoveResult::Moved(player) => player.try_collect_item(&self.board),
            MoveResult::Won(id) => return Ok(Some(id)),
            MoveResult::InvalidPlayer => return Err(MovePlayerError::InvalidPlayer.into()),
        }

        self.players.next_player_turn();
        self.phase = GamePhase::MoveTiles;
        Ok(None)
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
