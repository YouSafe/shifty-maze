use std::collections::BTreeMap;

use rand::{rngs::ThreadRng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use wasm_interop::wasm_interop;

use crate::{
    board::Board,
    player::{Player, PlayerId, Position},
    tile::{Item, Rotation, Side},
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Game {
    board: Board,
    players: Players,
    phase: GamePhase,
}

#[wasm_interop]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GamePhase {
    TurnMoveTiles,
    TurnMovePlayer,
}

const BINARY_GAME_VERSION: u8 = 1;

#[wasm_interop]
#[derive(Clone)]
pub struct BinaryGame {
    version: u8,
    bytes: Vec<u8>,
}

#[wasm_interop]
#[derive(Clone)]
pub struct GameStartSettings {
    number_of_items: u8,
    items_per_player: u8,
    side_length: usize,
    players: Vec<PlayerId>,
}

impl Game {
    pub fn new(settings: GameStartSettings) -> Self {
        Self {
            board: Board::new(settings.number_of_items, settings.side_length),
            players: Players::new(&settings),
            phase: GamePhase::TurnMoveTiles,
        }
    }

    pub fn shift_tiles(&mut self, side: Side, index: usize, insert_rotation: Rotation) {
        self.board.shift_tiles(side, index, insert_rotation);
        self.phase = GamePhase::TurnMovePlayer;
    }

    pub fn add_player(&mut self, player_id: PlayerId) {
        self.players.add_player(
            player_id,
            self.board.get_number_of_items(),
            self.board.get_side_length(),
        );
    }

    pub fn remove_player(&mut self, player_id: PlayerId) {
        self.players.remove(player_id);
    }

    pub fn move_player(&mut self, player_id: PlayerId, position: Position) {
        self.players.move_player(player_id, position);
        if let Some(item) = self.board.get_item(position) {
            self.players.try_collect_item(player_id, item);
        }
        self.phase = GamePhase::TurnMoveTiles;
    }

    pub fn get_game_bytes(&self) -> BinaryGame {
        BinaryGame {
            version: BINARY_GAME_VERSION,
            bytes: bincode::serialize(self).unwrap(),
        }
    }

    pub fn load_game_from_bytes(bytes: BinaryGame) -> Result<Self, String> {
        if bytes.version != BINARY_GAME_VERSION {
            return Err(format!(
                "Unsupported game version: expected {}, got {}",
                BINARY_GAME_VERSION, bytes.version
            ));
        }
        bincode::deserialize(&bytes.bytes).map_err(|err| err.to_string())
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_players(&self) -> impl Iterator<Item = &Player> {
        self.players.get_players()
    }

    pub fn get_player_turn(&self) -> PlayerId {
        self.players.get_player_turn()
    }

    pub fn get_phase(&self) -> GamePhase {
        self.phase
    }
}

fn generate_items_to_collect(
    number_of_items: u8,
    items_per_player: u8,
    rng: &mut ThreadRng,
) -> Vec<Item> {
    let mut items = (0..number_of_items).map(Item).collect::<Vec<_>>();
    items.shuffle(rng);
    items.truncate(items_per_player as usize);
    items
}

fn get_start_position(player_id: PlayerId, side_length: usize) -> Position {
    match player_id % 4 {
        0 => Position::new(0, 0),
        1 => Position::new(0, side_length - 1),
        2 => Position::new(side_length - 1, side_length - 1),
        3 => Position::new(side_length - 1, 0),
        _ => unreachable!(),
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Players {
    /// Stores the players in the order of their turn.
    players: BTreeMap<PlayerId, Player>,
    player_turn: PlayerId,
    items_per_player: u8,
}

impl Players {
    pub fn new(settings: &GameStartSettings) -> Self {
        let mut rng = rand::thread_rng();

        let players = settings
            .players
            .iter()
            .copied()
            .map(|id| {
                (
                    id,
                    Player::new(
                        id,
                        get_start_position(id, settings.side_length),
                        generate_items_to_collect(
                            settings.number_of_items,
                            settings.items_per_player,
                            &mut rng,
                        ),
                    ),
                )
            })
            .collect();
        let mut result = Self {
            players,
            player_turn: Default::default(),
            items_per_player: settings.items_per_player,
        };
        result.update_player_turn();
        result
    }

    pub fn get_players(&self) -> impl Iterator<Item = &Player> {
        self.players.values()
    }

    pub fn get_player_turn(&self) -> PlayerId {
        self.player_turn
    }

    pub fn add_player(&mut self, player_id: PlayerId, number_of_items: u8, side_length: usize) {
        let mut rng = rand::thread_rng();

        self.players.insert(
            player_id,
            Player::new(
                player_id,
                get_start_position(player_id, side_length),
                generate_items_to_collect(number_of_items, self.items_per_player, &mut rng),
            ),
        );

        self.update_player_turn();
    }

    pub fn remove(&mut self, player_id: PlayerId) {
        self.players.remove(&player_id);
        self.update_player_turn();
    }

    pub fn update_player_turn(&mut self) {
        if self.players.is_empty() {
            self.player_turn = Default::default();
        } else {
            let next_player = self
                .players
                .range(self.player_turn..)
                .next()
                .or_else(|| self.players.iter().next())
                .map(|(id, _)| *id)
                .unwrap();
            self.player_turn = next_player;
        }
    }

    pub fn move_player(&mut self, player_id: u8, position: Position) {
        if let Some(player) = self.players.get_mut(&player_id) {
            player.set_position(position);
        }
    }

    pub fn try_collect_item(&mut self, player_id: u8, item: Item) {
        if let Some(player) = self.players.get_mut(&player_id) {
            player.try_collect_item(item);
        }
    }
}
