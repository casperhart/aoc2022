use serde_json::{json, Value};
use std::{fs::read_to_string, str::Lines};

fn main() {
    let mut i = 1;
    let mut tot = 0;

    let f = read_to_string("d13.txt").expect("Failed to read file");
    let mut lines1 = f.lines();

    let mut l = Value::Null;
    let mut r = Value::Null;

    loop {
        match read_next_lines(&mut lines1) {
            Some((l_str, r_str)) => {
                l = serde_json::from_str(l_str).expect("failed to parse json left");
                r = serde_json::from_str(r_str).expect("failed to parse json right");
            }
            None => break,
        }

        match compare(Some(l), Some(r)) {
            Some(true) => {
                tot += i;
            }
            Some(false) => (),
            None => unreachable!(),
        }
        i += 1
    }

    println!("Part 1 total: {:?}", tot);

    // part 2
    let lines2 = f.lines();

    let mut packets = lines2
        .into_iter()
        .filter(|x| x.len() != 0)
        .map(|x| serde_json::from_str(x).expect("failed to parse line"))
        .collect::<Vec<Value>>();

    packets.push(json!([[2]]));
    packets.push(json!([[6]]));

    packets.sort_by(|x, y| match compare(Some(x.clone()), Some(y.clone())) {
        Some(true) => std::cmp::Ordering::Less,
        Some(false) => std::cmp::Ordering::Greater,
        None => std::cmp::Ordering::Equal,
    });

    let ind1 = packets.iter().position(|x| *x == json!([[2]])).unwrap() + 1;
    let ind2 = packets.iter().position(|x| *x == json!([[6]])).unwrap() + 1;

    println!("{:?}", ind1);
    println!("{:?}", ind2);
    println!("{:?}", ind1 * ind2);
}

fn read_next_lines<'a>(lines: &'a mut Lines) -> Option<(&'a str, &'a str)> {
    let l_str = lines.next();
    if l_str == None {
        return None;
    }
    let r_str = lines.next();
    lines.next();

    Some((l_str.unwrap(), r_str.unwrap()))
}

fn compare(l: Option<Value>, r: Option<Value>) -> Option<bool> {
    let matches = match (l, r) {
        (Some(v0), Some(v1)) => match (v0, v1) {
            (Value::Array(mut u), Value::Array(mut v)) => {
                u.reverse();
                v.reverse();
                loop {
                    let next_l = u.pop();
                    let next_r = v.pop();

                    // both arrays are empty
                    if next_l == None && next_r == None {
                        break None;
                    }

                    if let Some(result) = compare(next_l, next_r) {
                        break Some(result);
                    }
                }
            }
            (Value::Number(u), Value::Array(v)) => compare(Some(json!([u])), Some(Value::Array(v))),
            (Value::Array(u), Value::Number(v)) => compare(Some(Value::Array(u)), Some(json!([v]))),
            (Value::Number(u), Value::Number(v)) => {
                match (v.as_i64().unwrap() - u.as_i64().unwrap()).signum() {
                    1 => Some(true),
                    0 => None,
                    -1 => Some(false),
                    _ => unreachable!(),
                }
            }
            (_, _) => unreachable!(),
        },
        (Some(_), None) => Some(false),
        (None, Some(_)) => Some(true),
        (None, None) => unreachable!(),
    };
    matches
}
