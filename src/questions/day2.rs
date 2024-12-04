use std::path::Display;

use super::Question;

question!(Day2, validate = [
    {
        input: r#"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
        "#,
        part1: "2",
        part2: "4"
    }
]);

impl Question for Day2 {
    fn part1(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let reports = input.lines().map(Report::try_from).collect::<Result<Vec<Report>, _>>()?;

        let safe_reports = reports.iter().filter(|report| report.is_safe()).count();

        Ok(safe_reports.to_string())
    }

    fn part2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let reports = input.lines().map(Report::try_from).collect::<Result<Vec<Report>, _>>()?;

        let safe_reports = reports.iter().filter(|report| report.is_safe_with_balancer()).count();

        Ok(safe_reports.to_string())
    }
}

#[derive(Debug)]
enum Direction {
    Increasing,
    Decreasing,
}

struct Report {
    levels: Vec<u32>,
}

impl Report {
    pub fn is_safe(&self) -> bool {
        let direction = self.get_direction();
        
        if let Some(first) = self.levels.first() {
            let mut prev = *first;
            for level in self.levels.iter().skip(1) {
                match &direction {
                    Direction::Increasing if *level < prev => return false,
                    Direction::Decreasing if *level > prev => return false,
                    d => d
                };

                let delta = prev.abs_diff(*level);
                if !(1..=3).contains(&delta) {
                    return false
                }

                prev = *level;
            }
        }

        true
    }

    pub fn is_safe_with_balancer(&self) -> bool {
        if self.is_safe() {
            return true
        }

        for i in 0..self.levels.len() {
            let report = self.excluding(i);
            if report.is_safe() {
                return true
            }
        }

        false
    }

    fn excluding(&self, index: usize) -> Report {
        let mut levels = self.levels.clone();
        levels.remove(index);

        Self { levels }
    }

    fn get_direction(&self) -> Direction {
        let pivot = self.levels.len() / 2;
        let (left, right) = self.levels.split_at(pivot);
        let left_sum: u32 = left.iter().sum();
        let right_sum: u32 = right.iter().sum();

        match (left_sum as f64/left.len() as f64).partial_cmp(&(right_sum as f64/right.len() as f64)) {
            Some(std::cmp::Ordering::Less) => Direction::Increasing,
            Some(std::cmp::Ordering::Greater) => Direction::Decreasing,
            _ => Direction::Increasing,
        }
    }
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for level in self.levels.iter() {
            write!(f, "{} ", level)?;
        }

        Ok(())
    }
}

impl TryFrom<&str> for Report {
    type Error = Box<dyn std::error::Error>;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut levels = Vec::new();
        for number in line.split_whitespace() {
            levels.push(number.parse::<u32>()?);
        }

        Ok(Self { levels })
    }
}