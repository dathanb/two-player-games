use crate::engine::GameEngine;
use crate::tictactoe::{Piece, TicTacToeGame, TicTacToeMove, TicTacToeMoveGenerator, TicTacToeOracle, TicTacToePlayer};
use crate::tictactoe::Piece::X;

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
    let player1 = TicTacToePlayer {move_generator: TicTacToeMoveGenerator{piece: Piece::X}};
    let player2 = TicTacToePlayer {move_generator: TicTacToeMoveGenerator{piece: Piece::O}};

    let mut engine = GameEngine::<TicTacToeGame, TicTacToeMove, TicTacToePlayer, TicTacToeOracle>::new(
        TicTacToeGame { board: [None; 9], last_player: 1 },
        player1,
        player2,
        TicTacToeOracle {});

    engine.run();
}