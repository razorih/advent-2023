use std::str::FromStr;

use crate::error::GameParseError;

pub type RGB = (u8, u8, u8);

#[derive(Debug)]
pub struct Game {
    pub id: usize,
    pub sets: Vec<RGB>,
}

impl FromStr for Game {
    type Err = GameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Game ").ok_or(GameParseError::MissingPrefix)?;
        let (id, s) = s.split_once(": ").ok_or(GameParseError::MissingSemicolon)?;
        let id = id.parse::<usize>().map_err(|_| GameParseError::InvalidGameId)?;

        let mut sets: Vec<RGB> = Vec::new();
        for set in s.split(';') {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            let set = set.trim();
            for group in set.splitn(3, ", ") {
                let (amount, color) = group.split_once(' ').ok_or(GameParseError::MalformedAmountColorPair)?;
                let amount = amount.parse::<u8>().map_err(|_| GameParseError::InvalidAmount)?;
                match color {
                    "red" => red = amount,
                    "green" => green = amount,
                    "blue" => blue = amount,
                    _ => return Err(GameParseError::InvalidColor),
                }
            }

            sets.push((red, green, blue));
        }

        Ok(Game { id, sets })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game: Game = input.parse().unwrap();

        assert_eq!(game.id, 1);
        assert_eq!(game.sets, &[(4, 0, 3), (1, 2, 6), (0, 2, 0)]);
    }
}
