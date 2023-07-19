use std::{
    cell::Cell,
    collections::BinaryHeap,
    fmt::{Debug, Display},
    fs::read_to_string,
};

static ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";

struct Grid {
    nodes: Vec<TreeNode>,
    width: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..(self.nodes.len() / self.width) {
            for col in 0..(self.width) {
                match self.index((row, col)).distance.get() {
                    Some(n) => write!(f, "{:0>2}", n % 100)?,
                    None => write!(f, "--")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(f: String) -> Self {
        let width = f.lines().next().unwrap().len();
        let height = (f.len() + 1) / (width + 1);
        let mut nodes = Vec::with_capacity(f.len());
        let mut i = 0;

        for c in f.chars() {
            if c == '\n' {
                continue;
            };

            let (node_pos, elevation) = match c {
                'S' => (NodePos::Start, 0),
                'E' => (NodePos::End, 25),
                d => (
                    NodePos::Other,
                    ALPHA.chars().position(|v| v == d).unwrap() as u8,
                ),
            };

            let (row, col) = (i / width, i % width);

            let mut neighbours = Vec::with_capacity(4);

            if row > 0 {
                neighbours.push((row - 1, col))
            }
            if col > 0 {
                neighbours.push((row, col - 1))
            }
            if row < height - 1 {
                neighbours.push((row + 1, col))
            }
            if col < width - 1 {
                neighbours.push((row, col + 1))
            }

            nodes.push(TreeNode {
                elevation,
                pos: node_pos,
                neighbours,
                distance: Cell::new(None),
            });

            i += 1;
        }

        Grid { nodes, width }
    }

    fn index(&self, pos: (usize, usize)) -> &TreeNode {
        if pos.0 * self.width + pos.1 >= 2870 {
            println!("oops")
        }
        &self.nodes[pos.0 * self.width + pos.1]
    }
}

#[derive(PartialEq, Debug, Eq)]
enum NodePos {
    Start,
    End,
    Other,
}

#[derive(PartialEq, Eq)]
struct TreeNode {
    elevation: u8,
    pos: NodePos,
    distance: Cell<Option<usize>>,
    neighbours: Vec<(usize, usize)>,
}

impl Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Elevation: {}, Pos: {:?}, Distance: {:?}, Neighbours: {:?}",
            self.elevation, self.pos, self.distance, self.neighbours
        )
    }
}

impl std::fmt::Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.distance.get() {
            Some(d) => write!(f, "{:0>2}", d % 100),
            None => write!(f, "--"),
        }
    }
}

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .distance
            .get()
            .unwrap()
            .cmp(&self.distance.get().unwrap())
    }
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TreeNode {
    fn set_distance(&self, distance: usize) {
        self.distance.set(Some(distance));
    }

    fn walk_neighbours(&self, grid: &Grid, max_iter: &usize) -> usize {
        let mut heap = BinaryHeap::new();
        heap.push(self);

        let mut dist = usize::MAX;
        let mut iter = 0;

        while let Some(node) = heap.pop() {
            if iter >= *max_iter {
                break;
            }

            if node.pos == NodePos::End {
                dist = node.distance.get().unwrap();
                break;
            }

            for coords in &node.neighbours {
                let neigbour = grid.index(*coords);

                if neigbour.elevation > node.elevation + 1 {
                    continue;
                }

                let current_dist = node.distance.get().unwrap();

                if neigbour.distance.get().unwrap_or(usize::MAX) > current_dist + 1 {
                    neigbour.set_distance(current_dist + 1);
                    heap.push(neigbour);
                }
            }
            iter += 1;
        }
        dist
    }
}

fn main() {
    let f = read_to_string("d12.txt").unwrap();
    let grid1 = Grid::new(f.clone());

    let max_iter = 1000;

    // part 1
    let start = grid1
        .nodes
        .iter()
        .filter(|x| x.pos == NodePos::Start)
        .collect::<Vec<_>>()
        .pop()
        .unwrap();

    start.set_distance(0);
    start.walk_neighbours(&grid1, &max_iter);

    println!("{}", grid1);

    // part 2
    let grid2 = Grid::new(f.clone());
    let start_positions: Vec<_> = grid2
        .nodes
        .iter()
        .filter(|node| node.elevation == 0)
        .collect();

    for pos in &start_positions {
        pos.set_distance(0);
    }

    let distances: Vec<_> = start_positions
        .iter()
        .map(|x| x.walk_neighbours(&grid2, &max_iter))
        .collect();

    let min_distance = distances.iter().min();

    println!("{:?}", min_distance)
}
