use crate::helpers;

use super::Question;

question!(Day4, validate = [
    {
        input: r#"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
        "#,
        part1: "18",
        part2: "9"
    }
]);

impl Question for Day4 {
    fn part1(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let grid = input.parse::<WordSearch>()?;

        Ok(grid.count_xmas().to_string())
    }

    fn part2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let grid = input.parse::<WordSearch>()?;
        Ok(grid.count_x_mas().to_string())
    }
}

tile!(Letter {
    A = 'A',
    M = 'M',
    S = 'S',
    X = 'X',
});

grid!(WordSearch<Letter> => {
    pub fn count_xmas(&self) -> usize {
        let mut count = 0;
        for y in 0..self.0.cols() {
            for x in 0..self.0.rows() {
                count += self.count_xmas_from(x, y);
            }
        }

        count
    }

    fn count_xmas_from(&self, x: usize, y: usize) -> usize {
        let steps = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1),
        ];

        let mut count = 0;
        for (dx, dy) in steps.into_iter() {
            if self.matches_xmas_in_direction(x, y, dx, dy) {
                count += 1;
            }
        }

        count
    }

    fn matches_xmas_in_direction(&self, x: usize, y: usize, dx: isize, dy: isize) -> bool {
        let mut x = x as isize;
        let mut y = y as isize;

        for letter in [Letter::X, Letter::M, Letter::A, Letter::S].into_iter() {
            if x < 0 || y < 0 {
                return false
            }

            if !matches!(self.get(x as usize, y as usize), Some(l) if l == letter) {
                return false
            }

            x += dx;
            y += dy;
        }

        true
    }


    pub fn count_x_mas(&self) -> usize {
        let mut count = 0;
        for y in 1..self.0.cols() -1 {
            for x in 1..self.0.rows() -1 {
                if self.is_x_mas(x, y) {
                    count += 1;
                }
            }
        }

        count
    }

    fn is_x_mas(&self, x: usize, y: usize) -> bool {
        if !matches!(self.get(x, y), Some(Letter::A)) {
            return false;
        }

        let top_left = self.get(x - 1, y - 1);
        if !matches!(top_left, Some(Letter::M | Letter::S)) {
            return false;
        }

        let top_right = self.get(x + 1, y - 1);
        if !matches!(top_right, Some(Letter::M | Letter::S)) {
            return false;
        }

        let bottom_right = self.get(x + 1, y + 1);
        if bottom_right != Some(Self::complement(top_left.unwrap())) {
            return false;
        }

        let bottom_left = self.get(x - 1, y + 1);
        if bottom_left != Some(Self::complement(top_right.unwrap())) {
            return false;
        }

        true
    }

    fn complement(letter: Letter) -> Letter {
        match letter {
            Letter::A => Letter::A,
            Letter::M => Letter::S,
            Letter::S => Letter::M,
            Letter::X => Letter::X,
        }
    }
});