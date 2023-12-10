use std::fs::File;
use std::io::{BufRead, BufReader};

const EXAMPLE: &str = "problem_data/day6/example.txt";
const INPUT: &str = "problem_data/day6/input.txt";

struct BoatRace {
	time_limit_ms: usize,
	record_distance: usize
}

impl BoatRace {
	fn new(time_limit: usize, record: usize) -> Self {
		BoatRace {
			time_limit_ms: time_limit,
			record_distance: record	   
		}
	}

	fn distance(&self, press_time: usize) -> usize {
		press_time*(self.time_limit_ms - press_time)	
	}

	fn margin_of_error(&self) -> usize {
		let tm = self.time_limit_ms as f64;
		let dr = self.record_distance as f64;
		let variance = (tm*tm*0.25 - dr).sqrt();
		let lower_limit = (tm*0.5 - variance).ceil() as usize;
		let upper_limit = (tm*0.5 + variance).floor() as usize;
		let mut result = 1+upper_limit - lower_limit;
		if self.distance(lower_limit) == self.record_distance {
			result = result - 1;
		}
		if self.distance(upper_limit) == self.record_distance {
			result = result - 1;
		}
		println!("BoatRace({}, {}).margin_of_error() = {}",
				 self.time_limit_ms, self.record_distance,
				 result);
		result
	}
}

fn extract_list(line: &String) -> Vec<usize> {
	line		
		.split_once(':')
		.unwrap().1.trim().split(' ')
		.filter(|word| *word != "")
		.map(|w| w.parse::<usize>().unwrap())
		.collect()
}

fn read_full_number(number_str: &String) -> usize {
	let (_, number_part) = number_str.split_once(':').unwrap();
	number_part.chars().filter(|c| *c != ' ').collect::<String>().parse::<usize>().unwrap()
}

fn read_boat_races(filename: &str) -> Vec<BoatRace> {
	let input_file = File::open(filename).unwrap();		
	let input_buffer = BufReader::new(input_file);
	let data_lines: Vec<String> = 
		input_buffer.lines().map(|line| line.unwrap()).take(2).collect();
	let time_limits: Vec<usize> = extract_list(&data_lines[0]);
	let record_distances: Vec<usize> = extract_list(&data_lines[1]);
	time_limits.iter().zip(record_distances.iter())
		.map(|(time, record)| BoatRace::new(*time, *record)).collect()
}

fn read_boat_race(filename: &str) -> BoatRace {
	let input_file = File::open(filename).unwrap();		
	let input_buffer = BufReader::new(input_file);
	let data_lines: Vec<String> =
		input_buffer.lines().map(|line| line.unwrap()).take(2).collect();
	let time_limit = read_full_number(&data_lines[0]);
	println!("time_limit: {}", time_limit);
	let record_distance = read_full_number(&data_lines[1]);
	println!("record_distance: {}", record_distance);
	BoatRace::new(time_limit, record_distance)
}

pub fn run_day6() {
	println!("Day 6");
	part1(EXAMPLE);	
	part1(INPUT);	
	part2(EXAMPLE);	
	part2(INPUT);	
}

fn part1(filename: &str) {
	println!("Part 1: {}", filename);
	let races = read_boat_races(filename);
	let margin_of_error_product: usize = races.iter().map(|race| race.margin_of_error()).product();
	println!("product: {}", margin_of_error_product);
}

fn part2(filename: &str) {
	println!("Part 2: {}", filename);
	let race = read_boat_race(filename);
	println!("{}", race.margin_of_error());
}

