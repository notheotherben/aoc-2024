/// Generates the tests used to run a daily question and validate it against the test input.
/// 
/// # Example
/// ```rust
/// question!(Day1, validate = [
///     {
///        input: r#"input"#
///        part1: "1",
///        part2: "2"
///     },
/// ], skip=true);
/// ```
macro_rules! question {
    ($day:ident, validate = [
        $(
            {
                input: $input:expr $(, part1: $part1:expr)? $(, part2: $part2:expr)?
            }
        ),+
    ]
    $(, skip=$skip:expr)?) => {
        struct $day;

        #[cfg(test)]
        #[test]
        fn part1() -> Result<(), Box<dyn std::error::Error>> {
            use $crate::questions::Question;

            $($(assert_eq!(<$day>::part1($input.trim())?, $part1);)?)+

            $(
            if $skip {
                return crate::results::run(concat!(stringify!($day), ".1"), || {
                    Ok("skipped")
                });
            }
            )?

            crate::results::run(concat!(stringify!($day), ".1"), || {
                <$day>::part1(include_str!(concat!("../../inputs/", stringify!($day), ".txt")).trim())
            })
        }

        #[cfg(test)]
        #[test]
        fn part2() -> Result<(), Box<dyn std::error::Error>> {
            use $crate::questions::Question;

            $($(assert_eq!(<$day>::part2($input.trim())?, $part2);)?)+
            
            $(
            if $skip {
                return crate::results::run(concat!(stringify!($day), ".2"), || {
                    Ok("skipped")
                });
            }
            )?

            crate::results::run(concat!(stringify!($day), ".2"), || {
                <$day>::part2(include_str!(concat!("../../inputs/", stringify!($day), ".txt")).trim())
            })
        }
    }
}