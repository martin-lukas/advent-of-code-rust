#![allow(unused)]

use std::fs;
use Choice::*;

pub fn solution() -> String {
    let data = fs::read_to_string("src/day2/input.txt").unwrap();
    let games: &Vec<(&str, &str)> = &data.trim().lines().into_iter().map(to_char_tuple).collect();
    format!(
        "Score if right side are choices: {}\n\
        Score if right side are outcomes: {}",
        get_score_for_strategy_choices(games),
        get_score_for_strategy_outcomes(games)
    )
}

fn get_score_for_strategy_choices(games: &Vec<(&str, &str)>) -> i32 {
    games.into_iter().map(|g| choice_choice_score(*g)).sum()
}

fn get_score_for_strategy_outcomes(games: &Vec<(&str, &str)>) -> i32 {
    games.into_iter().map(|g| choice_outcome_score(*g)).sum()
}

fn result_score(game: &(Choice, Choice)) -> i32 {
    match game {
        (Rock, choice) => match choice {
            Rock => ROCK + DRAW,
            Paper => PAPER + WIN,
            Scissors => SCISSORS + LOSS,
        },
        (Paper, choice) => match choice {
            Rock => ROCK + LOSS,
            Paper => PAPER + DRAW,
            Scissors => SCISSORS + WIN,
        },
        (Scissors, choice) => match choice {
            Rock => ROCK + WIN,
            Paper => PAPER + LOSS,
            Scissors => SCISSORS + DRAW,
        },
    }
}

fn to_char_tuple(game_str: &str) -> (&str, &str) {
    let mut elements = game_str.split_whitespace();
    let opponent = match elements.next() {
        Some(elem) => elem,
        None => {
            panic_for_game(game_str);
            ""
        }
    };
    let me = match elements.next() {
        Some(elem) => elem,
        None => {
            panic_for_game(game_str);
            ""
        }
    };
    (opponent, me)
}

fn choice_choice_score(game: (&str, &str)) -> i32 {
    match game {
        ("A", choice) => match choice {
            "X" => ROCK + DRAW,
            "Y" => PAPER + WIN,
            "Z" => SCISSORS + LOSS,
            _ => {
                panic_for_game(format!("{}{}", game.0, game.1).as_str());
                0
            }
        },
        ("B", choice) => match choice {
            "X" => ROCK + LOSS,
            "Y" => PAPER + DRAW,
            "Z" => SCISSORS + WIN,
            _ => {
                panic_for_game(format!("{}{}", game.0, game.1).as_str());
                0
            }
        },
        ("C", choice) => match choice {
            "X" => SCISSORS + WIN,
            "Y" => PAPER + LOSS,
            "Z" => SCISSORS + DRAW,
            _ => {
                panic_for_game(format!("{}{}", game.0, game.1).as_str());
                0
            }
        },
        (_, _) => {
            panic_for_game(format!("{}{}", game.0, game.1).as_str());
            0
        }
    }
}

fn choice_outcome_score(game: (&str, &str)) -> i32 {
    match game {
        ("A", outcome) => match outcome {
            "X" => LOSS + SCISSORS,
            "Y" => DRAW + ROCK,
            "Z" => WIN + PAPER,
            _ => {
                panic_for_game(format!("{}{}", game.0, game.1).as_str());
                0
            }
        },
        ("B", outcome) => match outcome {
            "X" => LOSS + ROCK,
            "Y" => DRAW + PAPER,
            "Z" => WIN + SCISSORS,
            _ => {
                panic_for_game(format!("{}{}", game.0, game.1).as_str());
                0
            }
        },
        ("C", outcome) => match outcome {
            "X" => LOSS + PAPER,
            "Y" => DRAW + SCISSORS,
            "Z" => WIN + ROCK,
            _ => {
                panic_for_game(format!("{}{}", game.0, game.1).as_str());
                0
            }
        },
        (_, _) => {
            panic_for_game(format!("{}{}", game.0, game.1).as_str());
            0
        }
    }
}

//     let first_choice = match first {
//         "A" => Rock,
//         "B" => Paper,
//         "C" => Scissors,
//         _ => panic!("The input for choice '{}' was invalid.", first),
//     };
//     let second_choice = match second {
//         "X" => Rock,
//         "Y" => Paper,
//         "Z" => Scissors,
//         _ => panic!("The input for choice '{}' was invalid.", second),
//     };
//     (first_choice, second_choice)
// }

fn panic_for_game(game: &str) {
    panic!("Unexpected no. of elements in line (expected 2): {}", game);
}

#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

const LOSS: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;
