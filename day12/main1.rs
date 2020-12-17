use std::io;
use std::vec::Vec;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    
    let dir: Vec<(i32, i32)> = vec![
        (0, 1), // east
        (1, 0), // south
        (0, -1), // west
        (-1, 0), // north
    ];

    let mut pos: (i32, i32) = (0, 0);
    let mut inc: usize = 0;

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        if buffer.ends_with('\n') {
            buffer.pop();
        }

        let chars: Vec<char> = buffer.chars().collect();
        let cmd = chars[0];
        let val: i32 = chars[1..chars.len()].iter().collect::<String>().parse().unwrap();

        match cmd {
            'E' => {
                pos.0 += dir[0].0 * val;
                pos.1 += dir[0].1 * val;
            },
            'S' => {
                pos.0 += dir[1].0 * val;
                pos.1 += dir[1].1 * val;
            },
            'W' => {
                pos.0 += dir[2].0 * val;
                pos.1 += dir[2].1 * val;
            },
            'N' => {
                pos.0 += dir[3].0 * val;
                pos.1 += dir[3].1 * val;
            },
            'F' => {
                pos.0 += dir[inc].0 * val;
                pos.1 += dir[inc].1 * val;
            },
            'R' => {
                let val_idx = val / 90;
                inc = (inc + val_idx as usize) % 4;
            },
            'L' => {
                let val_idx = (360 - val) / 90;
                inc = (inc + val_idx as usize) % 4;
            },
            _ => (),
        }

        buffer.clear();
    }

    println!("{}", pos.0.abs() + pos.1.abs());
}