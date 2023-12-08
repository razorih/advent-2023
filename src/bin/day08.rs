use std::{str::FromStr, convert::Infallible, collections::HashMap};

use advent::read_input;

// https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

// https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[derive(Debug, Clone, Copy)]
enum Direction { Left, Right }

#[derive(Debug)]
struct Instructions {
    dirs: Vec<Direction>
}

impl FromStr for Instructions {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            dirs: s.chars().map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _   => panic!("invalid direction"),
            }).collect()
        })
    }
}

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> HashMap<String, (String, String)> {
    let mut out = HashMap::new();

    for line in lines {
        let (origin, dest) = line.split_once(" = ").expect("invalid line");
        // origin -> AAA
        // dest   -> (BBB, CCC)
        let (left, right) = dest
            .strip_prefix('(').unwrap()
            .strip_suffix(')').unwrap()
            .split_once(", ").unwrap();

        out.insert(origin.to_string(), (left.to_string(), right.to_string()));
    }

    out
}

fn silver(
    instructions: &Instructions,
    map: &HashMap<String, (String, String)>
) -> usize {
    let mut visitor = &String::from("AAA");
    let mut steps = 0;

    for instruction in instructions.dirs.iter().cycle() {
        let Some(directions) = map.get(visitor) else {
            panic!("no such node in map: {}", visitor);
        };

        match instruction {
            Direction::Left => visitor = &directions.0,
            Direction::Right => visitor = &directions.1,
        }

        steps += 1;
        if visitor == "ZZZ" {
            // println!("reached ZZZ!");
            break;
        }
    }

    steps
}

fn gold(
    instructions: &Instructions,
    map: &HashMap<String, (String, String)>
) -> usize {
    // First, find all starting positions
    let mut cursors: Vec<&String> = map.keys().filter(|key| key.ends_with('A')).collect();
    // List of current cycle lengths
    let mut cycle_lengths = vec![0_usize; cursors.len()];
    // List of stable cycle lengths, i.e. the true length of the cycle
    // There is always one node in a cycle
    let mut stable = vec![1_usize; cursors.len()];

    println!("starting with {} cursors", cursors.len());

    // Run instructions until we have gathered all cycle lengths
    for instruction in instructions.dirs.iter().cycle() {
        for (i, cursor) in cursors.iter_mut().enumerate() {
            let Some(directions) = map.get(&**cursor) else {
                panic!("no such node in map: {}", cursor);
            };

            // Move cursor
            match instruction {
                Direction::Left => *cursor = &directions.0,
                Direction::Right => *cursor = &directions.1,
            }

            if cursor.ends_with('Z') {
                if cycle_lengths[i] > 0 {
                    println!("cycle {i} is {}", cycle_lengths[i]);
                    stable[i] = cycle_lengths[i];
                }
            } else {
                cycle_lengths[i] += 1;
            }
        }

        if stable.iter().all(|&n| n > 1) {
            break;
        }
    }

    println!("Stable cycle lengths: {:?}", stable);
    // We have now gathered stable cycle counts.
    // Answer is least-common multiple of them all.
    // I.e. at what point all cycles align
    lcm(&stable)
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let mut lines = input.trim().lines();
    let instructions: Instructions = lines.next().unwrap().parse().unwrap();
    let _ = lines.next();

    let map = parse(&mut lines);
    // println!("{:#?}", map);
    

    // println!("Silver: {}", silver(&instructions, &map));
    println!("  Gold: {}", gold(&instructions, &map));

    Ok(())
}
