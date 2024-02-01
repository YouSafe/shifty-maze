use crate::{tile::Item, vec2::Vec2};

pub type PlayerId = u8;

pub struct Player {
    pub id: PlayerId,
    pub position: Vec2,
    pub start_position: Vec2,
    pub collected: Vec<Item>,
    pub to_collect: Vec<Item>,
}
