pub mod board;
pub mod game;
pub mod player;
pub mod tile;

#[cfg(test)]
mod tests {
    use crate::{
        board::{Board, NewBoardError},
        game::{Game, GameStartSettings, NewGameError},
        player::{MoveResult, Players, Position},
        tile::{Side, SideIndex},
    };

    fn new_board() -> Result<Board, NewBoardError> {
        Board::new(7)
    }

    fn new_players() -> Option<Players> {
        Players::new([0, 1, 2, 3].into(), 6, &new_board().unwrap())
    }

    fn new_game() -> Result<Game, NewGameError> {
        Game::new(GameStartSettings::new([0, 1, 2, 3].into(), 7, 6))
    }

    #[test]
    fn new_board_ok() {
        assert!(new_board().is_ok());
    }

    #[test]
    fn shift_tiles() {
        assert!(new_board()
            .unwrap()
            .shift_tiles(SideIndex::new(Side::Top, 1))
            .is_ok());
    }

    #[test]
    fn big_boards() {
        for i in 4..30 {
            assert!(Board::new(2 * i + 1).is_ok());
        }
    }

    #[test]
    fn new_players_ok() {
        assert!(new_players().is_some());
    }

    #[test]
    fn remove_player() {
        assert!(new_players().unwrap().remove_player(0).is_ok());
    }

    #[test]
    fn move_player() {
        let new_pos = Position::new(0, 0);
        match new_players()
            .unwrap()
            .move_player(0, new_pos, &new_board().unwrap())
        {
            MoveResult::Moved(player) => assert!(player.get_position() == new_pos),
            _ => panic!("Wrong result"),
        };
    }

    #[test]
    fn new_game_ok() {
        assert!(new_game().is_ok());
    }

    #[test]
    fn game_actions() {
        let mut game = new_game().unwrap();
        assert!(game.shift_tiles(SideIndex::new(Side::Top, 1)).is_ok());
        assert!(game.remove_player(0).is_ok());
        assert!(game.move_player(1, Position::new(6, 0)).is_ok());
    }
}
