use rust_utils::utils;

/// Compute the number of included and overlapping pairs
fn compare_pairs() {
	let input = utils::get_input(2022, 4, None);
	let (mut include, mut overlap) = (0, 0);
	
	for l in input {
		let pair: Vec<Vec<i32>> = l.trim().split(',').map(
			|x| x.split('-').map(
				|y| i32::from_str_radix(y, 10).unwrap()
			).collect()
		).collect();
		
		// PART ONE
		if (pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1]) || (pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1]) {
			include += 1;
		}

		// PART TWO
		if !(pair[0][1] < pair[1][0] || pair[0][0] > pair[1][1]) {
			overlap += 1;
		}
	}

	println!("There are {} assignment pairs where one range includes the other", include);
	println!("There are {} overlapping assignment pairs", overlap);
}

/// https://adventofcode.com/2022/day/4
pub fn main() {
	let mut t = utils::build_timer(file!());
	compare_pairs();
	t.step(file!());
}
