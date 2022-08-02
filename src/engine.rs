use std::fmt::Debug;
use crate::game::Game;
use crate::oracle::Oracle;
use crate::player::Player;
use crate::r#move::Move;

pub struct GameEngine<GameType, MoveType, PlayerType, OracleType>
    where GameType: Game<GameType, MoveType> + Debug,
          MoveType: Move,
          PlayerType: Player<GameType, MoveType>,
          OracleType: Oracle<GameType, MoveType, PlayerType>
{
    initial_game_state: GameType,
    moves: Vec<MoveType>,
    current_game_state: GameType,
    // TODO: We maybe don't want to own these.
    player_x: PlayerType,
    player_o: PlayerType,
    oracle: OracleType
}

impl<GameType, MoveType, PlayerType, OracleType> GameEngine<GameType, MoveType, PlayerType, OracleType>
    where GameType: Game<GameType, MoveType> + Debug,
          MoveType: Move,
          PlayerType: Player<GameType, MoveType>,
          OracleType: Oracle<GameType, MoveType, PlayerType>
{
    pub fn new(initial_game_state: GameType, player1: PlayerType, player2: PlayerType, oracle: OracleType) -> GameEngine<GameType, MoveType, PlayerType, OracleType> {
        GameEngine {
            initial_game_state,
            moves: vec![],
            current_game_state: initial_game_state,
            player_x: player1,
            player_o: player2,
            oracle
        }
    }

    pub fn run(&mut self) {
        while !self.oracle.is_terminal(&self.current_game_state) {

            let next_player = match self.oracle.next_player(&self.current_game_state) {
                Some(0) => &self.player_x,
                Some(1) => &self.player_o,
                _ => return
            };

            let next_move = next_player.pick_move(&self.current_game_state);
            self.current_game_state = self.current_game_state.apply(&next_move);
            println!("{:?}", self.current_game_state);
            self.moves.push(next_move);
        }
    }
}