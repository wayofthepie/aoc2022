pub fn execute(data: &str) -> Vec<isize> {
    let mut screen_rows = vec![String::new(); 6];
    let x = 1;
    let answer = data
        .lines()
        .fold((x, vec![x]), |(x, mut acc), line| {
            let instruction: Vec<&str> = line.split(' ').collect();
            match instruction.as_slice() {
                ["addx", num] => {
                    add_cycle(x, &mut acc, &mut screen_rows);
                    add_cycle(x, &mut acc, &mut screen_rows);
                    (x + num.parse::<isize>().unwrap(), acc)
                }
                _ => {
                    add_cycle(x, &mut acc, &mut screen_rows);
                    (x, acc)
                }
            }
        })
        .1;
    screen_rows.iter().for_each(|row| println!("{row}"));
    answer
}

fn add_cycle(x: isize, cycles: &mut Vec<isize>, screen_rows: &mut [String]) {
    let position = cycles.len() as isize % 40;
    let screen_index = (cycles.len() - 1) / 40;
    if x == position || x + 1 == position || x + 2 == position {
        screen_rows[screen_index].push('#');
    } else {
        screen_rows[screen_index].push('.');
    }
    cycles.push(x * cycles.len() as isize);
}

#[cfg(test)]
mod test {
    use crate::day10::execute;

    #[test]
    fn test_small_example() {
        let data = r#"noop
addx 3
addx -5"#;
        let result = execute(data);
        assert_eq!(result[5], 20);
    }

    #[test]
    fn test_example_at_20() {
        let data = include_str!("../resources/day10example");
        let result = execute(data);
        assert_eq!(result[20], 420);
    }

    #[test]
    fn test_example_at_60() {
        let data = include_str!("../resources/day10example");
        let result = execute(data);
        assert_eq!(result[60], 1140);
    }

    #[test]
    fn test_example_at_100() {
        let data = include_str!("../resources/day10example");
        let result = execute(data);
        assert_eq!(result[100], 1800);
    }

    #[test]
    fn test_example_at_140() {
        let data = include_str!("../resources/day10example");
        let result = execute(data);
        assert_eq!(result[140], 2940);
    }

    #[test]
    fn test_example_at_180() {
        let data = include_str!("../resources/day10example");
        let result = execute(data);
        assert_eq!(result[180], 2880);
    }

    #[test]
    fn test_example_at_220() {
        let data = include_str!("../resources/day10example");
        let result = execute(data);
        assert_eq!(result[220], 3960);
    }

    #[test]
    fn test_example() {
        let data = include_str!("../resources/day10example");
        let result = execute(data);
        let answer =
            result[20] + result[60] + result[100] + result[140] + result[180] + result[220];
        assert_eq!(answer, 13140);
    }

    #[test]
    fn test_part_one() {
        let data = include_str!("../resources/day10part1");
        let result = execute(data);
        let answer =
            result[20] + result[60] + result[100] + result[140] + result[180] + result[220];
        assert_eq!(answer, 13680);
    }
}
