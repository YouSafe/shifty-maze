pub mod board;
pub mod game;
pub mod player;
pub mod tile;

#[cfg(test)]
mod tests {
    use crate::board::Board;

    #[test]
    fn it_works() {
        Board::new(0, 7);
    }
}
