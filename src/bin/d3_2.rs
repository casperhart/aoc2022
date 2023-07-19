use std::fs::read_to_string;

const priorities: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let f = read_to_string("d3.txt").unwrap();

    let mut p_tot = 0;
    let mut lines = f.lines();
    let mut line;
    let mut line_enc: u64;

    loop {
        line = lines.next();
        match line {
            Some(l) => {
                line_enc = encode_line(l);
                println!("{}", l);
            }
            None => {
                println!("End of loop");
                break;
            }
        }

        for _ in 0..2 {
            let l = lines.next().unwrap();
            line_enc = line_enc & encode_line(l);
            println!("{}", l);
        }
        println!(
            "{}",
            priorities
                .chars()
                .nth(line_enc.trailing_zeros() as usize)
                .unwrap()
        );
        p_tot = p_tot + line_enc.trailing_zeros() + 1;
    }

    println!("{}", p_tot);
}

fn encode_line(line: &str) -> u64 {
    let mut result: u64 = 0;
    for ch in line.chars() {
        result = result | 1 << priorities.chars().position(|c| c == ch).unwrap();
    }

    result
}
