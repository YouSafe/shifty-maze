use std::collections::BTreeMap;

use crate::{
    board::Board,
    player::{Player, PlayerId},
};

pub struct Game {
    board: Board,
    number_of_items: u8,
    /// Stores the players in the order of their turn.
    players: BTreeMap<PlayerId, Player>,
    player_turn: PlayerId,
}

impl Game {
    pub fn new(number_of_items: u8, side_length: usize) -> Self {
        Self {
            board: Board::new(number_of_items, side_length),
            number_of_items,
            players: Default::default(),
            player_turn: Default::default(),
        }
    }
    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_players(&self) -> impl Iterator<Item = &Player> {
        self.players.values()
    }

    pub fn get_player_turn(&self) -> PlayerId {
        self.player_turn
    }
}
