use crate::core::engine::GameEngine;
use crate::core::move_strategy::MinimaxMoveStrategy;
use crate::connect_four::{Piece, ConnectFourPosition, ConnectFourMove, ConnectFourMoveGenerator, ConnectFourOracle, ConnectFourPositionEvaluator};
use crate::core::player::DefaultPlayer;

mod tictactoe;
mod core;
mod connect_four;

fn main() {
    let player1 = DefaultPlayer::new(
        Box::new(MinimaxMoveStrategy::new(ConnectFourPositionEvaluator { player_piece: Piece::Red },
                                          ConnectFourMoveGenerator {},
                                          ConnectFourOracle {position_evaluator: ConnectFourPositionEvaluator{player_piece: Piece::Phantom}}))
    );
    let player2 = DefaultPlayer::new(
        Box::new(MinimaxMoveStrategy::new(ConnectFourPositionEvaluator { player_piece: Piece::Black },
                                          ConnectFourMoveGenerator {},
                                          ConnectFourOracle {position_evaluator: ConnectFourPositionEvaluator{player_piece: Piece::Phantom}}))
    );

    let mut engine = GameEngine::<ConnectFourPosition, ConnectFourMove, DefaultPlayer<ConnectFourPosition, ConnectFourMove>, ConnectFourOracle>::new(
        ConnectFourPosition::new(),
        player1,
        player2,
        ConnectFourOracle {
            position_evaluator: ConnectFourPositionEvaluator{player_piece: Piece::Phantom}
        });

    engine.run();
}