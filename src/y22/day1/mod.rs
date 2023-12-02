#![allow(unused)]

use crate::utils::{puzzle_input, AocYear};
use std::fs;

pub fn solution() -> String {
    let data = puzzle_input(AocYear::Y22, 1);
    let elfs: Vec<Elf> = data
        .split("\n\n")
        .into_iter()
        .map(|s| to_elf(s.split("\n").collect()))
        .collect();
    format!(
        "Elf with most calories: {}\n\
        Top 3 elf calories: {}",
        get_elf_with_max_load(&elfs),
        get_top_3_elf_calories(elfs)
    )
}

fn get_elf_with_max_load(elfs: &Vec<Elf>) -> u32 {
    elfs.iter()
        .map(|e| e.snacks.iter().sum::<u32>())
        .max()
        .unwrap()
}

fn get_top_3_elf_calories(elfs: Vec<Elf>) -> u32 {
    let mut maxs = elfs
        .iter()
        .map(|e| e.snacks.iter().sum::<u32>())
        .collect::<Vec<u32>>();
    maxs.sort();
    maxs.reverse();
    maxs.iter().take(3).sum::<_>()
}

fn to_elf(snacks_str: Vec<&str>) -> Elf {
    let snacks: Vec<u32> = snacks_str
        .iter()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();
    Elf { snacks }
}

struct Elf {
    snacks: Vec<u32>,
}
