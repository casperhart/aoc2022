use std::fs::read_to_string;

fn main() {
    let f = read_to_string("d6.txt").unwrap();
    let alpha = "abcdefghijklmnopqrstuvwxyz";

    let line = f.lines().next().unwrap();

    let encoded_line: Vec<u32> = line
        .chars()
        .map(|c| 1 << alpha.chars().position(|ch| ch == c).unwrap())
        .collect();

    let mut i = 14;

    for ind in encoded_line.windows(14) {
        if (ind.iter().fold(0, |a, b| (a | b))).count_ones() == 14 {
            break;
        }
        i += 1;
    }
    println!("{}", i)
}
