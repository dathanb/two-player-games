use crate::engine::GameEngine;
use crate::movestrategy::{MinimaxMoveStrategy};
use crate::tictactoe::{Piece, TicTacToeGame, TicTacToeMove, TicTacToeMoveGenerator, TicTacToeOracle, TicTacToePlayer, TicTacToePositionEvaluator};

mod player;
mod r#move;
mod movegenerator;
mod engine;
mod game;
mod tictactoe;
mod oracle;
mod movestrategy;
mod positionevaluator;

fn main() {
    let player1 = TicTacToePlayer::new(
        Box::new(MinimaxMoveStrategy::new(TicTacToePositionEvaluator { player_piece: Piece::X }, TicTacToeMoveGenerator {}, TicTacToeOracle {}))
    );
    let player2 = TicTacToePlayer::new(
        Box::new(MinimaxMoveStrategy::new(TicTacToePositionEvaluator { player_piece: Piece::O }, TicTacToeMoveGenerator {}, TicTacToeOracle {}))
    );

    let mut engine = GameEngine::<TicTacToeGame, TicTacToeMove, TicTacToePlayer, TicTacToeOracle>::new(
        TicTacToeGame { board: [None; 9], last_player: 1 },
        player1,
        player2,
        TicTacToeOracle {});

    engine.run();
}