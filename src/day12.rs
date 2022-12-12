use std::collections::{HashMap, HashSet, VecDeque};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

pub struct Grid {
    nodes: Vec<Vec<char>>,
}

impl Grid {
    fn new(nodes: Vec<Vec<char>>) -> Self {
        Self { nodes }
    }

    fn neighbours(&self, from: Point) -> Vec<Point> {
        vec![
            self.up(from),
            self.down(from),
            self.left(from),
            self.right(from),
        ]
        .iter()
        .filter_map(|&p| p)
        .collect()
    }

    fn up(&self, from @ Point { x, y }: Point) -> Option<Point> {
        if x == 0 {
            return None;
        }
        self.should_move(from, Point { x: x - 1, y })
    }

    fn down(&self, from @ Point { x, y }: Point) -> Option<Point> {
        if x == self.nodes.len() - 1 {
            return None;
        }
        self.should_move(from, Point { x: x + 1, y })
    }

    fn left(&self, from @ Point { x, y }: Point) -> Option<Point> {
        if y == 0 {
            return None;
        }
        self.should_move(from, Point { x, y: y - 1 })
    }

    fn right(&self, from @ Point { x, y }: Point) -> Option<Point> {
        if y == self.nodes[0].len() - 1 {
            return None;
        }
        self.should_move(from, Point { x, y: y + 1 })
    }

    fn should_move(&self, from: Point, to: Point) -> Option<Point> {
        let from_value = self.nodes[from.x][from.y] as i32;
        let to_value = self.nodes[to.x][to.y] as i32;
        let value = if self.nodes[to.x][to.y] == 'E' {
            'z' as i32
        } else {
            self.nodes[to.x][to.y] as i32
        };
        (value <= from_value || (to_value > from_value && to_value - from_value == 1)).then_some(to)
    }

    fn find_a(&mut self) -> Vec<Point> {
        self.nodes
            .iter()
            .enumerate()
            .flat_map(|(rix, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &column)| column == 'S' || column == 'a')
                    .map(|(cix, _)| Point { x: rix, y: cix })
                    .collect::<Vec<Point>>()
            })
            .collect()
    }

    fn start(&mut self) -> Vec<Point> {
        let mut x = 0;
        let mut y = 0;
        for (row_index, row) in self.nodes.iter().enumerate() {
            x = row_index;
            if let Some(column_index) = row.iter().position(|&c| c == 'S') {
                y = column_index;
                break;
            }
        }
        self.nodes[x][y] = 'a';
        vec![Point { x, y }]
    }
}

pub fn part_one(data: &str) -> usize {
    let data: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let mut grid = Grid::new(data);
    let search = grid.start();
    execute(&grid, search)
}

pub fn part_two(data: &str) -> usize {
    let data: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let mut grid = Grid::new(data);
    let search = grid.find_a();
    execute(&grid, search)
}

pub fn execute(grid: &Grid, search: Vec<Point>) -> usize {
    let mut min_count = usize::MAX;
    for start in search {
        let mut discovered = HashSet::<Point>::new();
        let mut parents = HashMap::<Point, Point>::new();
        let mut queue = VecDeque::new();
        queue.push_front(start);
        discovered.insert(start);
        while !queue.is_empty() {
            let point = queue.pop_back().unwrap();
            if grid.nodes[point.x][point.y] == 'E' {
                if let Some(mut parent) = parents.get(&point) {
                    let mut count = 1;
                    while let Some(next) = parents.get(parent) {
                        count += 1;
                        parent = next;
                    }
                    min_count = usize::min(min_count, count);
                }
                break;
            }
            for neighbour in grid.neighbours(point) {
                if !discovered.contains(&neighbour) {
                    discovered.insert(neighbour);
                    queue.push_front(neighbour);
                    parents.insert(neighbour, point);
                }
            }
        }
    }
    min_count
}

#[cfg(test)]
mod test {

    #[test]
    fn test_example() {
        let data = include_str!("../resources/day12example");
        assert_eq!(super::part_one(data), 31);
    }

    #[test]
    fn test_part_one() {
        let data = include_str!("../resources/day12part1");
        assert_eq!(super::part_two(data), 321);
    }
}
