use std::cmp::Ordering;
use std::cmp::Ordering::Less;
use crate::game::Game;
use crate::r#move::Move;

/// An evaluation of a game position. For sophisticated position evaluators and strategies, this should incorporate
/// any information discovered about positions reachable from the current position, especially positions that can be
/// forced by any player.
pub enum PositionEvaluation {
    /// Winning indicates that the game position leads to a forced win for the current player if the player
    /// plays perfectly.
    Winning,
    /// Losing indicates that the game position leads to a forced loss for the current player if the opposing player(s)
    /// play(s) perfectly.
    Losing,
    /// Estimate means that it's not clear whether the position is winning or losing for the current player, and
    /// contains a numerical estimate of the advantage to the player from the position. Positive means the
    /// player appears to be at an advantage, and negative means that the player appears to be at a disadvantage.
    Estimate(f64)
}

impl PartialOrd for PositionEvaluation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (PositionEvaluation::Winning, PositionEvaluation::Winning) => Some(Ordering::Equal),
            (PositionEvaluation::Winning, _) => Some(Ordering::Greater),
            (_, PositionEvaluation::Winning) => Some(Ordering::Less),
            (PositionEvaluation::Losing, PositionEvaluation::Losing) => Some(Ordering::Equal),
            (PositionEvaluation::Losing, _) => Some(Ordering::Less),
            (_, PositionEvaluation::Losing) => Some(Ordering::Greater),
            (PositionEvaluation::Estimate(x), PositionEvaluation::Estimate(y)) => x.partial_cmp(y)
        }
    }
}

impl PartialEq<Self> for PositionEvaluation {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PositionEvaluation::Winning, PositionEvaluation::Winning) => true,
            (PositionEvaluation::Losing, PositionEvaluation::Losing) => true,
            (PositionEvaluation::Estimate(x), PositionEvaluation::Estimate(y)) => x== y,
            _ => false
        }
    }
}

pub trait PositionEvaluator<GameType, MoveType>
    where GameType: Game<GameType, MoveType>,
          MoveType: Move {

    /// Evaluate a position. This evaluation should incorporate only the position itself, and not attempt to
    /// analyze transitive positions.
    fn evaluate(&self, position: &GameType) -> PositionEvaluation;
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use crate::positionevaluator::PositionEvaluation;

    #[test]
    pub fn test_partial_ord_for_position_evaluation() {
        let ordering = PositionEvaluation::Winning.partial_cmp(&PositionEvaluation::Winning);
        assert_eq!(ordering.unwrap(), Ordering::Equal);
        let ordering = PositionEvaluation::Winning.partial_cmp(&PositionEvaluation::Estimate(5.0));
        assert_eq!(ordering.unwrap(), Ordering::Greater);
        let ordering = PositionEvaluation::Winning.partial_cmp(&PositionEvaluation::Losing);
        assert_eq!(ordering.unwrap(), Ordering::Greater);
        let ordering = PositionEvaluation::Estimate(5.0).partial_cmp(&PositionEvaluation::Winning);
        assert_eq!(ordering.unwrap(), Ordering::Less);
        let ordering = PositionEvaluation::Estimate(5.0).partial_cmp(&PositionEvaluation::Estimate(-5.0));
        assert_eq!(ordering.unwrap(), Ordering::Greater);
        let ordering = PositionEvaluation::Estimate(5.0).partial_cmp(&PositionEvaluation::Estimate(5.0));
        assert_eq!(ordering.unwrap(), Ordering::Equal);
        let ordering = PositionEvaluation::Estimate(5.0).partial_cmp(&PositionEvaluation::Estimate(10.0));
        assert_eq!(ordering.unwrap(), Ordering::Less);
        let ordering = PositionEvaluation::Estimate(5.0).partial_cmp(&PositionEvaluation::Losing);
        assert_eq!(ordering.unwrap(), Ordering::Greater);
        let ordering = PositionEvaluation::Losing.partial_cmp(&PositionEvaluation::Winning);
        assert_eq!(ordering.unwrap(), Ordering::Less);
        let ordering = PositionEvaluation::Losing.partial_cmp(&PositionEvaluation::Estimate(0.0));
        assert_eq!(ordering.unwrap(), Ordering::Less);
        let ordering = PositionEvaluation::Losing.partial_cmp(&PositionEvaluation::Losing);
        assert_eq!(ordering.unwrap(), Ordering::Equal);
    }
}