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
        "3_1" => puzzle3_1,
        "3_2" => puzzle3_2,
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

fn puzzle3_1(input: &str) {
    let (values, width) = puzzle3_parse_input(input).unwrap();
    let gamma_rate = puzzle3_most_common_bits(&values[..], width);
    let mask = (1 << width) - 1;
    let epsilon_rate = gamma_rate ^ mask;
    let power = gamma_rate * epsilon_rate;
    println!(
        "gamma_rate {}, epsilon_rate {}, power {}",
        gamma_rate, epsilon_rate, power,
    );
}

fn puzzle3_2(input: &str) {
    let (values, width) = puzzle3_parse_input(input).unwrap();
    let mut o2_generator_candidates = values.clone();
    for i in 0..width {
        let most_common = puzzle3_most_common_bits(&o2_generator_candidates[..], width);
        let mask = 1 << (width - i - 1);
        o2_generator_candidates = o2_generator_candidates
            .iter()
            .map(|v| *v)
            .filter(|v| ((*v ^ most_common) & mask) == 0)
            .collect();
        if o2_generator_candidates.len() <= 1 {
            break;
        }
    }
    if o2_generator_candidates.len() != 1 {
        panic!("did not find exactly one O2 generator candidate")
    }
    let o2_generator_rating = o2_generator_candidates[0];

    let mut co2_scrubber_candidates = values;
    for i in 0..width {
        let most_common = puzzle3_most_common_bits(&co2_scrubber_candidates[..], width);
        let least_common = most_common ^ ((1 << width) - 1);
        let mask = 1 << (width - i - 1);
        co2_scrubber_candidates = co2_scrubber_candidates
            .iter()
            .map(|v| *v)
            .filter(|v| ((*v ^ least_common) & mask) == 0)
            .collect();
        if co2_scrubber_candidates.len() <= 1 {
            break;
        }
    }
    if co2_scrubber_candidates.len() != 1 {
        panic!("did not find exactly one CO2 scrubber candidate")
    }
    let co2_scrubber_rating = co2_scrubber_candidates[0];

    let life_support_rating = o2_generator_rating * co2_scrubber_rating;
    println!(
        "o2 generator rating {}, co2 scrubber rating {}, life support rating {}",
        o2_generator_rating, co2_scrubber_rating, life_support_rating
    );
}

fn puzzle3_parse_input(input: &str) -> Result<(Vec<usize>, usize), String> {
    let mut values: Vec<usize> = Vec::new();
    let mut width = 0;
    for (i, line) in input.lines().enumerate() {
        if line.len() == 0 {
            continue;
        }
        if width == 0 {
            width = line.len()
        } else if width != line.len() {
            return Err(format!(
                "line {}: different length {} than earlier lines {}",
                i,
                line.len(),
                width
            ));
        }
        values.push(usize::from_str_radix(line, 2).map_err(|e| e.to_string())?);
    }
    Ok((values, width))
}

fn puzzle3_most_common_bits(values: &[usize], width: usize) -> usize {
    let mut counts = Vec::new();
    counts.resize(width, 0);
    for value in values {
        for i in 0..width {
            if (value & (1 << (width - i - 1))) != 0 {
                counts[i] += 1;
            }
        }
    }

    let mut most_common = 0;
    let half = (values.len() + 1) / 2;
    for (i, count) in counts.iter().enumerate() {
        if *count >= half {
            most_common |= 1 << (width - i - 1);
        }
    }
    most_common
}
