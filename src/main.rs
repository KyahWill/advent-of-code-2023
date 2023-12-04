use regex::Regex;
use std::fs::read_to_string;
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

struct PartNumber {
    value: usize,
    y_coordinate: isize,
    x_start: isize,
    x_end: isize,
}

struct Symbol {
    value: char,
    x_coordinate: isize,
    y_coordinate: isize,
}

fn day_3_q1() {
    let mut sum: usize = 0;
    let mut max_sum: usize = 0;
    let re = Regex::new(r"([0-9]+)").unwrap();
    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut y_coordinate: usize = 0;
    for line in read_lines("./input/day3.txt") {
        for (_index, [digit]) in re.captures_iter(&line).map(|c| c.extract()) {
            let part_number = PartNumber {
                value: digit.parse().unwrap(),
                y_coordinate: y_coordinate as isize,
                x_start: line.find(digit).unwrap() as isize,
                x_end: (line.find(digit).unwrap() + digit.len() - 1 )as isize,
            };
            println!("{} =  ({},{}), {}",digit, part_number.x_start, part_number.x_end, y_coordinate);
            max_sum += part_number.value;
            part_numbers.push(part_number)
        }

        for (index, character) in line.chars().enumerate() {
            if !character.is_ascii_digit() && character != '.' {
                let symbol = Symbol {
                    value: character,
                    x_coordinate: index as isize,
                    y_coordinate: y_coordinate as isize,
                };
                println!("{} =  ({},{})",character, index, y_coordinate);
                symbols.push(symbol);
            }
        }
        y_coordinate += 1;
    }

    for symbol in symbols {
        let mut indexes: Vec<usize> = Vec::new();
        for  (index,part_number) in &mut part_numbers.iter().enumerate() {
            //top / bottom logic
            if (symbol.y_coordinate == part_number.y_coordinate + 1
                || symbol.y_coordinate == part_number.y_coordinate - 1)
                && (symbol.x_coordinate >= part_number.x_start - 1
                    && symbol.x_coordinate <= part_number.x_end + 1)
            {
                sum += part_number.value;
                indexes.push(index);
            }else if (symbol.y_coordinate == part_number.y_coordinate) &&
                (symbol.x_coordinate >= part_number.x_start - 1
                    && symbol.x_coordinate <= part_number.x_end + 1)
            {
                sum += part_number.value;
                indexes.push(index);
            }
        }
        let mut counter = 0;

        println!("Symbol {} ({},{}):", symbol.value, symbol.x_coordinate, symbol.y_coordinate);
        for index in indexes{
        println!("REMOVED {} {},{}:", 
                part_numbers[index-counter].value,
                part_numbers[index-counter].x_start,
                part_numbers[index-counter].x_end,

            );
            part_numbers.remove(index - counter);
            counter += 1;
        }


    }

        for part_number in part_numbers.iter(){

            println!("Remaining {} {},{}:", 
                part_number.value,
                part_number.x_start,
                part_number.x_end,

            );
        }
    println!("DONE");
    println!("{}", sum);
    println!("{}", max_sum);
}

//
// hashmap of (number, column, row_start, row_end)
//
// symbol (x_coordinate, y_coordinate)
//467..114..
//...*......
//..35..633.
//......#...
//617*......
//.....+.58.
//..592.....
//......755.
//...$.*....
//.664.598..
//
//

//
//For every number:
//  Check if close to Symbol
//      If close, then add to sum

fn main() {
    day_3_q1();
}
