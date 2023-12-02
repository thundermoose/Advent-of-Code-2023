use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;

const EXAMPLE_DATA_DAY1_PART1: &str = "problem_data/day1/example_part1.txt";
const EXAMPLE_DATA_DAY1_PART2: &str = "problem_data/day1/example_part2.txt";
const INPUT_DAY1_PART: &str = 		  "problem_data/day1/input.txt";

const MATCH_DIGIT: &str = "(\\d)|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)|(zero)";
const MATCH_TIGID: &str = "(\\d)|(eno)|(owt)|(eerht)|(ruof)|(evif)|(xis)|(neves)|(thgie)|(enin)|(orez)";

pub fn run_day1() {
	println!("Day 1:");
	part1(EXAMPLE_DATA_DAY1_PART1);
	part1(INPUT_DAY1_PART);
	part2(EXAMPLE_DATA_DAY1_PART2);
	part2(INPUT_DAY1_PART);
}

fn part1(filename: &str) {
	println!("Part1: Reading: {}", filename);
	let input_file = File::open(filename).unwrap();	
	let input_buffer = BufReader::new(input_file);
	println!("Calibration: {}",
			 input_buffer.lines()
			 .map(|row| get_calibration_contribution(row.unwrap().as_str()))
			 .sum::<u64>());
}

fn part2(filename: &str) {
	println!("Part2: Reading: {}", filename);
	let input_file = File::open(filename).unwrap();	
	let input_buffer = BufReader::new(input_file);
	println!("Calibration: {}",
			 input_buffer.lines()
			 .map(|row| get_calibration_contribution_regex(row.unwrap().as_str()))
			 .sum::<u64>());
}

fn get_calibration_contribution(row: &str) -> u64 {
	combine_digits(first_digit(&row), last_digit(&row)) as u64
}

fn get_calibration_contribution_regex(row: &str) -> u64 {
	combine_digits(first_regex_digit(&row), last_regex_digit(&row)) as u64
}

fn first_digit(row: &str) -> u8 {
	for d in row.chars() {
		if d.is_numeric() {
			return d.to_digit(10).unwrap() as u8;
		}
	}
	return 0;
}

fn last_digit(row: &str) -> u8 {
	for d in row.chars().rev() {
		if d.is_numeric() {
			return d.to_digit(10).unwrap() as u8;
		}
	}
	return 0;
}

fn first_regex_digit(row: &str) -> u8 {
	let regex = Regex::new(MATCH_DIGIT).unwrap();
	if let Some(digit_match) = regex.find(row) {
		parse_string(digit_match.as_str())
	} else {
		0
	}
}

fn last_regex_digit(row: &str) -> u8 {
	let regex = Regex::new(MATCH_TIGID).unwrap();
	let rev_row: String = row.chars().rev().collect();	
	if let Some(tigid_match) = regex.find(rev_row.as_str()) {
		let digit_match_string: String = 
			tigid_match.as_str().chars().rev().collect();
		parse_string(digit_match_string.as_str())
	} else {
		0
	}
}

fn parse_string(word: &str) -> u8 {
	if word.len() == 1 {
		word.chars().next().unwrap().to_digit(10).unwrap() as u8
	} else {
		match word {
			"zero" => 0,
			"one" => 1,
			"two" => 2,
			"three" => 3,
			"four" => 4,
			"five" => 5,
			"six" => 6,
			"seven"=> 7,
			"eight" => 8,
			"nine" => 9,
			_=> panic!("Not a digit")
		}
	}
}

fn combine_digits(left_digit: u8, right_digit: u8) -> u8 {
	left_digit*10 + right_digit
}
