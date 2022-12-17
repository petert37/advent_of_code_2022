use std::{fs, ops::RangeInclusive};

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::tuple,
    IResult, Parser,
};

#[derive(Debug)]
struct Sensor {
    sensor_x: i32,
    sensor_y: i32,
    beacon_x: i32,
    beacon_y: i32,
}

fn manhattan_distance(x1: &i32, y1: &i32, x2: &i32, y2: &i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

impl Sensor {
    fn get_beacon_distance(&self) -> i32 {
        manhattan_distance(
            &self.sensor_x,
            &self.sensor_y,
            &self.beacon_x,
            &self.beacon_y,
        )
    }

    fn get_sensor_range(&self, y: &i32) -> Option<RangeInclusive<i32>> {
        let y_distance = (self.sensor_y - y).abs();
        let beacon_distance = self.get_beacon_distance();
        if y_distance > beacon_distance {
            return None;
        }
        let distance = beacon_distance - y_distance;
        let start = self.sensor_x - distance;
        let end = self.sensor_x + distance;
        if start <= end {
            Some(start..=end)
        } else {
            None
        }
    }

    fn get_non_beacon_ranges(&self, y: i32) -> Vec<RangeInclusive<i32>> {
        if let Some(range) = self.get_sensor_range(&y) {
            if self.beacon_y == y {
                [
                    *range.start()..=(self.beacon_x - 1),
                    (self.beacon_x + 1)..=*range.end(),
                ]
                .into_iter()
                .filter(|range| range.start() <= range.end())
                .collect()
            } else {
                vec![range]
            }
        } else {
            vec![]
        }
    }
}

fn parse_sensors(input: &str) -> IResult<&str, Vec<Sensor>> {
    separated_list1(
        line_ending,
        tuple((
            tag("Sensor at x="),
            complete::i32,
            tag(", y="),
            complete::i32,
            tag(": closest beacon is at x="),
            complete::i32,
            tag(", y="),
            complete::i32,
        ))
        .map(
            |(_, sensor_x, _, sensor_y, _, beacon_x, _, beacon_y)| Sensor {
                sensor_x,
                sensor_y,
                beacon_x,
                beacon_y,
            },
        ),
    )(input)
}

fn parse_sensors_from_filename(filename: &str) -> Vec<Sensor> {
    let input = fs::read_to_string(format!("src/day15/{}", filename)).unwrap();
    let (_, sensors) = parse_sensors(&input).unwrap();
    sensors
}

fn merge_ranges(ranges: Vec<RangeInclusive<i32>>) -> Vec<RangeInclusive<i32>> {
    let mut ranges = ranges;
    ranges.sort_by(|r1, r2| r1.start().cmp(r2.start()));
    let mut result: Vec<RangeInclusive<i32>> = vec![];
    for range in ranges {
        if let Some(last_range) = result.pop() {
            if range.start() <= &(last_range.end() + 1) {
                result.push(*last_range.start()..=*range.end().max(last_range.end()));
            } else {
                result.push(last_range);
                result.push(range.clone());
            }
        } else {
            result.push(range.clone());
        }
    }
    result
}

fn get_non_beacon_position_count(sensors: &Vec<Sensor>, row: i32) -> i32 {
    let ranges = sensors
        .iter()
        .flat_map(|sensor| sensor.get_non_beacon_ranges(row))
        .collect::<Vec<RangeInclusive<i32>>>();
    let merged = merge_ranges(ranges);
    merged.into_iter().map(|range| range.count() as i32).sum()
}

fn get_not_covered_ranges_in_range(sensors: &Vec<Sensor>, y_max: i32) -> Option<(i32, i32)> {
    for y in 0..=y_max {
        let ranges = sensors
            .iter()
            .filter_map(|sensor| sensor.get_sensor_range(&y))
            .collect::<Vec<RangeInclusive<i32>>>();
        let merged = merge_ranges(ranges);
        if merged.len() == 2 {
            let first = &merged[0];
            let x = first.end() + 1;
            if x > 0 {
                return Some((x, y));
            }
        }
    }
    None
}

fn get_tuning_frequency(sensors: &Vec<Sensor>, y_max: i32) -> i64 {
    if let Some((x, y)) = get_not_covered_ranges_in_range(sensors, y_max) {
        (x as i64) * 4_000_000 + (y as i64)
    } else {
        -1
    }
}

fn main() {
    let sensors = parse_sensors_from_filename("input.txt");
    println!(
        "Non beacon position count: {}",
        get_non_beacon_position_count(&sensors, 2_000_000)
    );
    println!(
        "Tuning frequency: {}",
        get_tuning_frequency(&sensors, 4_000_000)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let sensors = parse_sensors_from_filename("test.txt");
        assert_eq!(get_non_beacon_position_count(&sensors, 10), 26);
    }

    #[test]
    fn test_part_2() {
        let sensors = parse_sensors_from_filename("test.txt");
        assert_eq!(get_tuning_frequency(&sensors, 20), 56_000_011);
    }
}
