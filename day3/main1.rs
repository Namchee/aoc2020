use std::io;
use std::vec::Vec;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let mut res: i32 = 0;
    let mut arr: Vec<Vec<char>> = Vec::new(); 

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        if buffer.ends_with('\n') {
            buffer.pop();   
        }

        let char_arr: Vec<char> = buffer.chars().collect();

        arr.push(char_arr);

        buffer.clear();
    }

    let mut x = 1;
    let mut y = 3;

    let a = arr.len();
    let b = arr[0].len();

    while x < a {
        if arr[x][y] == '#' {
            res += 1;
        }

        x += 1;
        y = (y + 3) % b;
    }

    println!("{}", res);
}