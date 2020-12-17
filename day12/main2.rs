use std::io;
use std::f64::consts::PI;
use std::vec::Vec;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    
    let mut wp: (i32, i32) = (1, 10);
    let mut pos: (i32, i32) = (0, 0);

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        if buffer.ends_with('\n') {
            buffer.pop();
        }

        let chars: Vec<char> = buffer.chars().collect();
        let cmd = chars[0];
        let val: i32 = chars[1..chars.len()].iter().collect::<String>().parse().unwrap();

        match cmd {
            'E' => {
                wp.1 += val;
            },
            'S' => {
                wp.0 -= val;
            },
            'W' => {
                wp.1 -= val;
            },
            'N' => {
                wp.0 += val;
            },
            'F' => {
                pos.0 += wp.0 * val;
                pos.1 += wp.1 * val;
            },
            'R' => {
                let val_rad: f64 = -val as f64 * PI / 180.0;

                let wpx = wp.1 as f64 * val_rad.cos() - wp.0 as f64 * val_rad.sin();
                let wpy = wp.1 as f64 * val_rad.sin() + wp.0 as f64 * val_rad.cos();

                wp.1 = wpx.round() as i32;
                wp.0 = wpy.round() as i32;
            },
            'L' => {
                let val_rad = val as f64 * PI / 180.0;

                let wpx = wp.1 as f64 * val_rad.cos() - wp.0 as f64 * val_rad.sin();
                let wpy = wp.1 as f64 * val_rad.sin() + wp.0 as f64 * val_rad.cos();

                wp.1 = wpx.round() as i32;
                wp.0 = wpy.round() as i32;
            },
            _ => (),
        }

        buffer.clear();
    }

    println!("{}", pos.0.abs() + pos.1.abs());
}