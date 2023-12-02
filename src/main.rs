mod day1;
mod day2;
use std::env;

use day1::run_day1;
use day2::run_day2;

fn main() {
	for command in env::args().skip(1) {
		match command.as_str() {
			"day1" | "1" => run_day1(),
			"day2" | "2" => run_day2(),
			otherwise => println!("Skipping: {}", otherwise)
		}
	}
}

