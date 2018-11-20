use std::collections::HashMap;
use std::collections::HashSet;

pub struct Board {
    pub(crate) dimensions: usize,
    pub(crate) size: Vec<i64>,
    pub(crate) stones: HashMap<Vec<i64>, i64>,
    pub(crate) direction_vectors: Vec<Vec<i64>>
}

fn possible_moves(board: &Board, player: i64) -> HashSet<Vec<i64>> {
    let mut result = HashSet::new();
    result.insert(vec![0 as i64]);
    for (v,p) in &board.stones {
        if *p == player {
            for i in free_for_player(v, board, player) {
                result.insert(i);
            }
        }
    }
    result.retain(|v| is_in_limit(v, board));
    return result;
}

fn is_in_limit(v: &Vec<i64>, board: &Board) -> bool {
    for i in 0..board.dimensions {
        if v[i] < 0 || v[i] >= board.size[i] {
            return false;
        }
    }
    return true;
}

pub fn free_for_player(stone: &Vec<i64>, board: &Board, player: i64) -> HashSet<Vec<i64>> {
    let mut result = HashSet::new();
    for v in &board.direction_vectors {
        let mut check_for = vector_add(&stone, &v);
        match board.stones.get(&check_for) {
            Some(x) => {
                if player != *x {
                    loop {
                        check_for = vector_add(&check_for, &v);
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
            },
            None => ()
        }
    }
    return result;
}

fn vector_add(v1: &Vec<i64>, v2: &Vec<i64>) -> Vec<i64> {
    let mut result = Vec::with_capacity(v1.len());
    for i in 0..v1.len() {
        result.push(v1[i] + v2[i]);
    }
    return result;
}
