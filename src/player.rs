use crate::game::Game;
use crate::r#move::Move;

pub trait Player<GameType: Game<GameType, MoveType>, MoveType: Move> {
}