use std::io;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut arr = vec![];

    stdin.read_line(&mut buffer);

    let nums: Vec<&str> = buffer.split(',').collect();

    for num in nums {
        let parsed: i32 = num.parse().unwrap();
        arr.push(parsed);
    }

    for idx in 0..arr.len() - 1 {
        map.insert(arr[idx], idx);
    }

    while arr.len() < 2020 {
        let last = arr[arr.len() - 1];
        
        if !map.contains_key(&last) {
            map.insert(last, arr.len() - 1);
            arr.push(0);
        } else {
            let last_idx: usize = *map.get(&last).unwrap();

            map.insert(last, arr.len() - 1);

            arr.push((arr.len() - 1 - last_idx) as i32);
        }
    }

    println!("{}", arr[arr.len() - 1]);
}