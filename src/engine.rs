use crate::game::Game;
use crate::player::Player;
use crate::r#move::Move;

pub struct GameEngine<GameType: Game<GameType, MoveType>, MoveType> {
    initial_game_state: GameType,
    moves: Vec<MoveType>,
    current_game_state: GameType,
    player1: Box<dyn Player<GameType, MoveType>>,
    player2: Box<dyn Player<GameType, MoveType>>,
}

impl<GameType, MoveType> GameEngine<GameType, MoveType>
    where GameType: Game<GameType, MoveType>,
          MoveType: Move
{
    pub fn new(initial_game_state: GameType, player1: Box<dyn Player<GameType, MoveType>>, player2: Box<dyn Player<GameType, MoveType>>) -> GameEngine<GameType, MoveType> {
        GameEngine {
            initial_game_state,
            moves: vec![],
            current_game_state: initial_game_state,
            player1,
            player2,
        }
    }
}