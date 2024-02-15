use ts_interop::ts_interop;

use crate::{
    board::{Board, NewBoardError, ShiftTileError},
    player::{MoveResult, PlayerId, Players, Position},
    tile::{Rotation, SideIndex},
};

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
    players: Vec<PlayerId>,
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
            winner: None,
        })
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
    ) -> ActionResult<MovePlayerError> {
        if self.winner.is_some() {
            return Err(GameError::GameOver);
        }

        if self.phase != GamePhase::MovePlayer {
            return Err(GameError::StateError);
        }

        if self.board.get_tile(position).is_none() {
            return Err(MovePlayerError::InvalidPosition.into());
        }

        // TODO: check validity
        match self.players.move_player(player_id, position) {
            MoveResult::Moved(player) => {
                player.try_collect_item(&self.board);
                self.players.next_player_turn();
            }
            MoveResult::Won(id) => self.winner = Some(id),
            MoveResult::InvalidPlayer => return Err(MovePlayerError::InvalidPlayer.into()),
        }

        self.phase = GamePhase::MoveTiles;
        Ok(())
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
