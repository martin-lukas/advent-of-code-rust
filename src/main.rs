mod utils;
mod y22;
mod y23;

fn main() {
    let _ = color_eyre::install();

    // println!("{}", y22::day1::solution());
    // println!("{}", y22::day2::solution());
    println!("{}", y22::day3::solution());
}
