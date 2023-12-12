use aoc2023::grid::CharGrid;
use aoc2023::utils::run;

fn internal(input: &str, multiplier: u64) -> u64 {
    let cg = CharGrid::new(input);
    let rows =  cg.rows_where(|r| r == '.').into_iter().flatten().collect::<Vec<_>>();
    let cols =  cg.columns_where(|c| c == '.').into_iter().flatten().collect::<Vec<_>>();
    let points = cg.points_where(|c| c == '#');

    let mut sum = 0;
    for (i,p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i+1) {
            let mut 
            count  = rows.iter().filter(|p| p.y >= p1.y.min(p2.y) && p.y <= p1.y.max(p2.y) && p.x == p2.x).count();
            count += cols.iter().filter(|p| p.x >= p1.x.min(p2.x) && p.x <= p1.x.max(p2.x) && p.y == p2.y).count();

            let count = count as u64;

            sum += p1.distance(p2) as u64;
            sum += (count * multiplier) as u64;
            sum -= count as u64;
        }
    }
    sum
}

fn part1(input: &str) -> u64 {
    internal(input, 2)
}

fn part2(input: &str) -> u64 {
    internal(input, 1000000)
}

fn main() {
    run(r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#, part1, Some(374));

    run(INPUT, part1, Some(9965032));
    run(INPUT, part2, Some(550358864332));
}

#[allow(dead_code)]
const INPUT: &str = r#"..............................................#......#...............................................#..........................#...........
.....#......................................................#.............................#.................................................
............#...................#........#....................................#..............................#...........#..................
...............................................................................................#............................................
.#....................#...........................#......................................................#...........#.....................#
............................................................................................................................................
...................................#.......................#...........................#....................................................
....#...................................................................#......#..................#.............#...................#.......
..............................................#.............................................................................................
...........#.............................#......................#...........................................................................
...................#.........#..............................................................................................#...............
.....................................................#.....................#...................#..........#...............................#.
...............#................................#.......................................................................#...................
....#..............................#......................................................#...........#..........................#..........
..............................................................#.............................................................................
...........................#.................#...............................................................#.....#..................#.....
.......#...........#..............................................................#.........................................................
..............#...........................................................................................................................#.
...............................#..................#.........................................................................................
..#............................................................................#............................................#...............
.........................#..........#.....................................#.................#...................#...........................
.................#......................................#............................#................................#................#....
.......................................................................................................#....................................
.....#................#........................................#............................................................................
........................................................................#................................................#..................
...........................................#...................................#..............................#.............................
..........................#..............................#..........................................................#.....................#.
..#............#......................................................................#..........................................#..........
................................................#..............................................#............................................
.......................................#..............................................................................................#.....
.....#......................#.................................#............................................................#................
................................................................................#..........................................................#
#.........#...........................................................................................#.....................................
.....................................................................................................................#...........#..........
...........................................................#..............................#.................................................
..............#.......#.........#........................................#...........................................................#......
......................................#.....#.....................#..........................................................#..............
...........................#............................#.........................................#......................................#..
...#...............#...........................................................#.....#..........................#...........................
...................................#........................................................................................................
............#....................................#.............#..........................................#..........#..............#.......
.......................#................................................................#...................................................
......#...............................#...............#.................................................................................#...
#..............#...........................#................................................................................................
............................#.............................#................................................................#................
..........#........#..............................................#...................#........#...............#............................
.......................................................................................................................#....................
.........................................#..............................#.....#.............................................................
.....................................................................................................................................#......
.....................................................#......................................................................................
........#.....#....................................................#..................................#.....................................
.....................#.........................#.......................................#...................................#................
...#.........................#..........................#..........................................................#............#...........
............................................................................................................................................
...........#...................................................#.............................#..............................................
..................#.........................................................................................#...............................
..........................#................................#..........................................#.....................................
................................#.....#...................................#............#....................................................
..............#............................#.......................#............#............................................#..............
.....#..................................................#.........................................#............#.....#.....................#
............................................................................................................................................
........................................#...............................................................#...................................
.....................................................#......#.................................#.............................................
..........#............................................................................................................#........#...........
............................................................................................................................................
.#................#.......#.......................#...................#.....#.........................#................................#....
...............................#..................................................#..............................#..........................
........#....................................#....................#......................#..................................................
............................................................................................................................................
.........................................................#...................................................#.........#....................
..............................................................#..............................................................#..............
......................................#...........#.........................#........................#................................#.....
.....................#......#...............#...............................................................................................
............#.......................................................................................................#...........#...........
...................................................................#......................................#...............................#.
...#................................#..........................................................................#............................
.........#.....................................................#............................................................................
..............................................................................#....................#........................................
.................#.....................................#.............................#.............................#....................#...
........................#......#.............#..............................................................................................
.....................................................................#.....................................#...............#................
............................................................................................................................................
#..............................................................................................#............................................
.................................#......................................#..............#....................................................
.................................................#.....................................................................................#....
...#...............#.....................#......................................#........................................#..................
........................#......................................#...........................................#.......................#........
....................................................#.....................#................#.........#......................................
............#...............................................................................................................................
...........................................................#.........................#.........................................#............
.....................................#............................#.........................................................................
......#.............................................................................................................#.......................
....................#...........#.............#...............#.........................#................................#...........#......
.........................................#.....................................#............................#...............................
...........#............................................#...................................................................................
.................................................................................................................................#..........
...#..............................................................#.............................#.................#.........#............#..
......................#..............................................................#................#................#....................
.........#.....................#......................................#.....................................................................
.........................................................................................#..................#...............................
.................#.........................#..............#...............................................................#.................
.....#...............................................#............................#.........................................................
....................................................................................................................#.......................
..........#..........#.......#................#......................#........#............#.......#........................................
...................................#.....#..................................................................................................
.............................................................#.........................#.........................#......#...................
.........................................................................#.......#.......................#........................#.........
.......................#.........................................................................#...........................#..............
........#.........#.........#...................#...........................................................................................
#.........................................................#............................................................................#....
........................................................................................#............#......................................
..................................#........................................................................................................#
..............#.....#.....................#..............................#.......................................#..........................
...#..........................................................#.............................................................................
.........#.......................................#............................#...............#..........................#........#.........
...........................#.................................................................................#..............................
.........................................................................................#...........................#......................
.....................................................................#.................................................................#....
.....#..................#.......#.........#.................#................................................................#..............
.....................................#................#....................................................#.......................#........
...............................................#............................................................................................
...........#........#.......#...................................................#...........................................................
................................................................#...........................................................................
................#............................................................................#..............................................
#......................#...............................................................................................#....................
.................................#...............................................................................#............#..........#..
.....#..........................................#..........#..........................................#.....................................
...........................................#...................................#.................#..........................................
..........#.......#...........#...............................................................................#.............................
.......................................#............................................................................................#.......
...................................................#.....................................................................#..................
#......................#........................................#..........................#...............#................................
.............................................................................................................................#..............
.............................................#.........................#..............#.....................................................
.......#.............................................#.......#...................................................................#..........
............#.....#.....................#..............................................................#............#.....#.................
...#.........................#................................................................................#.............................
...............................................#..................#.........................................................................
...................................#....................................#.......#........#.........#..........................#.............
..........#.....#......................................................................................................#..............#....."#;
