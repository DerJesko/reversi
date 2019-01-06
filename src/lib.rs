mod board;
pub mod position;
pub mod state;

#[cfg(test)]
mod tests {
    use state;
    use std::fs;
    use std::str::FromStr;
    #[test]
    fn creation() {
        let contents =
            fs::read_to_string("test.rev").expect("Something went wrong reading the file");
        println!("{}", contents);
        let s = state::GameState::from_str(&contents).unwrap();
        println!("{}", s);
    }
}
