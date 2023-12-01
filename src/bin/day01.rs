use advent::read_input;

fn main() -> anyhow::Result<()> {
    let input = read_input()?;

    let mut calibration_values: Vec<u32> = Vec::new();

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

        let first = first.unwrap();
        let last = last.unwrap();

        // Rebuild
        let value: u32 = format!("{}{}", first, last).parse().unwrap();
        calibration_values.push(value);
    }

    println!("Silver: {}", calibration_values.iter().sum::<u32>());

    Ok(())
}
