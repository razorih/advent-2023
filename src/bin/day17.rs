use std::collections::{BinaryHeap, HashSet};

use advent::{read_input, print_grid};
use grid::Grid;

/// A frontier node in uniform-cost search
#[derive(Debug)]
struct Node {
    /// Current cumulative cost
    cost: usize,
    /// Metadata fields
    /// - Position: Used to discover neighbours given some grid
    /// - Moved: How many times we have moved in a single direction
    /// - Direction: The direction we entered this node
    pos: (usize, usize),
    /// How many times we have moved to this "direction" during the search.
    /// Never exceeds 3
    moved: u8,
    /// What direction we came from, used for neighbour discovery
    direction: Direction,
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

    /// Discover and return a neighbour in a direction if any
    fn discover(
        &self,
        direction: Direction,
        grid: &Grid<u8>,
    ) -> Option<Self> {
        let offset: (i8, i8) = direction.as_offset();

        // Calculate next coordinate based on absolute direction
        // Out of bounds nodes are not valid discoveries
        let next_row = self.pos.0.checked_add_signed(offset.0 as isize)
            .and_then(|row| if row < grid.rows() { Some(row) } else { None })?;
        let next_col = self.pos.1.checked_add_signed(offset.1 as isize)
            .and_then(|col| if col < grid.cols() { Some(col) } else { None })?;
        let new_pos = (next_row, next_col);

        // Check move limit
        let moved = if self.direction == direction {
            self.moved + 1
        } else {
            1
        };

        if moved > 3 {
            // Exceeded move limit, invalid node
            // println!("!! abandoning direction !!");
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

fn solve(grid: &Grid<u8>, end: (usize, usize)) -> usize {
    let mut frontier: BinaryHeap<Node> = BinaryHeap::new();

    // As we are starting from top-left, we can manually insert two roots;
    // one going right and one going down
    let start_down  = (1, 0);
    let start_right = (0, 1);

    let mut visited: HashSet<(usize, usize, u8, Direction)> = HashSet::new();

    // Insert root nodes
    frontier.push(Node::new(start_down, grid[start_down] as usize, 1, Direction::Down));
    frontier.push(Node::new(start_right, grid[start_right] as usize, 1, Direction::Right));

    while let Some(node) = frontier.pop() {
        // println!("Enter: {node:?}");
        if node.pos == end {
            return node.cost;
        }

        // it seems that paths can do weird circles.
        // solution works if we cache combination of
        // - node position
        // - how many tiles we have moved straight
        // - and what direction we came from
        if !visited.insert((node.pos.0, node.pos.1, node.moved, node.direction)) {
            // println!("  already visited, skipping");
            // Already visited this position, do nothing
            continue;
        }

        // Enqueue all neighbours we can possible reach:
        // - Based on direction, always Left and Right
        // - Based on direction, Forward if moved < 3
        
        // First, map relative direction to absolute
        let (left, right, forward) = match node.direction {
            Direction::Up    => (Direction::Left,  Direction::Right, Direction::Up),
            Direction::Down  => (Direction::Right, Direction::Left,  Direction::Down),
            Direction::Left  => (Direction::Down,  Direction::Up,    Direction::Left),
            Direction::Right => (Direction::Up,    Direction::Down,  Direction::Right),
        };

        // Try to discover nodes at those directions, if valid
        // Node discovery handles all out of bounds and move count related cases
        let left = node.discover(left, &grid);
        let right = node.discover(right, &grid);
        let forward = node.discover(forward, &grid);

        // Put valid nodes to the priority queue
        for node in [left, right, forward] {
            if let Some(node) = node {
                // println!("    discover: {node:?}");
                frontier.push(node)
            }
        }
    }

    unreachable!("search didn't find a valid end node")
}


fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let grid = parse(&input);

    print_grid(&grid);
    let min_cost = solve(&grid, (grid.rows()-1, grid.cols()-1));

    println!("Silver: {}", min_cost);

    Ok(())
}


fn parse(s: &str) -> Grid<u8> {
    let cols = s.lines().next().expect("got empty input").len();
    let tiles: Vec<u8> = s.chars()
        .filter_map(|ch| u8::try_from(ch).ok().and_then(|n| n.checked_sub(b'0')))
        .collect();

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
