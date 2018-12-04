use std::ops::Index;

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct Position {v: Vec<i64>}

impl Position{
    pub fn new(v: Vec<i64>) -> Position {
        return Position{v: v};
    }

    pub fn add(&self, p: &Position) -> Position {
        return Position{v: vec![]}
    }
}

impl Index<usize> for Position {
    type Output = i64;

    fn index(&self, index: usize) -> &i64 {
        return &self.v[index];
    }
}