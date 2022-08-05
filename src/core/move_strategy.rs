use std::marker::PhantomData;
use crate::core::game::{Position, Oracle};
use crate::core::position_evaluator::{PositionEvaluation, PositionEvaluator};
use crate::core::r#move::{Move, MoveGenerator};

/**
 * A MoveStrategy is a strategy for choosing moves (it sounds tautological, but it's true).
 *
 * In simple games (e.g., nonograms), it might just be picking moves that are forced by the current position, or bailing out if one isn't obvious.
 * In more complex games, it may involve recursively trying moves multiple levels deep and picking the best move according to some algorithm -- e.g.,
 * Minimax algorithm.
 *
 * The MoveStrategy should be stateless. We'll work on adding support for stateful components (e.g., hashing of positions to prevent re-evaluating
 * them) later.
 *
 * The MoveStrategy works closely with the {@link MoveGenerator}, since MoveStrategy's rely on the MoveGenerator to produce the moves that should be
 * evaluated.
 */
pub trait MoveStrategy<PositionType, MoveType>
    where PositionType: Position<PositionType, MoveType>,
          MoveType: Move {
    fn choose_move(&self, game: &PositionType) -> MoveType;
}

/// A MaxMoveStrategy always picks the move that leads to the best-encountered position for the player.
/// It's not a very good strategy, because it might also permit the opponent to force a terrible position for the player.
/// But it's a good starter strategy for building out the game APIs without getting bogged down in algorithms like minimax.
pub struct MaxMoveStrategy<PositionType, MoveType, PositionEvaluatorType, MoveGeneratorType, OracleType>
    where PositionType: Position<PositionType, MoveType>,
          MoveType: Move,
          PositionEvaluatorType: PositionEvaluator<PositionType, MoveType>,
          MoveGeneratorType: MoveGenerator<PositionType, MoveType>,
          OracleType: Oracle<PositionType, MoveType>
{
    phantom_game: PhantomData<PositionType>,
    phantom_move: PhantomData<MoveType>,
    position_evaluator: PositionEvaluatorType,
    move_generator: MoveGeneratorType,
    oracle: OracleType
}

impl<PositionType, MoveType, PositionEvaluatorType, MoveGeneratorType, OracleType> MaxMoveStrategy<PositionType, MoveType, PositionEvaluatorType, MoveGeneratorType, OracleType>
    where PositionType: Position<PositionType, MoveType>,
          MoveType: Move,
          PositionEvaluatorType: PositionEvaluator<PositionType, MoveType>,
          MoveGeneratorType: MoveGenerator<PositionType, MoveType>,
          OracleType: Oracle<PositionType, MoveType>
{
    pub fn new(position_evaluator: PositionEvaluatorType, move_generator: MoveGeneratorType, oracle: OracleType) -> MaxMoveStrategy<PositionType, MoveType, PositionEvaluatorType, MoveGeneratorType, OracleType> {
        MaxMoveStrategy {
            phantom_game: PhantomData,
            phantom_move: PhantomData,
            position_evaluator,
            move_generator,
            oracle
        }
    }

    fn choose_move_recursive(&self, game: &PositionType) -> Option<(MoveType, PositionEvaluation)> {
        if self.oracle.is_terminal(game) {
            return None;
        }

        let moves = self.move_generator.get_moves(&game);
        let mut best_move = moves[0];
        let mut best_position_evaluation = PositionEvaluation::Losing;
        for r#move in moves {
            let new_position = &game.apply(&r#move);

            let recursive = self.choose_move_recursive(new_position);
            let new_position_evaluation = match recursive {
                None => self.position_evaluator.evaluate(new_position),
                Some((_, recursive_evaluation)) =>  recursive_evaluation
            };
            if new_position_evaluation > best_position_evaluation {
                best_position_evaluation = new_position_evaluation;
                best_move = r#move;
            }
        }
        Some((best_move, best_position_evaluation))
    }
}

impl<PositionType, MoveType, PositionEvaluatorType, MoveGeneratorType, OracleType> MoveStrategy<PositionType, MoveType>
for MaxMoveStrategy<PositionType, MoveType, PositionEvaluatorType, MoveGeneratorType, OracleType>
    where PositionType: Position<PositionType, MoveType>,
          MoveType: Move,
          PositionEvaluatorType: PositionEvaluator<PositionType, MoveType>,
          MoveGeneratorType: MoveGenerator<PositionType, MoveType>,
          OracleType: Oracle<PositionType, MoveType> {
    fn choose_move(&self, game: &PositionType) -> MoveType {
        match self.choose_move_recursive(game) {
            Some((best_move, _)) => best_move,
            None => panic!("Expected to be able to make a move!")
        }
    }
}

/// A MinimaxMoveStrategy always picks the move that maximizes the worst case scenario for the player.
///
/// Put another way, it assumes that the opponent will always pick the best move for them, and it picks the best move
/// it can under that assumption.
pub struct MinimaxMoveStrategy<PositionType, MoveType, PositionEvaluatorType, MoveGeneratorType, OracleType>
    where PositionType: Position<PositionType, MoveType>,
          MoveType: Move,
          PositionEvaluatorType: PositionEvaluator<PositionType, MoveType>,
          MoveGeneratorType: MoveGenerator<PositionType, MoveType>,
          OracleType: Oracle<PositionType, MoveType>
{
    phantom_game: PhantomData<PositionType>,
    phantom_move: PhantomData<MoveType>,
    position_evaluator: PositionEvaluatorType,
    move_generator: MoveGeneratorType,
    oracle: OracleType
}

impl<PositionType, MoveType, PositionEvaluatorType, MoveGeneratorType, OracleType> MinimaxMoveStrategy<PositionType, MoveType, PositionEvaluatorType, MoveGeneratorType, OracleType>
    where PositionType: Position<PositionType, MoveType>,
          MoveType: Move,
          PositionEvaluatorType: PositionEvaluator<PositionType, MoveType>,
          MoveGeneratorType: MoveGenerator<PositionType, MoveType>,
          OracleType: Oracle<PositionType, MoveType>
{
    pub fn new(position_evaluator: PositionEvaluatorType, move_generator: MoveGeneratorType, oracle: OracleType) -> MinimaxMoveStrategy<PositionType, MoveType, PositionEvaluatorType, MoveGeneratorType, OracleType> {
        MinimaxMoveStrategy {
            phantom_game: PhantomData,
            phantom_move: PhantomData,
            position_evaluator,
            move_generator,
            oracle
        }
    }

    fn choose_move_recursive(&self, game: &PositionType, maximizing_player: bool) -> Option<(MoveType, PositionEvaluation)> {
        if self.oracle.is_terminal(game) {
            return None;
        }

        let moves = self.move_generator.get_moves(&game);
        let mut best_move = moves[0];
        let mut best_position_evaluation = match maximizing_player { true => PositionEvaluation::Losing, false => PositionEvaluation::Winning };
        for r#move in moves {
            let new_position = &game.apply(&r#move);
            let recursive = self.choose_move_recursive(new_position, !maximizing_player);
            let new_position_evaluation = match recursive {
                None => self.position_evaluator.evaluate(new_position),
                Some((_, recursive_evaluation)) => recursive_evaluation
            };
            if maximizing_player && new_position_evaluation > best_position_evaluation {
                best_position_evaluation = new_position_evaluation;
                best_move = r#move;
            } else if !maximizing_player && new_position_evaluation < best_position_evaluation {
                best_position_evaluation = new_position_evaluation;
                best_move = r#move;
            }
        }
        Some((best_move, best_position_evaluation))
    }
}

impl<PositionType, MoveType, PositionEvaluatorType, MoveGeneratorType, OracleType> MoveStrategy<PositionType, MoveType>
for MinimaxMoveStrategy<PositionType, MoveType, PositionEvaluatorType, MoveGeneratorType, OracleType>
    where PositionType: Position<PositionType, MoveType>,
          MoveType: Move,
          PositionEvaluatorType: PositionEvaluator<PositionType, MoveType>,
          MoveGeneratorType: MoveGenerator<PositionType, MoveType>,
          OracleType: Oracle<PositionType, MoveType> {
    fn choose_move(&self, game: &PositionType) -> MoveType {
        match self.choose_move_recursive(game, true) {
            Some((best_move, _)) => best_move,
            None => panic!("Expected to be able to make a move!")
        }
    }
}