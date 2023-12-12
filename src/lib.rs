pub mod grid {
    use std::ops::{Add, Sub};

    pub struct CharGrid {
        pub grid: Vec<Vec<char>>,
        pub width: usize,
        pub height: usize,
    }

    impl CharGrid {
        pub fn new(input: &str) -> Self {
            let width = input.lines().next().unwrap().len();
            let height = input.lines().count();

            let grid = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
            Self { grid, width, height }
        }

        pub fn path_to_string(&self, path: &Vec<Point>) -> String {
            path.iter().map(|p| self.get_char(*p)).collect::<String>()
        }

        pub fn get_char(&self, pos: Point) -> char {
            self.grid[pos.y as usize][pos.x as usize]
        }

        pub fn print(&self) {
            for (x,line) in self.grid.iter().enumerate() {
                print!("{:02}|", x);
                for c in line.iter() {
                    print!("{}", c);
                }
                println!();
            }
        }

        pub fn points_where(&self, predicate: fn(char) -> bool) -> Vec<Point> {
            let mut result = Vec::new();
            for y in 0..self.height {
                for x in 0..self.width {
                    if predicate(self.grid[y][x]) {
                        result.push(Point::new(x as i32, y as i32));
                    }
                }
            }
            result
        }

        // pub fn rows_where(&self, predicate: fn(char) -> bool) -> Vec<usize> {
        //     let mut result = Vec::new();
        //     for y in 0..self.height {
        //         let mut found = true;
        //         for x in 0..self.width {
        //             found &= predicate(self.grid[y][x]);
        //         }
        //         if !found {
        //             continue;
        //         }
        //         result.push(y);
        //     }
        //     result
        // }

        // pub fn columns_where(&self, predicate: fn(char) -> bool) -> Vec<usize> {
        //     let mut result = Vec::new();
        //     for x in 0..self.width {
        //         let mut found = true;
        //         for y in 0..self.height {
        //             found &= predicate(self.grid[y][x]);
        //         }
        //         if !found {
        //             continue;
        //         }
        //         result.push(x);
        //     }
        //     result
        // }

        pub fn clone_column(&mut self, x: usize) -> Result<(), String> {
            for y in 0..self.height {
                let c = self.grid[y].get(x).ok_or(format!("Column {} does not exist", x))?.clone();
                self.grid[y].insert(x, c);
            }
            self.width += 1;
            Ok(())
        }

        pub fn clone_row(&mut self, y: usize) -> Result<(), String> {
            let row = self.grid.get(y).ok_or(format!("Row {} does not exist", y))?;
            self.grid.insert(y, row.clone());
            self.height += 1;
            Ok(())
        }


        pub fn columns_where(&self, predicate: fn(char) -> bool) -> Vec<Vec<Point>> {
            let mut result = Vec::new();
            for x in 0..self.width {
                let mut found = true;
                for y in 0..self.height {
                    found &= predicate(self.grid[y][x]);
                }

                if found {
                    let mut column = Vec::new();
                    for y in 0..self.height {
                        if predicate(self.grid[y][x]) {
                            column.push(Point::new(x as i32, y as i32));
                        }
                    }
                    result.push(column);
                }
            }
            result
        }

        pub fn rows_where(&self, predicate: fn(char) -> bool) -> Vec<Vec<Point>> {
            let mut result = Vec::new();
            for y in 0..self.height {
                let mut found = true;
                for x in 0..self.width {
                    found &= predicate(self.grid[y][x]);
                }

                if found {
                    let mut row = Vec::new();
                    for x in 0..self.width {
                        if predicate(self.grid[y][x]) {
                            row.push(Point::new(x as i32, y as i32));
                        }
                    }
                    result.push(row);
                }
            }
            result
        }


        pub fn add_column(&mut self, c: char) {
            for line in self.grid.iter_mut() {
                line.push(c);
            }
            self.width += 1;
        }

        pub fn paint_path(&mut self, path: &Vec<Point>, c: char) {
            for p in path.iter() {
                self.grid[p.y as usize][p.x as usize] = c;
            }
        }

    }



    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Direction {
        North,
        South,
        East,
        West,
    }


    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point::new(self.x + other.x, self.y + other.y)
        }
    }

    impl Sub for Point {
        type Output = Point;

        fn sub(self, rhs: Self) -> Self::Output {
            Point::new(self.x - rhs.x, self.y - rhs.y)
        }
    }

    impl Point {
        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }

        pub fn path_to(&self, to: &Point) -> Vec<Point> {
            let mut path = Vec::new();
            let mut current = *self;
            while current != *to {
                let dir = current.direction(to);
                current = match dir {
                    Direction::North => Point::new(current.x, current.y - 1),
                    Direction::South => Point::new(current.x, current.y + 1),
                    Direction::East => Point::new(current.x + 1, current.y),
                    Direction::West => Point::new(current.x - 1, current.y),
                };
                path.push(current);
            }

            path
        }

        pub fn adjacent(&self) -> Vec<Point> {
            if self.x == 0 && self.y == 0 {
                return vec![Point::new(1, 0), Point::new(0, 1)];
            }
            if self.x == 0 {
                return vec![Point::new(0, self.y - 1), Point::new(0, self.y + 1)];
            }
            if self.y == 0 {
                return vec![Point::new(self.x - 1, 0), Point::new(self.x + 1, 0)];
            }
            vec![
                Point::new(self.x - 1, self.y),
                Point::new(self.x + 1, self.y),
                Point::new(self.x, self.y - 1),
                Point::new(self.x, self.y + 1),
            ]
        }

        pub fn direction(&self, other: &Point) -> Direction {
            if self.x == other.x {
                if self.y > other.y {
                    Direction::North
                } else {
                    Direction::South
                }
            } else {
                if self.x > other.x {
                    Direction::West
                } else {
                    Direction::East
                }
            }
        }

        pub fn distance(&self, other: &Point) -> i32 {
            (self.x - other.x).abs() + (self.y - other.y).abs()
        }
    }
}

pub mod utils {

    pub fn run<TIn, TOut>(input: TIn, func: fn(TIn) -> TOut, ans: Option<TOut>) 
    where TIn: std::fmt::Debug + std::cmp::PartialEq,
          TOut: std::fmt::Debug + std::cmp::PartialEq
    {
        let answer = func(input);

        if let Some(correct) = ans {
            if answer == correct {
                println!("Correct!");
            } else {
                println!("Suggested: {:?}", answer);
                println!("Incorrect! Correct answer: {:?}", correct);
            }
        } else {
            println!("Answer: {:?}", answer);
        }
    }

    pub fn take_char(input: &str, c: char) -> Option<(char, &str)> {
        if let Some(ch) = input.chars().next() {
            if ch == c {
                return Some((ch, &input[1..]));
            }
        }
        None
    }

    pub fn take_whitespace(input: &str) -> Option<(&str, &str)> {
        take_while1(input, |c| c.is_whitespace())
    }

    pub fn take_word<'a>(input: &'a str, word: &'static str) -> Option<(&'a str, &'a str)> {
        if input.starts_with(word) {
            Some((word, &input[word.len()..]))
        } else {
            None
        }
    }

    pub fn take_w(word: &'static str) -> impl Fn(&str) -> Option<(&str, &str)> {
        move |input: &str| if input.starts_with(word) {
            Some((word, &input[word.len()..]))
        } else {
            None
        }
    }

    pub fn take_list(input: &str, parser: fn(&str) -> Option<(&str, &str)>, delimiter: fn(&str) -> Option<(&str, &str)>) -> Option<(Vec<&str>, &str)> {
        let mut result = Vec::new();
        let mut input = input;
        while let Some((item, rest)) = parser(input) {
            result.push(item);
            input = rest;
            if let Some((_, rest)) = delimiter(input) {
                input = rest;
            } else {
                break;
            }
        }
        if result.is_empty() {
            None
        } else {
            Some(( result, input ))
        }
    }

    pub fn take_tuple(input: &str, parsers: (fn(&str) -> Option<(&str, &str)>, fn(&str) -> Option<(&str, &str)>)) -> Option<(&str, &str)> {
        let (parser1, parser2) = parsers;
        if let Some((item1, rest)) = parser1(input) {
            if let Some((item2, rest)) = parser2(rest) {
                return Some((&input[..(item1.len() + item2.len())], rest));
            }
        }
        None
    }

    pub fn take_maybe(input: &str, parser: fn(&str) -> Option<(&str, &str)>) -> Option<(&str, &str)> {
        if let Some((item, rest)) = parser(input) {
            Some((item, rest))
        } else {
            Some(("", input))
        }
    }

    pub fn take_many(input: &str, func: fn(&str) -> Option<(&str, &str)>) -> Option<Vec<&str>> {
        let mut result = Vec::new();
        let mut input = input;
        while let Some((item, rest)) = func(input) {
            result.push(item);
            input = rest;
        }
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    pub fn take_number(input: &str) -> Option<(&str, &str)> {
        take_while1(input, |c| c.is_digit(10))
    }

    pub fn take_while1(input: &str, predicate: fn(char) -> bool) -> Option<(&str, &str)> {
        if let Some(ch) = input.chars().next() {
            if predicate(ch) {
                let mut len = 1;
                for ch in input[1..].chars() {
                    if predicate(ch) {
                        len += 1;
                    } else {
                        break;
                    }
                }
                return Some((&input[..len], &input[len..]));
            }
        }
        None

    }
}
