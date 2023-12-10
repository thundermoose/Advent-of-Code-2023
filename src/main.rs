mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
use std::env;

use day1::run_day1;
use day2::run_day2;
use day3::run_day3;
use day4::run_day4;
use day5::run_day5;
use day6::run_day6;

fn main() {
	for command in env::args().skip(1) {
		match command.as_str() {
			"day1" | "1" => run_day1(),
			"day2" | "2" => run_day2(),
			"day3" | "3" => run_day3(),
			"day4" | "4" => run_day4(),
			"day5" | "5" => run_day5(),
			"day6" | "6" => run_day6(),
			otherwise => println!("Skipping: {}", otherwise)
		}
	}
}

