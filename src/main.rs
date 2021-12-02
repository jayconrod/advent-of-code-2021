use std::env;
use std::fs;
use std::process;

fn main() {
    if env::args().len() != 2 {
        eprint!("usage: advent-of-code-2021 N\nwhere N is the puzzle to run\n");
        process::exit(1);
    }
    let name = env::args().nth(1).unwrap();
    let func = match name.as_str() {
        "1_1" => puzzle1_1,
        "1_2" => puzzle1_2,
        _ => {
            eprint!("no such puzzle: {}\n", name);
            process::exit(1);
        }
    };
    let path = format!("data/{}.txt", name);
    let data = match fs::read_to_string(path.as_str()) {
        Ok(data) => data,
        Err(err) => {
            eprint!("reading file {}: {}\n", path.as_str(), err);
            process::exit(1);
        }
    };
    func(data.as_str());
}

fn puzzle1_1(input: &str) {
    let depths: Vec<i64> = input
        .split_ascii_whitespace()
        .map(|w| w.parse::<i64>().unwrap())
        .collect();
    let mut i = 1;
    let mut increases = 0;
    while i < depths.len() {
        if depths[i] > depths[i - 1] {
            increases += 1;
        }
        i += 1;
    }
    println!("depth increases: {}", increases);
}

fn puzzle1_2(input: &str) {
    let depths: Vec<i64> = input
        .split_ascii_whitespace()
        .map(|w| w.parse::<i64>().unwrap())
        .collect();
    let mut i = 3;
    let mut increases = 0;
    while i < depths.len() {
        let prev_sum = depths[i - 3] + depths[i - 2] + depths[i - 1];
        let sum = depths[i - 2] + depths[i - 1] + depths[i];
        if sum > prev_sum {
            increases += 1;
        }
        i += 1;
    }
    println!("depth increases: {}", increases);
}
