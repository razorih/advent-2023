use std::{str::FromStr, convert::Infallible, collections::HashMap};

use advent::read_input;

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

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let mut lines = input.trim().lines();
    let instructions: Instructions = lines.next().unwrap().parse().unwrap();
    let _ = lines.next();

    let map = parse(&mut lines);
    // println!("{:#?}", map);
    

    println!("Silver: {}", silver(&instructions, &map));

    Ok(())
}
