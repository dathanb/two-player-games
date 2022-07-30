use crate::game::Game;
use crate::movegenerator::MoveGenerator;
use crate::oracle::Oracle;
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
    fn apply(&self, m: &TicTacToeMove) -> TicTacToeGame {
        let mut new_position = *self;
        new_position.board[m.position] = Some(m.piece);
        new_position
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
    fn pick_move(&self, game: &TicTacToeGame) -> TicTacToeMove {
        todo!()
    }
}

pub struct TicTacToeMoveGenerator {
    piece: Piece
}

pub struct TicTacToeOracle {
}

impl Oracle<TicTacToeGame, TicTacToeMove, TicTacToePlayer> for TicTacToeOracle {
    fn next_player(&self, game: &TicTacToeGame) -> &TicTacToePlayer {
        todo!()
    }

    fn is_terminal(&self, game: &TicTacToeGame) -> bool {
        // this is kind of dumb, but let's just hardcode all possible win conditions
        let is_three_in_a_row = |x:usize, y:usize, z:usize| {
            game.board[x].is_some() && game.board[y].is_some() && game.board[z].is_some()
                && game.board[x].unwrap() == game.board[y].unwrap() && game.board[y].unwrap() == game.board[z].unwrap()
        };

        return is_three_in_a_row(0, 1, 2)
            || is_three_in_a_row(3, 4, 5)
            || is_three_in_a_row(6, 7, 8)
            || is_three_in_a_row(0, 3, 6)
            || is_three_in_a_row(1, 4, 7)
            || is_three_in_a_row(2, 5, 8)
            || is_three_in_a_row(0, 4, 8)
            || is_three_in_a_row(2, 4, 6);
    }
}

impl MoveGenerator<TicTacToeGame, TicTacToeMove> for TicTacToeMoveGenerator {
    fn get_moves(&self, game: &TicTacToeGame) -> Vec<TicTacToeMove> {
        let mut moves = vec!();
        for i in 0..9 {
            match game.board[i] {
                None => moves.push(TicTacToeMove{position:i, piece: self.piece}),
                _ => ()
            }
        }

        moves
    }
}


#[cfg(test)]
mod tests {
    use crate::{TicTacToeGame, TicTacToeMove, TicTacToeOracle, TicTacToePlayer, X};
    use crate::game::Game;
    use crate::oracle::Oracle;

    #[test]
    pub fn test_apply() {
        let game = TicTacToeGame {
            board: [None; 9],
            last_player: 1,
        };

        let new_game = game.apply(&TicTacToeMove{position: 0, piece: X});

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

    #[test]
    pub fn test_is_terminal() {
        let mut game = TicTacToeGame {
            board: [None; 9],
            last_player: 1,
        };

        let oracle = TicTacToeOracle{};

        assert!(!oracle.is_terminal(&game));

        //  X | X | X
        // -----------
        //    |   |
        // -----------
        //    |   |
        game.board = [Some(X), Some(X), Some(X), None, None, None, None, None, None];
        assert!(oracle.is_terminal(&game));

        //    |   |
        // -----------
        //  X | X | X
        // -----------
        //    |   |
        game.board = [None, None, None, Some(X), Some(X), Some(X), None, None, None];
        assert!(oracle.is_terminal(&game));

        //    |   |
        // -----------
        //    |   |
        // -----------
        //  X | X | X
        game.board = [None, None, None, None, None, None, Some(X), Some(X), Some(X)];
        assert!(oracle.is_terminal(&game));

        //  X |   |
        // -----------
        //  X |   |
        // -----------
        //  X |   |
        game.board = [Some(X), None, None, Some(X), None, None, Some(X), None, None];
        assert!(oracle.is_terminal(&game));

        //    | X |
        // -----------
        //    | X |
        // -----------
        //    | X |
        game.board = [None, Some(X), None, None, Some(X), None, None, Some(X), None];
        assert!(oracle.is_terminal(&game));

        //    |   | X
        // -----------
        //    |   | X
        // -----------
        //    |   | X
        game.board = [None, None, Some(X), None, None, Some(X), None, None, Some(X)];
        assert!(oracle.is_terminal(&game));

        //  X |   |
        // -----------
        //    | X |
        // -----------
        //    |   | X
        game.board = [Some(X), None,  None, None, Some(X), None, None, None, Some(X)];
        assert!(oracle.is_terminal(&game));

        //    |   | X
        // -----------
        //    | X |
        // -----------
        //  X |   |
        game.board = [None, None, Some(X), None, Some(X), None, Some(X), None, None];
        assert!(oracle.is_terminal(&game));
    }
}