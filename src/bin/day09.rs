use advent::read_input;

fn all_elements_equal<I>(iter: I) -> Option<I::Item>
where
    I: IntoIterator,
    I::Item: PartialEq,
{
    let mut iter = iter.into_iter();
    let Some(head) = iter.next() else {
        return None;
    };

    if iter.all(|elem| elem == head) {
        Some(head)
    } else {
        None
    }
}

/// Calculate extrapolated value for given slice
fn extrapolate(values: &[isize]) -> isize {
    println!("values: {:?}", values);
    let diff: Vec<isize> = values.windows(2).map(|pair| pair[1] - pair[0]).collect();

    println!("diff {:?}", diff);
    if let Some(common) = all_elements_equal(diff.as_slice()) {
        println!("common element: {common}");
        return values[values.len() - 1] + *common;
    } else {
        // Need to recurse
        let next = extrapolate(diff.as_slice());
        return values[values.len() - 1] + next;
    }
}

fn parse(line: &str) -> Vec<isize> {
    line.split_whitespace().map(|num| num.parse().unwrap()).collect()
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let lines = input.trim().lines();

    let mut silver_sum = 0;
    for line in lines {
        let line = parse(line);
        let extrapolated = extrapolate(&line);
        silver_sum += extrapolated;
        println!("new: {extrapolated}\n")
    }
    println!("Silver: {}", silver_sum);

    Ok(())
}
