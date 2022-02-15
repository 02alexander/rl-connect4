extern crate fastrand;

mod connect4;
mod search;
mod evaluators;
mod matchmaker;

use connect4::{Connect4, Player, BOARD_HEIGHT, BOARD_WIDTH, GameState, Action};
use evaluators::{Evaluator, SimpleEval, LinesEval};
use search::{minimax_action, minimax, abpruning_action, abpruning_best_action};
use std::io::{self, BufRead};
use matchmaker::{Agent, MatchMaker};

struct MinimaxAgent<T> {
    evaluator: T,
    depth: u32
}

impl<T: Evaluator> MinimaxAgent<T> {
    pub fn new(evaluator: T, depth: u32) -> Self {
        MinimaxAgent {
            evaluator,
            depth
        }
    }
}

impl<T: Evaluator> Agent for MinimaxAgent<T> {
    fn get_action(&self, board: &Connect4) -> Action {
        abpruning_best_action(board, self.depth, &self.evaluator)
    }
    fn set_player(&mut self, player: Player) {
        self.evaluator.set_player(player);
    }
}

fn main() {
    let mut agenta = MinimaxAgent::new(LinesEval::new(Player::Red), 4);
    //let mut agenta = MinimaxAgent::new(SimpleEval::new(Player::Red), 4);
    let mut agentb = MinimaxAgent::new(SimpleEval::new(Player::Red), 4);
    let mut mm = MatchMaker::new();
    mm.add_agent(Box::new(agenta));
    mm.add_agent(Box::new(agentb));
    mm.play_n_games(100);
    println!("{:?}", mm.get_scores());
    
    //user_vs_ai();
}



fn get_move_from_minimax<T: Evaluator>(board: &Connect4, evaluator: &T) -> Action {
    abpruning_best_action(board, 5, evaluator)
}

// returns (action, is_reverse)
fn get_move_from_user(board: &Connect4) -> (Action, bool) {
    let mut stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.as_bytes()[0] == 'z' as u8 {
            return (0, true);
        } else if let Ok(a) = line.parse::<usize>() {
            if a >= 0 && a < connect4::BOARD_WIDTH {
                if !board.is_valid_move(a) {
                    println!("Column alread full");
                    continue;
                }
                return (a, false);
            } else {
                println!("Not in range 0..{}", connect4::BOARD_WIDTH);    
            }
        } else {
            println!("Invalid input: try again");
        }
    }
    panic!("Failed to get input from user");
}

fn user_vs_user() {
    let mut board = Connect4::new();
    let evaluator = SimpleEval::new(!board.cur_player);
    loop {
        println!("{:?}", board);
        println!("{:?}", board.game_state);
        let (action, reverse) = get_move_from_user(&board);
        if reverse {
            board.reverse_last_move();
        } else {
            board.play_move(action);
            match board.game_state {
                GameState::Draw => {
                    println!("Draw");
                }
                GameState::InProgress => {},
                GameState::Won(player) => {
                    println!("{:?} won", player);   
                }
            }
        }
    }
}

fn user_vs_ai() {
    let mut board = Connect4::new();
    let levaluator = LinesEval::new(board.cur_player);
    let evaluator = SimpleEval::new(!board.cur_player);
    loop {
        println!("{:?}", board);
        println!("{:?}", board.game_state);
        println!("{:?}", levaluator.value(&board));
        let (action, reverse) = get_move_from_user(&board);
        if reverse {
            board.reverse_last_move();
            board.reverse_last_move();
            continue
        } else {
            board.play_move(action);
            match board.game_state {
                GameState::Draw => {
                    println!("Draw");
                }
                GameState::InProgress => {},
                GameState::Won(player) => {
                    println!("{:?} won", player);   
                }
            }
        }
        let action = get_move_from_minimax(&board, &evaluator);
        board.play_move(action);
        match board.game_state {
            GameState::Draw => {
                println!("Draw");
            }
            GameState::InProgress => {},
            GameState::Won(player) => {
                println!("{:?} won", player);   
            }
        }
    }

}