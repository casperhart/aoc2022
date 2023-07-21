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
        // display two-digit distance for each node that was visited
        // for row in 0..(self.nodes.len() / self.width) {
        //     for col in 0..(self.width) {
        //         match self.index((row, col)).distance.get() {
        //             Some(n) => write!(f, "{:0>2}", n % 100)?,
        //             None => write!(f, "--")?,
        //         }
        //     }
        //     writeln!(f)?;
        // }
        // Ok(())

        // display solution path through the terrain
        let mut path_nodes = Vec::new();

        let end = self
            .nodes
            .iter()
            .find(|x| x.node_pos == NodePos::End)
            .unwrap();
        path_nodes.push(end);
        let mut chars = vec!["-"; self.nodes.len()];

        let mut i = 0;

        while let Some(node) = path_nodes.pop() {
            chars[node.coords.0 * self.width + node.coords.1] = "x";
            if node.distance.get() == Some(1) {
                break;
            }
            for coord in &node.neighbours {
                let n = self.index(*coord);
                if let Some(dist) = n.distance.get() {
                    if dist == 0 {
                        break;
                    }
                    if dist == node.distance.get().unwrap() - 1 {
                        path_nodes.push(n);
                        i += 1;
                    }
                }
            }
        }

        for row in 0..(self.nodes.len() / self.width) {
            for col in 0..self.width {
                write!(f, "{}", chars[row * self.width + col])?;
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
                node_pos,
                neighbours,
                distance: Cell::new(None),
                crowfly: Cell::new(None),
                coords: (i / width, i % width),
            });

            i += 1;
        }

        let end = nodes
            .iter()
            .position(|x| x.node_pos == NodePos::End)
            .unwrap();

        let end_coords = (end / width, end % width);

        for node in &nodes {
            node.crowfly.set(Some(
                (node.coords.0 as i32 - end_coords.0 as i32).pow(2)
                    + (node.coords.1 as i32 - end_coords.1 as i32).pow(2),
            ))
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
    node_pos: NodePos,
    distance: Cell<Option<usize>>,
    neighbours: Vec<(usize, usize)>,
    crowfly: Cell<Option<i32>>, // heuristic for A* algorithm
    coords: (usize, usize),
}

impl Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Elevation: {}, Pos: {:?}, Distance: {:?}, Neighbours: {:?}",
            self.elevation, self.node_pos, self.distance, self.neighbours
        )
    }
}

impl Display for TreeNode {
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
            .crowfly
            .get()
            .unwrap()
            .cmp(&self.crowfly.get().unwrap())
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
            let node_dist = node.distance.get().unwrap();

            if node.node_pos == NodePos::End {
                dist = node_dist;
            }

            if node_dist >= dist {
                continue;
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

    let max_iter = usize::MAX;

    // part 1
    let start = grid1
        .nodes
        .iter()
        .filter(|x| x.node_pos == NodePos::Start)
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
