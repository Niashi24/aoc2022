use std::cmp::Ordering;
use std::collections::HashSet;
use std::ops::Add;
use std::str::FromStr;
use crate::day::Day;

pub struct Day18;

pub struct Info {
    points: Vec<Point>
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Point(i8, i8, i8);

impl Point {
    #[inline]
    pub fn min_component(&self, other: &Self) -> Self {
        Point(self.0.min(other.0), self.1.min(other.1), self.2.min(other.2))
    }
    #[inline]
    pub fn max_component(&self, other: &Self) -> Self {
        Point(self.0.max(other.0), self.1.max(other.1), self.2.max(other.2))
    }
}

const SIDES: [Point; 6] = [
    Point(1, 0, 0),  // right
    Point(0, 1, 0),  // up
    Point(0, 0, 1),  // forward
    Point(-1, 0, 0), // left
    Point(0, -1, 0), // down
    Point(0, 0, -1)  // back
];

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<i8> = s
            .split(',')
            .map(|part| part.trim().parse::<i8>())
            .collect::<Result<Vec<i8>, _>>()
            .map_err(|_| ())?;

        if values.len() != 3 {
            return Err(());
        }

        Ok(Point(values[0], values[1], values[2]))
    }
}

impl Day<Info> for Day18 {
    fn parse_file(&self, file_content: String) -> Info {
        Info {
            points: file_content.lines().map(|x| x.trim().parse().unwrap()).collect()
        }
    }

    fn part_1(&self, data: &Info) -> usize {
        let point_set: HashSet<Point> = HashSet::from_iter(data.points.clone().into_iter());
        
        let mut sum = 0;
        
        for point in point_set.iter() {
            for side in SIDES.iter() {
                if !point_set.contains(&(*point + *side)) {
                    sum += 1;
                }
            }
        }
        
        sum
    }

    fn part_2(&self, data: &Info) -> usize {
        let solid_set: HashSet<Point> = HashSet::from_iter(data.points.clone().into_iter());
        
        #[inline]
        fn get_max_min(points: &Vec<Point>) -> (Point, Point) {
            if points.len() == 0 {
                return (Point(0, 0, 0), Point(0, 0, 0));
            }
            
            let mut min = Point(std::i8::MAX, std::i8::MAX, std::i8::MAX);
            let mut max = Point(std::i8::MIN, std::i8::MIN, std::i8::MIN);
            
            for point in points {
                min = min.min_component(point);
                max = max.max_component(point);
            }

            (min, max)
        }
        
        let mut visited = HashSet::new();
        let mut visitable_sides = 0;
        let (min, max) = get_max_min(&data.points);
        let (min, max) = (min + Point(-1, -1, -1), max + Point(1, 1, 1));
        
        let mut visiting_stack = Vec::new();
        visiting_stack.push(min.clone());
        
        while let Some(point) = visiting_stack.pop() {
            if solid_set.contains(&point) {
                visitable_sides += 1;
                continue;
            }
            if visited.contains(&point) {
                continue;
            }

            visited.insert(point.clone());

            #[inline]
            fn can_move_to(point: &Point, visited: &HashSet<Point>, min: &Point, max: &Point) -> bool {
                !visited.contains(point)
                && point.0 >= min.0 && point.1 >= min.1 && point.2 >= min.2
                && point.0 <= max.0 && point.1 <= max.1 && point.2 <= max.2
            }

            for side in SIDES.iter() {
                let new_pos = point + *side;
                if can_move_to(&new_pos, &visited, &min, &max) {
                    visiting_stack.push(new_pos);
                }
            }
        }

        visitable_sides
    }
}