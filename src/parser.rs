use std::io::{self, BufRead, Write};

struct Parser<Reader> 
	where 
		Reader: BufRead {
	reader: &Reader,	
	current_line: String
}

impl<Reader> Parser<Reader>
	where
		Reader: BufRead {
	fn new(_reader: &'a Reader) -> 'a Self {
		return Parser {
			reader: _reader,	
			current_line: String::new()
		}
	}
	fn next_word(&mut self) -> String {
		String::from("")	
	}
}

#[cfg(test)]
mod parser_tests {
	use super::*;

	#[test]
	fn read_single_word() {
		let input = b"First_word second_word";
		let mut parser = Parser::new(&input[..]);
		assert!(parser.next_word() == String::from("First_word"));
	}
}
