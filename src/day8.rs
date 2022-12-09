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
    println!("{state:?}");
    state.iter().flatten().filter(|&&b| b).count()
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
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part_one() {
        let data = include_str!("../resources/day8part1").trim();
        let result = part_one(data);
        assert_eq!(result, 1695);
    }
}
