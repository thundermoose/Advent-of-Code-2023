use std::fs::File;
use std::io::{BufReader, BufRead};

const EXAMPLE_DATA_DAY2: &str = "problem_data/day2/example_part1.txt";
const INPUT_DAY2: &str = "problem_data/day2/input.txt";
const MAX_NUM_RED: usize = 12;
const MAX_NUM_GREEN: usize = 13;
const MAX_NUM_BLUE: usize = 14;

struct CubeSet {
	number_blue: usize,
	number_red: usize,
	number_green: usize
}

impl CubeSet {
	fn new(description: &str) -> Option<Self> {
		let mut count_blue: usize = 0;
		let mut count_red: usize = 0;
		let mut count_green: usize = 0;
		for element in description.split(',') {
			let (count, color) = element.trim().split_once(' ')?;
			match color {
				"blue" => count_blue = count_blue + count.parse::<usize>().unwrap(),
				"red" => count_red = count_red + count.parse::<usize>().unwrap(),
				"green" => count_green = count_green + count.parse::<usize>().unwrap(),
				_ => {}

			};
		}
		Some(CubeSet {
			number_blue: count_blue,
			number_red: count_red,
			number_green: count_green,
		})
	}

	fn parse_cube_sets(description: &str) -> Option<Vec<Self>> {
		description
			.split(';')
			.map(|sub_description| Self::new(sub_description))
			.collect::<Option<Vec<Self>>>()
	}

	fn is_possible(&self, 
				   max_blue: usize, 
				   max_red: usize, 
				   max_green: usize) -> bool {
		self.number_blue <= max_blue &&
			self.number_red <= max_red &&
			self.number_green <= max_green
	}

	fn power(&self) -> usize {
		self.number_blue * self.number_red * self.number_green
	}
}

struct Game {
	game_number: usize,
	cube_sets: Vec<CubeSet>
}

impl Game {
	fn new(description: &str) -> Option<Game> {
		let (head, tail) = description.split_once(':')?;
		let (_, number) = head.split_once(' ')?;	
		let current_game_number = number.parse::<usize>().ok()?;
		let current_cube_sets = CubeSet::parse_cube_sets(tail)?;
		Some(Game {
			game_number: current_game_number,
			cube_sets: current_cube_sets
			 })
	}

	fn is_possible(&self, 
				   max_blue: usize, 
				   max_red: usize, 
				   max_green: usize) -> bool {
		self.cube_sets.iter().all(|cube_set| cube_set.is_possible(max_blue, 
														   max_red, 
														   max_green))
	}
	fn minimum_cube_set(&self) -> CubeSet {
		CubeSet {
			number_blue: self.cube_sets.iter().map(|cs| cs.number_blue).max().unwrap(),
			number_red: self.cube_sets.iter().map(|cs| cs.number_red).max().unwrap(),
			number_green: self.cube_sets.iter().map(|cs| cs.number_green).max().unwrap()
		}
	}
}

pub fn run_day2() {
	println!("Day 2:");
	part1(EXAMPLE_DATA_DAY2);
	part1(INPUT_DAY2);
	part2(EXAMPLE_DATA_DAY2);
	part2(INPUT_DAY2);
}

fn part1(filename: &str) {
	println!("Part1: {}", filename);
	let games = read_games(filename).unwrap();	
	let id_sum = games
		.iter()
		.map(|game| {
			 if game.is_possible(MAX_NUM_BLUE, MAX_NUM_RED, MAX_NUM_GREEN) { 
			 game.game_number 
			 } else {
			 0 as usize
			 }
			 }).sum::<usize>();
	println!("ID sum: {}", id_sum);
}

fn part2(filename: &str) {
	println!("Part2: {}", filename);
	let games = read_games(filename).unwrap();	
	let total_power: usize = 
		games.iter().map(|game| game.minimum_cube_set().power()).sum();
	println!("Total power: {}", total_power);
}

fn read_games(filename: &str) -> Option<Vec<Game>> {
	let input_file = File::open(filename).ok()?;	
	let input_buffer = BufReader::new(input_file);
	input_buffer
		.lines()
		.map(|line| Game::new(line.unwrap().as_str()))
		.collect::<Option<Vec<Game>>>()
}
