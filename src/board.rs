use std::collections::HashMap;
use std::collections::HashSet;

use position::Position;

pub struct Board {
    pub(crate) dimensions: usize,
    pub(crate) size: Vec<i64>,
    pub(crate) stones: HashMap<Position, i64>,
    pub(crate) direction_vectors: Vec<Position>,
}
/*
fn do_move(board: &mut Board, player: i64, set_stone: Vec<i64>) {
    for v in &board.direction_vectors {
        let mut check_for = set_stone;
        let mut insert_if_correct = HashSet::new();
        let mut correct_direction = false;
        loop {
            check_for = vector_add(&v, &check_for);
            match board.stones.get(&check_for) {
                Some(x) => {
                    if player != *x {
                        insert_if_correct.insert(check_for);
                    } else {
                        correct_direction = true;
                        break;
                    }
                }
                None => {
                    break;
                },
            }
        }

        if correct_direction {
            for i in insert_if_correct {
                board.stones.insert(i, player);
            }
        }
    }
    board.stones.insert(set_stone, player);
    return;
}
*/
fn possible_moves(board: &Board, player: i64) -> HashSet<Position> {
    let mut result = HashSet::new();
    for (v, p) in &board.stones {
        if *p == player {
            for i in free_for_player(v, board, player) {
                result.insert(i);
            }
        }
    }
    result.retain(|v| is_in_limit(v, board));
    return result;
}

fn is_in_limit(v: &Position, board: &Board) -> bool {
    for i in 0..board.dimensions {
        if v[i] < 0 || v[i] >= board.size[i] {
            return false;
        }
    }
    return true;
}

pub fn free_for_player(stone: &Position, board: &Board, player: i64) -> HashSet<Position> {
    let mut result = HashSet::new();
    for v in &board.direction_vectors {
        let mut check_for = &stone.add(&v);
        match board.stones.get(&check_for) {
            Some(x) => {
                if player != *x {
                    loop {
                        check_for = &check_for.add(&v);
                        match board.stones.get(&check_for) {
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
