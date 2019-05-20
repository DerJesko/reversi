pub mod board;
pub mod position;
mod state;

use std::env;
use std::fs;
use std::io;
use std::str::FromStr;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let mut game_state;
    if args.len() == 1 {
        println!("The functionality to start a new game has not yet been implemented");
        // TODO
        return;
    } else if args.len() == 2 {
        let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
        game_state = state::GameState::from_str(&contents).unwrap();
    } else {
        println!("These command line options are not valid.\nStart the program without anything in order to start a new game or add a savestate to continue with that.", );
        return;
    }

    while !game_state.is_over() {
        println!("The current board looks like this:\n{}", game_state);
        let possible_moves: Vec<position::Position> =
            game_state.possible_moves().iter().cloned().collect();
        if possible_moves.len() == 0 {
            println!(
                "You can't do any more moves. You lost. Your stones will be removed from the board"
            );
            return;
        }
        println!(
            "All the possible moves for player {} are:",
            game_state.current_player()
        );
        let mut j = 0;
        for i in &possible_moves {
            println!("{}: {}", j, i);
            j += 1;
        }
        let mut selected_move;
        loop {
            println!("Which move do you want to make");
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    match input.parse::<i64>() {
                        Ok(n) => selected_move = n,
                        Err(_) => continue,
                    }
                    if selected_move >= 0 && selected_move < (possible_moves.len() as i64) {
                        break;
                    }
                }
                Err(_) => {
                    println!("This is not a correct move");
                }
            }
        }
        game_state.do_move(possible_moves[selected_move as usize].clone());
    }
}
