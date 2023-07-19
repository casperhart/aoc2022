use std::{collections::HashSet, fs::read_to_string};

struct Rope {
    knots: Vec<(i32, i32)>,
    visited: HashSet<(i32, i32)>,
}

impl Rope {
    fn new() -> Self {
        Self {
            knots: vec![(0, 0); 10],
            visited: HashSet::from([(0, 0)]),
        }
    }

    fn step(&mut self, direction: char) {
        match direction {
            'U' => self.knots[0].1 += 1,
            'D' => self.knots[0].1 -= 1,
            'L' => self.knots[0].0 -= 1,
            'R' => self.knots[0].0 += 1,
            _ => unreachable!(),
        }

        let len = self.knots.len();
        for i in 0..(len - 1) {
            let s = &mut self.knots[i..(i + 2)];
            match (s[0].0 - s[1].0).pow(2) + (s[0].1 - s[1].1).pow(2) {
                4 | 5 | 8 => {
                    s[1].0 += (s[0].0 - s[1].0).signum();
                    s[1].1 += (s[0].1 - s[1].1).signum();
                    println!(
                        "{} Moved: H: {:?}, T: {:?}",
                        i,
                        (s[0].0, s[0].1),
                        (s[1].0, s[1].1)
                    );
                }
                _ => {
                    println!(
                        "{} Not moved: H: {:?}, T: {:?}",
                        i,
                        (s[0].0, s[0].1),
                        (s[1].0, s[1].1)
                    );
                }
            }
        }

        self.visited.insert(*self.knots.last().unwrap());
    }
}

fn main() {
    let f = read_to_string("d9.txt").unwrap();
    let lines = f.lines();
    let mut rope = Rope::new();
    let mut direction: char;
    let mut n: usize;

    for line in lines {
        direction = line.chars().nth(0).unwrap();
        n = line[2..].parse::<usize>().unwrap();

        for _ in 0..n {
            rope.step(direction)
        }
    }

    println!("Number of squares visited: {}", rope.visited.len());

    for row in (-15..15).rev() {
        for col in -30..30 {
            if rope.visited.contains(&(col, row)) {
                print!("#")
            } else {
                print!("-")
            }
        }
        print!("\n")
    }
}
