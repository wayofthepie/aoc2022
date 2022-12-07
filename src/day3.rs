use std::collections::HashSet;

pub fn part_one(data: &str) -> u32 {
    data.lines().fold(0, |acc, line| {
        let (first, second) = line.split_at(line.len() / 2);
        let first = HashSet::<char>::from_iter(first.chars());
        let second = HashSet::<char>::from_iter(second.chars());
        acc + first.intersection(&second).map(char_to_digit).sum::<u32>()
    })
}

pub fn part_two(data: &str) -> u32 {
    let mut sum = 0;
    let mut lines = data.lines();
    while let (Some(first), Some(second), Some(third)) = (lines.next(), lines.next(), lines.next())
    {
        let first = HashSet::<char>::from_iter(first.chars());
        let second = HashSet::<char>::from_iter(second.chars());
        let third = HashSet::<char>::from_iter(third.chars());
        let badge: HashSet<char> = first.intersection(&second).copied().collect();
        let badge: HashSet<char> = badge.intersection(&third).copied().collect();
        sum += badge.iter().map(char_to_digit).sum::<u32>()
    }
    sum
}

fn char_to_digit(c: &char) -> u32 {
    if c.is_lowercase() {
        *c as u32 - 96
    } else {
        *c as u32 - 38
    }
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let data = include_str!("../resources/day3part1").trim();
        let result = part_one(data);
        assert_eq!(result, 8243)
    }

    #[test]
    fn test_part_two() {
        let data = include_str!("../resources/day3part1").trim();
        let result = part_two(data);
        assert_eq!(result, 2631)
    }
}
