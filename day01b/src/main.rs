use std::fs;
use std::ops::Index;

fn parse_string_to_number(s: &str) -> Option<u32> {
    match s.to_lowercase().as_str() {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn rev(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("error reading a file");

    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let result: u32 = contents.split_whitespace().map(
        |word| {
            let letter_first: (u32, u32) = numbers.iter().map(
                |&n| {
                    let index = word.find(n).unwrap_or(1000) as u32;
                    (index, parse_string_to_number(n).unwrap())
                }
            ).min().unwrap();

            let digit_first = (word.find(|x: char| x.is_digit(10)).unwrap() as u32,
                               word.chars().find(|x| x.is_digit(10)).unwrap().to_digit(10).unwrap());

            let first: u32 =
                if letter_first.0 <= digit_first.0 {
                    letter_first.1
                } else {
                    digit_first.1
                };

            let letter_second: (u32, u32) = numbers.iter().map(
                |&n| {
                    let index = rev(word).find(rev(n).as_str()).unwrap_or(1000) as u32;
                    (index, parse_string_to_number(n).unwrap())
                }
            ).min().unwrap();

            let digit_second = (rev(word).find(|x: char| x.is_digit(10)).unwrap() as u32,
                               rev(word).chars().find(|x| x.is_digit(10)).unwrap().to_digit(10).unwrap());

            let second: u32 =
                if letter_second.0 <= digit_second.0 {
                    letter_second.1
                } else {
                    digit_second.1
                };

            10 * first + second
        }
    ).sum();

    println!("result: {}", result);
}
