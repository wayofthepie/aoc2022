pub fn part_one(data: &str) -> usize {
    data.lines()
        .map(parse_to_vec)
        .filter(|nums: &Vec<usize>| is_fully_overlapping(nums))
        .count()
}

pub fn part_two(data: &str) -> usize {
    data.lines()
        .map(parse_to_vec)
        .filter(|nums: &Vec<usize>| is_overlapping(nums))
        .count()
}

fn parse_to_vec(line: &str) -> Vec<usize> {
    line.split(',')
        .flat_map(|line| line.split('-').collect::<Vec<&str>>())
        .map(|s| s.parse().unwrap())
        .collect()
}

fn is_fully_overlapping(nums: &[usize]) -> bool {
    (nums[0]..=nums[1]).contains(&nums[2]) && (nums[0]..=nums[1]).contains(&nums[3])
        || (nums[2]..=nums[3]).contains(&nums[0]) && (nums[2]..=nums[3]).contains(&nums[1])
}

fn is_overlapping(nums: &[usize]) -> bool {
    (nums[0]..=nums[1]).contains(&nums[2])
        || (nums[0]..=nums[1]).contains(&nums[3])
        || (nums[2]..=nums[3]).contains(&nums[0])
        || (nums[2]..=nums[3]).contains(&nums[1])
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let data = include_str!("../resources/day3part1").trim();
        let result = part_one(data);
        assert_eq!(result, 547);
    }

    #[test]
    fn test_part_two() {
        let data = include_str!("../resources/day3part1").trim();
        let result = part_two(data);
        assert_eq!(result, 843);
    }
}
