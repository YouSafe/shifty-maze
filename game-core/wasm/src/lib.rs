use std::collections::VecDeque;

use game::{
    board::{NewBoardError, ShiftTileError},
    game::{Game, GameError, GameStartSettings, MovePlayerError, NewGameError},
    player::{PlayerId, Position},
    tile::{Rotation, SideIndex},
};
use log::Level;
use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    pub type Observer;

    #[wasm_bindgen(method)]
    fn next(this: &Observer, value: JsValue);

    #[wasm_bindgen(method)]
    fn error(this: &Observer, value: JsValue);
}

#[wasm_bindgen(start)]
fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(Level::Debug);
}

#[wasm_bindgen]
pub struct Observable {
    result: Result<JsValue, JsValue>,
}

#[wasm_bindgen]
pub struct GameCore {
    history: VecDeque<Game>,
    serializer: Serializer,
}

impl From<Result<JsValue, JsValue>> for Observable {
    fn from(result: Result<JsValue, JsValue>) -> Self {
        Self { result }
    }
}

#[wasm_bindgen]
impl Observable {
    pub fn subscribe(self, observer: Observer) {
        match self.result {
            Ok(value) => observer.next(value),
            Err(err) => observer.error(err),
        };
    }
}

impl GameCore {
    fn serialize<T: Serialize>(&self, value: T) -> Result<JsValue, JsValue> {
        value.serialize(&self.serializer).map_err(Into::into)
    }

    fn do_action<T: Serialize>(
        &mut self,
        action: impl FnOnce(&mut Game) -> Result<T, JsValue>,
    ) -> Result<JsValue, JsValue> {
        if let Some(mut current) = self.history.back().cloned() {
            let result = action(&mut current)?;
            if self.history.len() < self.history.capacity() {
                self.history.push_back(current);
            } else {
                self.history.rotate_left(1);
                *self.history.back_mut().unwrap() = current;
            }
            self.serialize(result)
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
            serializer: Serializer::new(),
        }
    }

    pub fn get_current_game(&self) -> Observable {
        self.history
            .back()
            .map_or_else(
                || Err("Cannot get game: Game not started".into()),
                |game| self.serialize(game),
            )
            .into()
    }

    pub fn set_game(&mut self, game: Game) {
        self.history.clear();
        self.history.push_back(game);
    }

    pub fn start_game(&mut self, settings: GameStartSettings) -> Observable {
        match Game::new(settings) {
            Ok(game) => {
                self.set_game(game.clone());
                self.serialize(game)
            }
            Err(NewGameError::PlayerError) => Err("Cannot start game: Not enough players".into()),
            Err(NewGameError::BoardError(NewBoardError::TooSmall)) => {
                Err("Cannot start game: Side length too small".into())
            }
            Err(NewGameError::BoardError(NewBoardError::EvenLength)) => {
                Err("Cannot start game: Invalid to side length".into())
            }
        }
        .into()
    }

    pub fn rotate_free_tile(&mut self, rotation: Rotation) -> Observable {
        self.do_action(|game| Ok(game.rotate_free_tile(rotation)))
            .into()
    }

    pub fn shift_tiles(&mut self, side_index: SideIndex) -> Observable {
        self.do_action(|game| {
            game.shift_tiles(side_index)
                .map_err(|err| {
                    match err {
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
                .map(|_| game.clone())
        })
        .into()
    }

    pub fn remove_player(&mut self, player_id: PlayerId) -> Observable {
        self.do_action(|game| {
            game.remove_player(player_id)
                .map_err(|_| "Cannot remove player: No such player exists".into())
                .map(|winner| (game.get_players().clone(), winner))
        })
        .into()
    }

    pub fn move_player(&mut self, player_id: PlayerId, position: Position) -> Observable {
        self.do_action(|game| {
            game.move_player(player_id, position)
                .map_err(|err| {
                    match err {
                        GameError::StateError => {
                            "Cannot move player: Player has to shift tiles first"
                        }
                        GameError::ActionError(MovePlayerError::InvalidPosition) => {
                            "Cannot move player: Position is not on the board"
                        }
                        GameError::ActionError(MovePlayerError::InvalidPlayer) => {
                            "Cannot move player: No such player exists"
                        }
                    }
                    .into()
                })
                .map(|winner| (game.get_players().clone(), game.get_phase(), winner))
        })
        .into()
    }

    pub fn undo_move(&mut self) -> Observable {
        if self.history.len() > 1 {
            self.history.pop_back();
            self.serialize(self.history.back().unwrap().clone())
        } else {
            Err("Cannot undo move: Last state in history".into())
        }
        .into()
    }
}
