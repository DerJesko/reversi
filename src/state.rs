use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

use board;
use position::Position;

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

impl GameState {
    pub fn new(total_players: i64, board: board::Board) -> GameState {
        GameState {
            current_player: 0,
            total_players,
            board,
        }
    }

    pub fn do_move(&mut self, set_stone: Position) {
        self.board.do_move(self.current_player, set_stone);
        self.current_player = (self.current_player + 1) % self.total_players;
    }

    pub fn current_player(&self) -> i64 {
        self.current_player
    }

    pub fn remove_current_player(&mut self) {
        self.board.remove_stones_of_player(self.current_player)
    }

    pub fn possible_moves(&self) -> HashSet<Position> {
        self.board.possible_moves(self.current_player)
    }

    pub fn current_player_has_stone(&self) -> bool {
        self.board.has_stone(self.current_player())
    }

    pub fn is_over(&self) -> bool {
        self.board.is_over()
    }

    pub fn who_won(&self) -> i64 {
        self.board.who_won()
    }
}

impl FromStr for GameState {
    type Err = RevParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s.split_whitespace().map(|e| e.parse::<i64>().unwrap());

        let current_player = numbers.next().unwrap();

        let total_players = numbers.next().unwrap();

        let dimensions = numbers.next().unwrap() as usize;

        assert!(current_player < total_players);
        assert!(total_players > 1);
        assert!(dimensions > 0, "too little dimensions");

        let mut size = Vec::with_capacity(dimensions);
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
            let mut current_stone = Vec::with_capacity(dimensions);
            for _ in 0..dimensions {
                current_stone.push(numbers.next().unwrap());
            }
            stones.insert(Position::new(current_stone), current_stone_player);
        }

        let state = GameState {
            current_player: current_player,
            total_players: total_players,
            board: board::Board::new(size, stones),
        };

        Ok(state)
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.current_player, self.total_players, self.board
        )
    }
}
