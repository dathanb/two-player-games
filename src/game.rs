
/**
Represents the state of a game at any point in time.
*/
pub trait Game<GameType, MoveType>: Copy {
    // public interface Game<GameType extends Game<GameType, MoveType>, MoveType extends Move> {
    /**
     * Compose the given move with the current game state, returning a new game state.
     * @param move The move to apply
     * @return The new game state resulting from applying the move to the current game state.
     */
    // GameType apply(MoveType move);
    fn apply(&self, m: MoveType) -> GameType;

    /**
     * Whether the game state represents a terminal position.
     * @return true if the game state is a terminal (play should not continue afterward);
     *         false if the game state is an intermediate position (play should continue from this state)
     */
    fn is_terminal(&self) -> bool;
}
