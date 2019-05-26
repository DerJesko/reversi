pub mod board;
pub mod position;
mod state;

use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::str::FromStr;

fn save_state(state: &state::GameState) {
    loop {
        println!("What file do you want to save to?");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Err(e) => {
                println!("received error '{}' when trying to read your input", e);
                continue;
            }
            _ => {}
        }
        let mut output = match File::create(input.clone()) {
            Ok(o) => o,
            Err(e) => {
                println!(
                    "received error '{}' when trying to create file '{}.",
                    e, input
                );
                continue;
            }
        };
        match write!(output, "{}", state) {
            Ok(_) => break,
            Err(e) => {
                println!(
                    "received error '{}' when trying to write to file '{}.",
                    e, input
                );
                continue;
            }
        };
    }
}

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
        print!("The current board looks like this:\n{}", game_state);
        let possible_moves: Vec<position::Position> =
            game_state.possible_moves().iter().cloned().collect();
        if possible_moves.len() == 0 {
            if game_state.current_player_has_stone() {
                println!(
                    "Player {} has no possible move and therefore loses.\nAll of its stones will be removed",//TODO
                    game_state.current_player()
                );
            }
            continue;
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
            println!("Type a number in order to make the move or 's' to save the game");
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    match input.split_whitespace().next() {
                        Some(s) => {
                            if s.to_lowercase() == "s" {
                                save_state(&game_state);
                            }
                            match s.parse::<i64>() {
                                Ok(n) => selected_move = n,
                                Err(_) => continue,
                            }
                        }
                        None => {
                            println!("Please enter something other than whitespace");
                            continue;
                        }
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
    print!("The game state is\n{}", game_state);
    println!("Player {} won", game_state.who_won());
}
