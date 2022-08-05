use std::fmt::Debug;

use crate::core::game::Position;

pub trait Move: Copy + Debug {
}

pub trait MoveGenerator<PositionType: Position<PositionType, MoveType>, MoveType: Move> {
    /// Produce a list of all possible proximal moves for a given game position.
    fn get_moves(&self, game: &PositionType) -> Vec<MoveType>;
}
