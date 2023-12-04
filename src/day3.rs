use std::fs::File;
use std::io::{BufReader, BufRead};

const EXAMPLE_DATA_PART1: &str = "problem_data/day3/example_part1.txt";
const INPUT_DATA: &str = "problem_data/day3/input.txt";

struct EngineSchematic {
cells: Vec<String>,
}

impl EngineSchematic {
	fn read(filename: &str) -> Self {
		let input_file = File::open(filename).unwrap();	
		let input_buffer = BufReader::new(input_file);
		EngineSchematic {
cells: input_buffer
		   .lines()
		   .map(|line| line.unwrap().to_string())
		   .collect()
		}
	}

	fn cell_indices(&self) -> Vec<(usize, usize)> {
		(0..self.get_number_of_rows())
		.flat_map(|row| (0..self.get_number_of_columns())
				  .map(move |column| (row, column)))
		.collect()
	}

	fn get_number_of_rows(&self) -> usize {
		return self.cells.len();
	}

	fn get_number_of_columns(&self) -> usize {
		if self.cells.len() == 0 {0} else {self.cells[0].len()}
	}

	fn get_cell(&self, row: usize, column: usize) -> char {
		self.cells[row].chars().nth(column).unwrap()
	}

	fn is_cell_number(&self, row: usize, column: usize) -> bool {
		self.get_cell(row, column).is_numeric()
	}	

	fn is_cell_gear(&self, row: usize, column: usize) -> bool {
		if self.get_cell(row, column) == '*' {
			self.get_number_neighbouring_numbers(row, column) == 2
		} else {
			false
		}
	}

	fn is_number_start(&self, row: usize, column: usize) -> bool {
		if self.is_cell_number(row,column) {
			if self.with_in_bounds(row as i64, column as i64 - 1) {
				!self.is_cell_number(row, column - 1)
			} else {
				true
			}
		} else {
			false
		}
	}

	fn with_in_bounds(&self, row: i64, column: i64) -> bool {
		row >= 0 && row < self.get_number_of_rows() as i64 
			&& column >= 0 && column < self.get_number_of_columns() as i64
	}

	fn get_neigbour_indicies(&self, row: usize, column: usize) -> Vec<(usize, usize)> {
		(0..3).flat_map(|row| (0..3).map(move |column| (row, column)))
			.map(|(i, j)|((i + row) as i64 - 1, (j + column) as i64 - 1))
			.filter(|(i, j)| {
					!(*i == row as i64 && *j == column as i64)
					})
			.filter(|(i, j)| self.with_in_bounds(*i,*j))	
			.map(|(i, j)| {
				 (i as usize, j as usize)
				 })
			.collect()
	}

	fn is_neighbours_symbol(&self, row: usize, column: usize) -> bool {
		self.get_neigbour_indicies(row, column)
			.iter()
			.any(|(i, j)| {
				 (!self.is_cell_number(*i,*j)) &&
				 (self.get_cell(*i,*j) != '.')
				 })
	}

	fn begining_of_number(&self, row: usize, column: usize) -> usize {
		(1..=column).rev().find(|c| !self.is_cell_number(row, *c - 1))
			.unwrap_or(0)
	}

	fn end_of_number(&self, row: usize, column: usize) -> usize {
		(column+1..self.get_number_of_columns())
			.find(|c| !self.is_cell_number(row, *c))
			.unwrap_or(self.get_number_of_columns())
	}

	fn is_part_number_start(&self, row: usize, column: usize) -> bool {
		if !self.is_number_start(row,column) {
			false
		} else {
			(column..self.end_of_number(row, column))
				.any(|c| self.is_neighbours_symbol(row, c))
		}
	}

	fn get_number_neighbouring_numbers(&self, row: usize, column: usize) -> usize {
		let mut previous_row: usize = usize::MAX;
		let mut previous_column: usize = usize::MAX;
		self
			.get_neigbour_indicies(row, column)
			.iter()
			.filter(|(row, column)| self.is_cell_number(*row, *column))
			.filter(|(row, column)| {
						let result = !(*row == previous_row && *column == previous_column+1);
						previous_row = *row;
						previous_column = *column;
						result
					})
			.count()
	}

	fn get_number_starting_at(&self, row: usize, column: usize) -> u32 {
		let mut accumulator: u32 = 0;
		for c in column..self.end_of_number(row, column) {
			accumulator = accumulator*10 + self.get_cell(row,c).to_digit(10).unwrap();
		}
		accumulator
	}

	fn get_number(&self, row: usize, column: usize) -> u32 {
		self.get_number_starting_at(row,self.begining_of_number(row,column))
	}

	fn gear_ratio(&self, row: usize, column: usize) -> u64 {
		let mut previous_row: usize = usize::MAX;
		let mut previous_column: usize = usize::MAX;
		self
			.get_neigbour_indicies(row, column)
			.iter()
			.filter(|(row, column)| self.is_cell_number(*row, *column))
			.filter(|(row, column)| {
						let result = !(*row == previous_row && *column == previous_column+1);
						previous_row = *row;
						previous_column = *column;
						result
					})
			.map(|(row, column)| self.get_number(*row, *column) as u64)
			.product()
	}
}

pub fn run_day3() {
	println!("Day 3");
	part1(EXAMPLE_DATA_PART1);
	part1(INPUT_DATA);
	part2(EXAMPLE_DATA_PART1);
	part2(INPUT_DATA);
}

fn part1(filename: &str) {
	println!("Part 1: {}", filename);
	let engine_schematic = EngineSchematic::read(filename);
	let sum_of_partnumbers: u32 = engine_schematic
		.cell_indices()
		.iter()
		.filter(|(row, column)| engine_schematic.is_part_number_start(*row, *column))
		.map(|(row, column)| engine_schematic.get_number_starting_at(*row, *column))
		.sum();
	println!("sum_of_partnumbers: {}", sum_of_partnumbers);
}

fn part2(filename: &str) {
	println!("Part 2: {}", filename);
	let engine_schematic = EngineSchematic::read(filename);
	let sum_of_gear_ratios: u64 = engine_schematic
		.cell_indices()
		.iter()
		.filter(|(row, column)| engine_schematic.is_cell_gear(*row, *column))
		.map(|(row, column)| engine_schematic.gear_ratio(*row, *column))
		.sum();
	println!("sum_of_gear_ratios: {}", sum_of_gear_ratios);
}
