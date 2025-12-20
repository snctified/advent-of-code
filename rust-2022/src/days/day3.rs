use rust_utils::utils;
use std::collections::HashSet;

/// Compute the priority of an item
fn priority(c: char) -> i32 {
    if c >= 'a' {
        return (c as i32) - ('a' as i32) + 1; // from 1 to 26
    }
    return (c as i32) - ('A' as i32) + 27; // from 27 to 52
}

/// Compute the sums of priorities in the rucksacks
fn compute_sums() {
	let input = utils::get_input(2022, 3, None);
    let input_len = input.len();

    // PART ONE
    let mut result = 0;
	for l in input.clone() {
		let line = l.trim();
        let (first, second) = line.split_at(line.len()/2);
        let first_set: HashSet<char> = HashSet::from_iter(first.chars());
        let second_set: HashSet<char> = HashSet::from_iter(second.chars());
        result += priority(*first_set.intersection(&second_set)
                                     .next()
                                     .unwrap());
	}
    println!("The sum of the priorities of the all the items common to both compartments of a rucksack is {}", result);

    // PART TWO
    result = 0;
    for i in (0..input_len).step_by(3) {
        let set1: HashSet<char> = HashSet::from_iter(input[i].trim().chars());
        let set2: HashSet<char> = HashSet::from_iter(input[i + 1].trim().chars());
        let set3: HashSet<char> = HashSet::from_iter(input[i + 2].trim().chars());
        result += priority(*set1.iter()
                                .filter(|k| set2.contains(k) && set3.contains(k))
                                .next()
                                .unwrap());
    }
    println!("The sum of the priorities of all the group badges is {}", result);
	}

/// https://adventofcode.com/2022/day/3
pub fn main() {
	let mut t = utils::build_timer(file!());
	compute_sums();
	t.step(file!());
}
