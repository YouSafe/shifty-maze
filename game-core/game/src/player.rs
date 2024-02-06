use std::{collections::BTreeMap, iter};

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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub enum MoveResult<'a> {
    Won(PlayerId),
    Moved(&'a mut Player),
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

    pub fn remove_player(&mut self, player_id: PlayerId) {
        self.players.remove(&player_id);
        if player_id == self.player_turn {
            self.next_player_turn();
        }
    }

    pub fn next_player_turn(&mut self) {
        self.player_turn = self
            .players
            .range(self.player_turn..)
            .next()
            .or_else(|| self.players.iter().next())
            .map(|(id, _)| *id)
            .unwrap_or_default();
    }

    pub fn move_player(&mut self, player_id: PlayerId, position: Position) -> MoveResult {
        let player = self.get_mut(player_id);
        player.set_position(position);

        if player.get_next_to_collect().is_none() && player.is_at_start() {
            MoveResult::Won(player_id)
        } else {
            MoveResult::Moved(player)
        }
    }

    fn get_mut(&mut self, player_id: PlayerId) -> &mut Player {
        self.players.get_mut(&player_id).unwrap()
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

    pub fn get_next_to_collect(&self) -> Option<Item> {
        self.to_collect.last().copied()
    }

    pub fn is_at_start(&self) -> bool {
        self.position == self.start_position
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn try_collect_item(&mut self, board: &Board) {
        let next = self.get_next_to_collect();
        if next.is_some() && next == board.get_item(self.position) {
            self.collected.push(self.to_collect.pop().unwrap());
        }
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
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
    let mut items: Vec<_> = iter::repeat(1..=num_board_items)
        .take(num_repeats)
        .flatten()
        .map(Item::new)
        .collect();
    items.extend((1..=num_remainder).map(Item::new));

    items
}

fn get_start_position(index: usize, side_length: usize) -> Position {
    match index % 4 {
        0 => Position::new(0, 0),
        1 => Position::new(0, side_length - 1),
        2 => Position::new(side_length - 1, side_length - 1),
        _ => Position::new(side_length - 1, 0),
    }
}
