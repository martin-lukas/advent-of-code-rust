mod utils;
mod y22;
mod y23;

fn main() {
    let _ = color_eyre::install();

    // Year 2023
    println!("{}", y23::day1::solution());

    // Year 2022
    // println!("{}", y22::day1::solution());
    // println!("{}", y22::day2::solution());
    // println!("{}", y22::day3::solution());
}
