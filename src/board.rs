use std::collections::HashMap;
use std::str::FromStr;
use std::error::Error;
use std::fmt;

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

pub struct Board {
    stones: HashMap<Vec<i64>, u8>
}

impl FromStr for Board {
    type Err = RevParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err(RevParseError)
    }
}
