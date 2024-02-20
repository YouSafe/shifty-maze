use crate::{board::Board, tile::Item};
use rand::{
    seq::{IteratorRandom, SliceRandom},
    Rng,
};
use std::collections::BTreeMap;
use ts_interop::ts_interop;

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
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: usize,
    y: usize,
}

pub enum MoveResult<'a> {
    Moved(&'a mut Player),
    Won(PlayerId),
    InvalidPlayer,
    Unreachable,
}

impl Players {
    pub fn new(mut ids: Vec<PlayerId>, items_per_player: usize, board: &Board) -> Option<Self> {
        if ids.len() < 2 {
            return None;
        }

        ids.sort();

        let player_turn = *ids.first().unwrap();
        let mut rng = rand::thread_rng();
        let mut items = get_items_to_collect(
            board.get_number_of_items(),
            ids.len() * items_per_player,
            &mut rng,
        );
        items.shuffle(&mut rng);

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

        Some(Self {
            players,
            player_turn,
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Player> {
        self.players.values_mut()
    }

    pub fn remove_player(&mut self, player_id: PlayerId) -> Result<Option<PlayerId>, ()> {
        if self.players.remove(&player_id).is_none() {
            return Err(());
        }

        if self.players.len() == 1 {
            return Ok(self.players.first_key_value().map(|(id, _)| *id));
        }

        if player_id == self.player_turn {
            self.next_player_turn();
        }

        Ok(None)
    }

    pub fn next_player_turn(&mut self) {
        self.player_turn = self
            .players
            .range(self.player_turn + 1..)
            .next()
            .or_else(|| self.players.first_key_value())
            .map(|(id, _)| *id)
            .unwrap();
    }

    pub fn move_player(
        &mut self,
        player_id: PlayerId,
        position: Position,
        board: &Board,
    ) -> MoveResult {
        if let Some(player) = self.players.get_mut(&player_id) {
            if !board.is_reachable(player.get_position(), position) {
                return MoveResult::Unreachable;
            }

            player.set_position(position);
            return if player.get_next_to_collect().is_none() && player.is_at_start() {
                MoveResult::Won(player_id)
            } else {
                MoveResult::Moved(player)
            };
        }
        MoveResult::InvalidPlayer
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

    pub fn get_position(&self) -> Position {
        self.position
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
        if next.is_some() && next == board[self.position].get_item() {
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

    pub fn top(&self, _: &Board) -> Option<Self> {
        if self.y == 0 {
            return None;
        }
        Some(Self {
            y: self.y - 1,
            ..*self
        })
    }

    pub fn bottom(&self, board: &Board) -> Option<Self> {
        if self.y > board.get_side_length() - 2 {
            return None;
        }
        Some(Self {
            y: self.y + 1,
            ..*self
        })
    }

    pub fn left(&self, _: &Board) -> Option<Self> {
        if self.x == 0 {
            return None;
        }
        Some(Self {
            x: self.x - 1,
            ..*self
        })
    }

    pub fn right(&self, board: &Board) -> Option<Self> {
        if self.x > board.get_side_length() - 2 {
            return None;
        }
        Some(Self {
            x: self.x + 1,
            ..*self
        })
    }
}

fn get_start_position(index: usize, side_length: usize) -> Position {
    match index % 4 {
        0 => Position::new(0, 0),
        1 => Position::new(side_length - 1, 0),
        2 => Position::new(side_length - 1, side_length - 1),
        _ => Position::new(0, side_length - 1),
    }
}

fn get_items_to_collect(
    num_board_items: usize,
    num_item_cards: usize,
    rng: &mut impl Rng,
) -> Vec<Item> {
    let mut items: Vec<_> = (1..=num_board_items).map(Item::new).collect();

    let extra = num_item_cards
        .checked_sub(num_board_items)
        .unwrap_or_default();

    items.extend(
        (0..extra)
            .map(|_| rng.gen_range(1..=num_board_items))
            .map(Item::new),
    );
    items.shuffle(rng);
    items.into_iter().choose_multiple(rng, num_item_cards)
}
