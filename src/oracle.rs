use crate::game::Game;
use crate::player::Player;
use crate::r#move::Move;

/// An Oracle is a class that's not part of any Player impl that's capable of some reasoning about the state
/// of a game, like who should play next, whether the game is over, and who won the game. It's consulted by
/// the game engine to control the flow of the game.
pub trait Oracle<GameType, MoveType>
    where GameType: Game<GameType, MoveType>,
          MoveType: Move
{
    /// Returns the index of the next player who should play, if it can be uniquely determined.
    /// If for some reason the oracle can't uniquely determine the next player (e.g., because the position is terminal),
    /// it may return None.
    fn next_player(&self, game: &GameType) -> Option<usize>;

    /**
     * Whether the game state represents a terminal position.
     * @return true if the game state is a terminal (play should not continue afterward);
     *         false if the game state is an intermediate position (play should continue from this state)
     */
    fn is_terminal(&self, game: &GameType) -> bool;
}