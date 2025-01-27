use regex::Regex;
use std::{collections::VecDeque, str::FromStr};
use tetrominos::Tetromino;

mod parsing;
mod pattern;

pub use parsing::Parsing;
pub use pattern::Pattern;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub struct Queue {
    pub sequence: VecDeque<Pattern>,
    pub hold: Option<Tetromino>,
    pub can_swap: bool,
}

impl Queue {
    pub fn next(&mut self) -> Tetromino {
        let mut pattern = self.sequence.pop_front().unwrap_or_default();
        if let Some(variant) = pattern.draw() {
            self.sequence.push_front(pattern);
            return variant;
        }
        self.next()
    }
}

impl Parsing for Queue {
    fn parse<T: Into<String>>(value: T) -> Result<Self, String> {
        let input = value.into();

        let hold_pattern = r"^((?<hold>.):)?";
        let sequence_pattern = r"(?<pattern>((\[.+?\])|\*)(p\d+)?)";

        let hold = Regex::new(format!("{hold_pattern}{sequence_pattern}").as_str())
            .map_err(|_| "Invalid regex pattern!")?
            .captures(input.as_str())
            .ok_or("Invalid queue format!")?
            .name("hold")
            .map(|capture| Tetromino::parse(&capture.as_str()[..1]))
            .transpose()
            .map_err(|err| err + " at hold!")?;

        let mut sequence = VecDeque::new();
        for (i, captures) in Regex::new(&sequence_pattern)
            .map_err(|_| "Invalid regex pattern!")?
            .captures_iter(input.as_str())
            .enumerate()
        {
            if let Some(capture) = captures.name("pattern") {
                sequence.push_back(
                    Pattern::parse(capture.as_str())
                        .map_err(|err| format!("{err} at pattern {}!", i + 1))?,
                );
            }
        }

        Ok(Self {
            sequence,
            hold,
            can_swap: true,
        })
    }
}

impl Parsing for Tetromino {
    fn parse<T: Into<String>>(input: T) -> Result<Self, String> {
        let input = input.into();
        Tetromino::from_str(input.as_str())
            .map_err(|_| format!("Invalid tetromino character '{}'", input))
    }
}
