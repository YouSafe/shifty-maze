pub mod board;
pub mod game;
pub mod player;
pub mod tile;

#[cfg(test)]
mod tests {
    use crate::{
        board::Board,
        game::{Game, GameStartSettings},
        player::Players,
    };

    #[test]
    fn new_board() {
        Board::new(7);
    }

    #[test]
    fn big_boards() {
        for i in 4..30 {
            Board::new(2 * i + 1);
        }
    }

    #[test]
    fn new_players() {
        Players::new(vec![0, 1, 2, 3], 6, &Board::new(7));
    }

    #[test]
    fn new_game() {
        Game::new(GameStartSettings::new(vec![0, 1, 2, 3], 7, 6));
    }
}
