use std::{collections::VecDeque, fmt::Debug, fs::read_to_string};

struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    divisor: u64,
    throw_to: (usize, usize),
    inspected: usize,
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            items: VecDeque::new(),
            operation: Operation::Identity,
            divisor: 0,
            throw_to: (0, 0),
            inspected: 0,
        }
    }
    fn with_items(&mut self, items: VecDeque<u64>) {
        self.items = items
    }
    fn with_operation(&mut self, operation: Operation) {
        self.operation = operation
    }
    fn with_divisor(&mut self, divisor: u64) {
        self.divisor = divisor
    }
    fn with_if_true(&mut self, if_true: usize) {
        self.throw_to.0 = if_true
    }
    fn with_if_false(&mut self, if_false: usize) {
        self.throw_to.1 = if_false
    }

    fn inspect_and_pop(&mut self) -> u64 {
        let worry_level = self.items.pop_front().unwrap();
        self.inspected += 1;
        self.operation.compute(worry_level)
    }

    fn get_dest_monkey(&self, worry_level: u64) -> usize {
        match worry_level % self.divisor == 0 {
            true => self.throw_to.0,
            false => self.throw_to.1,
        }
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Items: {:?}, Divisor: {:?}, Throw_to: {:?}",
            self.items, self.divisor, self.throw_to
        )
    }
}

enum Operation {
    AddVal(u64),
    MulVal(u64),
    AddSelf,
    MulSelf,
    Identity,
}

impl Operation {
    fn compute(&self, old: u64) -> u64 {
        match self {
            Self::AddVal(val) => old + val,
            Self::MulVal(val) => old * val,
            Self::AddSelf => old + old,
            Self::MulSelf => old * old,
            Self::Identity => old,
        }
    }
}
fn main() {
    let f = read_to_string("d11.txt").unwrap();
    let mut monkeys: Vec<Monkey> = Vec::with_capacity(10);
    let mut lines = f.lines();
    let mut monkey = Monkey::new();

    loop {
        match lines.next() {
            Some("") => {
                monkeys.push(monkey);
                monkey = Monkey::new();
            }
            None => {
                monkeys.push(monkey);
                break;
            }
            Some(line) => {
                let s = line.split(':').collect::<Vec<_>>();
                if s.len() == 2 {
                    match s[0].trim() {
                        "Starting items" => monkey.with_items(
                            s[1].split(',')
                                .map(|v| v.trim().parse::<u64>().unwrap())
                                .collect(),
                        ),
                        "Operation" => {
                            let op_str: Vec<_> = s[1][7..].split(' ').collect();
                            let operation = match op_str[2] {
                                "old" => match op_str[1] {
                                    "+" => Operation::AddSelf,
                                    "*" => Operation::MulSelf,
                                    _ => unreachable!(),
                                },
                                v => {
                                    let v_parsed = v.parse::<u64>().unwrap();
                                    match op_str[1] {
                                        "+" => Operation::AddVal(v_parsed),
                                        "*" => Operation::MulVal(v_parsed),
                                        _ => unreachable!(),
                                    }
                                }
                            };
                            monkey.with_operation(operation);
                        }
                        "Test" => monkey.with_divisor(s[1][13..].trim().parse::<u64>().unwrap()),
                        "If true" => {
                            monkey.with_if_true(s[1][16..].trim().parse::<usize>().unwrap())
                        }
                        "If false" => {
                            monkey.with_if_false(s[1][16..].trim().parse::<usize>().unwrap())
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    let worry_divisor: u64 = monkeys.iter().map(|x| x.divisor).product();

    for _ in 1..=10000 {
        for m_num in 0..monkeys.len() {
            for _ in 0..monkeys[m_num].items.len() {
                let worry_level = monkeys[m_num].inspect_and_pop() % worry_divisor;
                let destination_monkey = monkeys[m_num].get_dest_monkey(worry_level);
                monkeys[destination_monkey].items.push_back(worry_level);
            }
        }
    }

    let mut inspected: Vec<_> = monkeys.iter().map(|m| m.inspected).collect();
    inspected.sort_by(|a, b| b.cmp(a));

    println!("{:?}", inspected[0] * inspected[1]);
}
