use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

struct World {
    points: Vec<Point>,
}

impl World {
    fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    fn surface_area(&mut self) -> usize {
        let map_x = Box::new(
            |point: &Point, map: &mut HashMap<usize, Vec<(usize, usize)>>| {
                map.entry(point.x).or_default().push((point.y, point.z));
            },
        );
        let map_y = Box::new(
            |point: &Point, map: &mut HashMap<usize, Vec<(usize, usize)>>| {
                map.entry(point.y).or_default().push((point.x, point.z));
            },
        );
        let map_z = Box::new(
            |point: &Point, map: &mut HashMap<usize, Vec<(usize, usize)>>| {
                map.entry(point.z).or_default().push((point.x, point.y));
            },
        );
        type MapperFn = Box<dyn Fn(&Point, &mut HashMap<usize, Vec<(usize, usize)>>)>;
        let workflow: Vec<MapperFn> = vec![map_x, map_y, map_z];
        let duplicates = workflow.iter().fold(0, |acc, mapper| {
            let mut map: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
            self.points.iter().for_each(|point| mapper(point, &mut map));
            acc + self.get_intersecting_sides(map)
        });
        self.points.len() * 6 - duplicates
    }

    fn get_intersecting_sides(&self, map: HashMap<usize, Vec<(usize, usize)>>) -> usize {
        let mut sum = 0;
        let mut map = map.iter().collect::<Vec<(&usize, &Vec<(usize, usize)>)>>();
        map.sort();
        map.iter().reduce(|(_, prev_axes), next @ (_, axes)| {
            let prev: HashSet<&(usize, usize)> = HashSet::from_iter(prev_axes.iter());
            let current: HashSet<&(usize, usize)> = HashSet::from_iter(axes.iter());
            sum += prev.intersection(&current).count() * 2;
            next
        });
        sum
    }
}

pub fn execute(data: &str) -> usize {
    let points = data
        .lines()
        .map(|line| {
            let point_vec: Vec<usize> = line
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect();
            Point::new(point_vec[0], point_vec[1], point_vec[2])
        })
        .collect::<Vec<Point>>();
    let mut world = World::new(points);
    world.surface_area()
}

#[cfg(test)]
mod test {
    use super::{execute, Point, World};

    #[test]
    fn example() {
        let data = include_str!("../resources/day18example");
        let result = execute(data);
        assert_eq!(result, 64);
    }

    #[test]
    fn small_example() {
        let points = vec![Point::new(1, 1, 1), Point::new(2, 1, 1)];
        let mut world = World::new(points);
        assert_eq!(world.surface_area(), 10);
    }

    #[test]
    fn part_one() {
        let data = include_str!("../resources/day18");
        let result = execute(data);
        assert_eq!(result, 64);
    }
}
