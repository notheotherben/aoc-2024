use std::{collections::{HashMap, HashSet}, fmt::Display, str::FromStr};

use super::Question;

question!(Day5, validate = [
    {
        input: r#"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
        "#,
        part1: "143",
        part2: "123"
    }
]);

impl Question for Day5 {
    fn part1(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let input: Input = input.parse()?;

        let mut sum = 0;
        for update in &input.updates {
            let ordered = update.order(&input.rules);
            if update == &ordered {
                sum += update.middle();
            }
        }

        Ok(sum.to_string())
    }

    fn part2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let input: Input = input.parse()?;

        let mut sum = 0;
        for update in &input.updates {
            let ordered = update.order(&input.rules);
            if update != &ordered {
                sum += ordered.middle();
            }
        }

        Ok(sum.to_string())
    }
}

struct Input {
    pub rules: HashMap<u32, Rule>,
    pub updates: Vec<Update>,
}

impl FromStr for Input {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules = HashMap::new();
        let mut updates = Vec::new();
        let mut is_rules = true;

        for line in s.trim().lines() {
            if line.is_empty() {
                is_rules = false;
                continue;
            }

            if is_rules {
                let (left, right) = line.split_once('|').ok_or("Invalid rule")?;
                let left = left.trim().parse()?;
                let right = right.trim().parse()?;

                rules.entry(left).or_insert_with(|| Rule { pages: HashSet::new() }).pages.insert(right);
            } else {
                updates.push(line.parse()?);
            }
        }

        Ok(Input { rules, updates })
    }
}

#[derive(Default)]
struct Rule {
    pub pages: HashSet<u32>,
}

#[derive(PartialEq)]
struct Update {
    pub pages: Vec<u32>,
}

impl Update {
    pub fn middle(&self) -> u32 {
        self.pages[self.pages.len() / 2]
    }

    pub fn order(&self, rules: &HashMap<u32, Rule>) -> Update {
        let mut ordered = Vec::new();

        ordered.push(self.pages[0]);

        let empty_rule = Rule::default();

        for page in &self.pages[1..] {
            let precedes = rules.get(page).unwrap_or(&empty_rule);
            
            let mut inserted = false;
            for (i, ordered_page) in ordered.iter().enumerate() {
                if precedes.pages.contains(ordered_page) {
                    ordered.insert(i, *page);
                    inserted = true;
                    break;
                }
            }

            if !inserted {
                ordered.push(*page);
            }
        }

        Update { pages: ordered }
    }
}

impl FromStr for Update {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pages = s.trim().split(',').map(|s| s.parse()).collect::<Result<Vec<u32>, _>>()?;
        Ok(Update { pages })
    }
}

impl Display for Update {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pages.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(","))
    }
}