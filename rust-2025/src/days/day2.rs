use rust_utils::utils;
use fancy_regex::Regex;

fn sum_invalid_ids(input: &Vec<String>) -> i64 {
	let mut result = 0;
	
	// Iterate over the ranges of numbers
	for line in input[0].split(',') {
		let (first, last) = line.split_once('-').unwrap();
		// Iterate over numbers inside each range
		for number in utils::atoi64(first)..=utils::atoi64(last) {
			let digits = number.to_string();
			// Check if number is even, as odd numbers can't be invalid
			if digits.len() % 2 == 0 {
				let half_size = digits.len() / 2;
				// Check if the ID is invalid
				if digits[..half_size] == digits[half_size..] {
					result += number;
				}
			}
		}
	}
	return result;
}

fn sum_invalid_ids_regex(input: &Vec<String>) -> i64 {
	let mut result = 0;
	let re = Regex::new(r"^(.+?)(?:\1)+$").unwrap();
	
	// Iterate over the ranges of numbers
	for line in input[0].split(',') {
		let (first, last) = line.split_once('-').unwrap();
		// Iterate over numbers inside each range
		for number in utils::atoi64(first)..=utils::atoi64(last) {
			let digits = number.to_string();
			// Check if the number is made of a repeated pattern with a regex
			if re.is_match(&digits).unwrap() {
				result += number;
			}
		}
	}
	return result;
}

/// https://adventofcode.com/2025/day/2
pub fn main() {
	let input = utils::get_input(2025, 2, None);
	let mut t = utils::build_timer(file!());
	
	// PART ONE
	let part_one_result = sum_invalid_ids(&input);

	println!("The sum of invalid IDs is {}", part_one_result);
	t.step("part 1");

	// PART TWO
	let part_two_result = sum_invalid_ids_regex(&input);

	println!("The new sum of invalid IDs is {}", part_two_result);
	t.step("part 2");
	t.total(file!());
}
