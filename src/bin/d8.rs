use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug)]
struct Forest {
    tree_heights: Vec<usize>,
    width: usize,
    height: usize,
}

impl Forest {
    fn get_index(&self, row: usize, col: usize) -> usize {
        self.tree_heights[col + row * self.width]
    }

    fn get_visible_left(&self) -> HashSet<(usize, usize)> {
        let mut visible = HashSet::new();
        let mut tree_height: i32 = -1;

        for row in 0..self.height {
            for col in 0..self.width {
                let h = &self.get_index(row, col);
                if *h as i32 > tree_height {
                    tree_height = *h as i32;
                    visible.insert((row, col));
                }
            }
            tree_height = -1;
        }
        visible
    }

    fn get_visible_right(&self) -> HashSet<(usize, usize)> {
        let mut visible = HashSet::new();
        let mut tree_height: i32 = -1;

        for row in 0..self.height {
            for col in (0..self.width).rev() {
                let h = &self.get_index(row, col);
                if *h as i32 > tree_height {
                    tree_height = *h as i32;
                    visible.insert((row, col));
                }
            }
            tree_height = -1;
        }
        visible
    }

    fn get_visible_top(&self) -> HashSet<(usize, usize)> {
        let mut visible = HashSet::new();
        let mut tree_height: i32 = -1;

        for col in 0..self.width {
            for row in 0..self.height {
                let h = &self.get_index(row, col);
                if *h as i32 > tree_height {
                    tree_height = *h as i32;
                    visible.insert((row, col));
                }
            }
            tree_height = -1;
        }
        visible
    }

    fn get_visible_bottom(&self) -> HashSet<(usize, usize)> {
        let mut visible = HashSet::new();
        let mut tree_height: i32 = -1;

        for col in 0..self.width {
            for row in (0..self.height).rev() {
                let h = &self.get_index(row, col);
                if *h as i32 > tree_height {
                    tree_height = *h as i32;
                    visible.insert((row, col));
                }
            }
            tree_height = -1;
        }
        visible
    }

    fn get_scenic_score(&self, row: usize, col: usize) -> usize {
        let tree_height = self.get_index(row, col);
        let trees_above: Vec<_> = (0..row).map(|i| self.get_index(i, col)).rev().collect();
        let trees_below: Vec<_> = ((row + 1)..self.height)
            .map(|i| self.get_index(i, col))
            .collect();
        let trees_right: Vec<_> = ((col + 1)..self.width)
            .map(|i| self.get_index(row, i))
            .collect();
        let trees_left: Vec<_> = (0..col).map(|i| self.get_index(row, i)).rev().collect();

        let neighbours = vec![trees_above, trees_below, trees_left, trees_right];

        neighbours
            .iter()
            .map(|v| match v.iter().position(|t| *t >= tree_height) {
                Some(v) => v + 1,
                None => std::cmp::max(v.len(), 1),
            })
            .product()
    }
}

fn main() {
    let f = read_to_string("d8.txt").expect("Could not read d8.txt");
    let mut lines = f.lines();
    let l = lines.next().unwrap();

    let width = l.len();
    let mut height = 1;

    let mut tree_heights: Vec<usize> = l
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    while let Some(line) = lines.next() {
        let mut new_line: Vec<usize> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        tree_heights.append(&mut new_line);
        height += 1;
    }

    let forest = Forest {
        tree_heights,
        width,
        height,
    };

    let visible_left = forest.get_visible_left();
    let visible_right = forest.get_visible_right();
    let visible_top = forest.get_visible_top();
    let visible_bottom = forest.get_visible_bottom();

    println!(
        "Visible trees: {:?}",
        visible_left
            .union(&visible_right)
            .cloned()
            .collect::<HashSet<_>>()
            .union(&visible_top)
            .cloned()
            .collect::<HashSet<_>>()
            .union(&visible_bottom)
            .cloned()
            .collect::<HashSet<_>>()
            .len()
    );

    let mut scenic_score = 0;
    for col in 0..forest.width {
        for row in 0..forest.height {
            let s = forest.get_scenic_score(row, col);
            if s > scenic_score {
                scenic_score = s;
            }
        }
    }
    println!("{}", scenic_score);
}
