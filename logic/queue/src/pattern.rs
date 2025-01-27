use rand::Rng;
use strum::{EnumCount, IntoEnumIterator};
use tetrominos::Tetromino;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Pattern {
    pub tetromino_set: [usize; Tetromino::COUNT],
    pub draw_count: usize,
}

impl Pattern {
    pub fn size(&self) -> usize {
        self.tetromino_set.iter().sum()
    }

    pub fn draw(&mut self) -> Option<Tetromino> {
        let total_weight = self.size();
        if total_weight == 0 {
            return None;
        }

        let bar = rand::thread_rng().gen_range(0..total_weight);

        let mut cumulative_weight = 0;
        for (i, weight) in self.tetromino_set.iter_mut().enumerate() {
            cumulative_weight += *weight;
            if bar < cumulative_weight {
                *weight -= 1;
                self.draw_count -= 1;
                return Some(Tetromino::from(i));
            }
        }

        None
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            tetromino_set: [1; Tetromino::COUNT],
            draw_count: Tetromino::COUNT,
        }
    }
}

impl super::Parsing for Pattern {
    fn parse<T: Into<String>>(input: T) -> Result<Self, String> {
        let input = input.into();

        let validation = regex::Regex::new(r"^((?<pattern>\[.+?\])|(?<all>\*))(p(?<amount>\d+))?$")
            .map_err(|_| "Invalid regex pattern!")?;

        let captures = validation
            .captures(input.as_str())
            .ok_or("Invalid pattern format!")?;

        let collection: Vec<_> = if captures.name("all").is_some() {
            Tetromino::iter().collect()
        } else if let Some(capture) = captures.name("pattern") {
            capture.as_str()[1..capture.len() - 1]
                .chars()
                .map(super::Tetromino::parse)
                .collect::<Result<Vec<_>, _>>()?
        } else {
            return Err(String::from("Invalid tetromino set!"));
        };

        let draw_count = captures
            .name("amount")
            .map_or(1, |r#match| r#match.as_str().parse::<usize>().unwrap_or(1));

        if draw_count > collection.len() {
            return Err(String::from(
                "Amount may not be greater than the number of tetrominos in the tetromino set!",
            ));
        }

        let mut tetromino_set = [0; Tetromino::COUNT];
        for variant in collection {
            tetromino_set[variant as usize] += 1;
        }

        Ok(Self {
            tetromino_set,
            draw_count,
        })
    }
}
