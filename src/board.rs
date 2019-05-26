use std::collections::{HashMap, HashSet};
use std::fmt;

use position::Position;

pub struct Board {
    dimensions: usize,
    size: Vec<i64>,
    stones: HashMap<Position, i64>,
    direction_vectors: Vec<Position>,
}

impl Board {
    pub fn new(size: Vec<i64>, stones: HashMap<Position, i64>) -> Board {
        let dimensions = size.len();
        let direction_vectors = Board::generate_direction_vectors(dimensions as u32);
        return Board {
            dimensions: dimensions,
            size: size,
            stones: stones,
            direction_vectors: direction_vectors,
        };
    }

    fn generate_direction_vectors(dimensions: u32) -> Vec<Position> {
        let size = 3_i64.pow(dimensions);
        let mut result = Vec::with_capacity(size as usize);
        for i in 0..size {
            let mut current_vec = Vec::with_capacity(dimensions as usize);
            let mut current_val = i;
            let mut current_mod;
            for _ in 0..dimensions {
                current_mod = current_val % 3;
                current_vec.push(current_mod - 1);
                current_val /= 3;
            }
            result.push(Position::new(current_vec));
        }
        return result;
    }

    pub fn do_move(&mut self, player: i64, set_stone: Position) {
        for v in &self.direction_vectors {
            let mut check_for = set_stone.clone();
            let mut insert_if_correct = HashSet::new();
            let mut correct_direction = false;
            loop {
                check_for.add_to(&v);
                match self.stones.get(&check_for) {
                    Some(x) => {
                        if player != *x {
                            insert_if_correct.insert(check_for.clone());
                        } else {
                            correct_direction = true;
                            break;
                        }
                    }
                    None => {
                        break;
                    }
                }
            }

            if correct_direction {
                for i in insert_if_correct {
                    self.stones.insert(i, player);
                }
            }
        }
        self.stones.insert(set_stone, player);
        return;
    }

    pub fn is_over(&self) -> bool {
        let mut possible_winner_option = None;
        for (_, player) in &self.stones {
            match possible_winner_option {
                None => {
                    possible_winner_option = Some(player);
                }
                Some(possible_winner) => {
                    if possible_winner != player {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    pub fn who_won(&self) -> i64 {
        for (_, player) in &self.stones {
            return *player;
        }
        panic!("Your board must be empty!")
    }

    pub fn has_stone(&self, player: i64) -> bool {
        for (_, p) in &self.stones {
            if *p == player {
                return true;
            }
        }
        false
    }

    pub fn possible_moves(&self, player: i64) -> HashSet<Position> {
        let mut result = HashSet::new();
        for (v, p) in &self.stones {
            if *p == player {
                for i in self.free_for_player(v, player) {
                    result.insert(i);
                }
            }
        }
        result.retain(|v| self.is_in_limit(v));
        return result;
    }

    fn is_in_limit(&self, v: &Position) -> bool {
        for i in 0..self.dimensions {
            if v[i] < 0 || v[i] >= self.size[i] {
                return false;
            }
        }
        return true;
    }
    fn free_for_player(&self, stone: &Position, player: i64) -> HashSet<Position> {
        let mut result = HashSet::new();
        for v in &self.direction_vectors {
            let mut check_for = stone.add(&v);
            match self.stones.get(&check_for) {
                Some(x) => {
                    if player != *x {
                        loop {
                            check_for = check_for.add(&v);
                            match self.stones.get(&check_for) {
                                Some(y) => {
                                    if *y == player {
                                        break;
                                    }
                                }
                                None => {
                                    result.insert(check_for);
                                    break;
                                }
                            }
                        }
                    }
                }
                None => (),
            }
        }
        return result;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.dimensions)?;
        for i in 0..self.dimensions {
            write!(f, "{} ", self.size[i])?;
        }
        write!(f, "\n")?;
        for (k, v) in &self.stones {
            write!(f, "{} {}\n", v, k)?;
        }
        write!(f, "")
    }
}
