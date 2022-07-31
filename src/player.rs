use crate::game::Game;
use crate::r#move::Move;

pub trait Player<GameType: Game<GameType, MoveType>, MoveType: Move> {
    // TODO: Instead of Optional<MoveType>, refactor this to return just MoveType. If we can't decide on a move, we'll
    //       panic instead of returning None. And if passing is an acceptable move type, then that should be encoded
    //       into the move type directly (e.g., making the move type an enum with Pass as one of the entries)
    fn pick_move(&self, game: &GameType) -> Option<MoveType>;
}