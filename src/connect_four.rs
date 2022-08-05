use crate::core::game::{Position, Oracle};
use crate::core::position_evaluator::{PositionEvaluation, PositionEvaluator};
use crate::core::r#move::{Move, MoveGenerator};

#[derive(Copy, Clone, Debug)]
pub enum Piece {
    Red,
    Black
}

#[derive(Copy, Clone, Debug)]
pub struct ConnectFourPosition {
    board: [Option<Piece>; 42],
    last_player: usize
}

impl ConnectFourPosition {
    pub fn new() -> ConnectFourPosition {
        // red always goes first, so the last player is Black or 1
        ConnectFourPosition {
            // the board in column-major order. 0 is the upper-left corner, and 1-5 are the cells below it.
            // 6-13 are column 2, etc.
            board: [None; 42],
            last_player: 1
        }
    }
}

impl Position<ConnectFourPosition, ConnectFourMove> for ConnectFourPosition {
    fn apply(&self, m: &ConnectFourMove) -> ConnectFourPosition {
        let mut new_position = self.clone();
        for i in (m.column*6)..((m.column+1)*6-1) {
            if new_position.board[i+1].is_some() {
                new_position.board[i] = Some(m.piece);
                return new_position;
            }
        }
        new_position.board[(m.column+1)*6-1] = Some(m.piece);
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

impl PositionEvaluator<ConnectFourPosition, ConnectFourMove> for ConnectFourPositionEvaluator {
    fn evaluate(&self, position: &ConnectFourPosition) -> PositionEvaluation {
        todo!()
    }
}

pub struct ConnectFourMoveGenerator{}

impl MoveGenerator<ConnectFourPosition, ConnectFourMove> for ConnectFourMoveGenerator {
    fn get_moves(&self, position: &ConnectFourPosition) -> Vec<ConnectFourMove> {
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
