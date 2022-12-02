use std::collections::HashMap;

pub fn part_one(data: &str) -> usize {
    let scores: HashMap<&str, usize> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let encoding: HashMap<(&str, &str), usize> = HashMap::from([
        (("A", "Y"), 8),
        (("B", "Z"), 9),
        (("C", "X"), 7),
        (("A", "Z"), 3),
        (("B", "X"), 1),
        (("C", "Y"), 2),
    ]);
    compute(data, &encoding, &scores, 1)
}

pub fn part_two(data: &str) -> usize {
    let scores: HashMap<&str, usize> = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
    let encoding: HashMap<(&str, &str), usize> = HashMap::from([
        (("A", "Z"), 8),
        (("B", "Z"), 9),
        (("C", "Z"), 7),
        (("A", "X"), 3),
        (("B", "X"), 1),
        (("C", "X"), 2),
    ]);
    compute(data, &encoding, &scores, 0)
}

fn compute(
    data: &str,
    encoding: &HashMap<(&str, &str), usize>,
    scores: &HashMap<&str, usize>,
    game_index: usize,
) -> usize {
    let mut score = 0;
    for line in data.lines() {
        let game = line.splitn(2, ' ').collect::<Vec<&str>>();
        score += match encoding.get(&(game[0], game[1])) {
            Some(score) => *score,
            _ => scores.get(game[game_index]).unwrap() + 3,
        }
    }
    score
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let data = include_str!("../resources/day2part1").trim();
        let result = part_one(data);
        assert_eq!(result, 13675);
    }

    #[test]
    fn test_part_two() {
        let data = include_str!("../resources/day2part1").trim();
        let result = part_two(data);
        assert_eq!(result, 14184);
    }
}
