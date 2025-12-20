use std::time::{Instant, Duration};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use curl::easy::Easy;

/// Return the input file corresponding to a given filename as a list.
pub fn parse_input(filename: &str, separator: Option<char>) -> Vec<String> {
    let path = Path::new("res").join(strip_name(filename) + ".txt");
    let mut buf_reader = BufReader::new(File::open(path).expect("Unable to open file"));
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Unable to read file content");
    return contents.split(separator.unwrap_or('\n')).map(String::from).collect()
}

pub fn get_cookie(filename: &str) -> String {
    let mut buf_reader = BufReader::new(File::open(filename).expect("Unable to open cookie file"));
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Unable to read file content");
    return contents.split('\n').map(String::from).collect()
}

/// Return the input file corresponding to a given filename as a list.
pub fn get_input(year: i32, day: i32, separator: Option<char>) -> Vec<String> {
    let cookie = get_cookie("../rust-utils/config/cookie");
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day).to_string();
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(&url).unwrap();
    handle.cookie(&cookie).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    let input: String = String::from_utf8_lossy(&data).to_string();
    return input.trim().split(separator.unwrap_or('\n')).map(String::from).collect();
}

/// Remove path and extension from a given file path, returning only the file name.
pub fn strip_name(filename: &str) -> String {
    return Path::new(&filename).file_stem().unwrap().to_str().unwrap().to_string()
}

/// Basic timer class for use in advent-of-code
pub struct Timer {
    start_time: Instant,
    step_time: Option<Instant>
}

impl Timer {
    /// Report the elapsed time since last step
    pub fn step(&mut self, name: &str) {
        let elapsed_time: Duration;
        elapsed_time = self.step_time.unwrap_or(self.start_time).elapsed();
        self.step_time = Some(Instant::now());
        println!("Elapsed time for {} : {:?}", strip_name(name), elapsed_time);
    }

    /// Report the total elapsed time
    pub fn total(&self, name: &str) {
        let elapsed_time = self.start_time.elapsed();
        println!("Total elapsed time for {} : {:?}\n", strip_name(name), elapsed_time);
    }
}

/// Start a timer and return it
pub fn build_timer(name: &str) -> Timer {
    println!("Starting timer for {}", strip_name(name));
    let timer = Timer { start_time: Instant::now(), step_time: None };
    return timer;
}

/// Convert a string slice to an integer
pub fn atoi64(a: &str) -> i64 {
    return i64::from_str_radix(a, 10).unwrap();
}

/// Convert a string slice to an integer
pub fn atou64(a: &str) -> u64 {
    return u64::from_str_radix(a, 10).unwrap();
}

/// Convert a string slice to an integer
pub fn atoi(a: &str) -> i32 {
    return i32::from_str_radix(a, 10).unwrap();
}

/// Convert a string slice to an integer
pub fn atou(a: &str) -> u32 {
    return u32::from_str_radix(a, 10).unwrap();
}
