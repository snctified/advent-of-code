use rust_utils::utils;

fn get_matches(card: &String) -> u32 {
	let mut matches = 0;
	let (winning_str, my_str) = card.split_once(": ").unwrap().1.split_once(" | ").unwrap();
	let winning_nums: Vec<i32> = winning_str.trim().replace("  ", " ").split(' ').map(
		|c| utils::atoi(c.trim())
	).collect();
	let my_nums: Vec<i32> = my_str.trim().replace("  ", " ").split(' ').map(
		|c| utils::atoi(c.trim())
	).collect();

	for num in my_nums {
		if winning_nums.contains(&num) {
			matches += 1;
		}
	}
	return matches;
}

fn sum_card_pile_points(input: &Vec<String>) -> i32 {
	let mut total_points = 0;

	for l in input {
		if !l.is_empty() {
			let matches = get_matches(l);
			if matches > 0 {
				total_points += i32::pow(2, matches - 1);
			}
		}
	}
	return total_points;
}

fn count_scratchcards(input: &Vec<String>) -> i32 {
	let pile_size = input.len();
	let mut cards: Vec<i32> = vec![1;pile_size];

	for i in 0..pile_size {
		let matches = get_matches(&input[i]);
		for j in 0..(matches as usize) {
			let to_inc = i + j + 1;
			if to_inc < pile_size {
				cards[to_inc] += cards[i];
			}
		}
	}
	return cards.iter().sum();
}

/// https://adventofcode.com/2023/day/4
pub fn main() {
	let input = utils::get_input(2023, 4, None);
	let mut t = utils::build_timer(file!());
	// PART ONE
	println!("The pile of cards is worth {} points", sum_card_pile_points(&input));
	t.step("part 1");

	// PART TWO
	println!("The pile now contains {} scratchcards", count_scratchcards(&input));
	t.step("part 2");
	t.total(file!());
}
