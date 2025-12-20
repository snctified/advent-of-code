use rust_utils::utils;
use iter_first_max::{self, IterFirstMaxExt};

fn get_joltages(line: &String) -> Vec<u32> {
	return line.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn find_max_and_pos(slice: &[u32]) -> (usize, u32) {
	let (pos, max) = slice.iter().enumerate().first_max_by(|x, y| x.1.cmp(y.1)).unwrap();
	return (pos, max.to_owned())
}

fn compute_total_joltage_for_2_batteries(input: &Vec<String>) -> u32 {
	let mut total_joltage = 0;
	
	for line in input {
		let joltages: Vec<u32> = get_joltages(line);
		let (pos, max) = find_max_and_pos(&joltages[..joltages.len() - 1]);
		let second = joltages[pos + 1..].iter().max().unwrap();
		total_joltage += 10 * max + second;
	}
	return total_joltage;
}

fn compute_total_joltage_for_n_batteries(input: &Vec<String>, n: usize) -> i64 {
	let mut total_joltage = 0;
	
	for line in input {
		let joltages: Vec<u32> = get_joltages(line);
		let size = joltages.len();
		let (mut pos, mut max) = find_max_and_pos(&joltages[..size - n + 1 ]);
		let mut joltage = max.to_string();
		for i in 1..n {
			let last_pos = size - n + 1 + i;
			pos += 1;
			let new_pos;
			(new_pos, max) = find_max_and_pos(&joltages[pos..last_pos]);
			joltage += &max.to_string();
			pos += new_pos;
		}
		total_joltage += utils::atoi64(&joltage);
	}
	return total_joltage;
}


/// https://adventofcode.com/2025/day/3
pub fn main() {
	let input = utils::get_input(2025, 3, None);
	let mut t = utils::build_timer(file!());
	
	// PART ONE
	let part_one_result: i64 = compute_total_joltage_for_2_batteries(&input).into();

	println!("The total joltage with 2 batteries per bank is {}", part_one_result);
	t.step("part 1");
	assert_eq!(part_one_result, compute_total_joltage_for_n_batteries(&input, 2));

	// PART TWO
	let part_two_result = compute_total_joltage_for_n_batteries(&input, 12);

	println!("The total joltage with 12 batteries per bank is {}", part_two_result);
	t.step("part 2");
	t.total(file!());
}
