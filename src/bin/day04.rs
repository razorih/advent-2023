use std::{str::FromStr, collections::BTreeSet};

use advent::read_input;
use anyhow::anyhow;

#[derive(Debug)]
struct Card {
    winning: BTreeSet<u16>,
    have: BTreeSet<u16>,
}

impl Card {
    fn score(&self) -> usize {
        let matching = self.winning.intersection(&self.have).count();
        
        if matching > 0 {
            2_usize.pow(matching as u32 - 1)
        } else {
            0
        }
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Card ").ok_or_else(|| anyhow!("missing card prefix"))?;
        let (_, rest) = s.split_once(':').ok_or_else(|| anyhow!("missing semicolon after card number"))?;
        let (winning, have) = rest.split_once('|').ok_or_else(|| anyhow!("invalid card number list format"))?;

        Ok(Self {
            winning: winning.split_whitespace().map(|n| n.parse().unwrap()).collect(),
            have: have.split_whitespace().map(|n| n.parse().unwrap()).collect(),
        })
    }
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;

    let sum: usize = input.trim().lines()
        .map(|line| line
            .parse()
            .map(|card: Card| card.score())
            .unwrap()
        ).sum();

    println!("Silver: {}", sum);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_parses() {
        let card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".parse::<Card>();
        assert!(card.is_ok());
    }

    #[test]
    fn card_info() {
        let card: Card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".parse().unwrap();
        assert!(card.winning.iter().eq(&[17, 41, 48, 83, 86]));
        assert!(card.have.iter().eq(&[6, 9, 17, 31, 48, 53, 83, 86]));
    }

    #[test]
    fn card_win_count() {
        let card: Card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".parse().unwrap();
        assert_eq!(card.score(), 8);
    }

    #[test]
    fn card_no_win() {
        let card: Card = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".parse().unwrap();
        assert_eq!(card.score(), 0);
    }
}
