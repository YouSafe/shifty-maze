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

impl GameCore {
    fn update_board(&self, board: Board) {
        self.callbacks.update_board(board);
    }

    fn update_players(&self, players: Players) {
        self.callbacks.update_players(players)
    }

    fn update_phase(&self, phase: GamePhase) {
        self.callbacks.update_phase(phase);
    }

    fn do_action(&mut self, action: impl FnOnce(&mut Game)) -> Option<Game> {
        if let Some(mut current) = self.history.last().cloned() {
            action(&mut current);
            if self.history.len() < self.history.capacity() {
                self.history.push(current);
            } else {
                self.history.rotate_left(1);
                *self.history.last_mut().unwrap() = current;
            }
            self.history.last().cloned()
        } else {
            None
        }
    }
}

#[wasm_bindgen]
impl GameCore {
    #[wasm_bindgen(constructor)]
    pub fn new(callbacks: GameCoreCallbacks, history_size: usize) -> Self {
        Self {
            history: Vec::with_capacity(history_size),
            callbacks,
        }
    }

    pub fn get_current_game(&self) -> Option<Game> {
        self.history.last().cloned()
    }

    pub fn set_game(&mut self, game: Game) {
        self.history.clear();
        self.history.push(game);
    }

    pub fn start_game(&mut self, settings: GameStartSettings) {
        let game = Game::new(settings);
        let (board, players, phase) = game.clone().into_parts();
        self.update_board(board);
        self.update_players(players);
        self.update_phase(phase);
        self.set_game(game);
    }

    pub fn rotate_free_tile(&mut self, rotation: Rotation) {
        if let Some(game) = self.do_action(|game| game.rotate_free_tile(rotation)) {
            self.update_board(game.into_parts().0);
        }
    }

    pub fn shift_tiles(&mut self, side_index: SideIndex) {
        if let Some(game) = self.do_action(|game| game.shift_tiles(side_index)) {
            let (board, players, phase) = game.into_parts();
            self.update_board(board);
            self.update_players(players);
            self.update_phase(phase);
        }
    }

    pub fn remove_player(&mut self, player_id: PlayerId) {
        if let Some(game) = self.do_action(|game| game.remove_player(player_id)) {
            self.update_players(game.into_parts().1);
        }
    }

    pub fn move_player(&mut self, player_id: PlayerId, position: Position) -> Option<PlayerId> {
        let mut winner = None;
        if let Some(game) = self.do_action(|game| winner = game.move_player(player_id, position)) {
            let (_, players, phase) = game.into_parts();
            self.update_players(players);
            self.update_phase(phase);
        }
        winner
    }

    pub fn undo_move(&mut self) {
        self.history.pop();
        if let Some(game) = self.history.last() {
            self.update_board(game.get_board().clone());
            self.update_players(game.get_players().clone());
            self.update_phase(game.get_phase());
        }
    }
}
