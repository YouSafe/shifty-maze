use game::{
    board::Board,
    game::{Game, GamePhase, GameStartSettings},
    player::{PlayerId, Players, Position},
    tile::{Rotation, SideIndex},
};
use log::Level;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    pub type GameCoreCallbacks;

    #[wasm_bindgen(method)]
    fn update_board(this: &GameCoreCallbacks, board: Board);

    #[wasm_bindgen(method)]
    fn update_players(this: &GameCoreCallbacks, players: Players);

    #[wasm_bindgen(method)]
    fn update_phase(this: &GameCoreCallbacks, phase: GamePhase);
}

#[wasm_bindgen(start)]
fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(Level::Debug);
}

#[wasm_bindgen]
pub struct GameCore {
    history: Vec<Game>,
    callbacks: GameCoreCallbacks,
}

#[wasm_bindgen]
impl GameCore {
    #[wasm_bindgen(constructor)]
    pub fn new(callbacks: GameCoreCallbacks, history_size: usize) -> Self {
        todo!()
    }

    pub fn get_current_game(&self) -> Option<Game> {
        todo!()
    }

    pub fn set_game(&mut self, game: Game) {
        todo!()
    }

    pub fn start_game(&mut self, settings: GameStartSettings) {
        todo!()
    }

    pub fn rotate_free_tile(&mut self, rotation: Rotation) {
        todo!()
    }

    pub fn shift_tiles(&mut self, side_index: SideIndex) {
        todo!()
    }

    pub fn remove_player(&mut self, player_id: PlayerId) {
        todo!()
    }

    pub fn move_player(&mut self, player_id: PlayerId, position: Position) {
        todo!()
    }

    pub fn undo_move(&mut self) {
        todo!()
    }
}
