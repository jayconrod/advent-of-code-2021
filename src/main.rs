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
        "2_1" => puzzle2_1,
        "2_2" => puzzle2_2,
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

fn puzzle2_1(input: &str) {
    let mut hpos = 0;
    let mut depth = 0;
    for line in input.lines() {
        let words: Vec<&str> = line.split_ascii_whitespace().collect();
        if words.len() == 0 {
            continue;
        }
        if words.len() != 2 {
            panic!("expected exactly 2 words in command");
        }
        let dist = words[1].parse::<i64>().expect("distance must be integer");
        match words[0] {
            "forward" => hpos += dist,
            "down" => depth += dist,
            "up" => depth -= dist,
            _ => panic!("unknown command {}", words[0]),
        }
    }
    println!("hpos {}, depth {}, product {}", hpos, depth, hpos * depth);
}

fn puzzle2_2(input: &str) {
    let mut hpos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in input.lines() {
        let words: Vec<&str> = line.split_ascii_whitespace().collect();
        if words.len() == 0 {
            continue;
        }
        if words.len() != 2 {
            panic!("expected exactly 2 words in command");
        }
        let n = words[1].parse::<i64>().expect("argument must be integer");
        match words[0] {
            "down" => aim += n,
            "up" => aim -= n,
            "forward" => {
                hpos += n;
                depth += aim * n;
            }
            _ => panic!("unknown command {}", words[0]),
        }
    }
    println!("hpos {}, depth {}, product {}", hpos, depth, hpos * depth);
}
