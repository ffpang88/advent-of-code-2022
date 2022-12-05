use advent_of_code_common;

#[derive(Debug)]
struct Range {
    min: i32,
    max: i32,
}

impl Range {
    fn fully_intersects(&self, other: &Range) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn intersects(&self, other: &Range) -> bool {
        other.min >= self.min && other.min <= self.max
            || other.max >= self.min && other.max <= self.max
    }
}

impl From<&String> for Range {
    fn from(string_range: &String) -> Self {
        let split = string_range.split('-').collect::<Vec<&str>>();
        Self {
            min: split[0].parse().unwrap(),
            max: split[1].parse().unwrap(),
        }
    }
}

fn main() {
    let mut count = 0;
    advent_of_code_common::cli_read_file_by_line(|pair_range| {
        let split = pair_range.split(',').map(|range_string| range_string.to_string()).collect::<Vec<String>>();
        let range1 = Range::from(&split[0]);
        let range2 = Range::from(&split[1]);

        if range1.intersects(&range2) || range2.intersects(&range1) {
            count += 1;
        }
    });

    println!("{}", count);
}
