use std::fmt::Debug;

use crate::core::game::Game;

pub trait Move: Copy + Debug {
}

pub trait MoveGenerator<GameType: Game<GameType, MoveType>, MoveType: Move> {
    /// Produce a list of all possible proximal moves for a given game position.
    fn get_moves(&self, game: &GameType) -> Vec<MoveType>;
}
