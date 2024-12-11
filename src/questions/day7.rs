use std::{fmt::Display, ops::{Div, Rem}, str::FromStr};

use super::Question;

question!(Day7, validate = [
    {
        input: r#"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
        "#,
        part1: "3749",
        part2: "11387"
    }
]);

impl Question for Day7 {
    fn part1(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let equations: Vec<Equation> = input.trim().lines().map(|l| l.parse()).collect::<Result<Vec<_>, _>>()?;

        let mut sum = 0;
        for eq in equations {
            if eq.is_solvable(false) {
                sum += eq.value;
            }
        }

        Ok(sum.to_string())
    }

    fn part2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let equations: Vec<Equation> = input.trim().lines().map(|l| l.parse()).collect::<Result<Vec<_>, _>>()?;
        let mut sum = 0;
        for eq in equations {
            if eq.is_solvable(true) {
                sum += eq.value;
            }
        }

        Ok(sum.to_string())
    }
}

struct Equation {
    value: u64,
    factors: Vec<u64>
}

impl Equation {
    pub fn is_solvable(&self, support_concat: bool) -> bool {
        can_factor(self.factors[0], self.value, &self.factors[1..], support_concat)
    }
}

impl FromStr for Equation {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(':').ok_or("Invalid equation")?;
        let value = left.trim().parse()?;
        let factors = right.split_whitespace().map(|f| f.parse()).collect::<Result<Vec<_>, _>>()?;

        Ok(Equation { value, factors })
    }
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.value, self.factors.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(" "))
    }
}

fn can_factor(current: u64, target: u64, factors: &[u64], support_concat: bool) -> bool {
    if let Some(factor) = factors.first().copied() {
        can_factor(current + factor, target, &factors[1..], support_concat)
        || can_factor(current * factor, target, &factors[1..], support_concat)
        || (support_concat && can_factor(concat(current, factor), target, &factors[1..], support_concat))
    } else {
        current == target
    }
}

fn concat(mut a: u64, b: u64) -> u64 {
    let mut temp = b;
    while temp > 0 {
        temp /= 10;
        a *= 10;
    }

    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_test() {
        assert_eq!(concat(123, 456), 123456);
        assert_eq!(concat(0, 456), 456);
        assert_eq!(concat(123, 1), 1231);
    }
}