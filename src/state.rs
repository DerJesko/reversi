use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

use board;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevParseError;

impl fmt::Display for RevParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.description())
    }
}

impl Error for RevParseError {
    fn description(&self) -> &str {
        "invalid reversi file syntax"
    }
}

pub struct GameState {
    current_player: i64,
    total_players: i64,
    board: board::Board,
}

impl FromStr for GameState {
    type Err = RevParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s.split_whitespace().map(|e| e.parse::<i64>().unwrap());

        let current_player = numbers.next().unwrap();
        println!("current player: {}", current_player);

        let total_players = numbers.next().unwrap();
        println!("total players: {}", total_players);

        let dimensions = numbers.next().unwrap();

        println!("dimensions: {}", dimensions);

        assert!(current_player < total_players);
        assert!(total_players > 1);
        assert!(dimensions > 0, "too little dimensions");

        let mut size = Vec::with_capacity(dimensions as usize);
        for _ in 0..dimensions {
            size.push(numbers.next().unwrap());
        }

        let mut stones = HashMap::new();
        loop {
            let current_stone_player;
            match numbers.next() {
                Some(x) => current_stone_player = x,
                None => break,
            }
            println!("New stone from player {}", current_stone_player);
            let mut current_stone = Vec::with_capacity(dimensions as usize);
            for _ in 0..dimensions {
                current_stone.push(numbers.next().unwrap());
            }
            stones.insert(current_stone, current_stone_player);
        }

        let state = GameState {
            current_player: current_player,
            total_players: total_players,
            board: board::Board {
                dimensions: dimensions,
                size: size,
                stones: stones,
            },
        };

        Ok(state)
    }
}
