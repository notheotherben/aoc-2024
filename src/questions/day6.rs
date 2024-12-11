use super::Question;

question!(Day6, validate = [
    {
        input: r#"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
        "#,
        part1: "41",
        part2: "6"
    }
], skip=true);

impl Question for Day6 {
    fn part1(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut map = input.parse::<Map>()?;

        let start = map.start().ok_or("No start found")?;
        map.walk_to_end(start);

        Ok(map.visited().to_string())
    }

    fn part2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut map = input.parse::<Map>()?;
        let mut test_map = map.clone();

        let start = map.start().ok_or("No start found")?;
        map.walk_to_end(start);

        // Now we loop through every point which the guard can visit and check
        // whether adding a wall there would create a loop.

        let mut count = 0;
        for x in 0..map.0.cols() {
            for y in 0..map.0.rows() {
                if let Some(Tile::Visited) = map.get(x, y) {
                    test_map.set(x, y, Tile::Wall);
                    if test_map.is_loop(start) {
                        count += 1;
                    }
                    test_map.set(x, y, Tile::Empty);
                }
            }
        }

        Ok(count.to_string())
    }
}

tile!(Tile {
    Start = '^',
    Empty = '.',
    Wall = '#',
    Visited = 'X',
});

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Guard {
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
}

impl Guard {
    pub fn turn(&self) -> Guard {
        let dx = -self.dy;
        let dy = self.dx;
        Guard { x: self.x, y: self.y, dx, dy }
    }

    pub fn forward(&self, bounds: (usize, usize)) -> Option<Guard> {
        let x = self.x as isize + self.dx;
        let y = self.y as isize + self.dy;

        if x < 0 || y < 0 || x >= bounds.0 as isize || y >= bounds.1 as isize {
            return None;
        }

        Some(Guard { x: x as usize, y: y as usize, dx: self.dx, dy: self.dy })
    }
}

grid!(Map<Tile> => {
    pub fn start(&self) -> Option<Guard> {
        for x in 0..self.0.cols() {
            for y in 0..self.0.rows() {
                if let Some(Tile::Start) = self.get(x, y) {
                    return Some(Guard { x, y, dx: 0, dy: -1 });
                }
            }
        }

        None
    }

    pub fn walk_to_end(&mut self, mut guard: Guard) {
        loop {
            match self.step(guard) {
                Some(next) => guard = next,
                None => return,
            }
        }
    }

    pub fn is_loop(&mut self, guard: Guard) -> bool {
        let mut slow = guard;
        let mut fast = guard;

        loop {
            match self.step(fast) {
                Some(next) => fast = next,
                None => return false,
            }

            if slow == fast {
                return true;
            }

            match self.step(fast) {
                Some(next) => fast = next,
                None => return false,
            }

            if slow == fast {
                return true;
            }

            match self.step(slow) {
                Some(next) => slow = next,
                None => return false,
            }
        }
    }

    pub fn visited(&self) -> usize {
        self.0.iter().flatten().filter(|t| **t == Tile::Visited).count()
    }

    fn step(&mut self, guard: Guard) -> Option<Guard> {
        self.set(guard.x, guard.y, Tile::Visited);

        match guard.forward((self.0.cols(), self.0.rows())) {
            Some(next) => {
                match self.get(next.x, next.y) {
                    Some(Tile::Wall) => Some(guard.turn()),
                    _ => Some(next),
                }
            }
            None => None,
        }
    }
});