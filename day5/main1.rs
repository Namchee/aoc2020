use std::io;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let mut res: i32 = 0;

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        if buffer.ends_with('\n') {
            buffer.pop();
        }

        let mut min_r = 0;
        let mut max_r = 127;
        let mut min_c = 0;
        let mut max_c = 7;

        let rows = buffer[..buffer.len() - 4].chars();
        let cols = buffer[buffer.len() - 3..buffer.len() - 1].chars();
        
        for row_move in rows {
            match row_move {
                'F' => max_r = floor_div(max_r, min_r),
                'B' => min_r = ceil_div(max_r, min_r),
                _ => (),
            }
        }

        let row = if buffer.chars().nth(6).unwrap() == 'F' { min_r } else { max_r };

        for column_move in cols {
            match column_move {
                'L' => max_c = floor_div(max_c, min_c),
                'R' => min_c = ceil_div(max_c, min_c),
                _ => (),
            }
        }

        let column = if buffer.chars().nth(9).unwrap() == 'L' { min_c } else { max_c };

        let id = row * 8 + column;
        res = if id > res { id } else { res };

        buffer.clear();
    }

    println!("{}", res);
}

fn floor_div(a: i32, b: i32) -> i32 {
    let mut sum = a + b;

    if sum & 1 == 1 {
        sum -= 1;
    }

    return sum / 2;
}

fn ceil_div(a: i32, b: i32) -> i32 {
    let mut sum = a + b;

    if sum & 1 == 1 {
        sum += 1;
    }

    return sum / 2;
}