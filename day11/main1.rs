use std::io;
use std::vec::Vec;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let mut arr: Vec<Vec<char> > = Vec::new();

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        if buffer.ends_with('\n') {
            buffer.pop();
        }

        arr.push(buffer.chars().collect());

        buffer.clear();
    }

    let x_len = arr.len();
    let y_len = arr[0].len();

    let loops: Vec<(i16, i16)> = vec![
        (0, 1),
        (1, 0),
        (-1, 0),
        (0, -1),
        (1, -1),
        (-1, 1),
        (1, 1),
        (-1, -1)
    ];

    loop {
        let mut changes = Vec::new();

        for i in 0..x_len {
            for j in 0..y_len {
                match arr[i][j] {
                    'L' => {
                        let mut will_change = true;

                        for loo in loops.iter() {
                            let cur_x = i as i16 + loo.0;
                            let cur_y = j as i16 + loo.1;

                            if cur_x >= 0 && cur_y >= 0 &&
                                cur_x < (x_len as i16) && cur_y < (y_len as i16) &&
                                arr[cur_x as usize][cur_y as usize] == '#' {
                                    will_change = false;
                                    break;
                                }
                        }

                        if will_change {
                            changes.push((i, j));
                        }
                    },
                    '#' => {
                        let mut sum = 0;

                        for loo in loops.iter() {
                            let cur_x = i as i16 + loo.0;
                            let cur_y = j as i16 + loo.1;

                            if cur_x >= 0 && cur_y >= 0 &&
                                cur_x < (x_len as i16) && cur_y < (y_len as i16) &&
                                arr[cur_x as usize][cur_y as usize] == '#' {
                                    sum += 1;
                                }
                        }

                        if sum >= 4 {
                            changes.push((i, j));
                        }
                    },
                    _ => (),
                }
            }
        }

        if changes.is_empty() {
            break;
        }

        for change in changes.iter() {
            match arr[change.0][change.1] {
                'L' => arr[change.0][change.1] = '#',
                '#' => arr[change.0][change.1] = 'L',
                _ => (),
            }
        }
    }

    let mut res = 0;

    for row in arr.iter() {
        for c in row.iter() {
            if *c == '#' {
                res += 1;
            }
        }
    }

    println!("{}", res);
}