use std::iter;

use rand::seq::{IteratorRandom, SliceRandom};
use ts_interop::ts_interop;

use crate::tile::{FreeTile, Item, Rotation, Tile, TileVariant};

#[ts_interop]
#[derive(Clone)]
pub struct Board {
    tiles: Vec<Tile>,
    side_length: usize,
    free_tile: FreeTile,
}

impl Board {
    pub fn new(side_length: usize) -> Self {
        assert!(side_length % 2 == 1 && side_length > 2);
        use Rotation::*;
        use TileVariant::*;

        let rotations = [Ninety, TwoSeventy, Zero, OneEighty];

        let mut tiles = Vec::with_capacity(side_length.pow(2));
        let mut movable_tiles = get_tile_assortment(side_length);

        let mut rng = rand::thread_rng();
        movable_tiles.shuffle(&mut rng);

        let num_movable_items = calculate_number_of_items(side_length) / 2;
        let movable_item_indices =
            (0..movable_tiles.len()).choose_multiple(&mut rng, num_movable_items);

        let mut index = 0;
        let mut item_id = 0;
        for row in 0..side_length {
            for col in 0..side_length {
                let (variant, rotation, add_item) = match (row, col) {
                    // Corners
                    (0, 0) => (LShape, Ninety, false),
                    (0, c) if c == side_length - 1 => (LShape, OneEighty, false),
                    (r, 0) if r == side_length - 1 => (LShape, Zero, false),
                    (r, c) if r == side_length - 1 && r == c => (LShape, TwoSeventy, false),
                    // Sides
                    (r, c) if r == 0 && c % 2 == 0 => (TShape, Zero, true),
                    (r, c) if c == 0 && r % 2 == 0 => (TShape, TwoSeventy, true),
                    (r, c) if r == side_length - 1 && c % 2 == 0 => (TShape, OneEighty, true),
                    (r, c) if c == side_length - 1 && r % 2 == 0 => (TShape, Ninety, true),
                    // Inners
                    (r, c) if r % 2 == 0 && c % 2 == 0 => {
                        index = (index + 1) % rotations.len();
                        (TShape, rotations[index], true)
                    }
                    // Movables
                    _ => (
                        movable_tiles.pop().unwrap(),
                        *rotations.choose(&mut rng).unwrap(),
                        movable_item_indices.contains(&movable_tiles.len()),
                    ),
                };

                let item = if add_item {
                    item_id += 1;
                    Some(Item::new(item_id))
                } else {
                    None
                };

                tiles.push(Tile::new(row * side_length + col, variant, rotation, item));
            }
        }

        let free_tile = movable_tiles.pop().unwrap();
        assert!(movable_tiles.is_empty());
        let item = if movable_item_indices.contains(&0) {
            Some(Item::new(item_id + 1))
        } else {
            None
        };
        let free_tile = FreeTile::new(Tile::new(side_length.pow(2), free_tile, Zero, item));

        Self {
            tiles,
            side_length,
            free_tile,
        }
    }

    pub fn get_side_length(&self) -> usize {
        self.side_length
    }

    pub fn get_number_of_items(&self) -> usize {
        calculate_number_of_items(self.side_length)
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

    let mut movable_tiles = Vec::with_capacity(num_moveable);
    movable_tiles.extend(iter::repeat(TileVariant::TShape).take(num_t_tiles));
    movable_tiles.extend(iter::repeat(TileVariant::IShape).take(num_i_tiles));
    movable_tiles.extend(iter::repeat(TileVariant::LShape).take(num_l_tiles));

    movable_tiles
}

fn calculate_number_of_items(side_length: usize) -> usize {
    // 2 * (ceil(side_length / 2)² - num_corners)
    2 * ((side_length / 2 + 1).pow(2) - 4)
}
