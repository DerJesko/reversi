use std::env;
use std::fs;

mod board;
mod state;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        println!("Please select a reversi file");
        return;
    }

    let contents = fs::read_to_string(&args[0]).expect("blub");

    let state = contents.parse::<state::GameState>().unwrap();
    
    println!("{:?}",board::free_for_player(&vec![1,1], &state.board, 0));
}


