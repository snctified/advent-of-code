use rust_utils::utils;

fn count_zero_pointed(input: &Vec<String>) -> i32 {
	let mut zero_pointed_count = 0;
	let mut number = 50;
	
	for l in input {
		let clicks = utils::atoi(&l.replace("R", "").replace("L", "-"));
		number = (number + clicks).rem_euclid(100);
		if number == 0 {
			zero_pointed_count += 1;
		}
	}

	return zero_pointed_count;
}

fn count_zero_hit(input: &Vec<String>) -> i32 {
	let mut zero_hit_count = 0;
	let mut number = 50;
	
	for l in input {
		let clicks = utils::atoi(&l.replace("R", "").replace("L", "-"));
		let mut modulus = (number + clicks).div_euclid(100).abs();
		let remainder = (number + clicks).rem_euclid(100);

		// println!("{} results in {} with mod {}", l, remainder, modulus);
		if remainder == 0 {
			//zero_hit_count += 1;
			modulus += 1;
			// println!("*zero pointed*");
		} 
		if modulus != 0 && number != 0 {
			zero_hit_count += modulus.abs();
			// println!("*zero hit*");
		}
		number = remainder;
	}
	
	return zero_hit_count;
}

/// https://adventofcode.com/2025/day/1
pub fn main() {
	let input : Vec<String> = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82".split('\n').map(String::from).collect();
	// let input = utils::get_input(2025, 1, None);
	let mut t = utils::build_timer(file!());
	
	// PART ONE
	let password = count_zero_pointed(&input);

	println!("The password is {}", password);
	t.step("part 1");

	// PART TWO
	let new_password = count_zero_hit(&input);

	println!("Using method 0x434C49434B, the password is {}", new_password);
	t.step("part 2");
	t.total(file!());
}
