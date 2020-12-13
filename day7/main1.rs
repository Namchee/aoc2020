use std::io;
use std::vec::Vec;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let mut map: HashMap<String, Vec<(String, i32)> > = HashMap::new();

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        if buffer.ends_with('\n') {
            buffer.pop();
        }

        let io: Vec<&str> = buffer.split("contain").collect();
        let input: Vec<&str> = io[0].trim().split_whitespace().collect();
        let outputs: Vec<&str> = io[1].trim().split(',').collect();

        let key = input[0..2].join(" ");
        let mut list = vec![];

        for output in outputs {
            let splitted_outputs: Vec<&str> = output.trim().split_whitespace().collect();

            let value = splitted_outputs[0];

            if value == "no" {
                break;
            }

            let actual_value: i32 = value.parse().unwrap();
            let id = splitted_outputs[1..3].join(" ").to_owned();

            list.push((id, actual_value));
        }

        map.insert(key, list);

        buffer.clear();
    }

    let mut expanded: HashMap<String, bool> = HashMap::new();

    for key in map.keys() {
        expanded.insert(key.to_string(), false);
    }
}

fn expand(
    key: String,
    mut map: HashMap<String, Vec<(String, i32)> >,
    mut expanded: HashMap<String, bool>
) -> Vec<(String, i32)> {
    if !expanded.get(&key).unwrap() {
        let seed = map.get(&key).unwrap().to_vec();
        let mut ans: Vec<(String, i32)> = seed.clone();

        for (id, val) in seed {
            let children = expand(key, map, expanded);
        }

        expanded.insert(key.to_owned(), true);
        map.insert(key.to_owned(), ans.to_owned());
    }
    
    return map.get(&key).unwrap().to_vec();
}