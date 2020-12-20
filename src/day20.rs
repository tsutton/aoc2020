#![allow(clippy::ptr_arg, clippy::clippy::needless_range_loop)]

use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Edge {
    Top,
    Right,
    Bottom,
    Left,
}

const EDGES: [Edge; 4] = [Edge::Top, Edge::Bottom, Edge::Left, Edge::Right];

type Tile = Vec<Vec<char>>;
#[allow(unused)]
fn tile_to_str(tile: &Tile) -> String {
    tile.iter() // Iterator<Item=&Vec<Char>>
        .map(|row| row.iter().collect::<String>()) // Iterator<Item=String>
        .collect::<Vec<_>>() // Vec<String> (since join is a slice method, we need this collect)
        .join("\n") // String
}

/// get_edge retrieves an edge oriented clockwise i.e. the top edge is left to right,
/// the right edge is top to bottom, the bottom edge is right to left, and the left edge
/// is tbottom to top.
fn get_edge(e: &Edge, tile: &Tile) -> Vec<char> {
    match e {
        Edge::Top => tile[0].clone(),
        Edge::Bottom => (0..10).map(|i| tile[9][9 - i]).collect(),
        Edge::Left => (0..10).map(|i| tile[9 - i][0]).collect(),
        Edge::Right => (0..10).map(|i| tile[i][9]).collect(),
    }
}

/// Returns Err(()) if they don't match, Ok(false) if the match without reflection,
/// and Ok(true) if they match with with reflection.
fn edges_match(e1: &[char], e2: &[char]) -> Result<bool, ()> {
    if e1 == e2 {
        Ok(true)
    } else if e1.iter().cloned().rev().collect::<Vec<_>>() == e2 {
        Ok(false)
    } else {
        Err(())
    }
}

fn parse(input: &str) -> HashMap<usize, Tile> {
    let mut map = HashMap::new();
    for group in input.split("\n\n") {
        let first_line = group.find('\n').unwrap();
        let tile_number: usize = group[5..first_line - 1].parse().unwrap();
        let tile = group[first_line + 1..]
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        map.insert(tile_number, tile);
    }
    map
}

/// I have a feeling that this problem is actually easier than it could be.
/// The general case for this problem seems quite hard - we parse the tiles into some kind
/// of complicated graph with nodes based on the tiles and edges based on the matchings
/// (with note as to whether they are flipped) and then look for some subset of edges
/// which gives us a rectangle.
/// My gut is that each edge of each tile actually only matches up with exactly one
/// other edge, which means the logic of building a rectangle is actually much simpler.
/// Let's see if that guess pans out for me.
#[aoc(day20, part1)]
fn part1(input: &str) -> usize {
    let tiles = parse(input);
    // an "edge" is a tuple (first-tile, which-edge, second-tile, second-edge, reflect?)
    // tiles are tile numbers, which-edge is top, bottom, left, right
    let mut answer = 1;
    for test_tile_num in tiles.keys() {
        let test_tile = tiles.get(test_tile_num).unwrap();
        let mut n_matches = 0;
        for (tile_num, tile) in tiles.iter() {
            if tile_num == test_tile_num {
                continue;
            }
            for test_edge in EDGES.iter() {
                for edge in EDGES.iter() {
                    if edges_match(&get_edge(test_edge, &test_tile), &get_edge(edge, &tile)).is_ok()
                    {
                        n_matches += 1;
                    }
                }
            }
        }
        // Yep, as expected, this print statement prints out 4 for most things, 3 for
        // a few things, and 2 for exactly four things
        println!("for tile {} found {} matches", test_tile_num, n_matches);
        if n_matches == 2 {
            println!("found corner (2 matches): {}", test_tile_num);
            answer *= test_tile_num;
        }
    }
    answer
}

// Based on the way part one worked out, the way we are going to build the image
// is by first finding a corner - some tile with two matches. Then we set its coordinates
// to be 0,0, a rotation such that the matches are Right and Bottom, and start building out
// from there. Pick one of its two matches to seed the rest of the row, and build across
// by repeatedly looking for a match on Right, then rotating/reflecting so the match occurs
// on the Left of the other tile.
// Then we go down a row by finding the match on Bottom of the leftmost tile on the previous row
// and so on.
// In the end, this gives us the full grid of tiles, and we can build the image easily from there.

// The more efficient way to do this would be to make ONE pass over the set of tiles, storing
// the edge patterns in a hash table (mapping edge => (tile_number, which edge of that tile it is))
// and making matches as we go.
// I'm going to make a way that's a bit easier to implement, which is the O(n^2) way instead of O(n)
// aka pick the top left corner by iterating over all tiles, then pick the tile next to it by iterating
// over all tiles, and so on.
// It's much simpler to keep track of the state this way (e.g in the hash table described before, we have to
// accomodate that each edge has TWO tiles, and they may be flipped as well, and we still need to be able
// to extract edges from tiles so we can continue the lookup chain.
#[aoc(day20, part2)]
pub fn part2(input: &str) -> usize {
    let mut tiles = parse(input);

    let mut rows = vec![];
    let mut first_row = vec![];
    let top_left = find_top_left_corner(&mut tiles);
    first_row.push(top_left);
    while let Some(tile) = find_right_match(&mut tiles, first_row.last().unwrap()) {
        first_row.push(tile);
    }
    rows.push(first_row);
    // same as above but seed with the bottom left of the previous row
    while !tiles.is_empty() {
        let prev = &rows.last().unwrap()[0];
        let seed = find_bottom_match(&mut tiles, prev).unwrap();
        // 	println!("================================================================================
        // First of prev row, and seed:
        // {}

        // {}
        // ================================================================================",
        // 		 tile_to_str(prev),
        // 		 tile_to_str(&seed),
        // 	);
        let mut row = vec![seed];
        while let Some(tile) = find_right_match(&mut tiles, row.last().unwrap()) {
            row.push(tile);
        }
        rows.push(row);
    }
    // side note: by observation, map ended up being a square. That makes
    // the subsequnt code which rotates and reflects it a bit simpler,
    // although it's not essential; we COULD rotate and reflect non-square rectangles, too.
    let map = unify(&rows);

    // I thought about parameterizing this so it's not just 8 cases
    // but it was more complicated, error prone, and not any more efficient
    // than just doing it this way.
    if let Some(answer) = search_for_monsters(&mut map.clone()) {
        println!("found answer in base");
        return answer;
    }
    if let Some(answer) = search_for_monsters(&mut rotated_90(&map)) {
        println!("found answer in 90-rotated");
        return answer;
    }

    if let Some(answer) = search_for_monsters(&mut rotated_180(&map)) {
        println!("found answer in 180-rotated");
        return answer;
    }

    if let Some(answer) = search_for_monsters(&mut rotated_270(&map)) {
        println!("found answer in 270-rotated");
        return answer;
    }
    let map = flipped_left_right(&map);
    if let Some(answer) = search_for_monsters(&mut map.clone()) {
        println!("found answer in base flipped");
        return answer;
    }
    if let Some(answer) = search_for_monsters(&mut rotated_90(&map)) {
        println!("found answer in 90-rotated flipped");
        return answer;
    }

    if let Some(answer) = search_for_monsters(&mut rotated_180(&map)) {
        println!("found answer in 180-rotated flipped");
        return answer;
    }

    if let Some(answer) = search_for_monsters(&mut rotated_270(&map)) {
        println!("found answer in 270-rotated flipped");
        return answer;
    }

    unreachable!()
}

fn find_top_left_corner(tiles: &mut HashMap<usize, Tile>) -> Tile {
    let (some_corner, mut edges) = tiles
        .keys()
        .find_map(|test_tile_num| {
            let test_tile = tiles.get(test_tile_num).unwrap();
            let mut matches = Vec::new();
            for (tile_num, tile) in tiles.iter() {
                if tile_num == test_tile_num {
                    continue;
                }
                for test_edge in EDGES.iter() {
                    for edge in EDGES.iter() {
                        if edges_match(&get_edge(test_edge, &test_tile), &get_edge(edge, &tile))
                            .is_ok()
                        {
                            matches.push(*test_edge);
                        }
                    }
                }
            }
            if matches.len() == 2 {
                Some((*test_tile_num, matches))
            } else {
                None
            }
        })
        .unwrap();
    edges.sort();
    let tile = tiles.remove(&some_corner).unwrap();
    // println!("{}", tile_to_str(&tile));
    if edges == vec![Edge::Top, Edge::Right] {
        rotated_90(&tile)
    } else if edges == vec![Edge::Bottom, Edge::Left] {
        rotated_270(&tile)
    } else if edges == vec![Edge::Top, Edge::Left] {
        rotated_180(&tile)
    } else {
        tile
    }
}

fn find_right_match(tiles: &mut HashMap<usize, Tile>, base_tile: &Tile) -> Option<Tile> {
    let find_result = tiles
        .keys()
        .find_map(|test_tile_num| -> Option<(usize, Edge, bool)> {
            let test_tile = tiles.get(test_tile_num).unwrap();
            for edge in EDGES.iter() {
                if let Ok(flip) = edges_match(
                    &get_edge(&Edge::Right, &base_tile),
                    &get_edge(edge, &test_tile),
                ) {
                    return Some((*test_tile_num, *edge, flip));
                }
            }
            None
        });
    let (matching_tile_num, matching_edge, flip) = match find_result {
        Some((a, b, c)) => (a, b, c),
        None => return None,
    };

    let tile = tiles.remove(&matching_tile_num).unwrap();
    let rotated_tile = if matching_edge == Edge::Bottom {
        rotated_90(&tile)
    } else if matching_edge == Edge::Top {
        rotated_270(&tile)
    } else if matching_edge == Edge::Right {
        rotated_180(&tile)
    } else {
        tile
    };
    if flip {
        Some(flipped_top_bottom(&rotated_tile))
    } else {
        Some(rotated_tile)
    }
}

// this is *almost* the same as find_right_match and probably could be refactored into one method
fn find_bottom_match(tiles: &mut HashMap<usize, Tile>, base_tile: &Tile) -> Option<Tile> {
    let find_result = tiles.keys().find_map(|test_tile_num| {
        let test_tile = tiles.get(test_tile_num).unwrap();
        for edge in EDGES.iter() {
            if let Ok(flip) = edges_match(
                &get_edge(&Edge::Bottom, &base_tile),
                &get_edge(edge, &test_tile),
            ) {
                return Some((*test_tile_num, *edge, flip));
            }
        }
        None
    });
    let (matching_tile_num, matching_edge, flip) = match find_result {
        Some((a, b, c)) => (a, b, c),
        None => return None,
    };

    let tile = tiles.remove(&matching_tile_num).unwrap();
    let rotated_tile = if matching_edge == Edge::Bottom {
        rotated_180(&tile)
    } else if matching_edge == Edge::Right {
        rotated_270(&tile)
    } else if matching_edge == Edge::Left {
        rotated_90(&tile)
    } else {
        tile
    };
    if flip {
        Some(flipped_left_right(&rotated_tile))
    } else {
        Some(rotated_tile)
    }
}

// all rotations are clockwise
fn rotated_90(tile: &Tile) -> Tile {
    let mut new_tile = vec![];
    let l = tile.len();
    for i in 0..l {
        let mut row = vec![];
        for j in 0..l {
            row.push(tile[l - 1 - j][i])
        }
        new_tile.push(row);
    }
    new_tile
}

// all rotations are clockwise
fn rotated_180(tile: &Tile) -> Tile {
    let mut new_tile = vec![];
    let l = tile.len();
    for i in 0..l {
        let mut row = vec![];
        for j in 0..l {
            row.push(tile[l - 1 - i][l - 1 - j])
        }
        new_tile.push(row);
    }
    new_tile
}

// all rotations are clockwise
fn rotated_270(tile: &Tile) -> Tile {
    let mut new_tile = vec![];
    let l = tile.len();
    for i in 0..l {
        let mut row = vec![];
        for j in 0..l {
            row.push(tile[j][l - 1 - i])
        }
        new_tile.push(row);
    }
    new_tile
}

fn flipped_left_right(tile: &Tile) -> Tile {
    let mut new_tile = vec![];
    let l = tile.len();
    for i in 0..l {
        let mut row = vec![];
        for j in 0..l {
            row.push(tile[i][l - 1 - j])
        }
        new_tile.push(row);
    }
    new_tile
}

fn flipped_top_bottom(tile: &Tile) -> Tile {
    let mut new_tile = vec![];
    let l = tile.len();
    for i in 0..l {
        let mut row = vec![];
        for j in 0..l {
            row.push(tile[l - 1 - i][j])
        }
        new_tile.push(row);
    }
    new_tile
}

// assumption: all tiles are 10x10, and grid is rectangular
fn unify(grid: &Vec<Vec<Tile>>) -> Vec<Vec<char>> {
    let tile_rows = grid[0][0].len(); // aka 10.
    let n_rows = grid.len() * (tile_rows - 2);
    let mut rows = Vec::with_capacity(n_rows);
    for grid_row in grid {
        for tile_row in 1..tile_rows - 1 {
            let mut row = vec![];
            for tile in grid_row {
                for c in &tile[tile_row][1..9] {
                    row.push(*c);
                }
            }
            rows.push(row);
        }
    }
    rows
}

// search_for_monsters searches the map for sea monsters
// it uses the given getter function to set orientation
// it searches point by point over the map for a sea monster whose top left is at that location
// it mutates map by setting any squares that have sea monster to O, as describd in the problem.
// This is not essential, it could take an immutable map and track the Os separately, but lazy.
fn search_for_monsters(map: &mut Vec<Vec<char>>) -> Option<usize> {
    let height = map.len();
    let width = map[0].len();

    // step one: convert the sea monster into the set of (row, col) coordinates
    // that need to show up # on our map to be a sea monster.
    let sea_monster: Vec<&str> = vec![
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];
    let indices = |row: &[char]| -> Vec<usize> {
        row.iter()
            .enumerate()
            .filter_map(|(i, &c)| if c == '#' { Some(i) } else { None })
            .collect()
    };
    // this is kinda gross iterator stuff
    let sea_monster_coords: Vec<(usize, usize)> = sea_monster
        .iter()
        .enumerate() // pairs (row_num, row: Vec<char>)
        // 1) replace each row with its (row, col) pair using indices
        //    so that we have (row, Vec<usize>) after
        .map(|(row_num, row)| {
            indices(&row.chars().collect::<Vec<_>>())
                .into_iter()
                .map(|col| (row_num, col))
                .collect()
        })
        .collect::<Vec<Vec<(usize, usize)>>>()
        // flatten using some Iterator magic
        .into_iter()
        .flatten()
        .collect();

    let mut monsters = false;
    for row in 0..height {
        for col in 0..width {
            // try to check for a sea monster at (row, col)
            let all_sea_monster_coords_found = sea_monster_coords.iter().all(|(d_row, d_col)| {
                row + d_row < map.len()
                    && col + d_col < map[0].len()
                    && map[row + d_row][col + d_col] != '.'
            });
            if all_sea_monster_coords_found {
                monsters = true;
                // we found a monster - mark its squares with O
                sea_monster_coords.iter().for_each(|(d_row, d_col)| {
                    map[row + d_row][col + d_col] = 'O';
                });
            }
        }
    }
    if !monsters {
        None
    } else {
        return Some(
            map.iter()
                .map(|row| row.iter().filter(|&x| *x == '#').count())
                .sum(),
        );
    }
}
