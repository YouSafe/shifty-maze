use game::{
    board::{NewBoardError, ShiftTileError},
    game::{Game, GameError, GameStartSettings, MovePlayerError, NewGameError},
    player::{PlayerId, Position},
    tile::{Rotation, SideIndex},
};
use std::collections::VecDeque;

use log::Level;
use wasm_bindgen::prelude::wasm_bindgen;

type ActionResult = Result<Game, String>;

#[wasm_bindgen]
extern "C" {
    pub type Observer;

    #[wasm_bindgen(method)]
    fn next(this: &Observer, game: Game);

    #[wasm_bindgen(method)]
    fn error(this: &Observer, error: String);
}

#[wasm_bindgen]
pub struct GameCore {
    history: VecDeque<Game>,
}

#[wasm_bindgen]
pub struct Observable {
    result: ActionResult,
}

impl From<Result<Game, String>> for Observable {
    fn from(result: Result<Game, String>) -> Self {
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

    fn do_action_observable(&mut self, action: impl FnOnce(&mut Game) -> Result<(), String>) -> Observable {
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

    pub fn set_game(&mut self, game: Game) {
        self.history.clear();
        self.history.push_back(game);
    }

    pub fn start_game(&mut self, settings: GameStartSettings) -> Observable {
        Game::new(settings)
            .map_err(|err| {
                match err {
                    NewGameError::PlayerError => "Cannot start game: Not enough players",
                    NewGameError::BoardError(NewBoardError::TooSmall) => "Cannot start game: Side length too small",
                    NewGameError::BoardError(NewBoardError::EvenLength) => "Cannot start game: Invalid to side length",
                }
                .into()
            })
            .map(|game| {
                self.set_game(game.clone());
                game
            })
            .into()
    }

    pub fn rotate_free_tile(&mut self, rotation: Rotation) -> Observable {
        self.do_action_observable(|game| {
            if game.rotate_free_tile(rotation) {
                Ok(())
            } else {
                Err("Cannot rotate tile: Game has ended".into())
            }
        })
    }

    pub fn shift_tiles(&mut self, side_index: SideIndex) -> Observable {
        use GameError as G;
        use ShiftTileError as S;
        self.do_action_observable(|game| {
            game.shift_tiles(side_index).map_err(|err| {
                match err {
                    G::GameOver => "Cannot shift tiles: Game has ended",
                    G::StateError => "Cannot shift tiles: Player has to end turn by moving figure",
                    G::ActionError(S::OutOfBounds) => "Cannot shift tiles: No such row/column exists",
                    G::ActionError(S::UnMovable) => "Cannot shift tiles: Specified row/column is not movable",
                    G::ActionError(S::UndoMove) => "Cannot shift tiles: Tile cannot be pushed back in where it was previously pushed out",
                }
                .into()
            })
        })
    }

    pub fn remove_player(&mut self, player_id: PlayerId) -> Observable {
        self.do_action_observable(|game| {
            game.remove_player(player_id).map_err(|err| {
                match err {
                    GameError::GameOver => "Cannot remove player: Game has ended",
                    _ => "Cannot remove player: No such player exists",
                }
                .into()
            })
        })
    }

    pub fn move_player(&mut self, player_id: PlayerId, position: Position) -> Observable {
        use GameError as G;
        use MovePlayerError as M;
        self.do_action_observable(|game| {
            game.move_player(player_id, position).map_err(|err| {
                match err {
                    G::GameOver => "Cannot move player: Game has ended",
                    G::StateError => "Cannot move player: Player has to shift tiles first",
                    G::ActionError(M::InvalidPosition) => "Cannot move player: Position is not on the board",
                    G::ActionError(M::InvalidPlayer) => "Cannot move player: No such player exists",
                    G::ActionError(M::UnreachablePosition) => "Cannot move player: Position is not reachable by player",
                }
                .into()
            })
        })
    }

    pub fn undo_move(&mut self) -> Observable {
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
