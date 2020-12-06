use std::io;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut set = HashSet::new();

    let mut result: i32 = -1;
    let mut buffer = String::new();

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        buffer.pop();

        let number: i32 = buffer.parse().unwrap();
        let target: i32 = 2020 - number;

        if set.contains(&target) {
            result = target * number;
        } else {
            set.insert(number);
        }

        buffer.clear();
    }

    println!("{}", result)
}