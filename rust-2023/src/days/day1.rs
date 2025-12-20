use rust_utils::utils;
use std::collections::HashMap;

fn compute_sum() {
	let input = utils::get_input(2023, 1, None);
	let (mut part_one_sum, mut part_two_sum): (u32, u32) = (0, 0);
	let digits: HashMap<&str, &str> = [
		("one", "o1e"),
		("two", "t2o"),
		("three", "t3e"),
		("four", "f4r"),
		("five", "f5e"),
		("six", "s6x"),
		("seven", "s7n"),
		("eight", "e8t"),
		("nine", "n9e")
	].into();
	let mut t = utils::build_timer(file!());
	
	// PART ONE
	for l in input.clone() {
		part_one_sum += get_calibration_value(l);
	}

	println!("The sum of calibration values is {}", part_one_sum);
	t.step("part 1");

	// PART TWO
	for l in input {
		part_two_sum += get_calibration_value(convert_spelled_digits(l, &digits));
	}

	println!("The new sum of calibration values is {}", part_two_sum);
	t.step("part 2");
	t.total(file!());
}

fn get_calibration_value(l: String) -> u32 {
	let ints: Vec<u32> = l.trim().chars().filter_map(
		|x| x.to_digit(10)
	).collect();
	return u32::from_str_radix((ints[0].to_string() + &ints[ints.len()-1].to_string()).as_str(),10).unwrap();
}

fn convert_spelled_digits(mut l: String, digits: &HashMap<&str, &str>) -> String {
	for i in digits {
		l = l.replace(i.0, i.1);
	}
	return l;
}

/// https://adventofcode.com/2023/day/1
pub fn main() {
	compute_sum();
}
