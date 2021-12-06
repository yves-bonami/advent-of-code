pub struct DayOne;
pub struct DayTwo;

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
}
