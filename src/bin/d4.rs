use std::fs::read_to_string;

fn main() {
    let f = read_to_string("d4.txt").unwrap();

    let cnt: Vec<u32> = f
        .lines()
        .map(|line| {
            let nums: Vec<u32> = line
                .split(|c| c == '-' || c == ',')
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            if !(nums[0] > nums[3] || nums[2] > nums[1]) {
                return 1;
            } else {
                return 0;
            }
        })
        .collect();
    println!("{}", cnt.iter().sum::<u32>())
}
