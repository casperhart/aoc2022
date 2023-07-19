use std::fs::read_to_string;

fn main() {
    let f = read_to_string("d1.txt").unwrap();

    let mut maxs = vec![0, 0, 0];
    let mut tot = 0;

    for line in f.lines() {
        match line {
            "" => {
                push_max(&mut maxs, tot);
                tot = 0
            }
            _ => tot += line.parse::<i32>().unwrap(),
        }
    }
    push_max(&mut maxs, tot);

    println!("{}, {}, {}", maxs[0], maxs[1], maxs[2]);
    println!("{}", maxs.iter().sum::<i32>());
}

fn push_max(v: &mut Vec<i32>, n: i32) -> &Vec<i32> {
    let (i, min) = v
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    if n > *min {
        v[i] = n;
    }
    v
}
