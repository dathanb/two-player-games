use crate::game::Game;
use crate::player::Player;
use crate::r#move::Move;

#[derive(Copy, Clone)]
pub struct TicTacToeGame {
    pub board: [Option<Piece>; 9],
    pub last_player: u8,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Piece {
    X,
    O
}

impl Game<TicTacToeGame, TicTacToeMove> for TicTacToeGame {
    fn apply(&self, m: TicTacToeMove) -> TicTacToeGame {
        let mut new_position = *self;
        new_position.board[m.position] = Some(m.piece);
        new_position
    }

    fn is_terminal(&self) -> bool {
        todo!()
    }
}

pub struct TicTacToeMove {
    pub position: usize,
    pub piece: Piece,
}

impl Move for TicTacToeMove {
}

pub struct TicTacToePlayer {
}

impl Player<TicTacToeGame, TicTacToeMove> for TicTacToePlayer {
}

#[cfg(test)]
mod tests {
    use crate::{TicTacToeGame, TicTacToeMove, X};
    use crate::game::Game;

    #[test]
    pub fn test_apply() {
        let game = TicTacToeGame {
            board: [None; 9],
            last_player: 1,
        };

        let new_game = game.apply(TicTacToeMove{position: 0, piece: X});

        assert!(new_game.board[0].is_some());
        assert_eq!(new_game.board[0].unwrap(), X);
        assert!(new_game.board[1].is_none());
        assert!(new_game.board[2].is_none());
        assert!(new_game.board[3].is_none());
        assert!(new_game.board[4].is_none());
        assert!(new_game.board[5].is_none());
        assert!(new_game.board[6].is_none());
        assert!(new_game.board[7].is_none());
        assert!(new_game.board[8].is_none());
    }
}