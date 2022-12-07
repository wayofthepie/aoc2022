use std::collections::HashMap;

const ROOT: &str = "|";
const SEPARATOR: &str = "/";

pub fn part_one(data: &str) -> isize {
    let dirs = parse_to_dirs(data);
    let sizes = get_path_sizes(&dirs);
    let space_marker = 100000;
    sizes
        .iter()
        .filter_map(|(_, &v)| (v <= space_marker).then_some(v))
        .sum::<isize>()
}

pub fn part_two(data: &str) -> isize {
    let disk_space: isize = 70000000;
    let needed_space: isize = 30000000;
    let dirs = parse_to_dirs(data);
    let sizes = get_path_sizes(&dirs);
    let required = needed_space - (disk_space - sizes.get(ROOT).unwrap());
    sizes
        .iter()
        .map(|(_, v)| v)
        .reduce(|prev, current| {
            if (current - required).abs() < (prev - required).abs() {
                current
            } else {
                prev
            }
        })
        .copied()
        .unwrap_or(0)
}

fn parse_to_dirs(data: &str) -> HashMap<String, isize> {
    let mut dirs = HashMap::<String, isize>::new();
    let mut current_dir = vec![ROOT];
    for line in data.lines() {
        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["$", "cd", "/"] => {
                dirs.entry(ROOT.to_owned()).or_insert(0);
            }
            ["$", "cd", ".."] => {
                current_dir.pop();
            }
            ["$", "cd", dir] => {
                current_dir.push(dir);
                dirs.entry(current_dir.join(SEPARATOR)).or_insert(0);
            }
            ["$", "ls"] | ["dir", _] => {}
            [size, _] => {
                let current_size = dirs.entry(current_dir.join(SEPARATOR)).or_insert(0);
                *current_size += size.parse::<isize>().unwrap();
            }
            _ => {}
        }
    }
    dirs
}

fn get_path_sizes(dirs: &HashMap<String, isize>) -> HashMap<String, isize> {
    let mut sizes = HashMap::<String, isize>::new();
    for (path, size) in dirs {
        let mut paths = path.split(SEPARATOR).collect::<Vec<&str>>();
        while !paths.is_empty() {
            *(sizes.entry(paths.join(SEPARATOR)).or_insert(0)) += size;
            paths.pop();
        }
    }
    sizes
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    #[test]
    fn test() {
        let data = include_str!("../resources/day7part1");
        let result = part_one(data);
        assert_eq!(result, 1517599);
    }

    #[test]
    fn test_part_two() {
        let data = include_str!("../resources/day7part1");
        let result = part_two(data);
        assert_eq!(result, 2481982);
    }
}
