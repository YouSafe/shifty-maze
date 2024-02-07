pub mod board;
pub mod game;
pub mod player;
pub mod tile;

#[cfg(test)]
mod tests {
    use crate::{board::Board, player::Players};

    #[test]
    fn new_board() {
        Board::new(7);
    }

    #[test]
    fn new_players() {
        Players::new(vec![0, 1, 2, 3], 6, &Board::new(7));
    }
}
