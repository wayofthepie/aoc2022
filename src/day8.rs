pub fn part_one(data: &str) -> usize {
    let data = build_data(data);
    let rows = data.len();
    let columns = data[0].len();

    let count = (rows * 2) + ((columns * 2) - 4);
    let mut state = vec![vec![false; columns]; rows];
    find_top_and_left(
        &data,
        &mut state,
        &(1..rows - 1).collect::<Vec<usize>>(),
        &(1..columns - 1).collect::<Vec<usize>>(),
        data[0].clone(),
        0,
    );
    find_top_and_left(
        &data,
        &mut state,
        &(1..rows - 1).rev().collect::<Vec<usize>>(),
        &(1..columns - 1).rev().collect::<Vec<usize>>(),
        data[rows - 1].clone(),
        rows - 1,
    );
    println!("{state:?}");
    count + state.iter().flatten().filter(|&&b| b).count()
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
    mut above: Vec<usize>,
    row_start: usize,
) {
    for &rix in rows_range {
        let row = data.get(rix).unwrap();
        let mut max = row[row_start];
        for &cix in columns {
            let column = row[cix];
            if column > max {
                println!("left (max {max}) {rix},{cix} {column}");
                state[rix][cix] = true;
                max = column;
            }
            if column > above[cix] {
                println!("above (above {above:?}) {rix},{cix} {column}");
                state[rix][cix] = true;
                above[cix] = column;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::part_one;
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
        println!("{}", data);
        assert_eq!(result, 20);
    }

    #[test]
    fn test_part_one() {
        let data = include_str!("../resources/day8part1").trim();
        let result = part_one(data);
        assert_eq!(result, 1695);
    }
}
