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

    let board = contents.parse::<state::GameState>();

}

fn generate_direction_vectors(dimensions: u32) -> Vec<Vec<i64>> {
    let size = 3_i64.pow(dimensions);
    let mut result = Vec::with_capacity(size as usize);
    for i in 0..size {
        let mut current_vec = Vec::with_capacity(dimensions as usize);
        let mut current_val = i;
        let mut current_mod;
        for j in 0..dimensions {
            current_mod = current_val % 3;
            current_vec.push(current_mod - 1);
            current_val /= 3;
        }
        result.push(current_vec)
    }
    result
}
