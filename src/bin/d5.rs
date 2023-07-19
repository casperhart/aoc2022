use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("d5.txt").unwrap();
    let mut lines = f.lines();

    let mut stacks: Vec<VecDeque<char>> = (0..10)
        .map(|_| VecDeque::<char>::with_capacity(30))
        .collect();

    loop {
        let line = lines.next().unwrap();

        for stack_num in 1..10 {
            match line.chars().nth(stack_num * 4 - 3) {
                Some(v) => match v {
                    ' ' => (),
                    _ => stacks[stack_num].push_back(v),
                },
                None => break,
            }
        }

        if line == "" {
            break;
        }
    }

    let mut cnt: usize;
    let mut from: usize;
    let mut to: usize;
    let mut c: char;

    loop {
        let line = lines.next();
        if line == None {
            break;
        }

        let s: Vec<&str> = line.unwrap().split(' ').collect();

        cnt = s[1].parse().unwrap();
        from = s[3].parse().unwrap();
        to = s[5].parse().unwrap();

        for _ in 0..cnt {
            c = stacks[from].pop_front().unwrap();
            stacks[0].push_front(c);
        }

        for _ in 0..cnt {
            c = stacks[0].pop_front().unwrap();
            stacks[to].push_front(c);
        }
    }
    for stack in stacks {
        println!("{:?}", stack)
    }
}
