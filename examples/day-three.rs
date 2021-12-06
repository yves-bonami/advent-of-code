use advent_of_code::DayThree;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<&str> = include_str!("./day-three.txt").split("\n").collect();

    println!("Day 3 - Part 1: {}", DayThree::part_one(input.clone()));

    Ok(())
}
