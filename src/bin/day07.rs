use std::{str::FromStr, cmp::Ordering};

use advent::read_input;
use anyhow::anyhow;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum WinType {
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
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.wintype().partial_cmp(&other.wintype()) {
            Some(Ordering::Equal) => if self.is_stronger(&other) { 
                Some(Ordering::Less) 
            } else {
                Some(Ordering::Greater)
            },
            Some(order) => Some(order),
            None => unreachable!(),
        }
    }
}

impl Hand {
    fn wintype(&self) -> WinType {
        let mut counts = [0_u8; 13];
        for &c in &self.cards {
            counts[c as usize - 2] += 1;
        }

        // dbg!(&counts);
        match counts.iter().max() {
            Some(&5) => return WinType::FiveOfAKind,
            Some(&4) => return WinType::FourOfAKind,
            Some(&3) => {
                // check for full house
                if counts.iter().any(|&c| c == 2) {
                    return WinType::FullHouse
                } else {
                    return WinType::ThreeOfKind
                }
            },
            _ => (),
        }

        match counts.iter().filter(|&&count| count == 2).count() {
            2 => return WinType::TwoPair,
            1 => return WinType::OnePair,
            _ => (),
        }

        WinType::HighCard
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

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let mut hands: Vec<Hand> = input.trim().lines()
        .map(|line| line.parse().unwrap())
        .collect();

    hands.sort();
    hands.reverse();

    let mut silver_sum = 0;
    for (rank, hand) in hands.iter().enumerate() {
        silver_sum += (rank + 1) * hand.bid;
        println!("{:?} -> {:?}", hand, hand.wintype());
    }

    println!("Silver: {}", silver_sum);

    Ok(())
}



impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').ok_or_else(|| anyhow!("invalid line format"))?;

        fn card_value(c: char) -> u8 {
            match c as u8 {
                // ASCII digits
                num @ b'2'..=b'9' => num - b'0',
                b'T' => 10,
                b'J' => 11,
                b'Q' => 12, 
                b'K' => 13, 
                b'A' => 14, 
                _ => panic!("invalid card value")
            }
        }

        let inner = Self {
            cards: hand.chars().take(5).map(card_value).collect(),
            bid: bid.parse()?
        };

        Ok(inner)
    }
}
