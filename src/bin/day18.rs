use advent::read_input;
use anyhow::anyhow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction { Up, Down, Left, Right }

#[derive(Debug)]
struct Digger {
    current: (isize, isize),
    vertices: Vec<(isize, isize)>,
    n_boundary_points: usize,
}

impl Digger {
    fn new() -> Self {
        Self { current: (0, 0), vertices: vec![(0, 0)], n_boundary_points: 1 }
    }

    fn dig(&mut self, direction: Direction, amount: usize) {
        self.current = translate(self.current, direction, amount);
        if self.current == (0, 0) {
            // we've looped back to start;
            // don't push vertex, but record the number of `boundary points - 1`
            // i.e. the distance to start without including start itself.
            self.n_boundary_points += amount - 1;
            return;
        }

        self.vertices.push(self.current);
        self.n_boundary_points += amount;
    }

    fn finish(self) -> (Vec<(isize, isize)>, usize) {
        (self.vertices, self.n_boundary_points)
    }
}


fn translate(point: (isize, isize), direction: Direction, amount: usize) -> (isize, isize) {
    let amount = amount as isize;
    match direction {
        Direction::Up    => (point.0 - amount, point.1),
        Direction::Down  => (point.0 + amount, point.1),
        Direction::Left  => (point.0, point.1 - amount),
        Direction::Right => (point.0, point.1 + amount),
    }
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let (vertices, boundary_points) = parse(&input);

    let area = shoelace(&vertices).abs();
    // Pick's theorem
    let interior = area - (boundary_points as isize / 2) + 1;

    // println!("vertices:        {vertices:?}");
    println!("boundary points: {boundary_points}");
    println!("interior points: {interior}");
    println!("polygon area:    {area}");

    println!("Silver: {}", boundary_points + interior as usize);

    Ok(())
}

/// Calculate signed area of a polygon given its vertices.
fn shoelace(vertices: &[(isize, isize)]) -> isize {
    /// Calculates determinant of 2x2 matrix formed from two points
    /// | x1  x2 |
    /// | y1  y2 |
    fn det((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> isize {
        x1*y2 - x2*y1
    }

    let mut incomplete_sum: isize = 0;
    for pair in vertices.windows(2) {
        incomplete_sum += det(pair[0], pair[1]);
    }

    let complete_sum = incomplete_sum + det(vertices[vertices.len() - 1], vertices[0]);
    complete_sum / 2
}

fn parse(s: &str) -> (Vec<(isize, isize)>, usize) {
    let mut digger = Digger::new();

    for line in s.trim().lines() {
        let mut parts = line.splitn(3, ' ');
        let (_, _, (direction, amount)) = (
            parts.next()
                .and_then(|dir| Direction::try_from(dir.chars().next()?).ok())
                .unwrap(),
            parts.next()
                .and_then(|n| n.parse::<usize>().ok())
                .unwrap(),
            parts.next()
                .and_then(|color| color.strip_prefix("(#"))
                .and_then(|color| color.strip_suffix(')'))
                .map(parse_rgb)
                .unwrap(),
        );

        digger.dig(direction, amount);
    }

    digger.finish()
}

fn parse_rgb(s: &str) -> (Direction, usize) {
    let amount = usize::from_str_radix(&s[..5], 16).unwrap();
    let direction = s.chars().last().and_then(|dir| Direction::try_from(dir).ok());

    (direction.unwrap(), amount)
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let inner = match value {
            // Silver
            'R' => Self::Right,
            'L' => Self::Left,
            'D' => Self::Down,
            'U' => Self::Up,
            // Gold
            '0' => Self::Right,
            '1' => Self::Down,
            '2' => Self::Left,
            '3' => Self::Up,
            _ => return Err(anyhow!("invalid direction"))
        };

        Ok(inner)
    }
}
