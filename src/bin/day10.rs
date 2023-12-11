use std::ops::{Add, Sub};
use std::slice::Chunks;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn adjacent(&self) -> Vec<Point> {
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

    fn direction(&self, other: &Point) -> Direction {
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



fn get_char(grid: &Vec<Vec<char>>, pos: Point) -> char {
    grid[pos.y as usize][pos.x as usize]
}

fn get_next_position(grid: &Vec<Vec<char>>, current: Point, previous: Point) -> Option<Point> {
    const NORTH: Point = Point { x: 0, y: -1 };
    const SOUTH: Point = Point { x: 0, y: 1 };
    const EAST:  Point = Point { x: 1, y: 0 };
    const WEST:  Point = Point { x: -1, y: 0 };

    match get_char(grid, current) {
        'F' => {
            if previous == current + SOUTH {
                Some(current + EAST)
            } else {
                Some(current + SOUTH)
            }
        },
        'L' => {
            if previous == current + NORTH {
                Some(current + EAST)
            } else {
                Some(current + NORTH)
            }
        },
        'J' => {
            if previous == current + NORTH {
                Some(current + WEST)
            } else {
                Some(current + NORTH)
            }
        },
        '7' => {
            if previous == current + SOUTH {
                Some(current + WEST)
            } else {
                Some(current + SOUTH)
            }
        },
        '-' => {
            if previous == current + EAST {
                Some(current + WEST)
            } else {
                Some(current + EAST)
            }
        },
        '|' => {
            if previous == current + SOUTH {
                Some(current + NORTH)
            } else {
                Some(current + SOUTH)
            }
        },
        'S' => {
            for adjacent in current.adjacent() {
                let c = get_char(grid, adjacent);
                match current.direction(&adjacent) {
                    Direction::North if [ '|', 'F', '7' ].contains(&c) => return Some(adjacent),
                    Direction::South if [ '|', 'L', 'J' ].contains(&c) => return Some(adjacent),
                    Direction::East  if [ '-', '7', 'J' ].contains(&c) => return Some(adjacent),
                    Direction::West  if [ '-', 'F', 'L' ].contains(&c) => return Some(adjacent),
                    _ => {}
                }
            }
            None
        },
        _ => None
    }
}

fn find_enclosure(path: &mut Vec<Point>) -> Option<Vec<Point>> {
    for (i, point) in path.iter().skip(1).enumerate() {
        for adjacent in point.adjacent() {
            if !path[..i].contains(&adjacent) {
                continue;
            }
            let adjacent_index = path.iter().position(|p| p == &adjacent).unwrap();

            return Some(path.drain(adjacent_index+1..=i).collect::<Vec<_>>());
        }
    }

    None
}

fn create_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>()
}

fn find_path(grid: &Vec<Vec<char>>) -> Vec<Point> {
    let start = grid.iter().enumerate().find_map(|(y, l)| l.iter().enumerate().find_map(|(x, c)| if *c == 'S' { Some(Point::new(x as i32,y as i32)) } else { None })).unwrap();
    let mut res = Vec::new();

    res.push(start);
    let (mut previous, mut current) = (start, get_next_position(&grid, start, start).unwrap());

    loop {
        if current == start {
            break;
        }
        res.push(current);
        (previous, current) = (current, get_next_position(&grid, current, previous).unwrap());
    }
    res
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for line in grid {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

fn print_path(path: &Vec<Point>, grid: &Vec<Vec<char>>) {
    let min_x = 0;
    let max_x = grid[0].len() as i32;
    let min_y = 0;
    let max_y = grid.len() as i32;

    for y in min_y..max_y {
        for x in min_x..max_x {
            let p = Point::new(x, y);
            if path.contains(&p) {
                print!("{}", get_char(grid, p));
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn part1(input: &str) -> usize {
    let grid = create_grid(input);
    find_path(&grid).len() / 2
}

fn path_to_char(path: &Vec<Point>, grid: &Vec<Vec<char>>) -> Vec<char> {
    path.iter().map(|p| get_char(grid, *p)).collect::<Vec<_>>()
}

fn path_to_directions(path: &Vec<Point>) -> Vec<Point> {
    let mut res = Vec::new();
    let mut previous = path[0];
    res.push(Point::new(0, 0));
    for p in path.iter().skip(1) {
        res.push(*p - previous);
        previous = *p;
    }
    res
}

fn path_to_corners(path: &Vec<Point>, grid: &Vec<Vec<char>>) -> Vec<Point> {
    const CORNERS: [char; 5] = [ 'F', '7', 'J', 'L', 'S' ];
    path.iter().filter(|p| CORNERS.contains(&get_char(grid, **p))).map(|p| *p).collect::<Vec<_>>()
}

fn set_grid(grid: &mut Vec<Vec<char>>, path: &Vec<Point>, c: char) {
    for p in path {
        grid[p.y as usize][p.x as usize] = c;
    }
}

fn path_to_string(path: &Vec<Point>, grid: &Vec<Vec<char>>) -> String {
    path.iter().map(|p| get_char(grid, *p)).collect::<String>()
}

fn calculate_area(vertices: &[Point]) -> i32 {
    let n = vertices.len();
    let mut area = 0;

    for i in 0..n {
        let p1 = vertices[i];
        let p2 = vertices[(i + 1) % n];
        area += p1.x * p2.y + 1 - p2.x * p1.y
    }

    (area.abs()) / 2
}

fn create_intersections<'a>(path: &'a Vec<Point>, grid: &Vec<Vec<char>>) -> Vec<Vec<&'a Point>> {
    let y_max = grid.len() as i32;

    let mut res = Vec::new();
    for y in 0..y_max {
        let mut v =path.iter().filter(|p| p.y == y).collect::<Vec<_>>();
        v.sort_by(|a, b| a.x.cmp(&b.x));
        res.push(v);
    }
    res
}

fn is_inside(point: &Point, intersections: &Vec<Vec<&Point>>, grid: &Vec<Vec<char>>) -> bool {
    let line = &intersections[point.y as usize];
    if line.contains(&point) {
        return false;
    }

    let mut intersections = 0;
    let mut n = false;
    let mut u = false;
    for c in line.iter().filter_map(|p| if p.x < point.x { Some(get_char(grid, **p)) } else { None }) {
        match c {
            'F' => n = true,
            '7' | 'S' if n => n = false,
            '7' | 'S' if u => {
                intersections += 1;
                u = false;
            },
            'L' => u = true,
            'J' if u => u = false,
            'J' if n => {
                intersections += 1;
                n = false;
            },
            '|' => intersections += 1,
            _ => {}
        }
    }

    if intersections % 2 == 1 {
        let str = line.iter().filter_map(|p| if p.x < point.x { Some(get_char(grid, **p)) } else { None }).collect::<String>();
        println!("{:?}: {}", point, str);
    }

    intersections % 2 == 1
}

fn paint(grid: &mut Vec<Vec<char>>, path: &Vec<Point>) {
    let x_max = grid[0].len() as i32;
    let y_max = grid.len() as i32;
    let intersections = create_intersections(&path, &grid);


    for y in 0..y_max {
        for x in 0..x_max {
            let p = Point::new(x, y);
            if is_inside(&p, &intersections, &grid) {
                grid[y as usize][x as usize] = 'I';
            }
        }
    }
}

fn count_inside(grid: &Vec<Vec<char>>, path: &Vec<Point>) -> i32 {
    let x_max = grid[0].len() as i32;
    let y_max = grid.len() as i32;
    let intersections = create_intersections(&path, &grid);


    let mut sum = 0;
    for y in 0..y_max {
        for x in 0..x_max {
            let p = Point::new(x, y);
            if is_inside(&p, &intersections, &grid) {
                sum += 1;
            }
        }
    }
    sum
}


fn part2(input: &str) -> i32 {
    let mut grid = create_grid(input);
    let path = find_path(&grid);

    let intersections = create_intersections(&path, &grid);

    //print grid
    println!("Grid:");
    print_grid(&grid);

    //print intersections
    println!("Intersections:");
    for line in intersections {
        for p in line {
            print!("{}", get_char(&grid, *p));
        }
        println!();
    }

    //print path
    println!("Path:");
    print_path(&path, &grid);

    // paint(&mut grid, &path);


    count_inside(&grid, &path)
}

fn calc() -> i32 {
    9 * 7 + (-4* 3) + (3 * -4) + (-7 * 4) + (4 * 3) + (-5 * -9)
}

fn main() {

//     aoc2023::utils::run(r#"..F7.
// .FJ|.
// SJ.L7
// |F--J
// LJ..."#, part1, Some(8));

//     aoc2023::utils::run(r#".....
// .S-7.
// .|.|.
// .L-J.
// ....."#, part2, None);

    aoc2023::utils::run(r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L--7F-J|.
.|...||..|.
.L---JL--J.
..........."#, part2, Some(5));

    // aoc2023::utils::run(INPUT, part1, Some(6768));
    // aoc2023::utils::run(INPUT, part1, Some(1772145754));
    aoc2023::utils::run(INPUT, part2, None);

//     aoc2023::utils::run(r#".F----7F7F7F7F-7....
// .|F--7||||||||FJ....
// .||.FJ||||||||L7....
// FJL7L7LJLJ||LJ.L-7..
// L--J.L7...LJS7F-7L7.
// ....F-J..F7FJ|L7L7L7
// ....L7.F7||L7|.L7L7|
// .....|FJLJ|FJ|F7|.LJ
// ....FJL-7.||.||||...
// ....L---J.LJ.LJLJ..."#, part2, None);

//     aoc2023::utils::run(r#"FF7FSF7F7F7F7F7F---7
// L|LJ||||||||||||F--J
// FL-7LJLJ||||||LJL-77
// F--JF--7||LJLJ7F7FJ-
// L---JF-JLJ.||-FJLJJ7
// |F|F-JF---7F7-L7L|7|
// |FFJF7L7F-JF7|JL---7
// 7-L-JL7||F7|L7F-7F7|
// L.L7LFJ|||||FJL7||LJ
// L7JLJL-JLJLJL--JLJ.L"#, part2, None);

}

#[allow(dead_code)]
const INPUT: &str = r#"J7F--7-|7FJ.FF|.FJ77.FJFL7FFF-J7F7-J-77F|-|7LL7.|FJ7F|JF|.FLF7-FF--7.-J.7-FF7F-77.LJFJ-FF.F|-F----77F77.JF|F|7F.|7-FJ-F777..F-7|77F7FFFF7FJ7
|L7.LL.|7J.F-.|-|JJF7-F7.|-JJ.|LLL.|7|JF|7|F7-FF7-.FLJLJL-.||LF|JLFFJLJ-7-JL77-7FJ-F-J.FJ-.|FL---7L7-FJ..FF-|-J-L|-||.L77-7-J.L-FJFL|-7-FJJ.
|.|FL7||F--JJ|-.JJ.J.LJ|7|J---J..L|--|F-|7F|FF-J|---J.F7J.F|7JL|F7JJ|.L-J|.7|F-JJ.FF7-J||--|J.|.F|FJ7|F-7-JJL-J7FL7F7FF.7JLLFFLJL--7|7JL|J.L
|FF7.||FF-F|.LL7|.7L-7.F7JL7FL|7.LJJ.-|F777LF|F7|J-FF7JJ.F||F7-L.||J|..LFFF-L.L|7||L77.L-7JJ.|7-FJ|7F|7|||.FFF-7-|FJL-|7JF7F|..F|FL7-L7..F|.
L7|7.L--L7F77|L|J-...-|.J7F||LL|.F7L-.L7||FF-J|LJF--J|J..J--||.F-7J.F|JFF7JLLJ.LF|J7JLJ-LJ7FF|F-JFJ-JLLJ-J--|J-|-FFJ7JLJ77LJ.J--77|||.|.L-7J
|LF77L.|LLL-J7FJJ.|7F7L--FJJJ-FJ7|7.LFLJFFJL7FJ|FJF--J.|77|JL7-|-J-7L|LF.LJ-L-JFFJ7|J|J7|F-|-LL-7|7J||..||L7.FL7FF|FJ|.LJ||||.|FJL-FJFFF7FL.
F7LL7.FL7|-L-FJ|F-J||F7|-7JJ-L7FJ.|FLLJ--|.||L--JFJF7J.FF7FF7J7.FJL7-|.L7LF7.JJ7|L-||L7|7L||FLF-JL-7-F77F|-F7--LJ-7J.LL-L7----JLJJL|-F||L7L|
LLJFLJLLJ-F7FL-LJ..L|J.7|.||.LLF-FL|JJL.|J-FJF-7FJ-||FF-JL7||.7-|LF-77|FF7.|-JLFLJ|F7J7F7FFF7LL-7F-JFJL77JFFJ.|L|J.|7.|.FJJ|.|FJJ-FL--J7F|F|
-77-LLFJFJ|FLL7|L-77|-J.L777F-FF7L|L7|FF.F7L7|FLJ-FJL7L--7LJL7-7J7JLF7JL7|7LF7JL|-F|L7FJL7F-7F--JL-7L7FJ--|7.L||L77JF---|.-L7LF|7.-F7|-FFJ-F
LF7.|7F-JJ7JFJLF.|LL7..FJ7--LL7|F---F77|FJ|FJL77JJL-7L7F7|F--J||L7-.|J-|FLJ.FJ-L|-FJFJ|F-JL7|L----7L-J|LL-7-7JL|7.JL|.|.-LJLJ.|.F|J.-L7FJ|7L
.L7FFL|J.FF-|7.F7F|J|--77|.|FFLFF7.L||F7L7LJF7L-7F7||FJ||||F7|FJF-.|JLF--7-FLJL|L-L7|FJL-7FJ|F7F--JF--J-LF|-J.7|.J7.|.---J-|F-FLJ..LJFFJ7|7L
F7J7|7||FLJFJ7---FF7J|-L7F7FL--FJ|FF||||FJF-JL--J||FJL-J||||L7F7JJF77|LJ||FF|L-L.|L||L7F-JL7|||L7F-JF7LFLJLJ.-JLL77F-7|LJF-JF7|-LF-L--|.LLFJ
L|LJJ|LF7J-L7JFJ.J.F-JLL|.F7|FFL7L7FJ||||FJF--7F7||L---7||LJFJ||..|F777FF77LLF-L.LJ|L-JL---J|||FJL-7|L-7-F7.|||JL7J|--7-FFJFLJ-F-JJ.L|FJ.L.F
|||.FL7L|777L7L77|F7.|.LF-||F--7L7|L7|||||-|F-J|||L7F7FJ||F-JFJL7F7||F-7|L77||.|..FL-------7LJ|L7F-JL7FJ7F-7|-|7J.FJ-F|.FL7L|7||LL7J.F||F--7
FL--|-L-J7J7|-FLJ--JFFF-7FJ|L-7L7|L-J||LJL-J|F7||L7|||L7|||F7|F-J|||LJFJ|FJLJJ|-F-----7F-7-|F-JL||F77|||F7.|JF7|.J7FFLJF|L-7LFJLJ|J7FJJ7L7F|
7-FFJ-L|J.LL|||F.|-FLFJFJL7|F7|FJL--7|L-7F7FJ|||||||||FJLJ|||||F7|LJF-JFJL7J7F7|L----7|L7|FJL-7FJLJL7||FJL-7-|L-7LJ777|LJ7L7-|-LF|7-JL7-7|F|
F.F7.-7|7F77L7FJ-F-|LL7L--J|||||F7F-JL-7LJ|L-JLJL7|||||F--J||||||L7FJL||F-J7-||7|FLF-JL-J||F--JL7F-7||||F--JL|F-J|LL7-7J.--JFJ..F7J7LL.LFL.|
|J.L7-J-F-J-L77..|7|FLL7F-7LJ|||||L--7FJF7|F7F---J|||LJL7-FJ||LJL-J|F7FJ|LF7-|L7FF7L----7LJL-7F7LJFJ||||L7F7FJL--777JF7F-7L|L---J7|-.|F-JJFL
.FF--.||L|-.|LF-FJL-JJJLJFJF7LJLJ|F77||FJ|||LJF-7-||L--7L7L7|L7F-7FJ||L7L7|L7|FJFJ|F7F-7L7F--J|L-7L7||||FJ|||F--7|F77|F|7-FLJF|LJ|||F77JF7.L
LLJ77FL7LL|-L.|JL7|....F-JFJL7F-7|||FJ|L7||L7FJFJFJL---JFJJ|L7||7LJFJL7L7||FJ|L7L7|||L7|J||LF7L7FJFJLJLJL-JLJL7LLJ||F---7.F|--J-L|J|JL7.-L7|
|7.LFLJJ.F7..7J.|.L-77FL--JF7LJ-||||L7|FJLJFJL7L7L--7F7FJF7L7LJL--7L7FJFJLJ||L7L7||||FJL7|L7|L7||FJF-7F-7F7F7FJ.LFJ||F--J.L77.7...F.|.LFJJ.J
.L7-J7..LLL77FF7-F|.LFF---7||F-7|LJ|FJ|L-7FJF7L7L7F7LJ||FJ|FJF----JFJ|7L7F-JF7|FJ||||L-7||FJ|FJ|LJFJ7LJLLJLJLJF-7L7LJL7LL-J.7-77--JF--7L|J.|
J-7-.FF..--F7FJ|FF77.FL--7|||L7|L7FJL7L--JL7||FJFJ||F7||L7||FJF7F77L7|F7|L-7|||L-JLJ|F7|||L7|||L-7|JF---7F--7|L7|FJF-7L7J|.-|JLL.|.|7|F-7L-J
|F||FJL777L||L7L7||F---7FJLJ|FJL7||F7L---7FJ||L7L-J|||||L|||L7|LJL7FJ||||F-J|||F--7FJ||||L7||L7F7|L7|F--J|F-JF7||L7||L-J-J.|FJ.|F-J-77F7L7.|
|LF77|7J|F7||F|FJ|||F--JL7F7LJF7LJ||L7F-7|L-J|FJF--J|LJ|FJLJFJL7F7||FJ||||F7||LJF7||FJLJ|FJ||FJ|LJFJ||F-7|L--J|||FJL----7J.L--J-|-FF.7LJ7F-7
FLLJ77F.F|LJL7|L7|||L7JF7LJL7FJL-7|L7|L7LJF--J|FJLF7L7FJL7F7L7F||LJ|L-JLJ||||L7FJ|||L--7|L7||L7|F-JFJ|L7||F---J|||F7F---J-F-F..|.FFJJ|F|-JL7
F7JF|JL|7L--7|L7|||L7|FJL7F7LJF7F||F||JL7FJLF7|L7FJ|FJ|F7||L-JFJ|F7|F----J|LJFJL7|||F7FJL7||L7||L77L7|||LJL--7|||||LJJF7F7J-L-F|-|J|.FF|7JF.
|J--7JJF----J|7||||F|||F-J||-FJL-J|FJ|F7|L--JLJFJL7|L7LJ|||F7FJFJ|LJL-7F7FJF-JF7||||||L-7|||FJ|L7L7FJL7|F----JFJLJL7F-JLJ|.LL-F|-|J..|LLJF7J
J.77|..L7F7F7L-JLJL7|||L-7||FJF--7|L7|||L7F--7FJF|||L|F-J||||L7L-JF--7|||L7L-7|||||||L--JLJLJFJFJFJL7FJ|L-7F7FJF7F7LJF---J77||F77JF-7JJL--|J
.-JF7FJ-LJLJL---7F-J|||F-J||L-JF-J|FJLJL-J|F7LJF7FJL-JL7FJ||L7L--7L-7LJ||||F-J|||||||F7F-7F--JLL7|F7|L-JF-J||L7|LJ|F-JF---7F--J|JJL7L7F.|-77
L-F..7-7LF7|F--7||F7|||L-7|L---JF7|L--7F-7LJL--J|L--7F7LJFJ|FJF-7L-7L77||FJ|F7|||LJ|LJLJFJ|F-7F-JLJLJF--JF-J|FJL7FLJF7|F--J|F--J|7LL7LFF77FJ
F7|F7J.FFJL-JF7LJLJLJ||F-JL-----J||F77LJFJF7F7F-JF7-LJL-7|FJL-JFJF7|FJFJ|L7|||||L-7L7F-7L7||FJL--7F--JF7|L-7||F-JF--JLJL-7.|L----7-|7|L|J-J7
LLJ7.FFFJF7F-JL-7F7F7|||F7F-7F7F-JLJ|F-7L7|LJLJF-JL-7F7FJ|L---7L7||||FJFJJ|||||L-7|FJL7|FJLJL7F7L|L-7J|L7F7|LJL-7L7F-----JFJF----JJ||-L|||L-
F|7|-.|L7|LJ|F7FJ|LJ|||||||FJ||L--7FJL7L-J|F--7L-7F-J|||FJF--7|FJ|LJ||FJF7|||||F-J|L7FJ||F---J|L7L-7|FJFJ|||F--7L-JL7FF7F7|FJF7F7-FL7LFJLL-|
L7-|7F|FLJ-F7||L7|F-J||||LJ|FJL7F-J|F7L--7||F-JF7||F7||||-|F-J||7|F-J||L||||||||F7L7||FJ||F7F7L7|F7|||FJL|LJL-7L--7FJFJLJLJL-JLJ|-JLJFJ-7L-J
||FJ-7L-LF-JLJL7LJL-7LJ||F-JL7FJL-7LJL7F7|LJL7FJ|||||||||FJ|F7||FJL7FJL7|LJ||||LJ|FJ||L7|||LJ|FJ||||LJ|F-JF---J7F7LJFJF7F--7F--7|L-7J-J.|7|7
--|-F|7.LL----7|F7F7L-7LJ|7F-JL7F7L-7FJ|||-F-JL7||||||LJ||LLJ||||F7|L7FJ|F-J|||LFJL7|L7|||L-7||FJ||L7FJ|F-JFS.F-JL7FJFJ|L-7|L-7||-FL||LFJJ-|
J.|-JLF7-F----JLJLJL--JF7L7|F7FJ||F-JL-J||FJF7FJLJLJ|L7||L--7|||LJ||FJL7|L7FJ||FJF7||FJLJ|F-JLJ|FJL7|L7|L7FJL7|F-7LJFJ.L--JL--JLJ.-7L-7-.|J.
JFLJ.FJL-JF7F----7F----JL7|LJ||FJ|L7F7F7|||FJ|L-7F--JFJFJF7FJLJL7FJLJF7|L7LJFJ||FJ|||L7F-JL--7FJL-7||-||LLJF7LJL7L--JF7.F-7F-7-L7|-7-7.LJ7.|
LJ|LFL---7|LJF---J|F-----J|F7||L7L-J||||LJ||FJF7||F7FJ7L7|||F---JL---J|L-JF-JFJ||FJ|L7|L7F7F7|L7F7|||FJ|F--JL-7FJ|F7-|L-JFJ|FJ7.FFJF77JLF|-F
.L77FF--7LJFFJF7F7||F7F7F7LJLJL-JF--JLJL-7|||FJ||||||F--J|LJ|F7F7F-7F7|.F-JF7|J||L7L7||FJ||||L7||||||L7|L----7LJF-J|FJF--J-|L---7.F|L7..J.F.
..|J||F7L---JFJ||LJLJLJLJL7F7F7F7L7F-7F-7|||||FJ|||LJL7F7L7|LJ|||L7|||L7|F7|LJFJL7L-J||L7|||L7|LJ||||7LJ7F---JF7L-7LJFJ|F--JF---JF7|FJ--L7|7
J-F7LLJ|F---7L7LJ.F------7||||||L-J|FLJ-||||LJL7||L-7FJ||FJF--J||FJ||L7|||LJF-JF7L--7LJFJ||L7|L-7LJLJF---JF--7|L-7|F-JLFJF--JFF7F|LJ|J7|-JLL
F-J|||L|L7F7|FJLF-JF7F7F7LJ|LJLJLF7L---7|||L--7|||F-JL7||L7|F7FJ||FJL7|||L7FJF7||F7FJF7L7|L7|L--JF7.FJF7F7|F-J|F7||L--7L7L----JL7L-7L-7J.J-.
JJ.F-LFJFJ|LJ|F7L--JLJLJL-7|F-7F-J|F7F-JLJL-7FJLJ|L7F-J||FJ|||L7LJL7FJLJL7|L7|||LJ|L-JL7||FJ|F---JL7L-JLJLJL7FJ|LJ|F--JFJF7F--7FJFL|F-JF7J77
.L7JF-L7|FJF-J|L---7F7F7F7LJL7|L-7LJLJF----7|L--7L7||F7||L7LJL7|F--JL7F-7||FJ|LJF-JF7F-J||L7|L7F--7L--------J|FJF7|L7F7|FJLJF7LJF7FJL--7.L-.
...FJ7.LJL-JF7L---7LJLJLJL---J|F7L----JF7F-J|F--JFJ|LJLJL7L7F7LJL-7F7|L7LJ|L7L-7|F7||L-7LJFJ|FJ|F7L7F--------JL-JLJFJ||||LF7||F7|||F---J-J.7
7FF-J|L-F---JL7F--JF-----7F7F7LJL-----7||L7FJL--7L7L---7.L7LJ|F---J|LJJL-7L7|F-JLJ|||F7L-7L-JL-J|L-J|F-------------JFJLJL7|||||||||L7|.LJJ|J
.-FJ.L-F|F-7F7LJF--JF--7-LJLJ|F7F7F7F7LJ|FJL----JF|F7F-JF7|F-JL7F-7L----7L7||L--7FJ|||L--JF-----JF--JL-----7F7F---7FJF--7LJLJLJLJLJFJJJ.|F-J
7J|J-LFFJ|7LJL-7|F--JF7L7F77FJ|||LJLJL-7LJJF-----7LJ||F-JLJ|-F7||FJF7F-7|FJ||F7FJL7||L---7|F7F-7FJF-7F----7|||L-7FJL7|F7L---------7L7|7FF|||
|.LJ-LLL7|F--7FJ|L7F7|L7LJL-JFJLJF-----JF-7L7F---JF7LJL-7F7|FJLJ||FJ|L7|||FJ||||F7LJ|F--7|||||FLJFL7|L-7F7|LJL--JL--JLJ|F--7F7-F-7L-J7--|.J.
F77|.F-7LJ|F7LJFJFJ|LJ7L--7F7|F7|L---7F7L7L-JL----J|F-7FJ|LJ|F--J||FJFJ||LJFJ|LJ|L--JL-7||LJLJF7LF7|L-7LJ||F--7F7F7F7F7LJF7LJL-JFJ-L--7.L7.7
-JF--F.77.||L7FJ|L7|F-----J|LJ|L--7F7LJL-JF--------J|FJL7|.FJL-7FJ|L7L7|L-7L7L-7|F--7F-JLJF-7FJL7||L-7L--JLJF7||LJLJLJL7FJL7F7F7|F777L-.FLF-
.||LL|-L7LLJJ|L--7LJL-----7L-7L7F7LJL--7F7L-----7F--JL7FJL7|F-7|L7|FJFJL--JFJF-J|||FJL-7F7L7LJF7LJL-7|F-----J||L------7|L-7|||||LJL-7-.FL..|
FLJL|JJ.-.F--JF7FJ|F7JF7F-JF-JFLJL----7LJL7F7F--J|F-7FJL-7|||-||FJ|L7|F7F--JFJF7LJFJF7FJ|L7L-7|L---7|LJF-----JL-------JL7FJLJ|||F---J|-|7-L7
|L7F7-.LL-L-7FJLJ.FJL-J|L-7L-7F-------J.F7|||L---JL7LJF-7LJ||L|||FJFJ|||L7F7L-J|F-JFJLJJL7L-7LJF---JL7LL-----7F-7JF7FF7LLJ|F7|||L----77|L.||
F|FLLJ-||||L||F---JF7F7L--JF-JL------7LFJ|||L7F-7F-JF-JFJF-JL7LJ||L|FJ|L-J||F7FJL-7L7F-77L-7L-7L7F7F7L7F7F---J|FJFJL-JL---7|||||F----JL--LJ|
|L7.||7LF7-J||L----JLJL7F--JF-------7|FJFJLJLLJFLJF7|F-J7L-7FJF7LJ|||F|F7FJ||LJ|F7|FJL7L---JF7L7LJLJL7LJ||F--7||FJF7F7F7F7LJ||||L-7J||-|-FF-
F-J.LL|JLJ7.LJF7F-7F7F7LJF7.L7F----7LJ|FJF7|.F7F7J|LJL7F7F7LJF|L-7FJ|FJ|LJ.|L7F-JLJ|F7L---7FJL7L---7FJF7LJ|F-J|LJFJ||LJ||L7FJ||L--J--|FJJ|7.
||-|.|.L|7FF7.||L7LJ||L--JL7.LJF7JFJ|FJ|F|L7FJLJL7|F--J|||||F-JF-JL-JL7L--7|FJL-7F-J|||F7FJ|F-JF--7|L7||F-JL--JF7L7||F7LJ|LJL|L--7JJLL-L-777
-7.LL77-|7L|L-JL-JF7|L---7FJF7FJL7L-7|FJFJFJ|F---J||.F7|LJL7|F7L7F-7J7L7F-J||F7FJ|F-JL-JLJFJL--JF-JL7||LJF-----JL-JLJ|L-----7|F7FJ777.7F-|77
LLJ.L|-.LJ|L------J|L-7F-JL7|||F7L--J|L7|FJ-||F7F7|L7|||F7FJ||L-J|FJJF-JL7FJ||LJFJL7F7F7F7|F---7L--7LJ|F-JF------7F7FL7F----JLJLJ7.LLLJ|-LLJ
|.|-.L|.|.FJF-7F---JF7|L--7LJ|LJL----JFJ||F-JLJLJ||FJ||||LJFJL7-FJ|FLL-7FJL7|L-7L7FJ|||LJLJL--7L-7|L--JL--JF----7LJL--JL---7.LLFL|.FFJ-J7.LJ
FL7F-JF|-L7-L7LJF7F-J||-F7L-7|F7.F7F-7L-JLJF7F7F7||L-JLJL--JF7L-JFJ7-L7LJJ7LJJLL-JL-JLJ-F7F7F7L-7|F7F------JF7F7L----------J..FF-7-|J||.--JF
FJL|7F7J|.||LL7FJLJF-JL-JL--JLJL-JLJJL-7F--JLJ||LJ|F7F7F-7F7|L--7L-77-||7.F|.LF7F7-F----JLJ||L--JLJLJF------JLJL---7F------77FJ.FL7|F|--L7F7
L7-L7|J|7-L7-FLJFF7L-7F--7F---7F7F-7F-7|L----7LJF7||LJ|L7|||L--7L7FJ.LLF-F-F--JLJL7L------7|L--------J7F----7F7F7F7LJF7F---JF|..FLJJ|7.|F|7.
FJ|L||L--.|.FF7F7||F-J|F-JL7F-J|||JLJFJL7F7F-JF7|LJ|F7L-JLJL-7FJ-LJ.||||-JFL7F---7|F-7F---JL-7|F--7F7F7L---7LJLJLJL--JLJF7F77F7F7.|-L--FJ|F|
JFJJ|FF|.F7-FJLJLJLJF-JL-7FJL7FJ||F-7L-7||||F-J|L-7LJL-7F7F-7||.FJLJJ-JJ.LJ.||JF-J||FJL---7F7L-JF7LJLJ|.F--J-F7LF----7F7|LJL-JLJ|.J.FJLL.JJJ
L----7L7.-JFL7F7F-7FJF---J|F-JL7LJL7L--JLJLJL-7L--JF7F-J|||FJLJ7.|.|7.|FF7.FLJFL7FJ|L7F7F7LJL7F7|L7F-7L7L---7|L7L7F--J|LJF7F----J.||L7.F-L|.
|||7LLJ|-J-F-LJ||FLJ-L----JL---JF7JL---7F7F7F7|F7F7|LJF-JLJL--7.FL7|FJ-F||77JFF-JL7L7LJLJ|7F7LJLJ-||7L7|F---J|FJFJL7F-JF-J||F7F7J7FJ7-L-.LF7
|-FL.|FJ7|.-JJ.LJF7F7F7FF---7F7FJ|F----J||||||||LJ|L--JF-7F7F7L7J|F77J-FJ77|LLL-7FJ.L--7FJFJL7F7F-J|F-J|L----JL-JF7LJF7L7LLJ|LJL--7JLL-LJ.LJ
J.7.7FJ.F----7FLL|||||L7L--7LJ|L7|L-----JLJLJLJL-7|F---JL||LJL-JL-JL-7FJLF--7FF-JL7F---J||L-7|||L--JL-7L--7F-7F-7|L7FJL7L7F7|F----J77.L|-7.|
FLJ---.FF7F--L-F-JLJ|L7|JF7L-7|FJL------7F-7F-7F-J|L7LF-7|L--7.LFJ.|L-JF.L-7L7L-7FJL---7L7F7||||F-7F7FL--7LJFJL7||JLJF7L7LJLJ|LF--777-FF-|.|
F--.F.LF.|J.||.L---7L-JL-JL--JLJF7F7F7F7LJ.LJF|L-7L7L-JFJL-7FJ7|FJ-|L|-|-.LL7L--JL7F7F7|FJ|LJLJ|L7LJ|F7F-JF7|F7||L---JL7L---7L-JF7L--77|L-7|
7-L7|.||-7JFLJ-F7F7L---7F7F--7F-JLJLJLJL---7F7|F7|FJF-7L7F7LJ|-FJFJJ.F.|LF-7|F-7F7LJLJLJL7L---7|FJF7LJLJF-J|||LJ|F--7F7L--7JL--7|L---JJ77-J7
L-7LF|7|L|L|-LFJLJL-7F7LJ||F-J|F------7F-7FJ||LJLJL7L7L7LJL--7.-.|.F7.F7.L7LJ|FJ|L------7|F7F-J|L7|L----JF-JLJF7LJF-J|L--7L7F7FLJ7L||JLLLJL|
JJ||LF-7-L-|JJL--7F7LJL--J|L--J|F----7|L7LJ-|L-7F-7L7L7|F7F-7|-|7|-|.F|7J7|F7|L-JF7F----JLJ|L-7|LLJF7F--7L---7|L-7L-7L-7.L7LJ|F7F77F7J|.L-F|
L-.-.|L|F7F7JF7F7||L--7F7FJF---J|F---JL-JF--JF7LJFJFJFJ||LJFLJF77|.7J||7FF|||L7F-JLJF7F---7L--JL7F-JLJF7L--7FJ|F-JF7L--JF7L-7||LJL7|L777|-J.
.|.|L|.7JFJL-J|||LJF7FLJLJLL----JL-------JF7FJ|F7L7|FJ7LJF7F7FJJJ7-|.LJL77LJL7|L--7FJLJ7F7L7F7F7LJF-7FJL--7LJ-||F7|L7F-7||F7LJL7F-J|FJF777F7
--77J|-JLL---7LJL-7||F7FF7F---------7F----J|L7|||FJ|L7F-7||||77|L|-J-JFLF7JJLLJF-7LJF---J|.||LJ|F-JFLJ.F7FJF7FJLJ||FJ|FJ|||L7F7|L--JL-J|7JL|
.FJ--J.7L|7LFJF--7|||||FJ|L-----7F-7LJF---7|FJLJLJFJFJ|FJ|||L7F7F7--J---|L77F|.L7L-7L-7F7L7||F-JL7F7F--JLJF|||F--J|L-JL-JLJFJ|||F---7F-JJJ||
F7.LLF7.F-7L|FJF-JLJLJ|L7L---7F-JL7L-7|F--J|L--7F7L-JFJL-J|L7||LJ|7.F7L||FJF7F-FJF7L--J|L7LJ|L---J|LJF-----JLJL---JF7F7F7F7L-JLJL--7LJ|J---|
|L7JLLF-7..F||FJF7F7F7L7|F7F7LJF--JF7LJL7F7L---J|L7LFJF--7|FJ||F-J.F7|FFJL7|L7JL-JL--7FJLL7FJJF---JF7|F---7F7F-7F7FJLJLJ|||F-7F7F-7L--77.|.|
L-J..||7F.L-LJL-JLJ||L7LJ||||F7|FF7||F-7LJL7F---JFJFJFJF7LJL7LJL7F7||F7L7FJL7|F--7F7FJ|F--J|F-JF---JLJL--7LJ|L7|||L--7F7LJLJFJ||L7L---JJ-FF7
F-7F-7|FF-.FF7-F--7LJ7L--JLJ||LJFJLJLJ|L---J|F--7L7|FJFJL--7L7F7LJLJ||L-JL-7||L-7LJ||FJL7F7||F7|F---7F---JF7L7||||F--J|L---7|FJL-J7F7.|J.|||
F-F-|L-7|FLL||FJF-JF7F7F7F7-LJF7L--7F7F7F7F-JL-7|FJ||FJF---J7LJL7F--JL7F---J|L-7L-7|LJF7||LJLJLJL--7|L----JL7|||||L7F7|F---JLJF7-F7|L77L--JJ
L7|LLFJFJ77L||L7|F7|LJLJ||L---J|.F7LJ||LJLJF7F-J||FJ|L7L--7F-7F7||F77-|L7F-7L-7L-7||JFJ|LJF----7F--J|FF7F--7|||||L7||||L------JL7|LJFJ|.F|LJ
|LJ-F-.|.FF-JL-J|||L---7|L----7|FJ|F7LJF7F-JLJF7LJL7|FL7F-J|FJ|||||L7FJFJ|FJF7|F7||L7|FJF7L--7FJL--7L7||L-7LJLJLJ-|LJLJF7F-7F---J|F-J|F777F7
FLJ-J|LJFLL7F7F7LJL7JF-J|F7LF7||L7|||F-J|L----JL7F7|L7-|L-7||FJLJ||FJL7|FJL7||LJ|||FJ|L7|L---JL--7FL7LJL--J7F----7L7F-7|LJ7LJ7F7J||FF7||7-||
|7JLLFJ.|FLLJLJL--7L-JF7LJL-JLJL-JLJ|L-7|F7F-7F-J||L7L7|F-J|||F-7|||F7|||F-J|L7FJLJ|FJFJL-7F--7F-JF-JF-7F--7|F---JJLJ7|L---7F7|L-JL-JLJL7JL7
|777||JL|F77-LF---JF7FJL7F---------7L--J|||L7||F7||L|FJ||F7||||7LJ|LJ||LJ|F7|FJL-7FJL7L7F7LJF-JL--JF-J|LJF-J||F-7LF7FFJF7F-J|||F---7F---J..-
-JF7L7.|F||F7LL----JLJJ|LJF-------7L---7|||FJLJ||||FJL7|LJ||LJL-7FJF-JL-7||LJ|F-7||F7|FJ||F7L--7F-7|.F---J|FJLJFJFJL7|FJLJF-JLJL7F7LJFF|.F.|
LFJ|.LFF7||||F-----7F77F-7L----7F7L--7FJLJLJF-7|LJLJF-JL-7|L7F--JL7L-7F-J||F-JL7||LJLJL7||||F7LLJFJL7L-----JF-7L-JF-JLJF77L-7F--J||7F77F7-J7
L7.LF7FJ|||||L----7LJL7L7|F7F-7LJL--7|L7|F7FJFJL--7FJF--7|L7|L---7L7FJ|F7LJL7F7||L---7FJ|||||L7F7L--J7F-----J-|F-7|.F7FJ|F-7|L---JL-JL-J|.JJ
F|7-L7L7LJ||L7F---JF-7|FJLJ|L7L-----JL7L-J|L7|F-7F||F|F-JL7LJF---JFJ||LJL7F7LJ|||F--7||.|||||FJ||F7JF7L--7F7F7LJ-|L-J|L7|L7||F----------J7|7
F-J.|FJL-7||FJL-7F-JJ||L--7L-JF-7F-7-LL-7FJFJ||FJFJL7|L-7L|F-JF7LFJFJF7JFLJ|F-J||L-7|||FJLJLJL7|||L-JL7F-J|LJL-7FJF--JFJL-JLJL-7-F7F----77LJ
LJLFFF7F-JLJL---J|F--JL--7L7F-J.LJLL7-F7LJFJFJ||FJF-J|F-JFJL-7|L7L7L-JL7F7FJ|F7||F7|LJ|L--7F--J|LJF--7|L-7|F---JL-J.F-JF7F7F7F7L-JLJF7F-J7|.
.L77.LFJF-7F-7F--JL---7F7L-J|F7-F77FJFJL-7L7L7||L7L-7||F-JF7FJL7L7|F---J||L7||LJ||||F-JF7FJ|F7-L--JF7||F7LJL-7F7F7F-JF7||||||||F7F--JLJF77J.
-||LF-L7|FJ|FJ|-F7F---J|L---J|L-JL-JLL7F-J7L7LJL-JF7|||L-7|LJF7L7|||F7F7|L7||L-7|||||F7||L7LJL7F---JLJLJL----J|LJLJF7|LJLJ||||LJ|L----7|L77L
FJ77FJ.LJL7||FJFJLJF-7FJJF---JF--7F7F7|L7LF7L--7F7|LJ||.FJ|F7|L-J||LJLJ|L7||L7FJLJ||LJ||L-JF--JL---7F7F7F----7|F-7FJLJF--7LJ||F-JF---7LJFJ7J
F||L7|-J7|LJLJJ|F7FJJLJF7L7F--JF7LJLJLJFJFJ|F77LJ||F7|L7L7|||L7F7LJF---J.|LJFJL--7LJF7|L--7L7LF7F7FLJLJ||F---J||LLJF7FJF7L-7|||F7L--7L7FJ|||
-F7JL77F-LLJJ-|LJLJF7F-J|-LJJF7|L---7F7|FJFJ|L-7FJLJ|L7L-JLJL7LJ|F7L7LF-7L7FJF7F7L-7|LJF--JFJFJLJ|F---7LJL---7|L---JLJFJL--JLJ||L7F7|JLJFLJJ
|LL7-L-L7|FJ..FF---J|L-7L----JLJF--7LJLJL7|JL-7|L7F-JLL7F----JF7LJL7L7L7|FJ|F|||L-7||F7L--7L7L7F-JL-7FJF--7F7LJF----7FJF----7.LJFJ||L7||JJJ.
LFJF-L|-LF|-77.L---7L7FJF--7F7F7|F7L-7F-7|L7F7||FJ|F---JL7F7F7|L-7.L7L7||L7|FJ|L-7LJLJ|F7FJFJFJ|F7F-J|JL-7LJL-7L---7|L-JF---JF-7L-JL7|-|F|LF
L77J||L77||7|.F7JF7L7||FJF-J|||LJ|L--JL7||FJ||||L7|L----7||||||F-JF7|FJ||7||L7|F7L-7F-J||L7|-L7|||L7FJF7|L--7LL7F7FJ|F-7L7F7|L7L-7J7||J.L-.|
FL|FF|JF-LJLFF||FJL-JLJ|FL-7||L7FJF-7F7||||FJ|||FJ|F7F-7|||||LJL7FJLJL7||FJL7||||F7|L7FJL7|L7FJ||L7|L7|L7F-7|F7LJLJ|LJFL7LJL7FJF-JJ-LJ7-||-7
J|L7.L7J|L77L|||L7F-7F-JF-7LJL-JL7|FJ|||||||FJ|LJJLJ||F||||||F--JL7F7F||||F-J|||LJ||FJ|F-J|FJL7LJF|L7||FJL7|LJL------7F7|F-7||FJJ.FFLJ-FJJ7J
LFJ|-L|--7JFF-J|-|L7LJF7L7L------J|L7|LJLJ|||FJF----JL7||||LJ|F7F7LJL7LJ||L7FJ||F-J|L7|L7FJL7FJJF-JFJ|||F7|L7F-----7FJ||LJ.|LJ|JL|F7J|||-L7.
F|L--7L-LLF7|F7|FJFJLFJL7|F7F--7F7|FJL--7FJ|||JL7F7F7FJ|||L7FJ|LJL7F7L-7||FJL7||L7FJ7|L7||F-JL-7|F7|FJ|LJ||FJL--7F7LJFJL--7|F-J|LL-JF77|.-J7
L.F..LJ7LF|LJ||LJFJF-JF7LJ|||F7||LJL-7F-J|FJLJF-J|||||FJ|L7|L7L7F-J||F7|||L-7|LJFJL-7L7|||L7F7FJ||||L7|F-J|L7F-7LJL--JF7F7||L--7.||FL-77-|7|
|7L-|7|L.L|F-JL7FJJL--JL-7|LJ|LJ|F---JL-7||F7-|F7||||||FJ7LJ-L-JL7FJ||LJ||F-JL-7L7F-JFJ|LJ7LJ||-LJ||FJ|L7FJFJL7|F7F-7FJLJ|||F7FJ-FF777L7.L|J
L77LJ-|.77LJF--J|F-------JL-7L7FJL-7F7F-J|||L-J|LJ||||||FF7F-----JL7|L7-LJ|F7F7|FJL-7L7|F7F--JL--7LJL7|.||7L7FJLJ|L7|L--7LJLJ|L77LJL7JLJ7-|J
.J77-LJF-JJ7|F-7|L7F7F7F7F7FJ|LJF7FJ||L7FJ||F7FJF-J|||||FJLJF-7F7F-J|FJF7FJ|||LJL7F7L7|||LJF-7F7FJJF-JL7||F-J|F7|L7||F-7L-7|-|FJ7LJ7|.||FJL7
|J.|F7JLFJ|L||-LJFJ|||||LJ||F---JLJFJ|FJ|FJ||LJ.L7FJ||LJL--7|FJ|||F-JL-J|L7|||F--J||FJ|||F-JL|||L-7L-7FJLJL-7LJ|F7LJLJ7L-7L7FLJJ--LF-FL-7-FJ
77-LJ7-LJJFJLJJF-JFJ||||F7LJL7F--7FJFJ|FJ|FJL-7-FJL7||.F---J||FJ||L7F-7FJFJ|||L7F7||L7LJLJF--J|L7FJF7||F-7F-JF7LJL7LF--7L|FJJLJ.LF-L.7J7L.J7
|F7FL7-7L7LFJ.|L-7|FJ||||L---J|F-JL7L-JL7|L7F7L7L7FJLJFJF7F7|||FJ|FJL7|L7L7|||FJ|||L7L7F--JF7FJFJ|7|LJLJFJL7FJL7F7L-JF7L7LJJLFJFJ|FLF..|F--7
7|JFLJFJ|LFJFJ7FFJ|L7|LJL7F7F7|L7F7L--7FLJFJ|L7L7LJF--JFJ|||||||FJ|F7|L-JF||LJL7|||FJFJL--7||L7|FJFJF-7FJF7|L-7|||F-7|L-JF7-LJ-F7FL-|.F7LJ7|
-JFFJ7.7LF--7L-FJFJFJ|7F-J||||L7LJ|F--JF-7L7|FJFJF7L7F7|FJ|||||||FLJ|L-7F7|L7F-J|LJ|FJF7F-J||FJLJ7L7|J|L-J||F-JLJ|L7|L---JL-7J.L|7J-L--F7.LJ
J-JJ7L-JF7J-J.LL-JLL7L7L7FJ|||FJF-JL7F-JFJL||L7|FJL-J|LJ|FJLJ||LJF--JF7LJ||FJL-7|F7|L-J|L7FJ|L7F---J|FJF-7|||F7F7|FJ|F7F-7F-J7FFJ|-7.-JFJ7LJ
..LFJ7-7777|7F-7|7|FL7|FJL7LJ||FL--7|L-7L--J|FJ||F-7FJ-FJL--7||F-JF7FJL-7|LJF--J||LJF--JLLJFJFJL-7F7|L7|FJ||LJLJ||L7||||FJL7|JL--7F-7.-L-JJ|
77J|LJ7JLJJ7-L77LJ|F-LJL7FJF-J|F---J|F-JF7F7||FJLJ-LJLFJF-7FJLJ|F7||L7F7LJF7|F-7||F7L--7-F7L7L7F-J|||FJ|L-J|F7F-JL-J||||L7FJ-77-L||LF7FJ77-F
-J7L-7F.J7FF.LLJ.F-F7|-L||-L7FJL-7F7||F7||||||L----7F-JFJF||F77LJ||L7LJL-7|LJ|FJ|LJ|F7FJFJ|J|FJL-7|||L7L-7FJ||L----7LJLJ-|L7-777JL7.LLLLFJ.|
LJF7--JJF7-L-|L7F|LLLF7JLJ7FJ|F-7||||||||||||L7F7F7|L7FJF-JLJL-7FJ|FJF7F-JL7FJ|FJF-J||L7L7L-JL7.FJ|||7|F-JL7||F----J-|7J.L-J-JJJF7L-F.L|L|--
|F-|F|JLFJ||-7F-FFJ|-LFF7F-|FJL7LJ|||||||||LJ.||||||FJ|.L-7F7F-JL7|L7||L-7FJ|FJL7L-7||FJ|L7F7FJFJFJ||FJL-7FJ||L-7F7.F-7-7-L7L-7F-|77.FF7JL-7
F--J7|7FF-LL7-J.7..|-LFJJ-LLJF7|F7|||||||||F--J|LJLJL7|F--J|||JF7LJ||||F7|L7|L7FJF-J||L-7FJ|||FJFJL||L7F-JL-J|F7LJL7L7|7|7LF--F-7|-7FLLL-JJ|
|.FJFJ||.FFJ|.|.|J-J.|.L|L-LFJLJ|||LJLJ||LJL7F7L---7J||L--7||L-JL7FFJ||||L7||FJ|L|F-J|F7||FJLJ|FJF-J|L||F7F--J||F-7L-JL--7J|J.|L|J.L|.L7JLF|
-7--7--J.-7J---FJ7|FL7-7JF7LL--7|||F---JL7F7LJ|F---JFJL7F-J|L7F-7|FJFJ||L7|||L7|FJL-7||LJ|L7F-JL7|F7|FJLJ|L--7|||-|F--7F7L7L7-L-J.|FJ7-L-JLJ
LJLLL|F---L77F7||7J7.|L77L-J.|7|||||F-7F7||L-7|L-7F7L7FJL7FJFJ|FJ|L7|FJL7LJLJ|||L7F7||L-7|FJL7F-JLJLJ|F-7|F--J||L7LJLL||L-J7L7J.LFLJ.7..F7F|
L-L7..7J.FJ-7F7JJJ.L7|-L|-|7F-FJ|LJLJFJ||||F7LJF7LJL7LJJFJ|FJFJL-JFLJ|F7L----7LJF|||||F7|||F-JL--7F--JL7LJ|F7FJL7L-77FLJ-JJ|L7FFF7|LFJ|-F-F-
L-----|-|||J||.||F|JLJFLJ7|L-7L-JJ-|L|FJLJLJL7FJL7F-JF-7L-JL7|7L|FFF-J|L-----JFLLLJ||||||||L-7F--JL-7F7L7|LJ||JFJF7L7JLJL|7.F-J7LJ|-|FJFJ.||
FF--J-J.LF77|JFLL7JJ.FLF.L7|JF|||LF|.||JLF---JL-7|L--JFJFLJJLJ77L-F|F7L----7JJF-||L||LJLJLJF-JL-7.F-J|L7|F--JL7L-JL-JJ.|LJJJ-7|-7-|L7JJFFFF7
LLJFLFJ7FJL.|J|77J|7.FJ|.L-77-LL77J|.LJ-FJF7F7F-JL7F7FJF|LLLLL|-|L-LJL7F7F-J7LJ.77L||LF|7JLL7F-7|-L-7|FLJL7F-7L--7J7.|-L-|FLFFF-J|F-||FJ7.LL
LL--7|.JL-JFJ7J|7.7.F7LF|JJ|.|7|-|.L7|.|L7|LJ|L7F7||LJ--7J-F7LJL7-|JLFJ||L-7-J7F|L7LJ-L-JFLFJL7LJ-F-JL---7||FJF--J-|7F7.7LF-FJL77FJ.7-J.L-JL
F7FF||LJ|.L7|.LF77LJ||.-|.|LJ-L-77-7FFF-.LJF-JFJ|LJL7L||.--7J..F|-77FL7|L7FJ7||-|--.|.||JL-|F-J.|LL-7F7F7|LJL7L-7JJFFJJ---J-J.-F-LJ-FJLF7|7J
F7-|L77LL-FLF7.L|.FL|7-LL-LJL-L-FJF-JL|-7-LL-7L7L7F7L-77-JFF--7-L-LJ7.LJJLJF-|J.77F-LJJJ7|LLJ.|-F-JLLJLJ||.|LL7FJ.F-JJJFJFJ.J.LF7-|FL-7LJLL7
FL7FJJ7F--J--7--J7|.LJF7-7L7.FJ||-JL7.|.F.|JFJFJFJ|L7FJJLFJ|L-JFLF|F-L7J7LJJL||L-7J|L-JJ|--JJ77F|L|7J|-LLJ--JFJL-7JLL7-|FJJ7JF-JL-|||-J7JJ7F
L-|JJ-L-FJJ.FJLFLJL7JJ---|J|-|FLF-|7.FLFF-JFL7L7|FJJ||F-7--|7.|-J.LL.L-77..|.L7.-J|L|--|LJ..L7LF|F7-.|77.L7FLL-7FJ..L|.LFJL-7F7LFL.FJL||LF|.
|.||7FJL|JJ7|7JFJ|J.|F-7L|-L|L|FJ7LJ-FJF|.FF-JFJ|L7-|LJFJ.F7F-7L77|L-|7F-77L|.7-LFJ|FJ-7|-|7..--J||L-J|-7-FL-|-LJJ-J.L7FJ7J.-JJ.JJ7LJ.L|.7|.
-|L--F7|||-J7F-JLFJ.F-77L7..7-L|JJ7JL7||LFLL--JLL-J-|F7|.JLJJ7|JLFJ-F-FLFJ77.7L|.|FLJJL|F.LJF7|L7F77F|..JLL7-77|.L7L|77JL-.-7|F|.F|.LJ7JFL|-
|JL.L|-L|J-L--..F.77--|JLJ-LLJ-7JL.|J.L|.|JLJ.|LJJJ.LJLJ7LFJJLF--|LF-J..7.L-.7L7J.J-7JJFJ-L7J|J-LJ7--J-7JJ.J.LLL---7LLJ.--J-JJ-L-JJ-|LF--JLJ"#;
