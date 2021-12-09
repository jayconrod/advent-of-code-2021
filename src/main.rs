use std::env;
use std::fmt;
use std::fmt::Display;
use std::fs;
use std::process;
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
