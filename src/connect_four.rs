use crate::core::game::{Position, Oracle};
use crate::core::position_evaluator::{PositionEvaluation, PositionEvaluator};
use crate::core::r#move::{Move, MoveGenerator};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    Red,
    Black,
    None
}

#[derive(Copy, Clone, Debug)]
pub struct ConnectFourPosition {
    board: [Piece; 42],
    last_player: usize
}

impl ConnectFourPosition {
    pub fn new() -> ConnectFourPosition {
        // red always goes first, so the last player is Black or 1
        ConnectFourPosition {
            // the board in column-major order. 0 is the upper-left corner, and 1-5 are the cells below it.
            // 6-13 are column 2, etc.
            board: [Piece::None; 42],
            last_player: 1
        }
    }
}

impl Position<ConnectFourPosition, ConnectFourMove> for ConnectFourPosition {
    fn apply(&self, m: &ConnectFourMove) -> ConnectFourPosition {
        let mut new_position = self.clone();
        for i in (m.column*6)..((m.column+1)*6-1) {
            if new_position.board[i+1] != Piece::None {
                new_position.board[i] = m.piece;
                return new_position;
            }
        }
        new_position.board[(m.column+1)*6-1] = m.piece;
        new_position
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ConnectFourMove {
    pub piece: Piece,
    pub column: usize
}

impl Move for ConnectFourMove {
}

pub struct ConnectFourPositionEvaluator {
    pub player_piece: Piece
}

impl ConnectFourPositionEvaluator {
    fn get_horizontal_winner(&self, position: &ConnectFourPosition, index: usize) -> Piece {
        if position.board[index] == position.board[index+6]
            && position.board[index] == position.board[index+12]
            && position.board[index] == position.board[index+18] {
            return position.board[index];
        }

        Piece::None
    }

    fn get_vertical_winner(&self, position: &ConnectFourPosition, index: usize) -> Piece {
        if position.board[index] == position.board[index+1]
            && position.board[index] == position.board[index+2]
            && position.board[index] == position.board[index+3] {
            return position.board[index];
        }

        Piece::None
    }

    fn get_down_right_winner(&self, position: &ConnectFourPosition, index: usize) -> Piece {
        if position.board[index] == position.board[index+7]
            && position.board[index] == position.board[index+14]
            && position.board[index] == position.board[index+21] {
            return position.board[index];
        }

        Piece::None
    }

    fn get_up_right_winner(&self, position: &ConnectFourPosition, index: usize) -> Piece {
        if position.board[index] == position.board[index+5]
            && position.board[index] == position.board[index+10]
            && position.board[index] == position.board[index+15] {
            return position.board[index];
        }

        Piece::None
    }
}

impl PositionEvaluator<ConnectFourPosition, ConnectFourMove> for ConnectFourPositionEvaluator {
    fn evaluate(&self, position: &ConnectFourPosition) -> PositionEvaluation {
        // detect horizontal winners
        for col in 0..7 {
            for row in 0..6 {
                let winner = self.get_horizontal_winner(position, row*7+col);
                if winner == self.player_piece {
                    return PositionEvaluation::Winning;
                } else if winner != Piece::None {
                    return PositionEvaluation::Losing;
                }
            }
        }

        // detect vertical winners
        for col in 0..7 {
            for row in 0..3 {
                let winner = self.get_vertical_winner(position, row*7+col);
                if winner == self.player_piece {
                    return PositionEvaluation::Winning;
                } else if winner != Piece::None {
                    return PositionEvaluation::Losing;
                }
            }
        }

        // detect down_right winners
        for col in 0..4 {
            for row in 0..3 {
                let winner = self.get_down_right_winner(position, row*7+col);
                if winner == self.player_piece {
                    return PositionEvaluation::Winning;
                } else if winner != Piece::None {
                    return PositionEvaluation::Losing;
                }
            }
        }


        // detect up_right winners
        for col in 3..6 {
            for row in 0..3 {
                let winner = self.get_up_right_winner(position, row*7+col);
                if winner == self.player_piece {
                    return PositionEvaluation::Winning;
                } else if winner != Piece::None {
                    return PositionEvaluation::Losing;
                }
            }
        }

        PositionEvaluation::Estimate(0.0)
    }

}

pub struct ConnectFourMoveGenerator{
    pub player_piece: Piece
}

impl MoveGenerator<ConnectFourPosition, ConnectFourMove> for ConnectFourMoveGenerator {
    fn get_moves(&self, position: &ConnectFourPosition) -> Vec<ConnectFourMove> {
        let mut moves = vec![];
        for index in 0..6 {
            if position.board[index*6] == Piece::None {
                moves.push(ConnectFourMove{piece: self.player_piece, column: 0});
            }

        }

        todo!()
    }
}

pub struct ConnectFourOracle{}

impl Oracle<ConnectFourPosition, ConnectFourMove> for ConnectFourOracle {
    fn next_player(&self, position: &ConnectFourPosition) -> Option<usize> {
        todo!()
    }

    fn is_terminal(&self, position: &ConnectFourPosition) -> bool {
        todo!()
    }
}
