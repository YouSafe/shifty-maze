mod result;

use game::{
    board::{NewBoardError, ShiftTileError},
    game::{Game, GameError, GameStartSettings, MovePlayerError, NewGameError},
    player::{PlayerId, Position},
    tile::{Rotation, SideIndex},
};
use std::collections::VecDeque;

use log::Level;
use wasm_bindgen::prelude::wasm_bindgen;

type ActionResult = result::Result<Game, String>;

#[wasm_bindgen]
pub struct GameCore {
    history: VecDeque<Game>,
}

impl GameCore {
    fn get_last_error() -> String {
        "Cannot complete action: Game not started".into()
    }

    fn get_last(&self) -> Result<&Game, String> {
        self.history.back().ok_or_else(Self::get_last_error)
    }

    fn get_last_mut(&mut self) -> Result<&mut Game, String> {
        self.history.back_mut().ok_or_else(Self::get_last_error)
    }

    fn do_action(
        &mut self,
        action: impl FnOnce(&mut Game) -> Result<(), String>,
    ) -> Result<Game, String> {
        self.get_last_mut()
            .map(|game| game.clone())
            .and_then(|mut game| {
                action(&mut game)?;
                if self.history.len() < self.history.capacity() {
                    self.history.push_back(game.clone());
                } else {
                    self.history.rotate_left(1);
                    *self.history.back_mut().unwrap() = game.clone();
                }
                Ok(game)
            })
    }

    fn do_action_result(
        &mut self,
        action: impl FnOnce(&mut Game) -> Result<(), String>,
    ) -> ActionResult {
        self.do_action(action).into()
    }
}

#[wasm_bindgen]
impl GameCore {
    #[wasm_bindgen(constructor)]
    pub fn new(history_size: usize) -> Self {
        Self {
            history: VecDeque::with_capacity(history_size),
        }
    }

    pub fn currently_reachable(&self) -> result::Result<Vec<Position>, String> {
        self.get_last()
            .and_then(|game| {
                game.currently_reachable()
                    .ok_or_else(|| "Cannot move player: Game has ended".into())
            })
            .into()
    }

    pub fn set_game(&mut self, game: Game) {
        self.history.clear();
        self.history.push_back(game);
    }

    pub fn start_game(&mut self, settings: GameStartSettings) -> ActionResult {
        Game::new(settings)
            .map_err(|err| {
                match err {
                    NewGameError::PlayerError => "Cannot start game: Not enough players",
                    NewGameError::BoardError(NewBoardError::TooSmall) => {
                        "Cannot start game: Side length too small"
                    }
                    NewGameError::BoardError(NewBoardError::EvenLength) => {
                        "Cannot start game: Invalid to side length"
                    }
                }
                .into()
            })
            .map(|game| {
                self.set_game(game.clone());
                game
            })
            .into()
    }

    pub fn rotate_free_tile(&mut self, rotation: Rotation) -> ActionResult {
        self.get_last_mut()
            .and_then(|game| {
                if game.rotate_free_tile(rotation) {
                    Ok(game.clone())
                } else {
                    Err("Cannot rotate tile: Game has ended".into())
                }
            })
            .into()
    }

    pub fn shift_tiles(&mut self, side_index: SideIndex) -> ActionResult {
        self.do_action_result(|game| {
            game.shift_tiles(side_index).map_err(|err| {
                match err {
                    GameError::GameOver => "Cannot shift tiles: Game has ended",
                    GameError::StateError => {
                        "Cannot shift tiles: Player has to end turn by moving figure"
                    }
                    GameError::ActionError(ShiftTileError::OutOfBounds) => {
                        "Cannot shift tiles: No such row/column exists"
                    }
                    GameError::ActionError(ShiftTileError::UnMovable) => {
                        "Cannot shift tiles: Specified row/column is not movable"
                    }
                    GameError::ActionError(ShiftTileError::UndoMove) => {
                        "Cannot shift tiles: Tile cannot be pushed back in where it was previously pushed out"
                    }
                }
                .into()
            })
        })
    }

    pub fn remove_player(&mut self, player_id: PlayerId) -> ActionResult {
        self.do_action_result(|game| {
            game.remove_player(player_id).map_err(|err| {
                match err {
                    GameError::GameOver => "Cannot remove player: Game has ended",
                    _ => "Cannot remove player: No such player exists",
                }
                .into()
            })
        })
    }

    pub fn move_player(&mut self, player_id: PlayerId, position: Position) -> ActionResult {
        self.do_action_result(|game| {
            game.move_player(player_id, position).map_err(|err| {
                match err {
                    GameError::GameOver => "Cannot move player: Game has ended",
                    GameError::StateError => "Cannot move player: Player has to shift tiles first",
                    GameError::ActionError(MovePlayerError::InvalidPosition) => {
                        "Cannot move player: Position is not on the board"
                    }
                    GameError::ActionError(MovePlayerError::InvalidPlayer) => {
                        "Cannot move player: No such player exists"
                    }
                    GameError::ActionError(MovePlayerError::UnreachablePosition) => {
                        "Cannot move player: Position is not reachable by player"
                    }
                }
                .into()
            })
        })
    }

    pub fn undo_move(&mut self) -> ActionResult {
        if self.history.len() > 1 {
            self.history.pop_back();
            Ok(self.history.back().cloned().unwrap())
        } else {
            Err("Cannot undo move: Last state in history".into())
        }
        .into()
    }
}

#[wasm_bindgen(start)]
fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(Level::Debug);
}
