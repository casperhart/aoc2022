use std::{collections::HashSet, fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct ParseCoordError;

impl FromStr for Coord {
    type Err = ParseCoordError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(", ");
        let x = split.next().unwrap()[2..]
            .parse()
            .map_err(|_| ParseCoordError)?;
        let y = split.next().unwrap()[2..]
            .parse()
            .map_err(|_| ParseCoordError)?;

        Ok(Self { x, y })
    }
}

impl Coord {
    fn sensor_in_range(&self, sensor: &Sensor) -> bool {
        let distance = (self.x - sensor.position.x).abs() + (self.y - sensor.position.y).abs();
        distance <= sensor.distance
    }

    fn get_horizontal_skip(&self, sensor: &Sensor) -> i32 {
        if !self.sensor_in_range(sensor) {
            return 0;
        }

        let y_diff = (self.y - sensor.position.y).abs();

        let x_pos = sensor.distance - y_diff + sensor.position.x + 1;
        x_pos - self.x
    }
}

type Beacon = Coord;

#[derive(Debug, Hash)]
struct Sensor {
    closest_beacon: Beacon,
    position: Coord,
    distance: i32,
}

#[derive(Debug)]
struct ParseSensorError;

impl FromStr for Sensor {
    type Err = ParseSensorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x_inds = s.match_indices('x').map(|(i, _)| i).collect::<Vec<_>>();
        let y_inds = s.match_indices(':').map(|(i, _)| i).collect::<Vec<_>>();

        let sensor_str = &s[x_inds[0]..y_inds[0]];
        let beacon_str = &s[x_inds[1]..s.len()];

        let sensor_coord = Coord::from_str(sensor_str).expect("could not parse sensor coordinate");
        let beacon_coord = Coord::from_str(beacon_str).unwrap();

        let distance =
            (sensor_coord.x - beacon_coord.x).abs() + (sensor_coord.y - beacon_coord.y).abs();

        Ok(Sensor {
            position: sensor_coord,
            closest_beacon: beacon_coord,
            distance,
        })
    }
}

#[derive(Debug)]
struct Grid {
    sensors: Vec<Sensor>,
    beacons: HashSet<Beacon>,
}

impl FromStr for Grid {
    type Err = ParseSensorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let sensors = lines
            .map(|x| {
                Sensor::from_str(x).unwrap_or_else(|_| panic!("could not parse sensor for {}", x))
            })
            .collect::<Vec<_>>();

        let beacons = sensors
            .iter()
            .map(|x| x.closest_beacon)
            .collect::<HashSet<_>>();
        Ok(Self { sensors, beacons })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..=20 {
            write!(f, "{:>2}", row)?;
            for col in 0..=20 {
                let c = Coord { x: col, y: row };
                if self.sensors.iter().any(|x| x.position == c) {
                    write!(f, "S")?;
                } else if self.sensors.iter().any(|x| x.closest_beacon == c) {
                    write!(f, "B")?;
                } else if self.any_in_range(Coord { x: col, y: row }) {
                    write!(f, "x")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl Grid {
    fn get_x_range(&self) -> (i32, i32) {
        let min = self
            .sensors
            .iter()
            .map(|s| s.position.x - s.distance - 1)
            .min()
            .unwrap();
        let max = self
            .sensors
            .iter()
            .map(|s| s.position.x + s.distance + 1)
            .max()
            .unwrap();
        (min, max)
    }

    fn any_in_range(&self, c: Coord) -> bool {
        let contains_beacon = self.beacons.contains(&c);
        let contains_sensor = self.sensors.iter().any(|x| x.position == c);
        self.sensors.iter().any(|s| c.sensor_in_range(s)) && !contains_beacon && !contains_sensor
    }
}

fn main() {
    // part 1
    let f = std::fs::read_to_string("d15.txt").unwrap();
    let grid = Grid::from_str(f.as_str()).unwrap();

    let y = 2000000;

    let (x_min, x_max) = grid.get_x_range();

    let n_squares: i32 = (x_min..=x_max)
        .map(|x| grid.any_in_range(Coord { x, y }) as i32)
        .sum();

    println!("n squares where a beacon cannot exist: {}", n_squares);

    // part 2
    let mut current_coord = Coord { x: 0, y: 0 };
    let mut skip_distance: i32;
    let range = 4_000_000;

    loop {
        skip_distance = grid
            .sensors
            .iter()
            .map(|s| current_coord.get_horizontal_skip(s))
            .max()
            .unwrap();

        if skip_distance == 0 {
            break;
        }
        current_coord.x += skip_distance;
        if current_coord.x > range {
            current_coord.x = 0;
            current_coord.y += 1;
        }
        if current_coord.y > range {
            break;
        }
    }

    println!("Beacon position: {:?}", current_coord);
    println!(
        "Tuning frequency: {:?}",
        current_coord.x as u64 * 4_000_000 + current_coord.y as u64
    );
}
