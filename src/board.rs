use std::collections::HashMap;

pub struct Board {
    pub(crate) dimensions: i64,
    pub(crate) size: Vec<i64>,
    pub(crate) stones: HashMap<Vec<i64>, i64>,
}
