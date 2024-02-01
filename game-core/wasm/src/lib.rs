pub mod utils;

use game::game::Game;
use log::Level;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn main() {
    set_panic_hook();
    let _ = console_log::init_with_level(Level::Debug);
}

#[wasm_bindgen]
pub struct GameCore {
    game: Game,
}

#[wasm_bindgen]
impl GameCore {
    #[wasm_bindgen(constructor)]
    pub fn new(number_of_items: u8, side_length: usize) -> Self {
        Self {
            game: Game::new(number_of_items, side_length),
        }
    }
}
