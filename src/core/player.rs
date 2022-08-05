use crate::core::game::Position;
use crate::core::r#move::Move;

pub trait Player<PositionType: Position<PositionType, MoveType>, MoveType: Move> {
    fn pick_move(&self, game: &PositionType) -> MoveType;
}