use std::io;
use std::vec::Vec;

fn main() {
    let stdin = io::stdin();

    let mut res: i32 = 0;
    let mut buffer = String::new();

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        if buffer.ends_with('\n') {
            buffer.pop();
        }

        let input: Vec<&str> = buffer.split(' ').collect();
        let min_max: Vec<&str> = input[0].split('-').collect();
        let chara: char = input[1].chars().nth(0).unwrap();
        let string: Vec<char> = input[2].chars().collect();

        let min: usize = min_max[0].parse().unwrap();
        let max: usize = min_max[1].parse().unwrap();

        if (string[min - 1] == chara) ^ (string[max - 1] == chara) {
            res += 1;
        }

        buffer.clear();
    }

    println!("{}", res);
}