use std::io;
use std::vec::Vec;

fn main() {
    let stdin = io::stdin();
    let mut count: i32 = 0;
    let mut buffer = String::new();

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        if buffer.ends_with('\n') {
            buffer.pop();
        }

        let input: Vec<&str> = buffer.split(' ').collect();
        let min_max: Vec<&str> = input[0].split('-').collect();
        let chara = input[1].chars().nth(0).unwrap();
        let string = input[2];

        let min: i32 = min_max[0].parse().unwrap();
        let max: i32 = min_max[1].parse().unwrap();
        let mut arr: [i32; 26] = [0; 26];

        for c in string.chars() {
            let code: usize = (c as usize) - 97;
            arr[code] = arr[code] + 1;
        }

        let chara: usize = (chara as usize) - 97;

        if (arr[chara] <= max) && (arr[chara] >= min) {
            count += 1;
        } 

        buffer.clear();
    }

    println!("{}", count);
}