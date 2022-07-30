use crate::game::Game;
use crate::oracle::Oracle;
use crate::player::Player;
use crate::r#move::Move;
use crate::TicTacToeOracle;

pub struct GameEngine<GameType, MoveType, PlayerType, OracleType>
    where GameType: Game<GameType, MoveType>,
          MoveType: Move,
          PlayerType: Player<GameType, MoveType>,
          OracleType: Oracle<GameType, MoveType, PlayerType>
{
    initial_game_state: GameType,
    moves: Vec<MoveType>,
    current_game_state: GameType,
    // TODO: We probably don't want to own these.
    player1: PlayerType,
    player2: PlayerType,
    oracle: OracleType
}

impl<GameType, MoveType, PlayerType, OracleType> GameEngine<GameType, MoveType, PlayerType, OracleType>
    where GameType: Game<GameType, MoveType>,
          MoveType: Move,
          PlayerType: Player<GameType, MoveType>,
          OracleType: Oracle<GameType, MoveType, PlayerType>
{
    pub fn new(initial_game_state: GameType, player1: PlayerType, player2: PlayerType, oracle: OracleType) -> GameEngine<GameType, MoveType, PlayerType, OracleType> {
        GameEngine {
            initial_game_state,
            moves: vec![],
            current_game_state: initial_game_state,
            player1,
            player2,
            oracle
        }
    }

    pub fn run(&mut self) {
        let next_move = self.oracle.next_player(&self.current_game_state).pick_move(&self.current_game_state);
        self.current_game_state = self.current_game_state.apply(&next_move);
        self.moves.push(next_move);
    }

    fn next_player_for(&self, game: &GameType) -> &PlayerType {
        todo!()
    }
}