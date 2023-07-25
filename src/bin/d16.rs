use std::{collections::BinaryHeap, fmt::Display, fs::read_to_string, rc::Rc, time::Instant};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Valve {
    name: String,
    flow_rate: usize,
    children: Vec<usize>,
}

impl Valve {
    fn new(s: &str, names: &[&str]) -> Self {
        let v = s
            .split(&[' ', '=', ';', ','])
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();

        Self {
            name: v[1].to_string(),
            flow_rate: v[5].parse().unwrap(),
            children: v[10..]
                .iter()
                .map(|n| names.iter().position(|p| p == n).unwrap())
                .collect(),
        }
    }
}

#[derive(Debug)]
struct Graph {
    valves: Vec<Valve>,
    working_valves: Vec<usize>,
    distance_matrix: Vec<usize>,
    n: usize,
}

impl Graph {
    fn new(s: &str) -> Self {
        let valve_names: Vec<&str> = s
            .lines()
            .map(|x| x.split_whitespace().nth(1).unwrap())
            .collect();

        let valves: Vec<_> = s.lines().map(|x| Valve::new(x, &valve_names)).collect();

        let n = valves.len();
        let mut distance_matrix = vec![usize::MAX; n.pow(2)];

        for (i, valve) in valves.iter().enumerate() {
            for child in &valve.children {
                distance_matrix[i * n + child] = 1;
            }
        }

        let working_valves = valves
            .iter()
            .enumerate()
            .filter(|(_, v)| v.flow_rate > 0)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        Self {
            valves,
            distance_matrix,
            n,
            working_valves,
        }
    }

    fn get_distance(&self, i: usize, j: usize) -> usize {
        self.distance_matrix[i * self.n + j]
    }

    fn set_distance(&mut self, i: usize, j: usize, v: usize) {
        self.distance_matrix[i * self.n + j] = v;
    }

    fn floyd_warshall(&mut self) {
        let mut s1;
        let mut s2;
        let mut s;
        for k in 0..self.n {
            for i in 0..self.n {
                for j in 0..self.n {
                    s = self.get_distance(i, j);
                    s1 = self.get_distance(i, k);
                    s2 = self.get_distance(k, j);
                    if i == j {
                        self.set_distance(i, j, 0)
                    } else if s1.checked_add(s2).unwrap_or(usize::MAX) < s {
                        self.set_distance(i, j, s1 + s2);
                    }
                }
            }
        }
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.n {
            write!(f, "{:>2}: ", i)?;
            for j in 0..self.n {
                let v = self.get_distance(i, j);
                if v == usize::MAX {
                    write!(f, ".. ")?;
                } else {
                    write!(f, "{:>2} ", v)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct State {
    graph: Rc<Graph>,
    current_valve: usize,
    path: Vec<usize>,
    time: usize,
    flow: usize,
    max_flow: usize,
}

impl State {
    fn new(graph: Rc<Graph>) -> Self {
        let starting_value = graph.valves.iter().position(|x| x.name == "AA").unwrap();
        Self {
            graph,
            current_valve: starting_value,
            path: Vec::with_capacity(15),
            time: 30,
            flow: 0,
            max_flow: 0,
        }
    }

    fn update_max_flow(&mut self) {
        let max_remaining_flow = self
            .graph
            .working_valves
            .iter()
            .filter(|f| !self.path.contains(f))
            .map(|&f| {
                let dist = self.graph.get_distance(self.current_valve, f);
                let remaining_time = self.time.saturating_sub(dist);
                self.graph.valves[f].flow_rate * remaining_time
            })
            .sum::<usize>();

        self.max_flow = self.flow + max_remaining_flow;
    }

    fn move_to(&mut self, new_valve: usize) -> Option<()> {
        let dist = self.graph.get_distance(self.current_valve, new_valve) + 1;
        if dist > self.time {
            return None;
        }
        self.time -= dist;
        self.path.push(new_valve);
        self.current_valve = new_valve;
        self.flow += self.graph.valves[new_valve].flow_rate * self.time;
        self.update_max_flow();
        Some(())
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.max_flow.cmp(&other.max_flow)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for State {}

fn main() {
    let start = Instant::now();
    let f = read_to_string("d16.txt").expect("could not read input");
    let mut graph = Graph::new(f.as_str());

    graph.floyd_warshall();

    let mut state = State::new(Rc::new(graph));
    let mut heap = BinaryHeap::new();
    let mut bound = 0;
    let mut best_state = state.clone();

    // heuristic to use as upper bound. I.e. if max_flow is less than the current
    // best answer, we can drop that state
    state.update_max_flow();
    heap.push(state);

    while let Some(s) = heap.pop() {
        for valve in s
            .graph
            .working_valves
            .iter()
            .filter(|v| !s.path.contains(v))
        {
            let mut next_state = s.clone();
            let res = next_state.move_to(*valve);
            if res.is_some() {
                if next_state.flow > bound {
                    bound = next_state.flow;
                    best_state = next_state.clone();
                }
                if next_state.max_flow > bound {
                    heap.push(next_state);
                }
            }
        }
    }

    println!("Max flow: {}", best_state.flow);
    println!("Path: {:?}", best_state.path);
    println!("Elapsed time: {:?}", start.elapsed());
}
