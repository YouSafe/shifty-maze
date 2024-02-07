use std::collections::BTreeMap;

use rand::seq::SliceRandom;
use ts_interop::ts_interop;

use crate::{board::Board, tile::Item};

#[cfg_attr(feature = "wasm", tsify::declare)]
pub type PlayerId = usize;

#[ts_interop]
#[derive(Clone)]
pub struct Players {
    /// Stores the players in the order of their turn.
    players: BTreeMap<PlayerId, Player>,
    player_turn: PlayerId,
}

#[ts_interop]
#[derive(Clone)]
pub struct Player {
    id: PlayerId,
    position: Position,
    start_position: Position,
    collected: Vec<Item>,
    to_collect: Vec<Item>,
}

#[ts_interop]
#[derive(Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Players {
    pub fn new(mut ids: Vec<PlayerId>, items_per_player: usize, board: &Board) -> Self {
        assert!(ids.len() > 1);
        ids.sort();

        let player_turn = *ids.first().unwrap();
        let mut items =
            get_items_to_collect(board.get_number_of_items(), ids.len() * items_per_player);
        items.shuffle(&mut rand::thread_rng());

        let players = ids
            .into_iter()
            .enumerate()
            .map(|(index, id)| {
                (
                    id,
                    Player::new(
                        id,
                        get_start_position(index, board.get_side_length()),
                        items.drain(0..items_per_player).collect(),
                    ),
                )
            })
            .collect();

        assert!(items.is_empty());

        Self {
            players,
            player_turn,
        }
    }
}

impl Player {
    pub fn new(id: PlayerId, start_position: Position, to_collect: Vec<Item>) -> Self {
        Self {
            id,
            position: start_position,
            start_position,
            collected: Vec::new(),
            to_collect,
        }
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn get_start_position(index: usize, side_length: usize) -> Position {
    match index % 4 {
        0 => Position::new(0, 0),
        1 => Position::new(0, side_length - 1),
        2 => Position::new(side_length - 1, side_length - 1),
        _ => Position::new(side_length - 1, 0),
    }
}

fn get_items_to_collect(num_board_items: usize, num_item_cards: usize) -> Vec<Item> {
    let num_repeats = num_item_cards / num_board_items;
    let num_remainder = num_item_cards % num_board_items;
    let mut items: Vec<_> = (0..num_repeats)
        .map(|_| (1..=num_board_items))
        .flatten()
        .map(Item::new)
        .collect();
    items.extend((1..=num_remainder).map(Item::new));

    items
}
