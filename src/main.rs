use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs;
use std::mem;
use std::process;
use std::str::from_utf8;
use std::str::FromStr;

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
        "4_1" => puzzle4_1,
        "4_2" => puzzle4_2,
        "5_1" => puzzle5_1,
        "5_2" => puzzle5_2,
        "6_1" => puzzle6_1,
        "6_2" => puzzle6_2,
        "7_1" => puzzle7_1,
        "7_2" => puzzle7_2,
        "8_1" => puzzle8_1,
        "8_2" => puzzle8_2,
        "9_1" => puzzle9_1,
        "9_2" => puzzle9_2,
        "10_1" => puzzle10_1,
        "10_2" => puzzle10_2,
        "11_1" => puzzle11_1,
        "11_2" => puzzle11_2,
        "12_1" => puzzle12_1,
        "12_2" => puzzle12_2,
        "13_1" => puzzle13_1,
        "13_2" => puzzle13_2,
        "14_1" => puzzle14_1,
        "14_2" => puzzle14_2,
        "15_1" => puzzle15_1,
        "15_2" => puzzle15_2,
        "16_1" => puzzle16_1,
        "16_2" => puzzle16_2,
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

fn puzzle4_1(input: &str) {
    let mut parsed = puzzle4_parse_input(input).unwrap();
    for n in parsed.numbers.iter() {
        for i in 0..parsed.boards.len() {
            if let Some(score) = parsed.boards[i].mark(*n) {
                println!("winning index {}, number {}, score {}", i, n, score);
                return;
            }
        }
    }

    panic!("all numbers called and nobody won")
}

fn puzzle4_2(input: &str) {
    let mut parsed = puzzle4_parse_input(input).unwrap();
    let play = |b: &mut BingoBoard| {
        for (i, n) in parsed.numbers.iter().enumerate() {
            if let Some(score) = b.mark(*n) {
                return (i, *n, score);
            }
        }
        return (parsed.numbers.len(), -1, -1);
    };
    let (board_index, (move_count, n, score)) = parsed
        .boards
        .iter_mut()
        .map(play)
        .enumerate()
        .max_by(|(_, (lmoves, _, _)), (_, (rmoves, _, _))| lmoves.cmp(rmoves))
        .unwrap();
    println!(
        "board_index {}, move_count {}, n {}, score {}",
        board_index, move_count, n, score
    )
}

fn puzzle4_parse_input(input: &str) -> Result<Puzzle4Input, String> {
    let mut lines = input.lines();
    let numbers = lines
        .next()
        .ok_or(String::from("numbers line not found"))
        .and_then(|s| parse_separated::<i64>(s, ","))?;

    let mut boards = Vec::<BingoBoard>::new();
    'board_loop: loop {
        match lines.next() {
            None => break,
            Some("") => (),
            _ => return Err(String::from("expected blank line")),
        }
        let mut squares: [i64; 25] = [0; 25];
        for row in 0..5 {
            let line = match lines.next() {
                None | Some("") => {
                    if row == 0 {
                        break 'board_loop;
                    } else {
                        return Err(String::from("incomplete board"));
                    }
                }
                Some(line) => line,
            };
            let square_numbers = parse_space_separated::<i64>(line)?;
            if square_numbers.len() != 5 {
                return Err(String::from("bingo row must contain 5 numbers"));
            }
            for col in 0..5 {
                squares[row * 5 + col] = square_numbers[col]
            }
        }
        boards.push(BingoBoard {
            squares: squares,
            marked: [false; 25],
        })
    }

    Ok(Puzzle4Input {
        numbers: numbers,
        boards: boards,
    })
}

struct Puzzle4Input {
    numbers: Vec<i64>,
    boards: Vec<BingoBoard>,
}

struct BingoBoard {
    squares: [i64; 25],
    marked: [bool; 25],
}

impl BingoBoard {
    fn mark(&mut self, n: i64) -> Option<i64> {
        for row in 0..5 {
            for col in 0..5 {
                if self.squares[row * 5 + col] == n {
                    self.marked[row * 5 + col] = true
                }
            }
        }
        if !(0..5).any(|r| (0..5).all(|c| self.marked[r * 5 + c]))
            && !(0..5).any(|c| (0..5).all(|r| self.marked[r * 5 + c]))
        {
            return None;
        }

        let mut unmarked_sum = 0;
        for i in 0..25 {
            if !self.marked[i] {
                unmarked_sum += self.squares[i]
            }
        }
        Some(unmarked_sum * n)
    }
}

fn puzzle5_1(input: &str) {
    let parsed = puzzle5_parse_input(input).unwrap();

    let max_x = parsed
        .iter()
        .fold(0, |x, line| x.max(line.a.x.max(line.b.x)));
    let max_y = parsed
        .iter()
        .fold(0, |y, line| y.max(line.a.y.max(line.b.y)));
    let mut floor_map = FloorMap::new((max_x + 1) as usize, (max_y + 1) as usize);

    for line in parsed {
        if line.a.x == line.b.x || line.a.y == line.b.y {
            floor_map.add_line(line)
        }
    }

    println!("danger_points {}", floor_map.danger_points());
}

fn puzzle5_2(input: &str) {
    let parsed = puzzle5_parse_input(input).unwrap();

    let max_x = parsed
        .iter()
        .fold(0, |x, line| x.max(line.a.x.max(line.b.x)));
    let max_y = parsed
        .iter()
        .fold(0, |y, line| y.max(line.a.y.max(line.b.y)));
    let mut floor_map = FloorMap::new((max_x + 1) as usize, (max_y + 1) as usize);

    for line in parsed {
        floor_map.add_line(line)
    }

    println!("danger_points {}", floor_map.danger_points());
}

fn puzzle5_parse_input<'a>(input: &'a str) -> Result<Vec<LineSegment>, String> {
    let expect = |s: &'a str, want: &str| -> Result<&'a str, String> {
        let s = s.trim_start_matches(' ');
        if !s.starts_with(want) {
            Err(format!("expected {}", want))
        } else {
            Ok(s.get(want.len()..).unwrap())
        }
    };

    let parse_int = |s: &'a str| -> Result<(i64, &'a str), String> {
        let s = s.trim_start_matches(' ');
        let mut end = 0;
        for (i, c) in s.char_indices() {
            end = i;
            if c < '0' || '9' < c {
                break;
            }
        }
        if end == 0 {
            return Err(format!("expected integer"));
        }
        let n = match s.get(..end).unwrap().parse::<i64>() {
            Err(err) => return Err(format!("{}", err)),
            Ok(n) => n,
        };
        Ok((n, s.get(end..).unwrap()))
    };

    let parse_point = |s: &'a str| -> Result<(Point, &'a str), String> {
        let (x, s) = parse_int(s)?;
        let s = expect(s, ",")?;
        let (y, s) = parse_int(s)?;
        Ok((Point { x: x, y: y }, s))
    };

    let parse_line = |s: &'a str| -> Result<(LineSegment, &'a str), String> {
        let (a, s) = parse_point(s)?;
        let s = expect(s, "->")?;
        let (b, s) = parse_point(s)?;
        Ok((LineSegment { a: a, b: b }, s))
    };

    let mut segments = Vec::<LineSegment>::new();
    let mut s = input;
    while s.len() > 0 {
        let line_and_s = parse_line(s)?;
        segments.push(line_and_s.0);
        s = line_and_s.1;
        s = expect(s, "\n")?;
    }
    Ok(segments)
}

struct LineSegment {
    a: Point,
    b: Point,
}

struct Point {
    x: i64,
    y: i64,
}

struct FloorMap {
    width: usize,
    height: usize,
    vents: Vec<i64>,
}

impl FloorMap {
    fn new(width: usize, height: usize) -> FloorMap {
        let mut vents = Vec::<i64>::new();
        vents.resize(width * height, 0);
        FloorMap {
            width: width,
            height: height,
            vents: vents,
        }
    }

    fn add_line(&mut self, line: LineSegment) {
        let len: usize;
        let x_begin = line.a.x as isize;
        let x_inc: isize;
        let y_begin = line.a.y as isize;
        let y_inc: isize;
        if line.a.y == line.b.y {
            // horizontal
            if line.a.x <= line.b.x {
                len = (line.b.x - line.a.x + 1) as usize;
                x_inc = 1;
            } else {
                len = (line.a.x - line.b.x + 1) as usize;
                x_inc = -1;
            }
            y_inc = 0;
        } else if line.a.x == line.b.x {
            // vertical
            if line.a.y <= line.b.y {
                len = (line.b.y - line.a.y + 1) as usize;
                y_inc = 1;
            } else {
                len = (line.a.y - line.b.y + 1) as usize;
                y_inc = -1;
            }
            x_inc = 0;
        } else {
            // diagonal
            if line.a.x <= line.b.x {
                len = (line.b.x - line.a.x + 1) as usize;
                x_inc = 1;
            } else {
                len = (line.a.x - line.b.x + 1) as usize;
                x_inc = -1;
            }
            let ylen: usize;
            if line.a.y <= line.b.y {
                ylen = (line.b.y - line.a.y + 1) as usize;
                y_inc = 1;
            } else {
                ylen = (line.a.y - line.b.y + 1) as usize;
                y_inc = -1;
            }
            assert_eq!(len, ylen);
        }

        let mut x = x_begin;
        let mut y = y_begin;
        for _ in 0..len {
            let pos = y as usize * self.width + x as usize;
            self.vents[pos] += 1;
            x += x_inc;
            y += y_inc;
        }
    }

    fn danger_points(&self) -> usize {
        self.vents.iter().filter(|&n| *n >= 2).count()
    }
}

impl fmt::Display for FloorMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        let mut sep = "";
        for y in 0..self.height {
            s.push_str(sep);
            sep = "\n";
            for x in 0..self.width {
                let i = y * self.width + x;
                if self.vents[i] == 0 {
                    s.push('.')
                } else if self.vents[i] <= 9 {
                    s.push((b'0' + self.vents[i] as u8) as char)
                } else {
                    s.push('!')
                }
            }
        }
        f.write_str(s.as_str())
    }
}

fn puzzle6_1(input: &str) {
    let mut lanternfish = input.trim().parse::<Lanternfish>().unwrap();
    let days = 80;
    for _ in 0..days {
        lanternfish.advance();
    }
    println!(
        "population after {} days: {}",
        days,
        lanternfish.population()
    );
}

fn puzzle6_2(input: &str) {
    let mut lanternfish = input.trim().parse::<Lanternfish>().unwrap();
    let days = 256;
    for _ in 0..days {
        lanternfish.advance();
    }
    println!(
        "population after {} days: {}",
        days,
        lanternfish.population()
    );
}

struct Lanternfish {
    count_days_until_spawn: Vec<usize>,
}

impl Lanternfish {
    fn population(&self) -> usize {
        self.count_days_until_spawn
            .iter()
            .fold(0, |sum, n| sum.checked_add(*n).unwrap())
    }

    fn advance(&mut self) {
        let spawned = self.count_days_until_spawn[0];
        for i in 0..self.count_days_until_spawn.len() - 1 {
            self.count_days_until_spawn[i] = self.count_days_until_spawn[i + 1]
        }
        self.count_days_until_spawn[6] += spawned;
        self.count_days_until_spawn[8] = spawned;
    }
}

impl FromStr for Lanternfish {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut count_days_until_spawn = Vec::<usize>::new();
        count_days_until_spawn.resize(9, 0);
        for days_until_spawn in parse_separated::<usize>(s, ",")? {
            if days_until_spawn >= count_days_until_spawn.len() {
                return Err(format!("invalid days until spawn: {}", days_until_spawn));
            }
            count_days_until_spawn[days_until_spawn] += 1;
        }
        Ok(Lanternfish {
            count_days_until_spawn: count_days_until_spawn,
        })
    }
}

impl Display for Lanternfish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sep = "";
        for n in &self.count_days_until_spawn {
            write!(f, "{}{}", sep, n)?;
            sep = ",";
        }
        Ok(())
    }
}

fn puzzle7_1(input: &str) {
    let positions = parse_separated::<i64>(input.trim(), ",").unwrap();
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    let total_distance = |pos: i64| positions.iter().fold(0, |dist, p| dist + (*p - pos).abs());
    let (min_pos, min_total_distance) = (min..=max)
        .map(|pos| (pos, total_distance(pos)))
        .min_by(|(_, d1), (_, d2)| d1.cmp(d2))
        .unwrap();

    println!(
        "min_pos {}, min_total_distance {}",
        min_pos, min_total_distance
    );
}

fn puzzle7_2(input: &str) {
    let positions = parse_separated::<i64>(input.trim(), ",").unwrap();
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    let distance = |x: i64, y: i64| {
        let d = (x - y).abs();
        d * (d + 1) / 2
    };
    let total_distance = |pos: i64| positions.iter().fold(0, |dist, p| dist + distance(*p, pos));
    let (min_pos, min_total_distance) = (min..=max)
        .map(|pos| (pos, total_distance(pos)))
        .min_by(|(_, d1), (_, d2)| d1.cmp(d2))
        .unwrap();

    println!(
        "min_pos {}, min_total_distance {}",
        min_pos, min_total_distance
    );
}

fn puzzle8_1(input: &str) {
    let entries = parse_puzzle8(input).unwrap();
    let easy_count = entries
        .iter()
        .flat_map(|e| e.output.iter())
        .filter(|d| d.is_easy())
        .count();
    println!("easy_count {}", easy_count);
}

fn puzzle8_2(input: &str) {
    let entries = parse_puzzle8(input).unwrap();
    let output_sum: u64 = entries.iter().map(|e| e.decode()).sum();
    println!("output_sum {}", output_sum);
}

fn parse_puzzle8(input: &str) -> Result<Vec<Puzzle8Entry>, String> {
    input
        .trim()
        .lines()
        .map(|l| l.parse::<Puzzle8Entry>())
        .collect()
}

struct Puzzle8Entry {
    unique: Vec<Digit>,
    output: Vec<Digit>,
}

impl Puzzle8Entry {
    fn decode(&self) -> u64 {
        let mut pattern_to_digit = [0u8; 128];
        let mut digit_to_pattern = [0u8; 10];

        // First pass: "1", "4", "7", "8".
        let mut found = 0;
        for pattern in self.unique.iter() {
            let digit = match pattern.bits.count_ones() {
                2 => 1u8,
                4 => 4u8,
                3 => 7u8,
                7 => 8u8,
                _ => continue,
            };
            pattern_to_digit[pattern.bits as usize] = digit;
            digit_to_pattern[digit as usize] = pattern.bits;
            found += 1;
        }
        assert_eq!(found, 4);

        // Second pass: "2", "3", "5".
        for pattern in self.unique.iter() {
            if pattern.bits.count_ones() != 5 {
                continue;
            }
            let common_with_one = (pattern.bits & digit_to_pattern[1]).count_ones();
            let common_with_four_not_one =
                (pattern.bits & digit_to_pattern[4] & !digit_to_pattern[1]).count_ones();
            let digit = if common_with_one == 2 {
                3u8
            } else if common_with_four_not_one == 2 {
                5u8
            } else {
                2u8
            };
            pattern_to_digit[pattern.bits as usize] = digit;
            digit_to_pattern[digit as usize] = pattern.bits;
            found += 1;
        }
        assert_eq!(found, 7);

        // Third pass: "6", "9", "0".
        for pattern in self.unique.iter() {
            if pattern.bits.count_ones() != 6 {
                continue;
            }
            let digit = if (pattern.bits & digit_to_pattern[4]).count_ones() == 4 {
                9u8
            } else if (pattern.bits & digit_to_pattern[1]).count_ones() == 2 {
                0u8
            } else {
                6u8
            };
            pattern_to_digit[pattern.bits as usize] = digit;
            digit_to_pattern[digit as usize] = pattern.bits;
            found += 1;
        }
        assert_eq!(found, 10);

        let mut output = 0;
        for pattern in &self.output {
            output = output * 10 + pattern_to_digit[pattern.bits as usize] as u64;
        }
        output
    }
}

impl FromStr for Puzzle8Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_ascii_whitespace().collect();
        if words.len() != 15 {
            return Err(String::from("line must have 15 words"));
        }
        if words[10] != "|" {
            return Err(String::from("word 10 must be '|' delimiter"));
        }
        let unique: Result<Vec<Digit>, Self::Err> =
            words[0..10].iter().map(|w| w.parse::<Digit>()).collect();
        let output: Result<Vec<Digit>, Self::Err> =
            words[11..15].iter().map(|w| w.parse::<Digit>()).collect();
        Ok(Puzzle8Entry {
            unique: unique?,
            output: output?,
        })
    }
}

impl Display for Puzzle8Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sep = "";
        for d in &self.unique {
            write!(f, "{}{}", sep, d)?;
            sep = " ";
        }
        f.write_str(" |")?;
        for d in &self.output {
            write!(f, "{}{}", sep, d)?;
        }
        Ok(())
    }
}

struct Digit {
    bits: u8,
}

impl Digit {
    fn is_easy(&self) -> bool {
        match self.bits.count_ones() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        }
    }
}

impl FromStr for Digit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut n = 0;
        for c in s.chars() {
            if c < 'a' || 'g' < c {
                return Err(String::from("pattern may only contain letters a-g"));
            }
            n |= 1 << (c as u8 - 'a' as u8);
        }
        Ok(Digit { bits: n })
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for i in 0..8 {
            if self.bits & (1 << i) != 0 {
                s.push((b'a' + i) as char);
            }
        }
        f.write_str(&s[..])
    }
}

fn puzzle9_1(input: &str) {
    let hm = input.parse::<HeightMap>().unwrap();
    let total_risk: u64 = hm.iter_low_points().map(|(x, y)| 1 + hm.at(x, y)).sum();
    println!("total_risk {}", total_risk);
}

fn puzzle9_2(input: &str) {
    let hm = input.parse::<HeightMap>().unwrap();
    let basins = hm.basins();
    let mut basin_sizes = Vec::<u64>::new();
    for bid in basins {
        if bid == 0 {
            continue;
        }
        let i = bid as usize;
        if basin_sizes.len() <= i {
            basin_sizes.resize(i + 1, 0);
        }
        basin_sizes[i] += 1;
    }
    basin_sizes.sort();
    let product_of_largest: u64 = basin_sizes[basin_sizes.len() - 3..].iter().product();
    println!("product_of_largest {}", product_of_largest);
}

struct HeightMap {
    size_x: usize,
    size_y: usize,
    heights: Vec<u64>,
}

impl HeightMap {
    fn at(&self, x: usize, y: usize) -> u64 {
        assert!(x < self.size_x);
        assert!(y < self.size_y);
        self.heights[y * self.size_x + x]
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        let h = self.at(x, y);
        let top = y == 0;
        let left = x == 0;
        let bottom = y == self.size_y - 1;
        let right = x == self.size_x - 1;
        if top && left {
            h < self.at(x, y + 1) && h < self.at(x + 1, y)
        } else if top && right {
            h < self.at(x, y + 1) && h < self.at(x - 1, y)
        } else if bottom && right {
            h < self.at(x, y - 1) && h < self.at(x - 1, y)
        } else if bottom && left {
            h < self.at(x, y - 1) && h < self.at(x + 1, y)
        } else if top {
            h < self.at(x - 1, y) && h < self.at(x, y + 1) && h < self.at(x + 1, y)
        } else if left {
            h < self.at(x, y - 1) && h < self.at(x, y + 1) && h < self.at(x + 1, y)
        } else if bottom {
            h < self.at(x, y - 1) && h < self.at(x - 1, y) && h < self.at(x + 1, y)
        } else if right {
            h < self.at(x, y - 1) && h < self.at(x - 1, y) && h < self.at(x, y + 1)
        } else {
            h < self.at(x, y - 1)
                && h < self.at(x - 1, y)
                && h < self.at(x, y + 1)
                && h < self.at(x + 1, y)
        }
    }

    fn iter_low_points(&self) -> HeightMapLowPointIterator {
        HeightMapLowPointIterator { hm: self, index: 0 }
    }

    fn basins(&self) -> Vec<u64> {
        let mut bs = Vec::<u64>::new();
        bs.resize(self.heights.len(), 0);
        let mut next_bid: u64 = 1;
        fn find_basin(
            hm: &HeightMap,
            bs: &mut Vec<u64>,
            next_bid: &mut u64,
            x: usize,
            y: usize,
        ) -> u64 {
            let i = y * hm.size_x + x;
            if bs[i] != 0 {
                // Basin already found for this point.
                return bs[i];
            }
            if hm.at(x, y) == 9 {
                // High point, not in a basin.
                return 0;
            }
            if hm.is_low_point(x, y) {
                // Bottom of new basin.
                bs[i] = *next_bid;
                *next_bid += 1;
                return bs[i];
            }

            // Recurse in lowest direction.
            let h = hm.at(x, y);
            let th = if y > 0 { hm.at(x, y - 1) } else { 9 };
            let lh = if x > 0 { hm.at(x - 1, y) } else { 9 };
            let bh = if y < hm.size_y - 1 {
                hm.at(x, y + 1)
            } else {
                9
            };
            let rh = if x < hm.size_x - 1 {
                hm.at(x + 1, y)
            } else {
                9
            };
            let (flow_x, flow_y) = if th < h && th <= lh && th <= bh && th <= rh {
                (x, y - 1)
            } else if lh < h && lh <= th && lh <= bh && lh <= rh {
                (x - 1, y)
            } else if bh < h && bh <= th && bh <= lh && bh <= rh {
                (x, y + 1)
            } else if rh < h && rh <= th && rh <= lh && rh <= bh {
                (x + 1, y)
            } else {
                panic!("flat point");
            };
            bs[i] = find_basin(hm, bs, next_bid, flow_x, flow_y);
            bs[i]
        }

        for y in 0..self.size_y {
            for x in 0..self.size_x {
                find_basin(self, &mut bs, &mut next_bid, x, y);
            }
        }
        bs
    }
}

struct HeightMapLowPointIterator<'a> {
    hm: &'a HeightMap,
    index: usize,
}

impl<'a> Iterator for HeightMapLowPointIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.hm.size_x * self.hm.size_y {
            let x = self.index % self.hm.size_x;
            let y = self.index / self.hm.size_x;
            self.index += 1;
            if self.hm.is_low_point(x, y) {
                return Some((x, y));
            }
        }
        None
    }
}

impl FromStr for HeightMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hm = HeightMap {
            size_x: 0,
            size_y: 0,
            heights: Vec::<u64>::new(),
        };
        for line in s.trim().lines() {
            if hm.size_x == 0 {
                hm.size_x = line.len();
            }
            hm.size_y += 1;
            for c in line.chars() {
                if c < '0' || '9' < c {
                    return Err(format!("invalid heighmap height: {}", c));
                }
                hm.heights.push(c as u64 - '0' as u64)
            }
        }
        Ok(hm)
    }
}

fn puzzle10_1(input: &str) {
    let err_score = |chunks: Result<Vec<char>, ChunkParseError>| {
        if let Err(err) = chunks {
            match err.got {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            }
        } else {
            0
        }
    };
    let total_score: usize = input
        .trim()
        .lines()
        .enumerate()
        .map(|(n, line)| parse_chunks(n, line))
        .map(err_score)
        .sum();
    println!("total_score {}", total_score);
}

fn puzzle10_2(input: &str) {
    let complete_score = |chunks: Result<Vec<char>, ChunkParseError>| {
        let char_score = |c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        };
        match chunks {
            Ok(cs) => Some(cs.iter().fold(0, |total, &c| 5 * total + char_score(c))),
            _ => None,
        }
    };
    let mut scores: Vec<usize> = input
        .trim()
        .lines()
        .enumerate()
        .map(|(n, line)| parse_chunks(n, line))
        .filter_map(complete_score)
        .collect();
    scores.sort();
    let median_score = scores[scores.len() / 2];
    println!("median_score {}", median_score);
}

fn parse_chunks(lineno: usize, s: &str) -> Result<Vec<char>, ChunkParseError> {
    let mut stack = Vec::<char>::new();
    for (col, c) in s.chars().enumerate() {
        let make_err = |want: &'static str| -> Result<Vec<char>, ChunkParseError> {
            Err(ChunkParseError {
                line: lineno,
                col: col,
                got: c,
                want: want,
            })
        };
        let open = match c {
            '<' => Some('>'),
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),
            _ => None,
        };
        if let Some(want) = open {
            stack.push(want);
            continue;
        }
        match stack.pop() {
            Some(want) if c == want => (),
            Some(_) => return make_err("matching closing character or open character"),
            None => return make_err("open character"),
        }
    }
    stack.reverse();
    Ok(stack)
}

#[derive(Debug)]
struct ChunkParseError {
    line: usize,
    col: usize,
    got: char,
    want: &'static str,
}

impl Display for ChunkParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}: expected {} but found '{}'",
            self.line, self.col, self.want, self.got
        )
    }
}

impl Error for ChunkParseError {
    fn description(&self) -> &str {
        "unexpected character"
    }
}

fn puzzle11_1(input: &str) {
    let mut om = input.trim().parse::<OctopusMap>().unwrap();
    let n = 100;
    for _ in 0..n {
        om.step();
    }
    println!("om.flash_count {}", om.flash_count)
}

fn puzzle11_2(input: &str) {
    let mut om = input.trim().parse::<OctopusMap>().unwrap();
    let mut n = 0;
    while !om.all_flashed() {
        om.step();
        n += 1;
    }
    println!("n {}", n);
}

struct OctopusMap {
    energies: Vec<u64>,
    size_x: usize,
    size_y: usize,
    flash_count: u64,
}

impl OctopusMap {
    fn step(&mut self) {
        // Increase energy.
        let mut flashers = Vec::<(usize, usize)>::new();
        for y in 0..self.size_y {
            for x in 0..self.size_y {
                let i = y * self.size_x + x;
                self.energies[i] += 1;
                if self.energies[i] == 10 {
                    flashers.push((x, y));
                }
            }
        }

        // Flashers propagate energy to neighbors, maybe making them flash.
        // Stop when there are no new flashers.
        let mut prev_flashers = Vec::<(usize, usize)>::new();
        while flashers.len() > 0 {
            self.flash_count += flashers.len() as u64;
            mem::swap(&mut flashers, &mut prev_flashers);
            let mut neighbor = |x, y| {
                let i = y * self.size_x + x;
                if self.energies[i] == 10 {
                    return;
                }
                self.energies[i] += 1;
                if self.energies[i] == 10 {
                    flashers.push((x, y))
                }
            };
            for (x, y) in &prev_flashers {
                let (x, y) = (*x, *y);
                let top = y == 0;
                let left = x == 0;
                let bottom = y == self.size_y - 1;
                let right = x == self.size_x - 1;
                if top && left {
                    neighbor(x, y + 1);
                    neighbor(x + 1, y + 1);
                    neighbor(x + 1, y);
                } else if bottom && left {
                    neighbor(x + 1, y);
                    neighbor(x + 1, y - 1);
                    neighbor(x, y - 1);
                } else if bottom && right {
                    neighbor(x, y - 1);
                    neighbor(x - 1, y - 1);
                    neighbor(x - 1, y);
                } else if top && right {
                    neighbor(x - 1, y);
                    neighbor(x - 1, y + 1);
                    neighbor(x, y + 1);
                } else if top {
                    neighbor(x - 1, y);
                    neighbor(x - 1, y + 1);
                    neighbor(x, y + 1);
                    neighbor(x + 1, y + 1);
                    neighbor(x + 1, y);
                } else if left {
                    neighbor(x, y - 1);
                    neighbor(x, y + 1);
                    neighbor(x + 1, y + 1);
                    neighbor(x + 1, y);
                    neighbor(x + 1, y - 1);
                } else if bottom {
                    neighbor(x, y - 1);
                    neighbor(x - 1, y - 1);
                    neighbor(x - 1, y);
                    neighbor(x + 1, y);
                    neighbor(x + 1, y - 1);
                } else if right {
                    neighbor(x, y - 1);
                    neighbor(x - 1, y - 1);
                    neighbor(x - 1, y);
                    neighbor(x - 1, y + 1);
                    neighbor(x, y + 1);
                } else {
                    neighbor(x, y - 1);
                    neighbor(x - 1, y - 1);
                    neighbor(x - 1, y);
                    neighbor(x - 1, y + 1);
                    neighbor(x, y + 1);
                    neighbor(x + 1, y + 1);
                    neighbor(x + 1, y);
                    neighbor(x + 1, y - 1);
                }
            }
            prev_flashers.resize(0, (0, 0));
        }

        // Reset flashers' energy to zero.
        for e in self.energies.iter_mut() {
            if *e == 10 {
                *e = 0;
            }
        }
    }

    fn all_flashed(&self) -> bool {
        self.energies.iter().all(|&e| e == 0)
    }
}

impl FromStr for OctopusMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut om = OctopusMap {
            size_x: 0,
            size_y: 0,
            energies: Vec::<u64>::new(),
            flash_count: 0,
        };
        for line in s.trim().lines() {
            if om.size_x == 0 {
                om.size_x = line.len();
            }
            om.size_y += 1;
            for c in line.chars() {
                if c < '0' || '9' < c {
                    return Err(format!("invalid octopus map height: {}", c));
                }
                om.energies.push(c as u64 - '0' as u64)
            }
        }
        Ok(om)
    }
}

impl Display for OctopusMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        s.reserve(self.size_x * self.size_y + self.size_y - 1);
        let mut sep = "";
        for y in 0..self.size_y {
            s.push_str(sep);
            sep = "\n";
            for x in 0..self.size_x {
                let i = y * self.size_y + x;
                let c = (self.energies[i] as u8 + b'0') as char;
                s.push(c);
            }
        }
        f.write_str(&s[..])
    }
}

fn puzzle12_1(input: &str) {
    let cm = input.parse::<CaveMap>().unwrap();
    let path_count = cm.count_paths("start", "end", false);
    println!("path_count {}", path_count);
}

fn puzzle12_2(input: &str) {
    let cm = input.parse::<CaveMap>().unwrap();
    let path_count = cm.count_paths("start", "end", true);
    println!("path_count {}", path_count);
}

struct CaveMap {
    name_to_index: HashMap<String, usize>,
    caves: Vec<Cave>,
}

impl CaveMap {
    fn new() -> CaveMap {
        CaveMap {
            name_to_index: HashMap::<String, usize>::new(),
            caves: Vec::<Cave>::new(),
        }
    }

    fn ensure_cave(&mut self, name: &str) -> usize {
        match self.name_to_index.get(name) {
            Some(ix) => *ix,
            None => {
                let ix = self.caves.len();
                let is_small = name.starts_with(|c: char| c.is_lowercase());
                self.name_to_index.insert(String::from(name), ix);
                self.caves.push(Cave {
                    is_small: is_small,
                    neighbors: Vec::<usize>::new(),
                });
                ix
            }
        }
    }

    fn ensure_passage(&mut self, from: &str, to: &str) {
        let from_ix = self.ensure_cave(from);
        let to_ix = self.ensure_cave(to);
        if self.caves[from_ix].neighbors.contains(&to_ix) {
            return;
        }
        self.caves[from_ix].neighbors.push(to_ix);
        self.caves[to_ix].neighbors.push(from_ix);
    }

    fn count_paths(&self, from: &str, to: &str, can_visit_twice: bool) -> usize {
        let from_index = *self
            .name_to_index
            .get(from)
            .expect("'from' vertex not found");
        let to_index = *self.name_to_index.get(to).expect("'to' index not found");
        let mut path = CavePath { caves: vec![] };
        let mut paths = HashSet::<CavePath>::new();

        fn visit(
            cm: &CaveMap,
            i: usize,
            start: usize,
            end: usize,
            can_visit_twice: bool,
            path: &mut CavePath,
            paths: &mut HashSet<CavePath>,
        ) {
            path.caves.push(i);
            if i == end {
                paths.insert(path.clone());
            } else {
                for &next in &cm.caves[i].neighbors {
                    if !cm.caves[next].is_small || !path.caves.contains(&next) {
                        visit(cm, next, start, end, can_visit_twice, path, paths);
                    } else if can_visit_twice && next != start && next != end {
                        visit(cm, next, start, end, false, path, paths);
                    }
                }
            }
            path.caves.pop();
        }

        visit(
            self,
            from_index,
            from_index,
            to_index,
            can_visit_twice,
            &mut path,
            &mut paths,
        );
        paths.len()
    }
}

impl FromStr for CaveMap {
    type Err = String;
    fn from_str<'b>(s: &'b str) -> Result<Self, Self::Err> {
        let mut cm = CaveMap::new();
        for (i, line) in s.lines().enumerate() {
            let (from, to) = match line.find("-") {
                Some(i) => (&line[..i], &line[i + 1..]),
                None => return Err(format!("line {}: '-' not found", i)),
            };
            cm.ensure_passage(from, to);
        }
        Ok(cm)
    }
}

struct Cave {
    is_small: bool,
    neighbors: Vec<usize>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct CavePath {
    caves: Vec<usize>,
}

fn puzzle13_1(input: &str) {
    let mut paper = input.trim().parse::<TransparentPaper>().unwrap();
    paper = paper.fold_n(1);
    println!("paper.dots.len() {}", paper.dots.len());
}

fn puzzle13_2(input: &str) {
    let mut paper = input.trim().parse::<TransparentPaper>().unwrap();
    paper = paper.fold_n(paper.folds.len());
    println!("{}", paper);
}

struct TransparentPaper {
    dots: Vec<(usize, usize)>,
    folds: Vec<TransparentPaperFold>,
}

impl TransparentPaper {
    fn fold_n(&self, n: usize) -> TransparentPaper {
        let mut transformed_dots = Vec::<(usize, usize)>::new();
        for dot in &self.dots {
            let mut transformed_dot = *dot;
            for i in 0..n {
                let fold = self.folds[i];
                transformed_dot = fold.transform(transformed_dot);
            }
            transformed_dots.push(transformed_dot);
        }

        transformed_dots.sort_by(|(x1, y1), (x2, y2)| match y1.cmp(y2) {
            Ordering::Equal => x1.cmp(x2),
            c => c,
        });
        transformed_dots.dedup();

        let mut remaining_folds = Vec::<TransparentPaperFold>::new();
        remaining_folds.extend_from_slice(&self.folds[n..]);

        TransparentPaper {
            dots: transformed_dots,
            folds: remaining_folds,
        }
    }
}

impl fmt::Display for TransparentPaper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size_x = self.dots.iter().fold(0, |s, (x, _)| s.max(*x)) + 1;
        let size_y = self.dots.iter().fold(0, |s, (_, y)| s.max(*y)) + 1;
        let mut buf = Vec::<u8>::new();
        buf.reserve((size_x + 1) * size_y);
        for _ in 0..size_y {
            for _ in 0..size_x {
                buf.push(b'.');
            }
            buf.push(b'\n');
        }
        for (x, y) in &self.dots {
            buf[y * (size_x + 1) + x] = b'#';
        }
        f.write_str(from_utf8(&buf[..]).unwrap())
    }
}

#[derive(Clone, Copy)]
enum TransparentPaperFold {
    X(usize),
    Y(usize),
}

impl TransparentPaperFold {
    fn transform(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            TransparentPaperFold::X(axis) if x > *axis => (axis - (x - axis), y),
            TransparentPaperFold::Y(axis) if y > *axis => (x, (axis - (y - axis))),
            _ => (x, y),
        }
    }
}

impl FromStr for TransparentPaper {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_dot = |s: &str| -> Result<(usize, usize), Self::Err> {
            let i = s.find(",").ok_or("expected ','")?;
            let x = s[..i].parse::<usize>().map_err(|_| "expected integer")?;
            let y = s[i + 1..]
                .parse::<usize>()
                .map_err(|_| "expected integer")?;
            Ok((x, y))
        };

        let parse_fold = |s: &str| -> Result<TransparentPaperFold, Self::Err> {
            if s.starts_with("fold along x=") {
                let axis = s["fold along x=".len()..]
                    .parse::<usize>()
                    .map_err(|_| "expected integer")?;
                Ok(TransparentPaperFold::X(axis))
            } else if s.starts_with("fold along y=") {
                let axis = s["fold along y=".len()..]
                    .parse::<usize>()
                    .map_err(|_| "expected integer")?;
                Ok(TransparentPaperFold::Y(axis))
            } else {
                Err(String::from("expected fold"))
            }
        };

        let mut dots = Vec::<(usize, usize)>::new();
        let mut folds = Vec::<TransparentPaperFold>::new();
        let mut line_iter = s.trim().lines();
        loop {
            match line_iter.next() {
                Some(line) if line != "" => {
                    dots.push(parse_dot(line)?);
                }
                _ => break,
            }
        }

        loop {
            match line_iter.next() {
                Some(line) if line != "" => {
                    folds.push(parse_fold(line)?);
                }
                _ => break,
            }
        }
        Ok(TransparentPaper {
            dots: dots,
            folds: folds,
        })
    }
}

fn puzzle14_1(input: &str) {
    let mut p = input.trim().parse::<Polymer>().unwrap();
    let n = 10;
    p.step(n);
    let mut hist = HashMap::<u8, usize>::new();
    p.poly.iter().for_each(|e| {
        match hist.get_mut(e) {
            Some(n) => *n += 1,
            None => {
                hist.insert(*e, 1);
            }
        };
    });
    let most_common = hist.values().max().unwrap();
    let least_common = hist.values().min().unwrap();
    let diff = most_common - least_common;

    println!("n {}, p.poly.len() {}, diff {}", n, p.poly.len(), diff);
}

fn puzzle14_2(input: &str) {
    let p = input.trim().parse::<Polymer>().unwrap();
    let n = 40;
    let hist = p.hist(n);
    let most_common = hist.iter().max().unwrap();
    let least_common = hist.iter().filter(|n| *n > &0).min().unwrap();
    let diff = most_common - least_common;

    println!("n {}, diff {}", n, diff);
}

struct Polymer {
    poly: Vec<u8>,
    rules: Vec<u8>,
}

impl Polymer {
    fn step(&mut self, n: usize) {
        let mut next = Vec::<u8>::new();
        for _ in 0..n {
            next.resize(0, 0);
            next.push(self.poly[0]);
            for i in 0..self.poly.len() - 1 {
                let (l, r) = (self.poly[i], self.poly[i + 1]);
                let ix = Self::rule_index(l, r);
                let b = self.rules[ix];
                if b != 0 {
                    next.push(b);
                }
                next.push(r);
            }
            mem::swap(&mut self.poly, &mut next);
        }
    }

    fn hist(&self, n: usize) -> [usize; 26] {
        fn pair_hist(
            l: u8,
            r: u8,
            n: usize,
            rules: &[u8],
            mem: &mut HashMap<(u8, u8, usize), [usize; 26]>,
        ) -> [usize; 26] {
            let key = (l, r, n);
            if let Some(&hist) = mem.get(&key) {
                return hist;
            }
            let b = rules[Polymer::rule_index(l, r)];
            if n == 0 || b == 0 {
                let hist = [0; 26];
                mem.insert(key, hist);
                return hist;
            }
            let lhist = pair_hist(l, b, n - 1, rules, mem);
            let rhist = pair_hist(b, r, n - 1, rules, mem);
            let mut hist = [0; 26];
            for i in 0..26 {
                hist[i] = lhist[i] + rhist[i];
            }
            hist[(b - b'A') as usize] += 1;
            mem.insert(key, hist);
            hist
        }

        let mut mem = HashMap::<(u8, u8, usize), [usize; 26]>::new();
        let mut hist = [0usize; 26];
        for i in 0..self.poly.len() - 1 {
            let (l, r) = (self.poly[i], self.poly[i + 1]);
            hist[(l - b'A') as usize] += 1;
            let phist = pair_hist(l, r, n, &self.rules[..], &mut mem);
            for j in 0..26 {
                hist[j] += phist[j];
            }
        }
        hist[(self.poly[self.poly.len() - 1] - b'A') as usize] += 1;
        hist
    }

    fn rule_index(l: u8, r: u8) -> usize {
        ((l - b'A') as usize) * 26 + (r - b'A') as usize
    }
}

impl FromStr for Polymer {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_rule = |line: &str| {
            match line.find(" -> ") {
                Some(2) if line.len() == 7 => (),
                _ => return Err(String::from("expected rule of the form AB -> C")),
            };
            let bytes = line.as_bytes();
            Ok((bytes[0], bytes[1], bytes[6]))
        };

        let mut line_iter = s.lines();
        let tpl = match line_iter.next() {
            Some(tpl_str) => {
                let mut tpl = Vec::<u8>::new();
                tpl.extend_from_slice(tpl_str.as_bytes());
                tpl
            }
            None => return Err(String::from("expected template on first line")),
        };
        match line_iter.next() {
            Some("") => (),
            _ => return Err(String::from("expected blank line after template")),
        };
        let mut rules = Vec::<u8>::new();
        rules.resize(26 * 26, 0);
        while let Some(line) = line_iter.next() {
            let (l, r, b) = parse_rule(line)?;
            rules[Self::rule_index(l, r)] = b;
        }
        Ok(Polymer {
            poly: tpl,
            rules: rules,
        })
    }
}

fn puzzle15_1(input: &str) {
    let cm = input.trim().parse::<ChitonMap>().unwrap();
    let lowest_risk = cm.lowest_risk();
    println!("lowest_risk {}", lowest_risk);
}

fn puzzle15_2(input: &str) {
    let cm = input.trim().parse::<ChitonMap>().unwrap().expand();
    let lowest_risk = cm.lowest_risk();
    println!("lowest_risk {}", lowest_risk);
}

struct ChitonMap {
    size_x: usize,
    size_y: usize,
    risks: Vec<usize>,
}

impl ChitonMap {
    fn expand(&self) -> ChitonMap {
        let exp_size_x = self.size_x * 5;
        let exp_size_y = self.size_y * 5;
        let mut exp_risks = Vec::<usize>::new();
        exp_risks.resize(exp_size_x * exp_size_y, 0);
        let ix = |x, y| y * self.size_x + x;
        let eix = |tx, ty, x, y| (ty * self.size_y + y) * exp_size_x + tx * self.size_x + x;

        for ty in 0..5 {
            for tx in 0..5 {
                for y in 0..self.size_x {
                    for x in 0..self.size_y {
                        let d = tx + ty;
                        let mut r = self.risks[ix(x, y)] + d;
                        if r > 9 {
                            r -= 9;
                        }
                        exp_risks[eix(tx, ty, x, y)] = r;
                    }
                }
            }
        }
        ChitonMap {
            size_x: exp_size_x,
            size_y: exp_size_y,
            risks: exp_risks,
        }
    }

    fn lowest_risk(&self) -> usize {
        // Dijkstra's algorithm, heavily borrowed from
        // https://doc.rust-lang.org/std/collections/binary_heap/index.html
        let ix = |x, y| self.size_x * y + x;
        let mut prev = Vec::<(usize, usize)>::new();
        prev.resize(self.size_x * self.size_y, (!0, !0));
        let mut total_risk = Vec::<usize>::new();
        total_risk.resize(self.size_x * self.size_y, !0);

        #[derive(Eq, PartialEq)]
        struct State {
            x: usize,
            y: usize,
            total_risk: usize,
        }
        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                other
                    .total_risk
                    .cmp(&self.total_risk)
                    .then_with(|| self.y.cmp(&other.y))
                    .then_with(|| self.x.cmp(&other.x))
            }
        }
        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut dist = Vec::<usize>::new();
        dist.resize(self.size_x * self.size_y, usize::MAX);
        dist[ix(0, 0)] = 0;
        let mut heap = BinaryHeap::<State>::new();
        heap.push(State {
            x: 0,
            y: 0,
            total_risk: 0,
        });

        while let Some(State { x, y, total_risk }) = heap.pop() {
            let end_x = x == self.size_x - 1;
            let end_y = y == self.size_y - 1;
            if end_x && end_y {
                return total_risk;
            }
            if total_risk > dist[ix(x, y)] {
                continue;
            }
            let mut visit_edge = |ex, ey| {
                let next = State {
                    x: ex,
                    y: ey,
                    total_risk: total_risk + self.risks[ix(ex, ey)],
                };
                if next.total_risk < dist[ix(ex, ey)] {
                    dist[ix(ex, ey)] = next.total_risk;
                    heap.push(next);
                }
            };
            if y > 0 {
                visit_edge(x, y - 1);
            }
            if x > 0 {
                visit_edge(x - 1, y);
            }
            if !end_y {
                visit_edge(x, y + 1);
            }
            if !end_x {
                visit_edge(x + 1, y);
            }
        }

        unreachable!();
    }
}

impl FromStr for ChitonMap {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cm = ChitonMap {
            size_x: 0,
            size_y: 0,
            risks: Vec::<usize>::new(),
        };
        for line in s.trim().lines() {
            if cm.size_x == 0 {
                cm.size_x = line.len();
            }
            cm.size_y += 1;
            for c in line.chars() {
                if c < '0' || '9' < c {
                    return Err(format!("invalid heighmap height: {}", c));
                }
                cm.risks.push(c as usize - '0' as usize)
            }
        }
        Ok(cm)
    }
}

fn puzzle16_1(input: &str) {
    let bytes = decode_hex(input.trim()).unwrap();
    let br = BitReader::new(&bytes[..]);
    let mut pr = PacketReader::new(br);

    let mut packets = Vec::<Packet>::new();
    while let Some(p) = pr.next() {
        packets.push(p);
    }
    fn visit(p: &Packet) -> u64 {
        let mut version_sum = p.version as u64;
        if let PacketBody::Operator { op: _, subpackets } = &p.body {
            for sp in subpackets {
                version_sum += visit(&sp);
            }
        }
        version_sum
    }
    let version_sum: u64 = packets.iter().map(visit).sum();
    println!("version_sum {}", version_sum);
}

fn puzzle16_2(input: &str) {
    let bytes = decode_hex(input.trim()).unwrap();
    let br = BitReader::new(&bytes[..]);
    let mut pr = PacketReader::new(br);
    let p = pr.next().unwrap();
    assert!(pr.next().is_none());
    let n = p.eval();
    println!("n {}", n);
}

fn decode_hex(s: &str) -> Result<Vec<u8>, String> {
    fn decode_digit(c: char) -> Result<u8, String> {
        if '0' <= c && c <= '9' {
            Ok(c as u8 - b'0')
        } else if 'A' <= c && c <= 'F' {
            Ok(c as u8 - b'A' + 10)
        } else if 'a' <= c && c <= 'f' {
            Ok(c as u8 - b'a' + 10)
        } else {
            Err(format!("not a hex digit: '{}'", c))
        }
    }
    let mut bytes = Vec::<u8>::with_capacity(s.len() / 2 + s.len() % 2);
    let mut hi: Option<u8> = None;
    for c in s.chars() {
        match hi {
            None => {
                hi = Some(decode_digit(c)?);
            }
            Some(hi_bits) => {
                let lo_bits = decode_digit(c)?;
                let b = hi_bits << 4 | lo_bits;
                bytes.push(b);
                hi = None;
            }
        }
    }
    if let Some(bits) = hi {
        bytes.push(bits << 4);
    }
    Ok(bytes)
}

struct BitReader<'a> {
    bytes: &'a [u8],
    bits_consumed: usize,
}

impl<'a> BitReader<'a> {
    fn new(bytes: &'a [u8]) -> BitReader<'a> {
        BitReader {
            bytes: bytes,
            bits_consumed: 0,
        }
    }

    fn read(&mut self, mut nbits: usize) -> usize {
        assert!(self.bits_consumed + nbits <= self.bytes.len() * 8);
        assert!(nbits <= mem::size_of::<usize>() * 8);
        let mut bits: usize = 0;
        while nbits > 0 {
            let nbits_from_this_byte = nbits.min(8 - self.bits_consumed % 8);
            let shift = 8 - nbits_from_this_byte - self.bits_consumed % 8;
            let mask = (1 << nbits_from_this_byte) - 1;
            let bits_from_this_byte = (self.bytes[self.bits_consumed / 8] as usize >> shift) & mask;
            bits = (bits << nbits_from_this_byte) | bits_from_this_byte as usize;
            nbits -= nbits_from_this_byte;
            self.bits_consumed += nbits_from_this_byte;
        }
        bits
    }

    fn remaining(&self) -> usize {
        self.bytes.len() * 8 - self.bits_consumed
    }
}

struct Packet {
    version: u8,
    body: PacketBody,
}

enum PacketBody {
    Literal(usize),
    Operator {
        op: PacketOp,
        subpackets: Vec<Packet>,
    },
}

enum PacketOp {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Packet {
    fn eval(&self) -> usize {
        match &self.body {
            PacketBody::Literal(n) => *n,
            PacketBody::Operator { op, subpackets } => {
                let mut values = subpackets.iter().map(|p| p.eval());
                match op {
                    PacketOp::Sum => values.sum(),
                    PacketOp::Product => values.product(),
                    PacketOp::Minimum => values.min().unwrap(),
                    PacketOp::Maximum => values.max().unwrap(),
                    _ => {
                        let x = values.next().unwrap();
                        let y = values.next().unwrap();
                        assert!(values.next().is_none());
                        (match op {
                            PacketOp::GreaterThan => x > y,
                            PacketOp::LessThan => x < y,
                            _ => x == y,
                        }) as usize
                    }
                }
            }
        }
    }
}

struct PacketReader<'a> {
    r: BitReader<'a>,
}

impl<'a> PacketReader<'a> {
    fn new(r: BitReader) -> PacketReader {
        PacketReader { r: r }
    }

    fn next(&mut self) -> Option<Packet> {
        if self.r.remaining() < 6 {
            return None;
        }
        let version = self.r.read(3) as u8;
        let type_id = self.r.read(3) as u8;
        if version == 0 && type_id == 0 && self.r.remaining() <= 2 {
            // zeroes padding end of stream.
            return None;
        }
        let body = match type_id {
            4 => {
                let mut n = 0;
                let mut width = 0;
                loop {
                    let next = self.r.read(5);
                    width += 4;
                    assert!(width <= mem::size_of::<usize>() * 8);
                    n = (n << 4) | (next & 0xF);
                    if (next & 0x10) == 0 {
                        break;
                    }
                }
                PacketBody::Literal(n)
            }
            _ => {
                let op = match type_id {
                    0 => PacketOp::Sum,
                    1 => PacketOp::Product,
                    2 => PacketOp::Minimum,
                    3 => PacketOp::Maximum,
                    5 => PacketOp::GreaterThan,
                    6 => PacketOp::LessThan,
                    _ => PacketOp::EqualTo,
                };
                let length_type_id = self.r.read(1);
                let subpackets = if length_type_id == 0 {
                    let subpacket_length_in_bits = self.r.read(15);
                    let end = self.r.bits_consumed + subpacket_length_in_bits;
                    let mut subpackets = Vec::<Packet>::new();
                    while self.r.bits_consumed < end {
                        subpackets.push(self.next()?);
                    }
                    assert_eq!(self.r.bits_consumed, end);
                    subpackets
                } else {
                    let subpacket_count = self.r.read(11);
                    let mut subpackets = Vec::<Packet>::with_capacity(subpacket_count);
                    for _ in 0..subpacket_count {
                        subpackets.push(self.next()?);
                    }
                    subpackets
                };
                PacketBody::Operator { op, subpackets }
            }
        };
        Some(Packet { version, body })
    }
}

fn parse_space_separated<T: std::str::FromStr>(s: &str) -> Result<Vec<T>, String> {
    s.split_ascii_whitespace()
        .map(|w| {
            w.parse::<T>()
                .map_err(|_| String::from("expected space-separated list"))
        })
        .collect()
}

fn parse_separated<T: std::str::FromStr>(s: &str, sep: &str) -> Result<Vec<T>, String> {
    s.split(sep)
        .map(|w| {
            w.parse::<T>()
                .map_err(|_| String::from("expected separated list"))
        })
        .collect()
}
