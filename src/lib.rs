pub struct DayOne;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_one() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(DayOne::part_one(input), 7);
    }

    #[test]
    fn test_day_two() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(DayOne::part_two(input), 5);
    }
}
