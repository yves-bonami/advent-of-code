use advent_of_code::DayTwo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<&str> = include_str!("./day-two.txt").split("\n").collect();

    println!("Day 2 - Part 1: {}", DayTwo::part_one(input.clone()));
    println!("Day 2 - Part 2: {}", DayTwo::part_two(input.clone()));
    
    Ok(())
}
