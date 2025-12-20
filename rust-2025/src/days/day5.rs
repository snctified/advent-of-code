use std::ops::RangeInclusive;

use rust_utils::utils;

#[derive(Debug)]
struct Inventory {
    fresh_ranges: Vec<RangeInclusive<u64>>,
    ingredients: Vec<u64>,
}
impl Inventory {
    pub fn new(lines: &Vec<String>) -> Self {
        let (r_str, i_str) = lines.split_at(
            lines
                .iter()
                .position(|s| s.is_empty())
                .unwrap()
        );

        return Inventory {
            fresh_ranges: r_str
                .iter()
                .map(|s| Self::get_range(s.clone()))
                .collect(),
            ingredients: i_str[1..]
                .iter()
                .map(|s| utils::atou64(s))
                .collect(),
        };
    }

    fn get_range(str: String) -> RangeInclusive<u64> {
        let (a, b) = str.split_once("-").unwrap();
        return utils::atou64(a)..=utils::atou64(b);
    }

    pub fn count_fresh_ingredients(&self) -> u64 {
        let mut fresh_ingredients = 0;
        for i in &self.ingredients {
            if
                self.fresh_ranges
                    .iter()
                    .find(|&r| r.contains(i))
                    .is_some()
            {
                fresh_ingredients += 1;
            }
        }
        return fresh_ingredients;
    }

    pub fn count_possible_fresh_ingredients(&self) -> u64 {
        let mut merged_ranges: Vec<RangeInclusive<u64>> = self.fresh_ranges.clone();
        merged_ranges.sort_by(|a, b| a.start().cmp(b.start()));

        let mut i = 0;
        while i + 1 < merged_ranges.len() {
            // If ranges overlap
            if merged_ranges[i + 1].start() <= merged_ranges[i].end() {
                let new_start = *merged_ranges[i].start();
                let new_end = std::cmp::max(*merged_ranges[i].end(), *merged_ranges[i + 1].end());
                merged_ranges[i] = new_start..=new_end;
                merged_ranges.remove(i + 1);
            } else {
                i += 1;
            }
        }
        return merged_ranges
            .iter()
            .map(|r| 1 + r.end() - r.start())
            .sum();
    }
}

/// https://adventofcode.com/2025/day/4
pub fn main() {
    //     let input: Vec<String> = "3-5
    // 10-14
    // 16-20
    // 12-18

    // 1
    // 5
    // 8
    // 11
    // 17
    // 32".split('\n').map(String::from).collect();
    let input = utils::get_input(2025, 5, None);
    let mut t = utils::build_timer(file!());

    let inventory = Inventory::new(&input);
    // PART ONE
    let part_one_result: u64 = inventory.count_fresh_ingredients();

    println!("There are {} fresh ingredients", part_one_result);
    t.step("part 1");

    // PART TWO
    let part_two_result = inventory.count_possible_fresh_ingredients();

    println!("{} Ingredient IDs are considered fresh", part_two_result);
    t.step("part 2");
    t.total(file!());
}
