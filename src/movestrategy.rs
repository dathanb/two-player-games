use std::marker::PhantomData;
use crate::game::Game;
use crate::movegenerator::MoveGenerator;
use crate::oracle::Oracle;
use crate::positionevaluator::{PositionEvaluation, PositionEvaluator};
use crate::r#move::Move;

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
pub trait MoveStrategy<GameType, MoveType>
    where GameType: Game<GameType, MoveType>,
          MoveType: Move {
    fn choose_move(&self, game: &GameType) -> MoveType;
}

/// A MaxMoveStrategy always picks the move that leads to the best-encountered position for the player.
/// It's not a very good strategy, because it might also permit the opponent to force a terrible position for the player.
/// But it's a good starter strategy for building out the game APIs without getting bogged down in algorithms like minimax.
pub struct MaxMoveStrategy<GameType, MoveType, PositionEvaluatorType, MoveGeneratorType, OracleType>
    where GameType: Game<GameType, MoveType>,
          MoveType: Move,
          PositionEvaluatorType: PositionEvaluator<GameType, MoveType>,
          MoveGeneratorType: MoveGenerator<GameType, MoveType>,
          OracleType: Oracle<GameType, MoveType>
{
    phantom_game: PhantomData<GameType>,
    phantom_move: PhantomData<MoveType>,
    position_evaluator: PositionEvaluatorType,
    move_generator: MoveGeneratorType,
    oracle: OracleType
}

impl<GameType, MoveType, PositionEvaluatorType, MoveGeneratorType, OracleType> MaxMoveStrategy<GameType, MoveType, PositionEvaluatorType, MoveGeneratorType, OracleType>
    where GameType: Game<GameType, MoveType>,
          MoveType: Move,
          PositionEvaluatorType: PositionEvaluator<GameType, MoveType>,
          MoveGeneratorType: MoveGenerator<GameType, MoveType>,
          OracleType: Oracle<GameType, MoveType>
{
    pub fn new(position_evaluator: PositionEvaluatorType, move_generator: MoveGeneratorType, oracle: OracleType) -> MaxMoveStrategy<GameType, MoveType, PositionEvaluatorType, MoveGeneratorType, OracleType> {
        MaxMoveStrategy {
            phantom_game: PhantomData,
            phantom_move: PhantomData,
            position_evaluator,
            move_generator,
            oracle
        }
    }

    fn choose_move_recursive(&self, game: &GameType) -> Option<(MoveType, PositionEvaluation)> {
        if self.oracle.is_terminal(game) {
            return None;
        }

        let moves = self.move_generator.get_moves(&game);
        let mut best_move = moves[0];
        let mut best_position_evaluation = PositionEvaluation::Losing;
        for r#move in moves {
            let new_position = &game.apply(&r#move);
            let new_position_evaluation;

            let recursive = self.choose_move_recursive(new_position);
            match recursive {
                None => new_position_evaluation = self.position_evaluator.evaluate(new_position),
                Some((_, recursive_evaluation)) =>  if recursive_evaluation >= best_position_evaluation { best_move = r#move }
            }
        }
        Some((best_move, best_position_evaluation))
    }
}

impl<GameType, MoveType, PositionEvaluatorType, MoveGeneratorType, OracleType> MoveStrategy<GameType, MoveType>
for MaxMoveStrategy<GameType, MoveType, PositionEvaluatorType, MoveGeneratorType, OracleType>
    where GameType: Game<GameType, MoveType>,
          MoveType: Move,
          PositionEvaluatorType: PositionEvaluator<GameType, MoveType>,
          MoveGeneratorType: MoveGenerator<GameType, MoveType>,
          OracleType: Oracle<GameType, MoveType> {
    fn choose_move(&self, game: &GameType) -> MoveType {
        match self.choose_move_recursive(game) {
            Some((best_move, _)) => best_move,
            None => panic!("Expected to be able to make a move!")
        }
    }
}