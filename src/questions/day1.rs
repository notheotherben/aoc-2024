use std::collections::HashMap;

use super::Question;

question!(Day1, validate = [
    {
        input: r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        "#,
        part1: "11",
        part2: "31"
    }
]);

impl Question for Day1 {
    fn part1(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut left = Vec::new();
        let mut right = Vec::new();
        
        for line in input.lines() {
            if let Some((l, r)) = line.trim().split_once("   ") {
                left.push(l.parse::<u32>()?);
                right.push(r.parse::<u32>()?);
            }
        }

        left.sort();
        right.sort();
        
        let mut sum: u32 = 0;

        for (l, r) in left.iter().zip(right.iter()) {
            eprintln!("{} {}", l, r);
            sum += l.abs_diff(*r);
        }

        Ok(sum.to_string())
    }

    fn part2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut left = Vec::new();
        let mut right: HashMap<u32, u32> = HashMap::new();
        
        for line in input.lines() {
            if let Some((l, r)) = line.trim().split_once("   ") {
                left.push(l.parse::<u32>()?);

                let r = r.parse::<u32>()?;
                right.entry(r).and_modify(|v| *v += 1).or_insert(1);
            }
        }

        let mut sum = 0;
        for l in left.iter() {
            if let Some(count) = right.get(l) {
                sum += *l * count;
            }
        }

        Ok(sum.to_string())
    }
}
