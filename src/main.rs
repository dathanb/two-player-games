use crate::engine::GameEngine;
use crate::tictactoe::{TicTacToeGame, TicTacToeMove, TicTacToeOracle, TicTacToePlayer};
use crate::tictactoe::Piece::X;

mod player;
mod r#move;
mod movegenerator;
mod engine;
mod game;
mod tictactoe;
mod oracle;

fn main() {
    let player1 = TicTacToePlayer {};
    let player2 = TicTacToePlayer {};

    let mut engine = GameEngine::<TicTacToeGame, TicTacToeMove, TicTacToePlayer, TicTacToeOracle>::new(
                                                 TicTacToeGame {board: [None; 9], last_player: 1},
                                                                 player1,
                                                 player2,
                                                 TicTacToeOracle{});

    engine.run();
}