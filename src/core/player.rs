use crate::core::game::Position;
use crate::core::move_strategy::MoveStrategy;
use crate::core::r#move::Move;

pub trait Player<PositionType: Position<PositionType, MoveType>, MoveType: Move> {
    fn pick_move(&self, game: &PositionType) -> MoveType;
}

pub struct DefaultPlayer<PositionType, MoveType>
    where PositionType: Position<PositionType, MoveType>,
          MoveType: Move {
    move_strategy: Box<dyn MoveStrategy<PositionType, MoveType>>,
}

impl<PositionType, MoveType> DefaultPlayer<PositionType, MoveType>
    where PositionType: Position<PositionType, MoveType>,
          MoveType: Move
{
    pub fn new(move_strategy: Box<dyn MoveStrategy<PositionType, MoveType>>) -> DefaultPlayer<PositionType, MoveType> {
        DefaultPlayer::<PositionType, MoveType> {
            move_strategy
        }
    }
}

impl<PositionType, MoveType> Player<PositionType, MoveType> for DefaultPlayer<PositionType, MoveType>
    where PositionType: Position<PositionType, MoveType>,
          MoveType: Move
{
    fn pick_move(&self, game: &PositionType) -> MoveType {
        self.move_strategy.choose_move(game)
    }
}

