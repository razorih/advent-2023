use advent::read_input;

fn silver(input: &str) -> u32 {
    let mut calibration_value: u32 = 0;

    for line in input.trim().lines() {
        let mut first: Option<u8> = None;
        let mut last: Option<u8> = None;

        for char in line
            .chars()
            .filter_map(
                |c| c.to_digit(10).map(|n| n as u8)
            )
        {
            if first.is_none() {
                first = Some(char)
            }

            last = Some(char)
        }

        if first.is_none() {
            println!("invalid line, no numbers found? {}", line);
            continue;
        }

        let first = first.unwrap() as u32;
        let last = last.unwrap() as u32;

        // Rebuild
        let value: u32 = first * 10 + last;
        calibration_value += value;
    }

    calibration_value
}

fn search_starting_digit(line: &str) -> Option<u8> {
    const DIGITS: [(&str, u8); 9] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    if line.is_empty() {
        return None
    }

    // Search for numerical digit
    if let Some(digit) = line.chars().nth(0).and_then(|digit| digit.to_digit(10)) {
        return Some(digit as u8)
    }

    // Search for written digit
    for (digit, value) in DIGITS {
        if line.starts_with(digit) {
            return Some(value);
        }
    }

    None
}

fn gold(input: &str) -> u32 {
    let mut calibration_value: u32 = 0;

    for line in input.trim().lines() {
        let mut first: Option<u8> = None;
        let mut last: Option<u8> = None;

        for i in 0..line.len() {
            if let Some(found) = search_starting_digit(&line[i..]) {
                if first.is_none() {
                    first = Some(found);
                }

                last = Some(found);
            }
        }
        
        let first = first.unwrap() as u32;
        let last = last.unwrap() as u32;

        let value: u32 = first * 10 + last;
        calibration_value += value;
    }

    calibration_value
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let silver = silver(input.as_str());
    let gold = gold(input.as_str());

    println!("Silver: {}", silver);
    println!("  Gold: {}", gold);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn written_digit() {
        assert_eq!(search_starting_digit("twothree"), Some(2));
    }

    #[test]
    fn numerical_digit() {
        assert_eq!(search_starting_digit("91"), Some(9));
    }

    #[test]
    fn invalid_digit() {
        assert_eq!(search_starting_digit("invalid"), None);
    }
}
