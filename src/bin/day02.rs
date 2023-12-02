use std::str::FromStr;
use anyhow::anyhow;

use advent::read_input;

type RGB = (u8, u8, u8);

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<RGB>,
}

impl FromStr for Game {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Game ").ok_or(anyhow!("Missing 'Game' prefix"))?;
        let (game_id, s) = s.split_once(": ").ok_or(anyhow!("Missing semicolon after game id"))?;
        let game_id = game_id.parse::<usize>()?;

        let mut sets: Vec<RGB> = Vec::new();
        for set in s.split(';') {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            let set = set.trim();
            for group in set.splitn(3, ", ") {
                let (amount, color) = group.split_once(' ').ok_or(anyhow!("Malformed (amount, color) pair"))?;
                let amount = amount.parse::<u8>()?;
                match color {
                    "red" => red = amount,
                    "green" => green = amount,
                    "blue" => blue = amount,
                    _ => (),
                }
            }

            sets.push((red, green, blue));
        }

        Ok(Game { id: game_id, sets: sets })
    }
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;

    let mut possible_sum = 0;
    'games: for line in input.trim().lines() {
        let game: Game = line.parse()?;

        for set in game.sets {
            if set.0 > 12 || set.1 > 13 || set.2 > 14 {
                //println!("Game {:3} is impossible", game.id);
                continue 'games;
            }
        }

        possible_sum += game.id;
    }

    println!("Silver: {}", possible_sum);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game: Game = input.parse().unwrap();

        assert_eq!(game.id, 1);
        assert_eq!(game.sets, vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]);
    }
}
