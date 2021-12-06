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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_one() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(DayOne::part_one(input), 7);
    }
}
