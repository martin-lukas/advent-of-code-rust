#![allow(unused)]

use std::collections::HashSet;
use std::fs;

use itertools::Itertools;

use crate::utils::{puzzle_input, AocYear};

pub fn solution() -> String {
    let data = puzzle_input(AocYear::Y22, 3);
    let rucksacks = data.lines().collect::<Vec<&str>>();
    let shared_item_priorities = rucksacks
        .clone()
        .into_iter()
        .map(split_in_half)
        .map(|(first, second)| (to_char_set(first), to_char_set(second)))
        .map(|(first, second)| {
            first
                .intersection(&second)
                .into_iter()
                .map(char_score)
                .sum::<u32>()
        })
        .sum::<u32>();
    // let shared_item_among_3_priorities = rucksacks
    //     .chunks(3)
    //     .map(|[f, s, t, ..]| [to_char_set(*f), to_char_set(*s), to_char_set(*t)])
    //     .map(|[f, s, t]| {
    //         f.intersection(&s)
    //             .collect::<HashSet<_>>()
    //             .iter()
    //             .map(|c| **c)
    //             .collect::<HashSet<_>>()
    //             .intersection(&t)
    //             .into_iter()
    //             .map(char_score)
    //             .sum::<u32>()
    //     })
    //     .sum::<u32>();

    format!(
        "Priorities for shared item in rucksack: {}\n\
        Priorities for shared item in 3 rucksacks: ",
        shared_item_priorities,
        // shared_item_among_3_priorities
    )
}

fn split_in_half(str: &str) -> (&str, &str) {
    str.split_at(str.len() / 2)
}

fn to_char_set(str: &str) -> HashSet<char> {
    HashSet::from_iter(str.chars().into_iter())
}

fn char_score(c: &char) -> u32 {
    if (c.is_ascii_uppercase()) {
        *c as u32 - 38
    } else {
        *c as u32 - 96
    }
}
