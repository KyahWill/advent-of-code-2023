use std::fs::read_to_string;
use regex::Regex;
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn day_3_q1() {

    for line in read_lines("./input/day2.txt"){
    }

    println!("DONE");
}

fn main() {
    day_3_q1();
}

