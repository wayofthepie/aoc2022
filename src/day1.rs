use std::cmp::Reverse;

pub fn part_one(data: &str) -> Option<usize> {
    let mut elves = get_elves(data);
    elves.sort_unstable();
    elves.pop()
}

pub fn part_two(data: &str) -> usize {
    let mut elves = get_elves(data);
    elves.sort_by_key(|w| Reverse(*w));
    elves.iter().take(3).sum()
}

fn get_elves(data: &str) -> Vec<usize> {
    let mut elves = Vec::new();
    let mut current = 0;
    for line in data.lines() {
        current = line.parse::<usize>().map_or(
            {
                elves.push(current);
                0
            },
            |calories| current + calories,
        );
    }
    elves
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let data = include_str!("../resources/day1part1.txt").trim();
        let result = part_one(data).unwrap();
        assert_eq!(70296, result);
    }

    #[test]
    fn test_part_two() {
        let data = include_str!("../resources/day1part1.txt").trim();
        let result = part_two(data);
        assert_eq!(205381, result)
    }
}
