use std::fmt::Display;
use std::fs::read_to_string;
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new() -> Self {
        Self { x: 500, y: 0 }
    }

    fn below(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn below_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }
    fn below_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }

    fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn interpolate(&self, other: &Self) -> Vec<Coord> {
        if self.x == other.x {
            let y_seq = if self.y < other.y {
                self.y..=other.y
            } else {
                other.y..=self.y
            };
            y_seq.map(|y| Self { x: self.x, y }).collect()
        } else {
            let x_seq = if self.x < other.x {
                self.x..=other.x
            } else {
                other.x..=self.x
            };
            x_seq.map(|x| Self { x, y: self.y }).collect()
        }
    }
}

#[derive(Debug)]
struct ParseCoordErr;

impl FromStr for Coord {
    type Err = ParseCoordErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let t = s.split(",").collect::<Vec<_>>();
        let x = t[0].parse().map_err(|_| ParseCoordErr)?;
        let y = t[1].parse().map_err(|_| ParseCoordErr)?;
        Ok(Self { x, y })
    }
}

type Path = Vec<Coord>;

struct Grid {
    rocks: HashSet<Coord>,
    sand: HashSet<Coord>,
    floor: usize,
    full: bool,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..170 {
            for x in 450..550 {
                if (x, y) == (500, 0) {
                    write!(f, "x")?
                } else if self.rocks.contains(&Coord { x, y }) {
                    print!("#")
                } else if self.sand.contains(&Coord { x, y }) {
                    print!("o")
                } else {
                    print!(".")
                }
            }
            println!()
        }
        Ok(())
    }
}

impl From<String> for Grid {
    fn from(value: String) -> Self {
        let lines = value.lines().collect::<Vec<_>>();

        let mut grid = Self {
            rocks: HashSet::new(),
            sand: HashSet::new(),
            floor: 0,
            full: false,
        };

        let paths: Vec<Path> = lines
            .iter()
            .map(|l| {
                l.split(" -> ")
                    .map(|x| Coord::from_str(x).expect("Could not parse coord from string"))
                    .collect::<Path>()
            })
            .collect();

        for path in &paths {
            grid.add_rock_path(path)
        }

        grid.floor = grid.rocks.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y + 2;

        grid
    }
}

impl Grid {
    fn add_rock_path(&mut self, p: &Path) {
        for coord_pair in p.windows(2) {
            for coord in coord_pair[0].interpolate(&coord_pair[1]) {
                self.rocks.insert(coord);
            }
        }
    }

    fn add_sand(&mut self) {
        let mut sand_pos = Coord::new();
        let mut below_pos: Coord;
        let mut below_left_pos: Coord;
        let mut below_right_pos: Coord;

        loop {
            below_pos = sand_pos.below();

            if self.check_candidate(&below_pos) {
                sand_pos = below_pos;
                continue;
            }

            below_left_pos = sand_pos.below_left();
            if self.check_candidate(&below_left_pos) {
                sand_pos = below_left_pos;
                continue;
            }

            below_right_pos = sand_pos.below_right();
            if self.check_candidate(&below_right_pos) {
                sand_pos = below_right_pos;
                continue;
            }

            if !self.check_candidate(&below_right_pos) && sand_pos.x == 500 && sand_pos.y == 0 {
                self.sand.insert(sand_pos);
                self.full = true;
                break;
            }

            self.sand.insert(sand_pos);
            break;
        }
    }

    fn check_candidate(&self, candidate: &Coord) -> bool {
        !self.rocks.contains(candidate)
            && !self.sand.contains(candidate)
            && candidate.y < self.floor
    }
}

fn main() {
    let f = read_to_string("d14.txt").unwrap();
    let mut grid = Grid::from(f);

    //while !grid.full {
    for _ in 0..50000 {
        grid.add_sand();
    }

    println!("{}", grid);
    println!("{}", grid.sand.len());
    println!("{}", grid.floor);
}
