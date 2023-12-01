use std::fs::read_to_string;
fn test() {

}
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn get_first_number(line: &str) -> u32 {
    let digit_strings: Vec<&str> = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    ].to_vec();
    let mut last_string: (u32, i32)  = (0,9999);
    let mut last_digit: (u32, i32)  = (0,9999);
    for (index,letter) in line.chars().enumerate() {
        if letter.is_digit(10) {
            if index as i32 <= last_digit.1 as i32  {
                last_digit = (letter.to_digit(10).unwrap(), index as i32)
            }
        }
    }
    let mut counter = 0;
    for digit in digit_strings {
        let matches: Vec<(usize, &str)>= line.match_indices(digit).collect();
        for m in matches {
            if m.0 as i32 <= last_string.1  {
                last_string = (counter , m.0 as i32) ;
            }
        }
        counter+=1;
    }
    if last_string.1 < last_digit.1{
        return last_string.0    
    }
    return last_digit.0    

}
fn get_last_number(line: &str) -> u32 {
    let digit_strings: Vec<&str> = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    ].to_vec();
    let mut last_string: (u32, i32)  = (0,-1);
    let mut last_digit: (u32, i32)  = (0,-1);
    for (index,letter) in line.chars().enumerate() {
        if letter.is_digit(10) {
            if index as i32 > last_digit.1 {
                last_digit = (letter.to_digit(10).unwrap(), index as i32)
            }
        }
    }
    let mut counter = 0;
    for digit in digit_strings {
        let matches: Vec<(usize, &str)>= line.match_indices(digit).collect();
        for m in matches {
            if m.0 as i32 > last_string.1  {
                last_string = (counter , m.0 as i32) ;
            }
        }
        counter+=1;
    }
    if last_string.1 > last_digit.1{
        return last_string.0    
    }
    return last_digit.0    

}
fn q2() {
    let mut sum = 0;
    for line in  read_lines("./input/q1.txt"){
        let mut numbers: Vec<u32> = Vec::new();
        let first_number = get_first_number(&line);
        let last_number = get_last_number(&line);
        sum += first_number * 10 + last_number;
    }

    println!("DONE");
    println!("{}",sum);
}
fn q1() {
    let mut sum = 0;
    for line in  read_lines("./input/q1.txt"){
        let mut numbers: Vec<u32> = Vec::new();
        for letter in line.chars() {
            if letter.is_digit(10) {
                numbers.push(letter.to_digit(10).unwrap());
            }
        }
        let array_length = numbers.len();
        if array_length == 1 {
            sum += 11 * numbers[0]
        } else if array_length > 1 {
            sum += 10 * numbers[0] + numbers[array_length-1]
        }
    }

    println!("DONE");
    println!("{}",sum);
}

fn main() {
    q2();
}

