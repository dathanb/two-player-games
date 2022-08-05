use crate::core::game::{Game, Oracle};
use crate::core::player::Player;
use crate::core::position_evaluator::{PositionEvaluation, PositionEvaluator};
use crate::core::r#move::{Move, MoveGenerator};
use crate::MinimaxMoveStrategy;

pub enum Piece {
    Red,
    Black
}

#[derive(Copy, Clone, Debug)]
pub struct ConnectFourGame {
}

impl ConnectFourGame {
    pub fn new() -> ConnectFourGame {
        todo!()
    }
}

impl Game<ConnectFourGame, ConnectFourMove> for ConnectFourGame {
    fn apply(&self, m: &ConnectFourMove) -> ConnectFourGame {
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

impl PositionEvaluator<ConnectFourGame, ConnectFourMove> for ConnectFourPositionEvaluator {
    fn evaluate(&self, position: &ConnectFourGame) -> PositionEvaluation {
        todo!()
    }
}

pub struct ConnectFourMoveGenerator{}

impl MoveGenerator<ConnectFourGame, ConnectFourMove> for ConnectFourMoveGenerator {
    fn get_moves(&self, game: &ConnectFourGame) -> Vec<ConnectFourMove> {
        todo!()
    }
}

pub struct ConnectFourOracle{}

impl Oracle<ConnectFourGame, ConnectFourMove> for ConnectFourOracle {
    fn next_player(&self, game: &ConnectFourGame) -> Option<usize> {
        todo!()
    }

    fn is_terminal(&self, game: &ConnectFourGame) -> bool {
        todo!()
    }
}

pub struct ConnectFourPlayer{
}

impl ConnectFourPlayer {
    pub fn new(p0: Box<MinimaxMoveStrategy<ConnectFourGame, ConnectFourMove, ConnectFourPositionEvaluator, ConnectFourMoveGenerator, ConnectFourOracle>>) -> ConnectFourPlayer {
        todo!()
    }
}

impl ConnectFourPlayer {

}

impl Player<ConnectFourGame, ConnectFourMove> for ConnectFourPlayer {
    fn pick_move(&self, game: &ConnectFourGame) -> ConnectFourMove {
        todo!()
    }
}