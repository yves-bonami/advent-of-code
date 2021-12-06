use advent_of_code::DayOne;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<i32> = include_str!("./day-one.txt")
        .split("\n")
        .map(|n| n.trim().parse::<i32>().unwrap())
        .collect();

    println!("Day 1 - Part 1: {}", DayOne::part_one(input.clone()));
    println!("Day 1 - Part 2: {}", DayOne::part_two(input.clone()));

    Ok(())
}
