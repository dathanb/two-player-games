use crate::core::game::Game;
use crate::core::r#move::Move;

pub trait Player<GameType: Game<GameType, MoveType>, MoveType: Move> {
    fn pick_move(&self, game: &GameType) -> MoveType;
}