use std::collections::HashMap;

pub struct Board {
    pub(crate) dimensions: i64,
    pub(crate) size: Vec<i64>,
    pub(crate) stones: HashMap<Vec<i64>, i64>,
    pub(crate) direction_vectors: Vec<Vec<i64>>
}

pub fn free_for_player(stone: Vec<i64>, board: Board, player: i64) -> Vec<Vec<i64>> {
    let mut result = Vec::new();
    for v in board.direction_vectors {
        let mut check_for = vector_add(&stone, &v);
        match board.stones.get(&check_for) {
            Some(x) => {
                if player == *x {
                    continue;
                }
                loop {
                    check_for = vector_add(&check_for, &v);
                    match board.stones.get(&check_for) {
                        Some(y) => {
                            if *y == player {
                                break;
                            }
                        }
                        None => {
                            result.push(check_for);
                            break;
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
