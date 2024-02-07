pub mod board;
pub mod game;
pub mod player;
pub mod tile;

#[cfg(test)]
mod tests {
    use crate::board::Board;

    #[test]
    fn new_board() {
        Board::new(7);
    }
}
