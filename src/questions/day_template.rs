use super::Question;

question!(DayN, validate = [
    {
        input: r#"
        "#,
        part1: "",
        part2: ""
    }
]);

impl Question for DayN {
    fn part1(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok("".to_string())
    }

    fn part2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok("".to_string())
    }
}
