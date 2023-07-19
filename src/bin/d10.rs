use std::fs::read_to_string;

fn check_cycle(cycle: &i32, x: &i32) {
    let pos = cycle % 40;
    if (pos - 1 - x).abs() <= 1 {
        print!("#")
    } else {
        print!(".")
    }

    if pos == 0 {
        print!("\n");
    }
}

fn main() {
    let f = read_to_string("d10.txt").unwrap();

    let mut x = 1;
    let mut cycle = 1;

    for line in f.lines() {
        match &line[0..4] {
            "noop" => {
                check_cycle(&cycle, &x);
                cycle += 1;
            }

            "addx" => {
                for _ in 0..2 {
                    check_cycle(&cycle, &x);
                    cycle += 1;
                }
                x += &line[5..].parse::<i32>().unwrap();
            }
            _ => unreachable!(),
        }
    }
}
