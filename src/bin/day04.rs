use std::{str::FromStr, collections::BTreeSet};

use advent::read_input;
use anyhow::anyhow;

#[derive(Debug)]
struct Card {
    winning: BTreeSet<u16>,
    have: BTreeSet<u16>,
}

impl Card {
    fn matching(&self) -> usize {
        self.winning.intersection(&self.have).count()
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

fn score(match_count: usize) -> usize {
    if match_count > 0 {
        2_usize.pow(match_count as u32 - 1)
    } else {
        0
    }
}

fn silver(input: &str) -> usize {
    input.trim().lines()
        .map(|line| line
            .parse()
            .map(|card: Card| score(card.matching()))
            .unwrap()
        ).sum()
}

fn gold(input: &str) -> usize {
    let matching = input.trim().lines()
        .map(|line| line
            .parse()
            .map(|card: Card| card.matching())
            .unwrap()
        ).collect::<Vec<_>>();
    
    let mut card_counts = vec![1_usize; matching.len()];
    for i in 0..card_counts.len() {
        let count = card_counts[i];

        // Accumulate extra cards if card has matching numbers
        for extra_i in 0..matching[i] {
            card_counts[i+extra_i+1] += count;
        }
    }

    card_counts.iter().sum()
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;

    println!("Silver: {}", silver(&input));
    println!("  Gold: {}", gold(&input));

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
        assert_eq!(card.matching(), 4);
    }

    #[test]
    fn card_win_score() {
        let card: Card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".parse().unwrap();
        assert_eq!(score(card.matching()), 8);
    }

    #[test]
    fn card_no_win() {
        let card: Card = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".parse().unwrap();
        assert_eq!(card.matching(), 0);
    }
}
