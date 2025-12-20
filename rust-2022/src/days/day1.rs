use rust_utils::utils;

fn compute_sums() {
	let input = utils::get_input(2022, 1, None);
	let mut sum = vec![0];

	for l in input {
		let line = l.trim();
		if line.is_empty(){
			sum.push(0);
		} else {
			let index = sum.len() - 1;
			sum[index] += i32::from_str_radix(line, 10).unwrap();
		}
	}
	sum.sort_unstable_by(|a, b| b.cmp(a));

	println!("The maximum gathered is {} calories", sum[0]);
	println!("The top three elves collectively gathered {} calories", sum[0] + sum[1] + sum[2]);
}

/// https://adventofcode.com/2022/day/1
pub fn main() {
	let mut t = utils::build_timer(file!());
	compute_sums();
	t.step(file!());
}
