use rust_utils::utils;

// Create a grid of booleans, with 'true' corresponding to '@' and 'false' to '.'
fn get_grid(lines: &Vec<String>) -> Vec<Vec<bool>> {
	let mut grid= vec![];

	for line in lines {
		grid.push(line.chars().map(|c| c == '@').collect());
	}

	return grid;
}

// Count neighboring rolls
fn count_neighbors(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> u32 {
	let mut neighbors = 0;
	let x_max = grid.len();
	let y_max = grid[0].len();

	for x_offset in -1isize..=1 {
		for y_offset in -1isize..=1 {
			if (x_offset, y_offset) == (0, 0) {
				continue;
			}
			let x = x_offset + x as isize;
			let y = y_offset + y as isize;
			if x >= 0 && (x as usize) < x_max && y >= 0 && (y as usize) < y_max {
				if grid[x as usize][y as usize] {
					neighbors += 1;
				}
			}
		}
	}
	
	return neighbors
}

// Count accessible rolls in grid
fn count_accessible_rolls(input: &Vec<String>, max_neighbors: u32) -> u32 {
	let mut accessible_rolls = 0;
	let grid = get_grid(input);

	for i in 0..grid.len() {
		for j in 0..grid[i].len() {
			if grid[i][j] && count_neighbors(&grid, i, j) <= max_neighbors {
				accessible_rolls += 1;
			}
		}
	}
	
	return accessible_rolls;
}

// Count accessible rolls in grid with removal
fn count_accessible_rolls_with_removal(input: &Vec<String>, max_neighbors: u32, log_grid: Option<bool>) -> u32 {
	let mut accessible_rolls = 0;
	let mut grid = get_grid(input);
	let mut done = false;

	while !done {
		let mut removed_rolls = 0;
		let mut new_grid = grid.clone();
		for i in 0..grid.len() {
			for j in 0..grid[i].len() {
				if grid[i][j] && count_neighbors(&grid, i, j) <= max_neighbors {
					removed_rolls += 1;
					new_grid[i][j] = false;
				}
			}
		}
		// Removing rolls no longer possible, we have all the accessible ones.
		if removed_rolls == 0 {
			done = true;
		}
		accessible_rolls += removed_rolls;
		grid = new_grid;

		if log_grid.is_some_and(|x| x) {
			for line in &grid {
				let strvec: Vec<String> = line.iter().map(|x| if *x {String::from("@")} else {String::from("_")}).collect();
				println!("{}", strvec.join(""));
			}
		}
	}
	
	return accessible_rolls;
}

/// https://adventofcode.com/2025/day/4
pub fn main() {
	// let input : Vec<String> = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.".split('\n').map(String::from).collect();
	let input = utils::get_input(2025, 4, None);
	let mut t = utils::build_timer(file!());
	
	// PART ONE
	let part_one_result: i64 = count_accessible_rolls(&input, 3).into();

	println!("There are {} rolls with fewer than 4 neighbors", part_one_result);
	t.step("part 1");
	
	// PART TWO
	let part_two_result = count_accessible_rolls_with_removal(&input, 12, None);

	println!("The total joltage with 12 batteries per bank is {}", part_two_result);
	t.step("part 2");
	t.total(file!());
}
