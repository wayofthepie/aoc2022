pub fn protocol(data: &str, num: usize) -> usize {
    let mut seen = Vec::new();
    for (index, c) in data.chars().enumerate() {
        if seen.len() == num {
            return index;
        }
        seen.iter()
            .position(|&ch| ch == c)
            .map(|position| seen.drain(0..=position));
        seen.push(c);
    }
    0
}

#[cfg(test)]
mod test {
    use super::protocol;

    #[test]
    fn test_part_one() {
        let data = include_str!("../resources/day6part1").trim();
        let result = protocol(data, 4);
        assert_eq!(result, 1578)
    }

    #[test]
    fn test_part_two() {
        let data = include_str!("../resources/day6part1").trim();
        let result = protocol(data, 14);
        assert_eq!(result, 2178)
    }
}
