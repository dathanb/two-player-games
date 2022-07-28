use crate::engine::GameEngine;
use crate::tictactoe::{TicTacToeGame, TicTacToeMove, TicTacToePlayer};
use crate::tictactoe::Piece::X;

mod player;
mod r#move;
mod movegenerator;
mod engine;
mod game;
mod tictactoe;

fn main() {
    let player1 = TicTacToePlayer {};
    let player2 = TicTacToePlayer {};

    let engine = GameEngine::<TicTacToeGame, TicTacToeMove>::new(TicTacToeGame {board: [None; 9], last_player: 1},
                                                                 Box::new(player1), Box::new(player2));
}