use std::iter;

use crate::{
    player::Position,
    tile::{FreeTile, Item, Rotation, Side, Tile, TileVariant},
};
use rand::seq::SliceRandom;
use wasm_interop::wasm_interop;

#[wasm_interop]
#[derive(Clone)]
pub struct Board {
    tiles: Vec<Tile>,
    side_length: usize,
    free_tile: FreeTile,
    number_of_items: u8,
}

impl Board {
    pub fn new(number_of_items: u8, side_length: usize) -> Self {
        assert!(side_length % 2 == 1 && side_length > 2);
        use Rotation::*;
        use TileVariant::*;

        let rotations = [OneEighty, Zero, Ninety, TwoSeventy];

        let mut index = 0;
        let mut tiles = Vec::with_capacity(side_length.pow(2));

        let mut rng = rand::thread_rng();
        let mut rand_tiles = get_tile_assortment(side_length);

        rand_tiles.shuffle(&mut rng);

        for row in 0..side_length {
            for col in 0..side_length {
                let (variant, rotation) = match (row, col) {
                    (0, 0) => (LShape, Zero),
                    (0, c) if c == side_length - 1 => (LShape, Ninety),
                    (r, 0) if r == side_length - 1 => (LShape, TwoSeventy),
                    (r, c) if r == side_length - 1 && r == c => (LShape, OneEighty),
                    (r, c) if c == 0 && r % 2 == 0 => (TShape, Zero),
                    (r, c) if r == 0 && c % 2 == 0 => (TShape, Ninety),
                    (r, c) if c == side_length - 1 && r % 2 == 0 => (TShape, OneEighty),
                    (r, c) if r == side_length - 1 && c % 2 == 0 => (TShape, TwoSeventy),
                    (r, c) if r % 2 == 0 && c % 2 == 0 => {
                        index = (index + 1) % rotations.len();
                        (TShape, rotations[index])
                    }
                    _ => (
                        rand_tiles.pop().unwrap(),
                        *rotations.choose(&mut rng).unwrap(),
                    ),
                };
                tiles.push(Tile::new(
                    (row * side_length + col) as u32,
                    variant,
                    rotation,
                    None,
                )); // TODO add item
            }
        }

        let free_tile = rand_tiles.pop().unwrap();
        assert!(rand_tiles.is_empty());
        let free_tile = FreeTile::new(Tile::new(
            (side_length * side_length) as u32,
            free_tile,
            Zero,
            None,
        ));

        Self {
            tiles,
            side_length,
            free_tile,
            number_of_items,
        }
    }

    pub fn get_number_of_items(&self) -> u8 {
        self.number_of_items
    }

    pub fn get_side_length(&self) -> usize {
        self.side_length
    }

    pub fn get_item(&self, position: Position) -> Option<Item> {
        let tile = &self.tiles[position.y * self.side_length + position.x];
        tile.get_item()
    }

    pub fn shift_tiles(&mut self, side: Side, index: usize, insert_rotation: Rotation) {
        todo!()
    }
}

fn get_tile_assortment(side_length: usize) -> Vec<TileVariant> {
    const T_RATIO: f64 = 6. / 34.;
    const I_RATIO: f64 = 13. / 34.;
    const L_RATIO: f64 = 15. / 34.;

    let num_moveable = side_length.pow(2) - (side_length / 2 + 1).pow(2) + 1;
    let num_t_tiles = (num_moveable as f64 * T_RATIO) as usize; // flooring
    let num_i_tiles = (num_moveable as f64 * I_RATIO) as usize;
    let mut num_l_tiles = (num_moveable as f64 * L_RATIO) as usize;

    if num_t_tiles + num_i_tiles + num_l_tiles != num_moveable {
        num_l_tiles += 1;
    }
    assert!(num_t_tiles + num_i_tiles + num_l_tiles == num_moveable);

    let mut rand_tiles = Vec::with_capacity(num_moveable);
    rand_tiles.extend(iter::repeat(TileVariant::TShape).take(num_t_tiles));
    rand_tiles.extend(iter::repeat(TileVariant::IShape).take(num_i_tiles));
    rand_tiles.extend(iter::repeat(TileVariant::LShape).take(num_l_tiles));

    rand_tiles
}
