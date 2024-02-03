use game::{
    board::Board,
    game::{BinaryGame, Game, GameStartSettings},
    player::{Player, PlayerId, Position},
    tile::{Item, Rotation, Side},
};
use log::Level;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type GameCoreCallbacks;

    #[wasm_bindgen(method)]
    fn update_board(this: &GameCoreCallbacks, board: Board);

    #[wasm_bindgen(method)]
    fn update_players(this: &GameCoreCallbacks, players: JsValue);

    #[wasm_bindgen(method)]
    fn update_player_turn(this: &GameCoreCallbacks, player_id: PlayerId);
}

#[wasm_bindgen(start)]
fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(Level::Debug);
}

#[wasm_bindgen]
pub struct GameCore {
    game: Game,
    callbacks: GameCoreCallbacks,
    serializer: serde_wasm_bindgen::Serializer,
}

impl GameCore {
    fn update_board(&self) {
        self.callbacks.update_board(self.game.get_board().clone());
    }
    fn update_players(&self) {
        let players: Vec<_> = self.game.get_players().cloned().collect();
        let serialized = players.serialize(&self.serializer);
        match serialized {
            Ok(serialized) => self.callbacks.update_players(serialized),
            Err(err) => log::error!("Failed to serialize players: {}", err),
        }
    }
    fn update_player_turn(&self) {
        self.callbacks
            .update_player_turn(self.game.get_player_turn());
    }
}

#[wasm_bindgen]
impl GameCore {
    #[wasm_bindgen(constructor)]
    pub fn new(settings: GameStartSettings, callbacks: GameCoreCallbacks) -> Self {
        Self {
            game: Game::new(settings),
            callbacks,
            serializer: serde_wasm_bindgen::Serializer::new(),
        }
    }

    pub fn update_game_settings(&mut self, settings: GameStartSettings) {
        self.game.update_settings(settings);
        self.update_board();
        self.update_players();
    }

    pub fn shift_tiles(&mut self, side: Side, index: usize, insert_rotation: Rotation) {
        self.game.shift_tiles(side, index, insert_rotation);
        self.update_board();
        self.update_players();
    }

    pub fn add_player(&mut self, player_id: PlayerId, position: Position) {
        self.game.add_player(player_id, position);
        self.update_players();
        self.update_player_turn();
    }

    pub fn remove_player(&mut self, player_id: PlayerId) {
        self.game.remove_player(player_id);
        self.update_players();
        self.update_player_turn();
    }

    pub fn move_player(&mut self, player_id: PlayerId, position: Position) {
        self.game.move_player(player_id, position);
        self.update_players();
        self.update_player_turn();
    }

    pub fn get_game_bytes(&self) -> BinaryGame {
        self.game.get_game_bytes()
    }

    pub fn set_game_bytes(&mut self, game_bytes: BinaryGame) {
        self.game = match Game::load_game_from_bytes(game_bytes) {
            Ok(game) => game,
            Err(err) => {
                log::error!("Failed to load game from bytes: {}", err);
                return;
            }
        };
        self.update_board();
        self.update_players();
        self.update_player_turn();
    }

    /// Convince TSify to generate those types as well

    pub fn dummy_convince_tsify_0(&self) -> Item {
        panic!("This method should not be called");
    }
    pub fn dummy_convince_tsify_1(&self) -> Player {
        panic!("This method should not be called");
    }
    pub fn dummy_convince_tsify_2(&self) -> Board {
        panic!("This method should not be called");
    }
}
