use std::fs::read_to_string;

const priorities: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let f = read_to_string("d3.txt").unwrap();

    let mut p_tot = 0;

    for line in f.lines() {
        p_tot += get_priority(line);
    }

    println!("{}", p_tot);
}

fn get_priority(line: &str) -> u32 {
    let n = line.len() / 2;

    let l = encode_line(&line[..n]);
    let r = encode_line(&line[n..]);

    let overlap = l & r;

    println!("{}", overlap.trailing_zeros());

    overlap.trailing_zeros() + 1
}

fn encode_line(line: &str) -> u64 {
    let mut result: u64 = 0;
    for ch in line.chars() {
        result = result | 1 << priorities.chars().position(|c| c == ch).unwrap();
    }

    result
}
