#[macro_use] mod macros;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

#[allow(unused_variables)]
pub trait Question {
    fn part1(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        todo!();
    }

    fn part2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        todo!();
    }
}

pub trait ExampleInput {
    fn part1_test_input() -> &'static str;

    fn part2_test_input() -> &'static str {
        Self::part1_test_input()
    }
}
  
pub trait RealInput {
    fn real_input() -> &'static str;
}