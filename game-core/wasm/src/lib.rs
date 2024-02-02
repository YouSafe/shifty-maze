use game::{board::Board, game::Game, player::Player, tile::Item};
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

#[wasm_bindgen]
impl GameCore {
    #[wasm_bindgen(constructor)]
    pub fn new(number_of_items: u8, side_length: usize, callbacks: GameCoreCallbacks) -> Self {
        Self {
            game: Game::new(number_of_items, side_length),
            callbacks,
            serializer: serde_wasm_bindgen::Serializer::new(),
        }
    }

    pub fn get_board(&self) -> Board {
        self.game.get_board().clone()
    }

    pub fn get_players(&self) -> Result<JsValue, JsValue> {
        let players: Vec<_> = self.game.get_players().cloned().collect();
        Ok(players.serialize(&self.serializer)?)
    }

    /// Convince TSify to generate those types as well
    pub fn dummy_convince_tsify_0(&self) -> Item {
        panic!("This method should not be called");
    }
    pub fn dummy_convince_tsify_1(&self) -> Player {
        panic!("This method should not be called");
    }
}
