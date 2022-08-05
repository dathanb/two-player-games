use crate::core::engine::GameEngine;
use crate::core::move_strategy::MinimaxMoveStrategy;
use crate::connect_four::{Piece, ConnectFourGame, ConnectFourMove, ConnectFourMoveGenerator, ConnectFourOracle, ConnectFourPlayer, ConnectFourPositionEvaluator};

mod tictactoe;
mod core;
mod connect_four;

fn main() {
    let player1 = ConnectFourPlayer::new(
        Box::new(MinimaxMoveStrategy::new(ConnectFourPositionEvaluator { player_piece: Piece::Red }, ConnectFourMoveGenerator {}, ConnectFourOracle {}))
    );
    let player2 = ConnectFourPlayer::new(
        Box::new(MinimaxMoveStrategy::new(ConnectFourPositionEvaluator { player_piece: Piece::Black }, ConnectFourMoveGenerator {}, ConnectFourOracle {}))
    );

    let mut engine = GameEngine::<ConnectFourGame, ConnectFourMove, ConnectFourPlayer, ConnectFourOracle>::new(
        ConnectFourGame::new(),
        player1,
        player2,
        ConnectFourOracle {});

    engine.run();
}