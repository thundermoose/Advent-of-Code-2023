use std::fs::File;
use std::io::{BufRead, BufReader};

const EXAMPLE: &str = "problem_data/day5/example.txt";
const INPUT: &str = "problem_data/day5/input.txt";

#[derive(PartialEq, Clone, Copy, Debug)]
struct Range {
	start: usize,
	length: usize
}

impl Range {
	fn new(current_start: usize, current_length: usize) -> Self {
		Range {
		   start: current_start,
		   length: current_length
		}
	}

	fn end(&self) -> usize {
		self.start + self.length
	}

	fn overlap(&self, other: Range) -> (Option<Range>, Vec<Range>) {
		if self.start <= other.start && self.end() >= other.end() {
			(Some(other), Vec::new())
		} else if self.start <= other.start && 
				  other.start < self.end() && 
				  self.end() < other.end() {
			(Some(Range::new(other.start, self.end() - other.start)),
			 vec![Range::new(self.end(), other.end() - self.end())])
		} else if self.start > other.start && 
				  self.start < other.end() && 
				  self.end() >= other.end(){
			(Some(Range::new(self.start, other.end() - self.start)),
			 vec![Range::new(other.start, self.start - other.start)])
		} else if self.start > other.start && self.end() < other.end(){
			(Some(Range::new(self.start, self.length)),
			 vec![Range::new(other.start, self.start - other.start),
			 	  Range::new(self.end(), other.end() - self.end())])
		} else {
			(None, vec![other])
		}
	}
}

#[derive(PartialEq)]
struct RangeMap {
	source_start: usize,
	target_start: usize,	
	range_length: usize
}

impl RangeMap {
	fn parse(input_string: &str) -> Option<Self> {
		let (target_start_str, tail) = input_string.split_once(' ')?;
		let (source_start_str, range_length_str) = tail.split_once(' ')?;
		Some(RangeMap {
				source_start: source_start_str.parse::<usize>().unwrap(),
				target_start: target_start_str.parse::<usize>().unwrap(),
				range_length: range_length_str.parse::<usize>().unwrap()
			 })
	}

	fn transform(&self, value: usize) -> Option<usize> {
		if self.source_start <= value && 
			value <= self.source_start + self.range_length {
			Some(self.target_start + (value - self.source_start))
		} else {
			None
		}		
	}

	fn transform_range(&self, range: Range) -> (Option<Range>, Vec<Range>) { 
		let source_range = Range {
			start: self.source_start, 
			length: self.range_length
		};	
		let (overlap, remainder) = source_range.overlap(range);
		(overlap.map(|o| Range::new(self.target_start + (o.start - self.source_start), 
			o.length)), remainder)
	}
}

struct IndexMap {
	ranges: Vec<RangeMap>
}

impl IndexMap {
	fn new() -> Self {
		IndexMap {
			ranges: Vec::new()
		}
	}

	fn add_range(&mut self, range: RangeMap) {
		self.ranges.push(range);
	}

	fn transform(&self, value: usize) -> usize {
		match self.ranges.iter().map(|range| range.transform(value)).find(|x| x.is_some()) {
			Some(v) => v.unwrap(),
			None => value
		}
	}

	fn transform_range(&self, range: Range) -> Vec<Range> {
		let mut result_vec: Vec<Range> = Vec::new();
		let mut reminder: Vec<Range> = self.ranges
			.iter()
			.fold(vec![range], 
				  |ranges, range_map| {
					let mut next_ranges: Vec<Range> = Vec::new();
					ranges.iter()
						  .for_each(|range_to_transform| {
										let (transformed, mut reminder) = 
											range_map
											.transform_range(*range_to_transform);
										if let Some(t) = transformed {
											result_vec.push(t);
										}
										next_ranges.append(&mut reminder);
									});
					next_ranges
				  });
		result_vec.append(&mut reminder);
		println!("Transformed {:?} to {:?}", range, result_vec);
		result_vec
	}
	}

struct DataFile {
	seeds: Vec<usize>,
	index_maps: Vec<IndexMap>
}

impl DataFile {
	fn read(filename: &str) -> Option<Self> {
		let input_file = File::open(filename).ok()?;	
		let input_buffer = BufReader::new(input_file);
		let mut current_seeds: Vec<usize> = Vec::new();
		let mut current_index_map_list: Vec<IndexMap> = Vec::new();
		for (i, line) in input_buffer.lines().enumerate() {
			let row = line.ok()?;
			if i == 0 {
				let (_, tail) = row.split_once(':')?;
				current_seeds = tail.trim().split(' ')
					.map(|word| word.parse::<usize>().unwrap()).collect();
			} else {
				if let Some(range) = RangeMap::parse(row.as_str()) {
					let len = current_index_map_list.len();
					current_index_map_list[len - 1].add_range(range);
				} else if row != "".to_string() {
					current_index_map_list.push(IndexMap::new());
				}
			}
		}
		Some(DataFile {
			seeds: current_seeds,
			index_maps: current_index_map_list
		})
	}

	fn expand_seeds(&self) -> Vec<Range> {
		let mut expanded_seeds: Vec<Range> = Vec::new();	
		self.seeds
			.iter()
			.enumerate()
			.fold(0,|previous, (index, value)| {
					if index % 2 == 1 {
						expanded_seeds.push(Range::new(previous, *value));
						0
					} else {
						*value
					}
				  });
		expanded_seeds
	}
}


pub fn run_day5() {
	println!("Day 5:");
	part1(EXAMPLE);
	part1(INPUT);
	part2(EXAMPLE);
	part2(INPUT);
}

fn part1(filename: &str) {
	println!("Part 1: {}", filename);
	let data_file = DataFile::read(filename).unwrap();
	let transformed_seeds = data_file
		.index_maps
		.iter()
		.fold(data_file.seeds, |current_seeds, index_map| {
			  current_seeds.iter().map(|index| {
									   index_map.transform(*index)
									   }).collect::<Vec<usize>>()
			  });
	println!("Min position: {}", transformed_seeds.iter().min().unwrap());
}

fn part2(filename: &str) {
	println!("Part 2: {}", filename);
	let data_file = DataFile::read(filename).unwrap();
	let seeds = data_file.expand_seeds();
	let minimum_postion = 
		seeds.iter().map(|seed_range| {
						data_file.index_maps
						         .iter()
								 .enumerate()
								 .fold(vec![*seed_range],
									   |seeds_to_apply, (map_index, index_map)| {
									   		println!("Applying {}", map_index);
											seeds_to_apply.iter().flat_map(|seed| {
																			index_map.transform_range(*seed)
																		   }).collect::<Vec<Range>>()
									   }).iter().map(|pos| pos.start).min().unwrap()

						 }).min().unwrap();
	println!("Min position: {}", minimum_postion);
}



#[cfg(test)]
mod day5_tests {
	use super::*;
#[test]
	fn parse_range() {
		let input_string = "45 23 5";
		assert!(RangeMap::parse(input_string) == Some(RangeMap {
			source_start: 23, 
			target_start: 45,
			range_length: 5
			}));
	}

#[test]
	fn range_maps_inside_value_correct() {
		let range = RangeMap {
			source_start: 45, 
		  	target_start: 23, 
			range_length: 5
		};
		assert!(range.transform(47) == Some(25));
	}
}
