mod day1;
mod day2;
mod day3;
use std::env;

use day1::run_day1;
use day2::run_day2;
use day3::run_day3;

fn main() {
	for command in env::args().skip(1) {
		match command.as_str() {
			"day1" | "1" => run_day1(),
			"day2" | "2" => run_day2(),
			"day3" | "3" => run_day3(),
			otherwise => println!("Skipping: {}", otherwise)
		}
	}
}

