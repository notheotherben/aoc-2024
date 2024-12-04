use std::ops::Index;

use super::Question;

question!(Day3, validate = [
    {
        input: r#"
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
        "#,
        part1: "161"
    },
    {
        input: "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        part2: "48"
    }
]);

impl Question for Day3 {
    fn part1(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let iter = MulExtractor { input, pos: 0 };
        let mut sum = 0;
        for (left, right) in iter {
            println!("{} * {}", left, right);
            sum += left * right;
        }
        Ok(sum.to_string())
    }

    fn part2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut sum = 0;
        let dos = input.split("do()");
        for do_ in dos {
            let instructions = do_.split_once("don't()").map(|(i,_)| i).unwrap_or(do_);
            let iter = MulExtractor { input: instructions, pos: 0 };

            for (left, right) in iter {
                println!("{} * {}", left, right);
                sum += left * right;
            }
        }

        Ok(sum.to_string())
    }
}

struct MulExtractor<'a> {
    input: &'a str,
    pos: usize,
}

impl Iterator for MulExtractor<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pos) = self.input[self.pos..].find("mul(") {
            let start = self.pos + pos + "mul(".len();
            if let Some(end) = self.input[start..].find(')').map(|i| start + i) {
                if let Some((left, right)) = self.input[start..end].split_once(',') {
                    match (left.parse(), right.parse()) {
                        (Ok(left), Ok(right)) => {
                            self.pos = end;
                            return Some((left, right));
                        }
                        _ => {
                            self.pos = start;
                            return self.next();
                        }
                    }
                } else {
                    self.pos = start;
                    return self.next();
                }
            }
        }

        None
    }
}