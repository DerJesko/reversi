use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

mod board;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please select a reversi file");
        return;
    }

    let path = Path::new(&args[1]);

    let mut file = File::open(&path).expect("couldn't open file");

    let mut s = String::new();
    file.read_to_string(&mut s).expect("couldn't read file");
}
