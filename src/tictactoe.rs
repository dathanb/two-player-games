use crate::game::Game;
use crate::movegenerator::MoveGenerator;
use crate::movestrategy::MoveStrategy;
use crate::oracle::Oracle;
use crate::player::Player;
use crate::positionevaluator::{PositionEvaluation, PositionEvaluator};
use crate::r#move::Move;

#[derive(Copy, Clone, Debug)]
pub struct TicTacToeGame {
    pub board: [Option<Piece>; 9],
    pub last_player: usize,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Piece {
    X,
    O,
}

impl Game<TicTacToeGame, TicTacToeMove> for TicTacToeGame {
    fn apply(&self, m: &TicTacToeMove) -> TicTacToeGame {
        let mut new_position = *self;
        new_position.board[m.position] = Some(m.piece);
        new_position.last_player = (new_position.last_player + 1) % 2;
        new_position
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TicTacToeMove {
    pub position: usize,
    pub piece: Piece,
}

impl Move for TicTacToeMove {}

// TODO: Make a generic implementation of Player, since it doesn't look like there's anything TicTacToe-specific about it
pub struct TicTacToePlayer {
    move_strategy: Box<dyn MoveStrategy<TicTacToeGame, TicTacToeMove>>,
}

impl TicTacToePlayer {
    pub fn new(move_strategy: Box<dyn MoveStrategy<TicTacToeGame, TicTacToeMove>>) -> TicTacToePlayer {
        TicTacToePlayer {
            move_strategy
        }
    }
}

impl Player<TicTacToeGame, TicTacToeMove> for TicTacToePlayer {
    fn pick_move(&self, game: &TicTacToeGame) -> TicTacToeMove {
        self.move_strategy.choose_move(game)
    }
}

pub struct TicTacToeOracle {}

impl Oracle<TicTacToeGame, TicTacToeMove> for TicTacToeOracle {
    fn next_player(&self, game: &TicTacToeGame) -> Option<usize> {
        match game.last_player {
            0 => Some(1),
            1 => Some(0),
            _ => None
        }
    }

    fn is_terminal(&self, game: &TicTacToeGame) -> bool {
        // this is kind of dumb, but let's just hardcode all possible win conditions
        let is_three_in_a_row = |x: usize, y: usize, z: usize| {
            game.board[x].is_some() && game.board[y].is_some() && game.board[z].is_some()
                && game.board[x].unwrap() == game.board[y].unwrap() && game.board[y].unwrap() == game.board[z].unwrap()
        };

        let is_cat_game = || {
            game.board[0].is_some()
                && game.board[1].is_some()
                && game.board[2].is_some()
                && game.board[3].is_some()
                && game.board[4].is_some()
                && game.board[5].is_some()
                && game.board[6].is_some()
                && game.board[7].is_some()
                && game.board[8].is_some()
        };

        return is_three_in_a_row(0, 1, 2)
            || is_three_in_a_row(3, 4, 5)
            || is_three_in_a_row(6, 7, 8)
            || is_three_in_a_row(0, 3, 6)
            || is_three_in_a_row(1, 4, 7)
            || is_three_in_a_row(2, 5, 8)
            || is_three_in_a_row(0, 4, 8)
            || is_three_in_a_row(2, 4, 6)
            || is_cat_game();
    }
}

pub struct TicTacToeMoveGenerator {
}

impl MoveGenerator<TicTacToeGame, TicTacToeMove> for TicTacToeMoveGenerator {
    fn get_moves(&self, game: &TicTacToeGame) -> Vec<TicTacToeMove> {
        let piece = match game.last_player {
            0 => Piece::O,
            _ => Piece::X
        };
        let mut moves = vec!();
        for i in 0..9 {
            match game.board[i] {
                None => moves.push(TicTacToeMove { position: i, piece }),
                _ => ()
            }
        }

        moves
    }
}

pub struct TicTacToePositionEvaluator {
    pub player_piece: Piece,
}

impl TicTacToePositionEvaluator {
    fn has_three_in_a_row(&self, position: &TicTacToeGame, piece: Piece) -> bool {
        let is_three_in_a_row = |p: Piece, x: usize, y: usize, z: usize| {
            position.board[x].is_some() && position.board[y].is_some() && position.board[z].is_some()
                && position.board[x].unwrap() == p && position.board[y].unwrap() == p && position.board[z].unwrap() == p
        };

        return is_three_in_a_row(piece, 0, 1, 2)
            || is_three_in_a_row(piece, 3, 4, 5)
            || is_three_in_a_row(piece, 6, 7, 8)
            || is_three_in_a_row(piece, 0, 3, 6)
            || is_three_in_a_row(piece, 1, 4, 7)
            || is_three_in_a_row(piece, 2, 5, 8)
            || is_three_in_a_row(piece, 0, 4, 8)
            || is_three_in_a_row(piece, 2, 4, 6);
    }
}

impl PositionEvaluator<TicTacToeGame, TicTacToeMove> for TicTacToePositionEvaluator {
    fn evaluate(&self, position: &TicTacToeGame) -> PositionEvaluation {
        let other_piece = match self.player_piece {
            Piece::X => Piece::O,
            _ => Piece::X
        };

        if self.has_three_in_a_row(position, self.player_piece) {
            return PositionEvaluation::Winning;
        } else if self.has_three_in_a_row(position, other_piece) {
            return PositionEvaluation::Losing;
        } else {
            return PositionEvaluation::Estimate(0.0);
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{Piece, TicTacToeGame, TicTacToeMove, TicTacToeOracle, TicTacToePlayer, X};
    use crate::game::Game;
    use crate::oracle::Oracle;

    #[test]
    pub fn test_apply() {
        let game = TicTacToeGame {
            board: [None; 9],
            last_player: 1,
        };

        let new_game = game.apply(&TicTacToeMove { position: 0, piece: X });

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

        let oracle = TicTacToeOracle {};

        assert!(!oracle.is_terminal(&game));

        //  X | X | X
        // -----------
        //    |   |
        // -----------
        //    |   |
        game.board = [Some(Piece::X), Some(Piece::X), Some(Piece::X), None, None, None, None, None, None];
        assert!(oracle.is_terminal(&game));

        //    |   |
        // -----------
        //  X | X | X
        // -----------
        //    |   |
        game.board = [None, None, None, Some(Piece::X), Some(Piece::X), Some(Piece::X), None, None, None];
        assert!(oracle.is_terminal(&game));

        //    |   |
        // -----------
        //    |   |
        // -----------
        //  X | X | X
        game.board = [None, None, None, None, None, None, Some(Piece::X), Some(Piece::X), Some(Piece::X)];
        assert!(oracle.is_terminal(&game));

        //  X |   |
        // -----------
        //  X |   |
        // -----------
        //  X |   |
        game.board = [Some(Piece::X), None, None, Some(Piece::X), None, None, Some(Piece::X), None, None];
        assert!(oracle.is_terminal(&game));

        //    | X |
        // -----------
        //    | X |
        // -----------
        //    | X |
        game.board = [None, Some(Piece::X), None, None, Some(Piece::X), None, None, Some(Piece::X), None];
        assert!(oracle.is_terminal(&game));

        //    |   | X
        // -----------
        //    |   | X
        // -----------
        //    |   | X
        game.board = [None, None, Some(Piece::X), None, None, Some(Piece::X), None, None, Some(Piece::X)];
        assert!(oracle.is_terminal(&game));

        //  X |   |
        // -----------
        //    | X |
        // -----------
        //    |   | X
        game.board = [Some(Piece::X), None, None, None, Some(Piece::X), None, None, None, Some(Piece::X)];
        assert!(oracle.is_terminal(&game));

        //    |   | X
        // -----------
        //    | X |
        // -----------
        //  X |   |
        game.board = [None, None, Some(Piece::X), None, Some(Piece::X), None, Some(Piece::X), None, None];
        assert!(oracle.is_terminal(&game));

        // cat game
        //  X | O | X
        // -----------
        //  X | O | O
        // -----------
        //  O | X | O
        game.board = [Some(Piece::X), Some(Piece::O), Some(Piece::X), Some(Piece::X), Some(Piece::O), Some(Piece::O), Some(Piece::O), Some(Piece::X), Some(Piece::O)];
        assert!(oracle.is_terminal(&game));
    }
}