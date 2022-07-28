
/**
 * A MoveStrategy is a strategy for choosing moves (it sounds tautologic, but it's true).
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
trait MoveStrategy<GameType, MoveType> {
    fn choose_move(game: GameType) -> Option<MoveType>;
}
