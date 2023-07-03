use std::str::FromStr;
use crate::day::Day;

pub struct Day18;

#[derive(Debug)]
pub struct Info {
    points: Vec<Point>
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point(u8, u8, u8);

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<u8> = s
            .split(',')
            .map(|part| part.trim().parse::<u8>())
            .collect::<Result<Vec<u8>, _>>()
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
        dbg!(data);
        todo!()
    }

    fn part_2(&self, data: &Info) -> usize {
        todo!()
    }
}