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
            players: BTreeMap::new(),
            player_turn: 0,
        }
    }
}
