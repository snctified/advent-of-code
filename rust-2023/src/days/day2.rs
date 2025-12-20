use rust_utils::utils;
use regex::Regex;
use core::cmp::max;

fn check_games() {
	let input = utils::get_input(2023, 2, None);
	let mut part_one_sum = 0;
	let mut part_two_sum = 0;
	let re_r = Regex::new(r"(\d+) red").unwrap();
	let re_g = Regex::new(r"(\d+) green").unwrap();
	let re_b = Regex::new(r"(\d+) blue").unwrap();
	let mut t = utils::build_timer(file!());
	
	// PART ONE
	let mut game_index = 1;
	let (r_max, g_max, b_max) = (12, 13, 14);
	for l in input.clone() {
		let (_, rounds) = l.split_once(": ").unwrap();
		let (mut red, mut green, mut blue);
		let mut possible = true;
		for round in rounds.split("; ") {
			match re_r.captures(round) {
				Some(caps) => { red = utils::atoi(&caps[1]); }
				None => { red = 0; }
			}
			match re_g.captures(round) {
				Some(caps) => { green = utils::atoi(&caps[1]); }
				None => { green = 0; }
			}
			match re_b.captures(round) {
				Some(caps) => { blue = utils::atoi(&caps[1]); }
				None => { blue = 0; }
			}
			if red > r_max || green > g_max || blue > b_max {
				possible = false;
				break
			}
		}
		if possible { part_one_sum += game_index }
		game_index += 1;
	}

	println!("The sum of calibration values is {}", part_one_sum);
	t.step("part 1");

	// PART TWO
	for mut l in input {
		let (_, rounds) = l.split_at_mut(8);
		let (mut r_min, mut g_min, mut b_min) = (1, 1, 1);
		for round in rounds.split("; ") {
			match re_r.captures(round) {
				Some(caps) => {
					let value = utils::atoi(&caps[1]); 
					r_min = max(r_min, value)
				}
				None => {}
			}
			match re_g.captures(round) {
				Some(caps) => {
					let value = utils::atoi(&caps[1]);
					g_min = max(g_min, value)
				}
				None => {}
			}
			match re_b.captures(round) {
				Some(caps) => {
					let value = utils::atoi(&caps[1]);
					b_min = max(b_min, value)
				}
				None => {}
			}
		}
		part_two_sum += r_min * g_min * b_min;
	}

	println!("The new sum of calibration values is {}", part_two_sum);
	t.step("part 2");
	t.total(file!());
}

/// https://adventofcode.com/2023/day/2
pub fn main() {
	check_games();
}
