use std::cmp::Ordering;

pub fn part_one(data: &str) -> usize {
    let data = build_data(data);
    let rows = data.len();
    let columns = data[0].len();

    let mut state = vec![vec![false; columns]; rows];
    find_top_and_left(
        &data,
        &mut state,
        &(0..rows).collect::<Vec<usize>>(),
        &(0..columns).collect::<Vec<usize>>(),
    );
    find_top_and_left(
        &data,
        &mut state,
        &(0..rows).rev().collect::<Vec<usize>>(),
        &(0..columns).rev().collect::<Vec<usize>>(),
    );
    state.iter().flatten().filter(|&&b| b).count()
}

struct World {
    grid: Vec<usize>,
    width: usize,
    height: usize,
}

impl World {
    fn new(width: usize, grid: Vec<usize>) -> Self {
        let height = grid.len() / width;
        Self {
            grid,
            width,
            height,
        }
    }

    fn get(&self, (x, y): (usize, usize)) -> Option<usize> {
        let index = x * self.width + y;
        self.grid.get(index).copied()
    }

    fn visibility_up(&self, position @ (x, _): (usize, usize)) -> usize {
        visibility_x(self, position, x + 1..self.height)
    }

    fn visibility_down(&self, position @ (x, _): (usize, usize)) -> usize {
        visibility_x(self, position, (0..x).rev())
    }

    fn visibility_left(&self, position @ (_, y): (usize, usize)) -> usize {
        visibility_y(self, position, (0..y).rev())
    }

    fn visibility_right(&self, position @ (_, y): (usize, usize)) -> usize {
        visibility_y(self, position, y + 1..self.width)
    }
}

fn visibility_x(world: &World, from: (usize, usize), iter: impl Iterator<Item = usize>) -> usize {
    let y = from.1;
    let value = world.get(from);
    let mut visibility = 0;
    for x in iter {
        let other = world.get((x, y));
        match value.cmp(&other) {
            Ordering::Greater => visibility += 1,
            Ordering::Equal | Ordering::Less => {
                visibility += 1;
                break;
            }
        }
    }
    visibility
}

fn visibility_y(world: &World, from: (usize, usize), iter: impl Iterator<Item = usize>) -> usize {
    let x = from.0;
    let value = world.get(from);
    let mut visibility = 0;
    for y in iter {
        let other = world.get((x, y));
        match value.cmp(&other) {
            Ordering::Greater => visibility += 1,
            Ordering::Equal | Ordering::Less => {
                visibility += 1;
                break;
            }
        }
    }
    visibility
}

pub fn part_two(data: &str) -> usize {
    let mut max = 0;
    let (width, data) = build_data_two(data);
    let world = World::new(width, data);
    for x in 0..world.height {
        for y in 0..width {
            let tree = (x, y);
            let visibility = world.visibility_up(tree)
                * world.visibility_down(tree)
                * world.visibility_left(tree)
                * world.visibility_right(tree);
            max = usize::max(max, visibility);
        }
    }
    max
}

fn build_data_two(data: &str) -> (usize, Vec<usize>) {
    let mut width = 0;
    let nums = data
        .lines()
        .flat_map(|line| {
            width = line.len();
            line.chars()
                .into_iter()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .rev()
        })
        .rev()
        .collect();
    (width, nums)
}

fn build_data(data: &str) -> Vec<Vec<usize>> {
    data.lines()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn find_top_and_left(
    data: &[Vec<usize>],
    state: &mut [Vec<bool>],
    rows_range: &[usize],
    columns: &[usize],
) {
    let mut above = data[rows_range[0]].clone();
    for &rix in rows_range {
        let row = data.get(rix).unwrap();
        let mut max = row[columns[0]];
        for &cix in columns {
            let column = row[cix];
            if rix == 0 || cix == 0 || rix == data.len() - 1 || cix == row.len() - 1 {
                state[rix][cix] = true;
                continue;
            }
            if column > max {
                state[rix][cix] = true;
                max = column;
            }
            if column > above[cix] {
                state[rix][cix] = true;
                above[cix] = column;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};
    use crate::day8::{build_data_two, World};

    #[test]
    fn test_example_part_one() {
        let data = r#"
30373
25512
65332
33549
35390"#
            .trim();
        let result = part_one(data);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part_one() {
        let data = include_str!("../resources/day8part1").trim();
        let result = part_one(data);
        assert_eq!(result, 1695);
    }

    #[test]
    fn test_example_part_two() {
        let data = r#"
30373
25512
65332
33549
35390"#
            .trim();
        let result = part_two(data);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_two() {
        let data = include_str!("../resources/day8part1").trim();
        let result = part_two(data);
        assert_eq!(result, 287040);
    }

    #[test]
    fn world_should_have_correct_visibility_up() {
        let data = r#"
30373
25512
65332
33549
35390"#
            .trim();
        let (width, data) = build_data_two(data);
        let world = World::new(width, data);
        let result = world.visibility_up((1, 2));
        assert_eq!(result, 2);
    }

    #[test]
    fn world_should_have_correct_visibility_left() {
        let data = r#"
30373
25512
65332
33549
35390"#
            .trim();
        let (width, data) = build_data_two(data);
        let world = World::new(width, data);
        let result = world.visibility_left((1, 2));
        assert_eq!(result, 2);
    }

    #[test]
    fn world_should_have_correct_visibility_right() {
        let data = r#"
30373
25512
65332
33549
35390"#
            .trim();
        let (width, data) = build_data_two(data);
        let world = World::new(width, data);
        let result = world.visibility_right((1, 2));
        assert_eq!(result, 2);
    }

    #[test]
    fn world_should_have_correct_visibility_down() {
        let data = r#"
30373
25512
65332
33549
35390"#
            .trim();
        let (width, data) = build_data_two(data);
        let world = World::new(width, data);
        let result = world.visibility_down((1, 2));
        assert_eq!(result, 1);
    }
}
