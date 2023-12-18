use std::collections::{BinaryHeap, HashSet};

use advent::read_input;
use grid::Grid;

/// A frontier node in uniform-cost search
#[derive(Debug, Clone, Copy)]
struct Node {
    /// Current cumulative cost
    cost: usize,
    /// Metadata fields
    /// - Position: Used to discover neighbours given some grid
    /// - Moved: How many times we have moved in a single direction
    /// - Direction: The direction we entered this node
    pos: (usize, usize),
    /// How many times we have moved to this `direction` during the search.
    moved: u8,
    /// What direction we came from, used for neighbour discovery
    direction: Direction,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct VisitedNode {
    pos: (usize, usize),
    moved: u8,
    direction: Direction
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction { Up, Down, Left, Right }

impl Direction {
    fn as_offset(&self) -> (i8, i8) {
        match self {
            Direction::Up    => (-1,  0),
            Direction::Down  => ( 1,  0),
            Direction::Left  => ( 0, -1),
            Direction::Right => ( 0,  1),
        }
    }
}

impl Node {
    fn new(pos: (usize, usize), cost: usize, moved: u8, direction: Direction) -> Self {
        Self { pos, cost, moved, direction }
    }

    /// Discover and return a neighbour in given direction if any
    fn discover(
        &self,
        direction: Direction,
        grid: &Grid<u8>,
    ) -> Option<Self> {
        let (row, col) = self.pos;
        let (row_offset, col_offset) = direction.as_offset();
        let next_row = row.checked_add_signed(row_offset as isize)?;
        let next_col = col.checked_add_signed(col_offset as isize)?;

        if next_col >= grid.cols() || next_row >= grid.rows() {
            return None
        }

        let new_pos = (next_row, next_col);

        // Check move limit
        let moved = if self.direction == direction {
            self.moved + 1
        } else {
            // Ultra crucible must have moved at least 4 tiles forward before
            // being able to turn
            if self.moved < 4 {
                return None
            }

            1
        };

        // Ultra crucible can move a maximum of 10 consecutive tiles
        if moved > 10 {
            return None
        }

        Some(Self {
            pos: new_pos,
            cost: self.cost + grid[new_pos] as usize,
            direction,
            moved,
        })
    }
}

fn solve(grid: &Grid<u8>, end: (usize, usize)) -> Option<usize> {
    let mut frontier: BinaryHeap<Node> = BinaryHeap::new();
    let mut visited: HashSet<VisitedNode> = HashSet::new();

    // Insert two "root" nodes, starting from top left.
    // One going to the right and one going down.
    let start_down  = (1, 0);
    let start_right = (0, 1);
    frontier.push(Node::new(start_down, grid[start_down] as usize, 1, Direction::Down));
    frontier.push(Node::new(start_right, grid[start_right] as usize, 1, Direction::Right));

    while let Some(node) = frontier.pop() {
        if node.pos == end {
            return Some(node.cost)
        }

        if !visited.insert(node.into()) {
            continue
        }

        // Enqueue all neighbours we can possible reach
        // First, map relative direction to absolute
        let (left, right, forward) = match node.direction {
            Direction::Up    => (Direction::Left,  Direction::Right, Direction::Up),
            Direction::Down  => (Direction::Right, Direction::Left,  Direction::Down),
            Direction::Left  => (Direction::Down,  Direction::Up,    Direction::Left),
            Direction::Right => (Direction::Up,    Direction::Down,  Direction::Right),
        };

        for direction in [left, right, forward] {
            if let Some(discovered_node) = node.discover(direction, &grid) {
                frontier.push(discovered_node)
            }
        }
    }

    None
}


fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let grid = parse(&input);

    let min_cost = solve(&grid, (grid.rows()-1, grid.cols()-1));

    println!("Gold: {}", min_cost.unwrap());

    Ok(())
}

fn parse(s: &str) -> Grid<u8> {
    let cols = s.lines().next().expect("got empty input").len();
    let tiles: Vec<u8> = s.chars()
        .filter_map(|ch| 
            u8::try_from(ch).ok()
                .and_then(|n| n.checked_sub(b'0'))
        ).collect();

    Grid::from_vec(tiles, cols)
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reversed comparison so we can build a min-heap
        // (minimize the distance)
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Node {}

impl From<Node> for VisitedNode {
    fn from(value: Node) -> Self {
        Self { pos: value.pos, moved: value.moved, direction: value.direction }
    }
}
