use std::io;
use std::vec::Vec;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let mut arr: Vec<Vec<char> > = Vec::new();
    let x_point: [usize; 5] = [1, 1, 1, 1, 2];
    let y_point: [usize; 5] = [1, 3, 5, 7, 1];

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        if buffer.ends_with('\n') {
            buffer.pop();
        }

        arr.push(buffer.chars().collect());

        buffer.clear();
    }

    let mut res: u64 = 1;
    let mut it = 0;

    while it < x_point.len() {
        let mut x = x_point[it];
        let mut y = y_point[it];

        let mut trees = 0;

        while x < arr.len() {
            if arr[x][y] == '#' {
                trees += 1;
            }

            x = x + x_point[it];
            y = (y + y_point[it]) % arr[0].len();
        }

        res *= trees;
        it += 1;
    }

    println!("{}", res);
}