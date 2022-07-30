use crate::game::Game;
use crate::r#move::Move;

pub trait MoveGenerator<GameType: Game<GameType, MoveType>, MoveType: Move> {
    /// Produce a list of all possible proximal moves for a given game position.
    fn get_moves(&self, game: &GameType) -> Vec<MoveType>;
}