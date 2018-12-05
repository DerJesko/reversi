use std::ops::Index;

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct Position {
    v: Vec<i64>,
}

impl Position {
    pub fn new(v: Vec<i64>) -> Position {
        return Position { v: v };
    }

    pub fn add(&self, p: &Position) -> Position {
        let mut result = Vec::with_capacity(self.v.len());
        for i in 0..self.v.len() {
            result.push(self.v[i] + p.v[i]);
        }
        return Position { v: result };
    }

    pub fn add_to(&mut self, p: &Position) {
        for i in 0..self.v.len() {
            self.v[i] += p.v[i];
        }
    }
}

impl Clone for Position {
    fn clone(&self) -> Position {
        let mut result = Position {
            v: Vec::with_capacity(self.v.len()),
        };
        for i in &self.v {
            result.v.push(*i);
        }
        return result;
    }
}

impl Index<usize> for Position {
    type Output = i64;

    fn index(&self, index: usize) -> &i64 {
        return &self.v[index];
    }
}
