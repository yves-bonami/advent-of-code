pub struct DayOne;
pub struct DayTwo;
pub struct DayThree;

struct Command {
    r#type: CommandType,
    value: i32,
}

enum CommandType {
    Forward,
    Up,
    Down,
}

impl DayOne {
    pub fn part_one(input: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 1..input.len() {
            if input[i] > input[i - 1] {
                count += 1;
            }
        }
        count
    }

    pub fn part_two(input: Vec<i32>) -> i32 {
        let mut count = 0;
        let windows = DayOne::get_windows(input);
        for i in 1..windows.len() {
            if windows[i] > windows[i - 1] {
                count += 1;
            }
        }
        count
    }

    fn get_windows(input: Vec<i32>) -> Vec<i32> {
        let mut windows = Vec::new();
        for i in 0..input.len() - 2 {
            windows.push(input[i] + input[i + 1] + input[i + 2]);
        }
        windows
    }
}

impl DayTwo {
    pub fn part_one(input: Vec<&str>) -> i32 {
        let mut horizontal = 0;
        let mut depth = 0;

        for line in input {
            let command = DayTwo::to_command(line);
            match command.r#type {
                CommandType::Forward => horizontal += command.value,
                CommandType::Up => depth -= command.value,
                CommandType::Down => depth += command.value,
            }
        }

        depth * horizontal
    }

    pub fn part_two(input: Vec<&str>) -> i32 {
        let mut horizontal = 0;
        let mut depth = 0;
        let mut aim = 0;

        for line in input {
            let command = DayTwo::to_command(line);
            match command.r#type {
                CommandType::Forward => {
                    horizontal += command.value;
                    depth += command.value * aim;
                }
                CommandType::Up => aim -= command.value,
                CommandType::Down => aim += command.value,
            }
        }

        depth * horizontal
    }

    fn to_command(input: &str) -> Command {
        let mut iter = input.split_whitespace();
        Command {
            r#type: match iter.next().unwrap() {
                "up" => CommandType::Up,
                "down" => CommandType::Down,
                "forward" => CommandType::Forward,
                _ => panic!("Unknown command"),
            },
            value: iter.next().unwrap().parse::<i32>().unwrap(),
        }
    }
}

impl DayThree {
    pub fn part_one(input: Vec<&str>) -> u64 {
        let mut gamma: u64 = 0;
        let mut epsilon: u64 = 0;
        let max_len: u64 = input.iter().map(|x| x.trim().len()).max().unwrap() as u64;
        let nums = input
            .iter()
            .map(|s| u64::from_str_radix(s.trim(), 2).unwrap())
            .collect::<Vec<u64>>();

        for i in 0..max_len {
            let mut set_count = 0;
            for n in &nums {
                if n & (1 << i) != 0 {
                    set_count += 1;
                }
            }

            if set_count * 2 > input.len() {
                gamma |= 1 << i;
            } else {
                epsilon |= 1 << i;
            }
        }

        gamma * epsilon
    }

    pub fn part_two(input: Vec<&str>) -> u64 {
        let oxygen: u64;
        let co_two: u64;
        let max_len: u64 = input.iter().map(|x| x.trim().len()).max().unwrap() as u64;
        let nums = input
            .iter()
            .map(|s| u64::from_str_radix(s.trim(), 2).unwrap())
            .collect::<Vec<u64>>();

        //for i in 0..max_len {
        oxygen = *DayThree::filter(nums.clone(), 0, max_len, true)
            .first()
            .unwrap();
        co_two = *DayThree::filter(nums.clone(), 0, max_len, false)
            .first()
            .unwrap();

        oxygen * co_two
    }

    fn filter(input: Vec<u64>, mask: u64, max: u64, most_common: bool) -> Vec<u64> {
        let mut result: Vec<u64> = input.clone();
        if mask != max - 1 {
            result = DayThree::filter(result, mask + 1, max, most_common);
        }

        if result.len() == 1 {
            return result;
        }

        let set: Vec<u64> = result
            .iter()
            .filter(|n| *n & (1 << mask) != 0)
            .map(|n| *n)
            .collect();

        let unset = result
            .iter()
            .filter(|n| *n & (1 << mask) == 0)
            .map(|n| *n)
            .collect();

        if set.len() * 2 >= result.len() as usize {
            if most_common {
                result = set;
            } else {
                result = unset;
            }
        } else {
            if most_common {
                result = unset;
            } else {
                result = set;
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_one_part_one() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(DayOne::part_one(input), 7);
    }

    #[test]
    fn test_day_one_part_two() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(DayOne::part_two(input), 5);
    }

    #[test]
    fn test_day_two_part_one() {
        let input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        assert_eq!(DayTwo::part_one(input), 150);
    }

    #[test]
    fn test_day_two_part_two() {
        let input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        assert_eq!(DayTwo::part_two(input), 900);
    }

    #[test]
    fn test_day_three_part_one() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!(DayThree::part_one(input), 198);
    }

    #[test]
    fn test_day_three_part_two() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!(DayThree::part_two(input), 230);
    }
}
