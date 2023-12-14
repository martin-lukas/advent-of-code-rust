#![allow(unused)]

use crate::utils::{puzzle_input, AocYear};
use itertools::{self, Itertools};
use std::{collections::HashMap, fs};

pub fn solution() -> String {
    let data = puzzle_input(AocYear::Y23, 1);

    format!(
        "Sum of edge digits: {}\n\
        Sum of edge digits including spelled-out: {}",
        get_sum_of_edge_digits(&data),
        get_sum_of_edge_digits_inc_spelled(&data)
    )
}

fn get_sum_of_edge_digits(data: &String) -> u32 {
    let char_lines = data
        .lines()
        .map(|l| l.to_string())
        .map(to_char_array)
        .collect::<Vec<_>>();
    let mut sum = 0_u32;
    for char_line in char_lines {
        let res = char_line
            .iter()
            .filter(|c| c.is_digit(10))
            .collect::<Vec<_>>();
        let first = res.first();
        let last = res.last();
        let r = match (first, last) {
            (Some(a), Some(b)) => format!("{}{}", a, b),
            (_, _) => {
                let line: String = char_line.iter().collect();
                panic!("The line {line} didn't contain at least 2 digits.")
            }
        }
        .parse::<u32>()
        .unwrap();
        sum += r;
    }
    sum
}

fn get_sum_of_edge_digits_inc_spelled(data: &String) -> u32 {
    let char_lines = data
        .lines()
        .map(|line| {
            let res = SPELLED_NUMS
                .iter()
                .map(|num| (line.find(num), num))
                .filter(|(i, _)| i.is_some())
                .map(|(i, n)| (i.unwrap(), *n))
                .min_by(|(i, _), (j, _)| i.cmp(j));
            line.replace()
        })
        .map(to_char_array)
        .collect::<Vec<_>>();
    let mut sum = 0_u32;
    for char_line in char_lines {
        let res = char_line
            .iter()
            .filter(|c| c.is_digit(10))
            .collect::<Vec<_>>();
        let first = res.first();
        let last = res.last();
        let r = match (first, last) {
            (Some(a), Some(b)) => format!("{}{}", a, b),
            (_, _) => {
                let line: String = char_line.iter().collect();
                panic!("The line {line} didn't contain at least 2 digits.")
            }
        }
        .parse::<u32>()
        .unwrap();
        sum += r;
    }
    sum
}

// fn get_first_and_last_index(str: &str, query: &str) -> (Option<usize>, Option<usize>) {}

fn to_char_array(str: String) -> Vec<char> {
    str.chars().collect::<Vec<char>>()
}

fn to_digit<'a>(str: &'a str) -> &'a str {
    match str {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        "zero" => "0",
        _ => panic!("The string '{str}' is not a valid digit"),
    }
}

const SPELLED_NUMS: [&str; 10] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero",
];

#[cfg(test)]
mod tests {
    use crate::y23::day1::*;

    #[test]
    fn test_part_2() {
        let data = "two1nine\n\
                    eightwothree\n\
                    abcone2threexyz\n\
                    xtwone3four\n\
                    4nineeightseven2\n\
                    zoneight234\n\
                    7pqrstsixteen";
        assert_eq!(get_sum_of_edge_digits_inc_spelled(&data.to_string()), 281);
    }
    #[test]
    fn test_to_char_array() {
        assert_eq!(to_char_array("".to_string()), vec![]);
        assert_eq!(to_char_array("line".to_string()), vec!['l', 'i', 'n', 'e']);
        assert_eq!(
            to_char_array("1234on".to_string()),
            vec!['1', '2', '3', '4', 'o', 'n']
        );
    }
}
