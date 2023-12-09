use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};

const EXAMPLE_DATA_PART1: &str = "problem_data/day4/example_part1.txt";
const INPUT: &str = "problem_data/day4/input.txt";

#[derive(Debug)]
struct CopiesRecord {
	current_number_of_copies: HashMap<usize, usize>,
	total_number_of_cards: usize
}

impl CopiesRecord {
	fn new() -> Self {
		CopiesRecord {
			current_number_of_copies: HashMap::new(),
			total_number_of_cards: 0
		}
	}
	fn update(self, 
			  card_number: usize, 
			  number_winning_numbers: usize, 
			  number_current_copies: usize) -> Self {
		let current_total_number_of_cards = 
			self.total_number_of_cards + number_current_copies;
		let mut number_of_coppies: HashMap<usize, usize> = self.current_number_of_copies;
		(1..=number_winning_numbers)
			.map(|k| k + card_number)	
			.for_each(|k| {
					  let increament = number_current_copies 
					  	+ number_of_coppies.get(&k).map(|x| *x).unwrap_or(1 as usize);
					  number_of_coppies.insert(k as usize, increament);
					  });
		CopiesRecord {
			current_number_of_copies: number_of_coppies,
			total_number_of_cards: current_total_number_of_cards
		}
	}
}

#[derive(Debug)]
struct ScratchCard {
	card_number: usize, 
	winning_numbers: HashSet<u64>,	
	lottery_numbers: Vec<u64>
}

impl ScratchCard {
	fn parse(card_string: &str) -> Result<Self, String> {
		let (head, body) = card_string.split_once(':').ok_or("Could not split on :".to_string())?;
		let (card, card_number_string) = head.split_once(' ').ok_or("Could not split head".to_string())?;
		let (winning_numbers_string, lottery_numbers_string) =
			body.split_once('|').ok_or("Could not split body")?;
		if card == "Card" {
			Ok(ScratchCard {
					card_number: card_number_string.trim()
					.parse::<usize>()
					.map_err(|_| "Could not parse card number".to_string())?,
					winning_numbers: 
						HashSet::from_iter(winning_numbers_string
										   .trim()
										   .split(' ')
										   .filter(|w| w.len()>0)
										   .map(|w| w.parse::<u64>().unwrap())),
					lottery_numbers: lottery_numbers_string
									.trim()
									.split(' ')
								    .filter(|w| w.len()>0)
									.map(|w| w.parse::<u64>().unwrap())
									.collect()
				 })
		} else {
			Err("Not a card".to_string())
		}
	}	
	
	fn number_winning_numbers(&self) -> usize {
		self.lottery_numbers.iter()
			.filter(|number| self.winning_numbers.contains(number)).count()
	}

	fn point(&self) -> usize {
		let exponent = self.number_winning_numbers();
		if exponent > 0 {
			(1 as usize) << (exponent-1)
		} else {
			0
		}
	}

	fn process(&self, old_record: CopiesRecord) -> CopiesRecord {
		let number_of_coppies = 
			old_record.current_number_of_copies
			.get(&self.card_number)
			.map(|v| *v)
			.unwrap_or(1);
		old_record.update(self.card_number, 
						  self.number_winning_numbers(), 
						  number_of_coppies)
	}
}

pub fn run_day4() {
	println!("Day 4:");
	part1(EXAMPLE_DATA_PART1);	
	part1(INPUT);	
	part2(EXAMPLE_DATA_PART1);
	part2(INPUT);
}

fn part1(filename: &str) {
	println!("Part 1: {}", filename);
	let total_number_of_points = read_cards(filename)
		.iter().map(|card| card.point()).sum::<usize>();
	println!("Total number of points: {}", total_number_of_points);
}

fn part2(filename: &str) {
	println!("Part 2: {}", filename);
	let total_number_of_scratch_cards = read_cards(filename)
		.iter()
		.fold(CopiesRecord::new(), |record, card| card.process(record))
		.total_number_of_cards;
	println!("Total number of scratch cards: {}",
			 total_number_of_scratch_cards);
}

fn read_cards(filename: &str) -> Vec<ScratchCard> {
	let input_file = File::open(filename).unwrap();	
	let input_buffer = BufReader::new(input_file);
	input_buffer.lines()
		.map(|line| ScratchCard::parse(line.unwrap().as_str()).unwrap())
		.collect()
}
