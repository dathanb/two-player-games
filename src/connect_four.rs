use crate::core::game::{Position, Oracle};
use crate::core::player::Player;
use crate::core::position_evaluator::{PositionEvaluation, PositionEvaluator};
use crate::core::r#move::{Move, MoveGenerator};
use crate::MinimaxMoveStrategy;

pub enum Piece {
    Red,
    Black
}

#[derive(Copy, Clone, Debug)]
pub struct ConnectFourPosition {
}

impl ConnectFourPosition {
    pub fn new() -> ConnectFourPosition {
        todo!()
    }
}

impl Position<ConnectFourPosition, ConnectFourMove> for ConnectFourPosition {
    fn apply(&self, m: &ConnectFourMove) -> ConnectFourPosition {
        todo!()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ConnectFourMove {
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
    fn get_moves(&self, game: &ConnectFourPosition) -> Vec<ConnectFourMove> {
        todo!()
    }
}

pub struct ConnectFourOracle{}

impl Oracle<ConnectFourPosition, ConnectFourMove> for ConnectFourOracle {
    fn next_player(&self, game: &ConnectFourPosition) -> Option<usize> {
        todo!()
    }

    fn is_terminal(&self, game: &ConnectFourPosition) -> bool {
        todo!()
    }
}

pub struct ConnectFourPlayer{
}

impl ConnectFourPlayer {
    pub fn new(p0: Box<MinimaxMoveStrategy<ConnectFourPosition, ConnectFourMove, ConnectFourPositionEvaluator, ConnectFourMoveGenerator, ConnectFourOracle>>) -> ConnectFourPlayer {
        todo!()
    }
}

impl ConnectFourPlayer {

}

impl Player<ConnectFourPosition, ConnectFourMove> for ConnectFourPlayer {
    fn pick_move(&self, game: &ConnectFourPosition) -> ConnectFourMove {
        todo!()
    }
}