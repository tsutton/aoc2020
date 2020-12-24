//! We can conveniently represent a hexagonal grid with two coordinates, but
//! rather than the typical perpendicular (x,y), it'll more more convenient
//! to have east = (1,0) and northeast = (0,1).
//! And, as in previous days, rather than try to implement dynamically resizing
//! 2D arrays, we'll just use a HashMap (of coord => color )to track colors of known tiles.
//! For this to work best, we should separately keep track of the range of known (x,y) values in the map
//! since we want to iterate not just over all coords in the map, but over also adjacent ones.
use std::collections::HashMap;

fn parse_line(input: &str) -> (i32, i32) {
    let mut chars = input.chars();
    let mut coords = (0, 0);
    while let Some(c) = chars.next() {
        match c {
            'e' => coords.0 += 1,
            'w' => coords.0 -= 1,
            'n' => match chars.next().unwrap() {
                'e' => coords.1 += 1,
                'w' => {
                    coords.1 += 1;
                    coords.0 -= 1;
                }
                x => panic!("unknown character following 'n': {}", x),
            },
            's' => match chars.next().unwrap() {
                'w' => coords.1 -= 1,
                'e' => {
                    coords.1 -= 1;
                    coords.0 += 1;
                }
                x => panic!("unknown character following 's': {}", x),
            },
            x => panic!("unknown character {}", x),
        }
    }
    coords
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Color {
    Black,
    White,
}

impl Color {
    fn flip(&mut self) {
        match self {
            Color::White => *self = Color::Black,
            Color::Black => *self = Color::White,
        }
    }
}

#[aoc(day24, part1)]
pub fn day24(input: &str) -> usize {
    let mut tiles = HashMap::new();
    input
        .lines()
        .map(|line| parse_line(line))
        .for_each(|coords| tiles.entry(coords).or_insert(Color::White).flip());
    tiles.values().filter(|&x| x == &Color::Black).count()
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Bounds {
    min_x: i32,
    min_y: i32,

    max_x: i32,
    max_y: i32,
}

impl Bounds {
    fn new() -> Bounds {
        Bounds {
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
        }
    }

    fn update(&mut self, x: i32, y: i32) {
        if x < self.min_x {
            self.min_x = x;
        }
        if x > self.max_x {
            self.max_x = x;
        }
        if y < self.min_y {
            self.min_y = y;
        }
        if y > self.max_y {
            self.max_y = y;
        }
    }
}

#[allow(dead_code)]
const EXAMPLE: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

#[aoc(day24, part2)]
pub fn part2(input: &str) -> usize {
    // let input = EXAMPLE;
    let mut tiles = HashMap::new();

    let mut bounds = Bounds::new();
    input
        .lines()
        .map(|line| parse_line(line))
        .for_each(|coords| {
            bounds.update(coords.0, coords.1);
            tiles.entry(coords).or_insert(Color::White).flip()
        });
    let n_steps = 100;
    for _ in 0..n_steps {
	// let black_count = tiles.values().filter(|&x| x == &Color::Black).count();
	// println!("black count: {}; bounds: {:?}", black_count, bounds);
        let output = step(&tiles, &bounds);
        tiles = output.0;
        bounds = output.1;
    }
    tiles.values().filter(|&x| x == &Color::Black).count()
}

fn step(
    grid: &HashMap<(i32, i32), Color>,
    bounds: &Bounds,
) -> (HashMap<(i32, i32), Color>, Bounds) {
    let mut new_grid = HashMap::new();
    let mut new_bounds = Bounds::new();
    for x in (bounds.min_x - 1)..=(bounds.max_x + 1) {
        for y in (bounds.min_y - 1)..=(bounds.max_y + 1) {
            let neighbors = black_neighbors(grid, (x, y));
            match grid.get(&(x, y)).unwrap_or(&Color::White) {
                Color::Black => {
                    if !(neighbors == 0 || neighbors > 2) {
                        new_grid.insert((x, y), Color::Black);
			new_bounds.update(x, y);
                    }
                    // no need to insert in the white case
                }
                Color::White => {
                    if neighbors == 2 {
                        new_grid.insert((x, y), Color::Black);
			new_bounds.update(x, y);
                    }
                    // no need to insert in the white case
                }
            }
        }
    }
    (new_grid, new_bounds)
}

// You could imagine doing this in a smarter way e.g. hardcoding the vec
// offsets = ((1, 0), ...) once but eh.
fn black_neighbors(grid: &HashMap<(i32, i32), Color>, (x, y): (i32, i32)) -> i32 {
    let mut ret = 0;
    if let Some(Color::Black) = grid.get(&(x + 1, y)) {
        ret += 1;
    }
    if let Some(Color::Black) = grid.get(&(x - 1, y)) {
        ret += 1;
    }
    if let Some(Color::Black) = grid.get(&(x, y + 1)) {
        ret += 1;
    }
    if let Some(Color::Black) = grid.get(&(x, y - 1)) {
        ret += 1;
    }
    if let Some(Color::Black) = grid.get(&(x - 1, y + 1)) {
        ret += 1;
    }
    if let Some(Color::Black) = grid.get(&(x + 1, y - 1)) {
        ret += 1;
    }
    ret
}

#[cfg(test)]
mod test {
    use super::parse_line;
    #[test]
    fn parse_hexagon() {
        // single instr
        assert_eq!(parse_line("w"), (-1, 0));
        assert_eq!(parse_line("e"), (1, 0));
        assert_eq!(parse_line("sw"), (0, -1));
        assert_eq!(parse_line("se"), (1, -1));
        assert_eq!(parse_line("ne"), (0, 1));
        assert_eq!(parse_line("nw"), (-1, 1));

        // some more complex ones
        assert_eq!(parse_line("nwwswee"), (0, 0));
    }
}
