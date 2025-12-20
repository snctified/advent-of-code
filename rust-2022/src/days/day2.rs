use rust_utils::utils;
use std::hash::Hash;
use std::collections::HashMap;

trait Memory<A: Eq + Hash, B: Eq + Hash> {

    fn get(&self, a: &A, b: &B) -> Option<&i32>;

    fn set(&mut self, a: A, b: B, v: i32);
}

pub struct Table<A: Eq + Hash, B: Eq + Hash> {
    table: HashMap<A, HashMap<B, i32>>,
}   

impl<A: Eq + Hash, B: Eq + Hash> Table<A, B> {
    fn new() -> Table<A, B> {
        Table {
            table: HashMap::new(),
        }
    }
}

impl<A: Eq + Hash, B: Eq + Hash> Memory<A, B> for Table<A, B> {

    fn get(&self, a: &A, b: &B) -> Option<&i32> {
        self.table.get(a)?.get(b)
    }

    fn set(&mut self, a: A, b: B, v: i32) {
        let inner = self.table.entry(a).or_insert(HashMap::new());
        inner.insert(b, v);
    }
}

fn play_rounds() {
	let input = utils::get_input(2022, 2, None);
	let base_value: HashMap<&str, i32> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

	// PART ONE
	let mut p1_total = 0;
	let mut p1_round_score: Table<&str, &str> = Table::new();
	p1_round_score.set("A", "X", 3);
	p1_round_score.set("A", "Y", 6);
	p1_round_score.set("A", "Z", 0);
	p1_round_score.set("B", "X", 0);
	p1_round_score.set("B", "Y", 3);
	p1_round_score.set("B", "Z", 6);
	p1_round_score.set("C", "X", 6);
	p1_round_score.set("C", "Y", 0);
	p1_round_score.set("C", "Z", 3);
	
	for l in &input {
		let round: Vec<&str> = l.trim().split(" ").collect();
		p1_total += base_value.get(round[1]).unwrap();
		p1_total += p1_round_score.get(&round[0], &round[1]).unwrap();

	}
    println!("The total score for part one is {}", p1_total);

	// PART TWO
	let mut p2_total = 0;
	let mut p2_round_score: Table<&str, &str> = Table::new();
	p2_round_score.set("A", "X", 3);
	p2_round_score.set("A", "Y", 4);
	p2_round_score.set("A", "Z", 8);
	p2_round_score.set("B", "X", 1);
	p2_round_score.set("B", "Y", 5);
	p2_round_score.set("B", "Z", 9);
	p2_round_score.set("C", "X", 2);
	p2_round_score.set("C", "Y", 6);
	p2_round_score.set("C", "Z", 7);
	
	for l in input {
		let round: Vec<&str> = l.trim().split(" ").collect();
		p2_total += p2_round_score.get(&round[0], &round[1]).unwrap();
	}
    println!("The total score for part two is {}", p2_total);

}

/// https://adventofcode.com/2022/day/2
pub fn main() {
	let mut t = utils::build_timer(file!());
	play_rounds();
	t.step(file!());
}
