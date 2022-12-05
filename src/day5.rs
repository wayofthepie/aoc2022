use std::{error::Error, str::Lines};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn execute(data: &str, crane: impl Crane) -> Result<Vec<char>> {
    let mut lines = data.lines();
    let mut cranes = parse_cranes(&mut lines, 9);
    parse_instructions(&mut lines)
        .iter()
        .try_for_each(|instruction| crane.mv(&mut cranes, instruction))?;
    Ok(cranes
        .into_iter()
        .filter_map(|mut crane| crane.pop())
        .collect())
}

pub trait Crane {
    fn get(&self, start: usize, from_crane: &mut Vec<char>) -> Vec<char>;

    fn mv(&self, cranes: &mut [Vec<char>], instruction: &[usize]) -> Result<()> {
        let &[amount, from, to]  = &instruction[..=2] else {
            return Err("Instruction was in a bad format!".into())
        };
        let items: Vec<char> = {
            let from_crane = cranes
                .get_mut(from)
                .ok_or("Error getting crane sending boxes!")?;
            let start = from_crane.len() - amount;
            self.get(start, from_crane)
        };
        let to_crane = cranes
            .get_mut(to)
            .ok_or("Error getting crane receiving boxes!")?;
        items.iter().for_each(|&item| to_crane.push(item));
        Ok(())
    }
}

struct CraneV1;

impl Crane for CraneV1 {
    fn get(&self, start: usize, from_crane: &mut Vec<char>) -> Vec<char> {
        from_crane.drain(start..).rev().collect()
    }
}

struct CraneV2;

impl Crane for CraneV2 {
    fn get(&self, start: usize, from_crane: &mut Vec<char>) -> Vec<char> {
        from_crane.drain(start..).collect()
    }
}

fn parse_cranes(lines: &mut Lines, num_cranes: usize) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![vec![]; num_cranes];
    for line in lines.skip(1) {
        if line.contains('1') {
            break;
        }
        let mut chars = line.chars();
        let mut stack_index = 0;
        while let (_, Some(letter), _) = (chars.next(), chars.next(), chars.next()) {
            if (letter).is_alphabetic() {
                stacks.get_mut(stack_index).unwrap().push(letter);
            }
            chars.next();
            stack_index += 1;
        }
    }
    for stack in stacks.iter_mut() {
        stack.reverse();
    }
    stacks
}

fn parse_instructions(lines: &mut Lines) -> Vec<Vec<usize>> {
    lines
        .filter(|line| line.chars().next().map(|c| c == 'm').is_some())
        .fold(Vec::new(), |mut acc, line| {
            let nums = line
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            acc.push(nums);
            acc
        })
        .into_iter()
        .map(|mut instruction| {
            instruction[1] -= 1;
            instruction[2] -= 1;
            instruction
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::day5::{execute, CraneV1, CraneV2};

    #[test]
    fn test_part_one() {
        let data = include_str!("../resources/day5part1").trim();
        let result = execute(data, CraneV1).unwrap();
        assert_eq!(String::from_iter(result), "FZCMJCRHZ");
    }

    #[test]
    fn test_part_two() {
        let data = include_str!("../resources/day5part1").trim();
        let result = execute(data, CraneV2).unwrap();
        assert_eq!(String::from_iter(result), "JSDHQMZGF");
    }
}
