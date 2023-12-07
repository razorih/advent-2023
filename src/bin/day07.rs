use std::cmp::Ordering;

use advent::read_input;
use anyhow::anyhow;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Win {
    FiveOfAKind = 0,
    FourOfAKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq, Ord)]
struct Hand {
    cards: Vec<u8>,
    bid: usize,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        // Bid doesn't matter here, hands are the same if they have same cards
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.wintype().partial_cmp(&other.wintype()) {
            // Cards have winning type,
            // need to check individual cards
            Some(Ordering::Equal) => if self.is_stronger(&other) { 
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            },
            // Hand's type differs, delegate win resolution to 
            Some(order) => Some(order),
            None => unreachable!(),
        }
    }
}

impl Hand {
    fn wintype(&self) -> Win {
        let mut counts = [0_u8; 14];
        for &c in &self.cards {
            counts[c as usize - 1] += 1;
        }

        let jokers = counts[0];
        counts[0] = 0; // Reset joker count so they don't interfere

        // Prerequisites
        // - Largest count of same number
        // - Number of pairs
        // - Number of jokers
        let max_same = *counts.iter().max().unwrap();
        let pairs = counts.iter().filter(|&&count| count == 2).count();

        return match (max_same, jokers) {
            // Check if we have N-of-a-kind using cards+jokers
            fives @ (0..=5, _) if fives.0 + jokers == 5 => Win::FiveOfAKind,
            fours @ (0..=4, _) if fours.0 + jokers == 4 => Win::FourOfAKind,

            // Full House special cases; Joker can form an extra pair
            (2, 1) if pairs == 2 => Win::FullHouse, // AABBJ -> AABBB
            (3, 0) if pairs == 1 => Win::FullHouse, // AAABB

            threes @ (1..=3, _) if threes.0 + jokers == 3 => Win::ThreeOfKind,

            // Special case for two pairs
            (2, 0) if pairs == 2 => Win::TwoPair, // AABBC

            twos @ (1..=2, _) if twos.0 + jokers == 2 => Win::OnePair,

            // Everything else
            (1, 0) => Win::HighCard, // ABCDE
            _      => unreachable!(),
        }
    }

    fn is_stronger(&self, other: &Self) -> bool {
        debug_assert_eq!(self.wintype(), other.wintype());

        for (&first, &second) in self.cards.iter().zip(other.cards.iter()) {
            if first == second {
                continue;
            }
            return first > second
        }

        unreachable!();
    }
}

fn solve(hands: &mut [Hand]) -> usize {
    let mut sum: usize = 0;

    hands.sort_unstable();
    for (hand, rank) in hands.iter().rev().zip(1_usize..) {
        sum += rank * hand.bid;
    }
    sum
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;

    let mut silver_hands: Vec<Hand> = input.trim().lines()
        .map(|line| Hand::from_str::<true>(line).unwrap())
        .collect();
    let silver_sum = solve(&mut silver_hands);

    let mut gold_hands: Vec<Hand> = input.trim().lines()
        .map(|line| Hand::from_str::<false>(line).unwrap())
        .collect();
    let gold_sum = solve(&mut gold_hands);

    println!("Silver: {}", silver_sum);
    println!("  Gold: {}", gold_sum);

    Ok(())
}

fn card_value(c: char, silver_joker: bool) -> u8 {
    match c as u8 {
        // ASCII digits
        num @ b'2'..=b'9' => num - b'0',
        b'T' => 10,
        b'J' => if silver_joker { 11 } else { 1 },
        b'Q' => 12, 
        b'K' => 13, 
        b'A' => 14, 
        _ => panic!("invalid card value")
    }
}

impl Hand {
    fn from_str<const S: bool>(s: &str) -> anyhow::Result<Self> {
        let (hand, bid) = s.split_once(' ').ok_or_else(|| anyhow!("invalid line format"))?;
        let inner = Self {
            cards: hand.chars().take(5).map(|c| card_value(c, S)).collect(),
            bid: bid.parse()?
        };

        Ok(inner)
    }
}
