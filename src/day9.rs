use std::collections::HashSet;

enum Move {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

pub fn execute(rope: &mut Rope, data: &str) -> usize {
    let mut moves = Vec::new();
    let mut visits: HashSet<Point> = HashSet::new();
    for line in data.lines() {
        match &line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["U", num] => moves.push(Move::Up(num.parse::<usize>().unwrap())),
            ["D", num] => moves.push(Move::Down(num.parse::<usize>().unwrap())),
            ["L", num] => moves.push(Move::Left(num.parse::<usize>().unwrap())),
            ["R", num] => moves.push(Move::Right(num.parse::<usize>().unwrap())),
            _ => unreachable!(),
        }
    }
    for mv in moves {
        visits = visits.union(&rope.mv(mv)).copied().collect();
    }
    visits.len()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct Rope {
    head: Point,
    tail: Point,
}

impl Rope {
    fn new(head: Point, tail: Point) -> Self {
        Self { head, tail }
    }

    fn mv(&mut self, mv: Move) -> HashSet<Point> {
        match mv {
            Move::Up(moves) => self.move_up(moves),
            Move::Down(moves) => self.move_down(moves),
            Move::Left(moves) => self.move_left(moves),
            Move::Right(moves) => self.move_right(moves),
        }
    }

    fn move_up(&mut self, moves: usize) -> HashSet<Point> {
        let mut tail_moves = HashSet::new();
        for _ in 0..moves {
            self.head.x += 1;
            let diff_x = self.head.x.abs_diff(self.tail.x);
            let diff_y = self.head.y.abs_diff(self.tail.y);
            match (diff_x, diff_y) {
                (2, 0) => {
                    self.tail.x += 1;
                }
                (2, y) if y == 1 || y == 2 => {
                    self.tail.x += 1;
                    self.tail.y = self.head.y;
                }
                _ => {}
            }
            tail_moves.insert(self.tail);
        }
        tail_moves
    }

    fn move_down(&mut self, moves: usize) -> HashSet<Point> {
        let mut tail_moves = HashSet::new();
        for _ in 0..moves {
            self.head.x -= 1;
            let diff_x = self.head.x.abs_diff(self.tail.x);
            let diff_y = self.head.y.abs_diff(self.tail.y);
            match (diff_x, diff_y) {
                (2, 0) => {
                    self.tail.x -= 1;
                }
                (2, y) if y == 1 || y == 2 => {
                    self.tail.x -= 1;
                    self.tail.y = self.head.y;
                }
                _ => {}
            }
            tail_moves.insert(self.tail);
        }
        tail_moves
    }

    fn move_left(&mut self, moves: usize) -> HashSet<Point> {
        let mut tail_moves = HashSet::new();
        for _ in 0..moves {
            self.head.y -= 1;
            let diff_x = self.head.x.abs_diff(self.tail.x);
            let diff_y = self.head.y.abs_diff(self.tail.y);
            match (diff_y, diff_x) {
                (2, 0) => {
                    self.tail.y -= 1;
                }
                (2, x) if x == 1 || x == 2 => {
                    self.tail.y -= 1;
                    self.tail.x = self.head.x;
                }
                _ => {}
            }
            tail_moves.insert(self.tail);
        }
        tail_moves
    }

    fn move_right(&mut self, moves: usize) -> HashSet<Point> {
        let mut tail_moves = HashSet::new();
        for _ in 0..moves {
            self.head.y += 1;
            let diff_x = self.head.x.abs_diff(self.tail.x);
            let diff_y = self.head.y.abs_diff(self.tail.y);
            match (diff_y, diff_x) {
                (2, 0) => self.tail.y += 1,
                (2, x) if x == 1 || x == 2 => {
                    self.tail.y += 1;
                    self.tail.x = self.head.x;
                }
                _ => {}
            }
            tail_moves.insert(self.tail);
        }
        tail_moves
    }
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::day9::{Point, Rope};

    #[test]
    fn tail_should_move_correctly_when_going_right_from_start() {
        let moves = r#"R 4"#;
        let mut rope = Rope::new(Point::new(0, 0), Point::new(0, 0));
        let result = execute(&mut rope, moves);
        assert_eq!(result, 4);
    }

    #[test]
    fn tail_should_move_correctly_when_diagonal_to_right() {
        let moves = r#"R 4"#;
        let mut rope = Rope::new(Point::new(2, 2), Point::new(1, 1));
        let result = execute(&mut rope, moves);
        println!("{rope:?}");
        assert_eq!(result, 4);
    }

    #[test]
    fn tail_should_move_correctly_when_going_up() {
        let moves = r#"U 4"#;
        let mut rope = Rope::new(Point::new(0, 0), Point::new(0, 0));
        let result = execute(&mut rope, moves);
        assert_eq!(result, 4);
    }

    #[test]
    fn tail_should_move_correctly_when_going_up_diagonal() {
        let moves = r#"U 4"#;
        let mut rope = Rope::new(Point::new(2, 2), Point::new(1, 1));
        let result = execute(&mut rope, moves);
        assert_eq!(result, 4);
    }

    #[test]
    fn tail_should_move_correctly_when_going_left() {
        let moves = r#"L 4"#;
        let mut rope = Rope::new(Point::new(2, 8), Point::new(2, 9));
        let result = execute(&mut rope, moves);
        assert_eq!(result, 4);
    }

    #[test]
    fn tail_should_move_correctly_when_going_left_diagonal() {
        let moves = r#"L 4"#;
        let mut rope = Rope::new(Point::new(2, 8), Point::new(3, 9));
        let result = execute(&mut rope, moves);
        assert_eq!(result, 4);
    }

    #[test]
    fn tail_should_move_correctly_when_going_down() {
        let moves = r#"D 4"#;
        let mut rope = Rope::new(Point::new(5, 0), Point::new(5, 0));
        let result = execute(&mut rope, moves);
        assert_eq!(result, 4);
    }

    #[test]
    fn tail_should_move_correctly_when_going_down_diagonal() {
        let moves = r#"D 4"#;
        let mut rope = Rope::new(Point::new(5, 0), Point::new(6, 1));
        let result = execute(&mut rope, moves);
        assert_eq!(result, 4);
    }

    #[test]
    fn part_one_example() {
        let moves = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
        let mut rope = Rope::new(Point::new(0, 0), Point::new(0, 0));
        let result = execute(&mut rope, moves);
        assert_eq!(result, 13);
    }

    #[test]
    fn part_one() {
        let moves = include_str!("../resources/day9part1");
        let mut rope = Rope::new(Point::new(0, 0), Point::new(0, 0));
        let result = execute(&mut rope, moves);
        assert_eq!(result, 13);
    }
}
