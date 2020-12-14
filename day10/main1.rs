use std::io;
use std::vec::Vec;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut arr = Vec::new();

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        if buffer.ends_with('\n') {
            buffer.pop();
        }

        let input: u32 = buffer.parse().unwrap();
        arr.push(input);

        buffer.clear();
    }

    arr.sort();

    let mut result: (u32, u32) = (0, 0);
    let mut prev: u32 = 0;

    for n in 0..arr.len() {
        let diff = arr[n] - prev;

        match diff {
            1 => result.0 += 1,
            3 => result.1 += 1,
            _ => (),
        }

        prev = arr[n];
    }

    println!("{}", result.0 * (result.1 + 1));
}