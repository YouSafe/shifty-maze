use std::collections::VecDeque;

use game::{
    board::{NewBoardError, ShiftTileError},
    game::{Game, GameError, GameStartSettings, MovePlayerError, NewGameError},
    player::{PlayerId, Position},
    tile::{Rotation, SideIndex},
};
use log::Level;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(Level::Debug);
}

#[wasm_bindgen]
pub struct GameCore {
    history: VecDeque<Game>,
}

pub type ActionResult = Result<Game, String>;

impl GameCore {
    fn do_action(&mut self, action: impl FnOnce(&mut Game) -> Result<(), String>) -> ActionResult {
        if let Some(mut current) = self.history.back().cloned() {
            action(&mut current)?;
            if self.history.len() < self.history.capacity() {
                self.history.push_back(current.clone());
            } else {
                self.history.rotate_left(1);
                *self.history.back_mut().unwrap() = current.clone();
            }
            Ok(current)
        } else {
            Err("Cannot complete action: Game not started".into())
        }
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
    }

    pub fn rotate_free_tile(&mut self, rotation: Rotation) -> ActionResult {
        self.do_action(|game| {
            if game.rotate_free_tile(rotation) {
                Ok(())
            } else {
                Err("Cannot rotate tile: Game has ended".into())
            }
        })
    }

    pub fn shift_tiles(&mut self, side_index: SideIndex) -> ActionResult {
        self.do_action(|game| {
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
        self.do_action(|game| {
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
        self.do_action(|game| {
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
    }
}
