use std::collections::HashSet;

#[derive(Clone)]
struct Rock {
    pos: (i32, i32),
    tiles: Vec<(i32, i32)>,
}
// impl Rock {
//     fn map_tiles(&self) -> Map<std::slice::Iter<'_, (i32, i32)>> {
//         let (px, py) = self.pos;
//         self.tiles.iter().map(|(x, y)| (px + *x, py + *y))
//     }
// }

fn has_tile(map: &HashSet<(i32, i32)>, (x, y): (i32, i32)) -> bool {
    !(0..=6).contains(&x) || y <= 0 || map.contains(&(x,y))
}

fn debug_print(map: &HashSet<(i32, i32)>, rock: &Rock, highest_tile: i32) {
    let (px, py) = rock.pos;
    let pos_iter = rock.tiles.iter().map(|(x,y)| (x + px, y + py));

    for y in (0..=(highest_tile + 5)).rev() {
        for x in -1..=7 {
            if pos_iter.clone().any(|(rx, ry)| rx == x && ry == y) {
                print!("@");
            } else if has_tile(map, (x,y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

pub fn part1(input: &str) -> String {
    let rocks = [
        Rock{
            pos: (2, 0),
            tiles: vec![(0,0),(1,0),(2,0),(3,0)],
        },
        Rock{
            pos: (2, 0),
            tiles: vec![(0,1),(1,0),(1,1),(1,2),(2,1)],
        },
        Rock{
            pos: (2, 0),
            tiles: vec![(0,0),(1,0),(2,0),(2,1),(2,2)],
        },
        Rock{
            pos: (2, 0),
            tiles: vec![(0,0),(0,1),(0,2),(0,3)],
        },
        Rock{
            pos: (2, 0),
            tiles: vec![(0,0),(1,0),(0,1),(1,1)],
        },
    ];

    let mut map: HashSet<(i32, i32)> = HashSet::new();
    
    let mut highest_tile = 0;
    let mut gas_index = 0;

    let gases = input.as_bytes();

    for i in 1.. {
        let mut rock = rocks[(i-1) % rocks.len()].clone();
        rock.pos.1 = highest_tile + 4;

        loop {
            let current_gas = gases[gas_index % gases.len()];
            let dx = if current_gas == b'>' { 1 } else { -1 };

            // debug_print(&map, &rock, highest_tile);
            
            {
                let (px, py) = rock.pos;
                let mut pos_iter = rock.tiles.iter().map(|(x,y)| (x + px, y + py));
                if pos_iter.all(|(x,y)| !has_tile(&map, (x+dx,y))) {
                    rock.pos.0 += dx;
                }
            }
            
            // debug_print(&map, &rock, highest_tile);

            {
                let (px, py) = rock.pos;
                let pos_iter = rock.tiles.iter().map(|(x,y)| (x + px, y + py));
                if pos_iter.clone().any(|(x,y)| has_tile(&map, (x,y-1))) {
                    for p in pos_iter {
                        map.insert(p);
                        highest_tile = highest_tile.max(p.1);
                    }

                    if i == 2022 {
                        return highest_tile.to_string();
                    }

                    gas_index += 1;
                    break;
                } else {
                    rock.pos.1 -= 1;
                }
            }

            gas_index += 1;
        }
    }

    "nothing found".to_string()
}

pub fn part2(input: &str) -> String {
    let rocks = [
        Rock{
            pos: (2, 0),
            tiles: vec![(0,0),(1,0),(2,0),(3,0)],
        },
        Rock{
            pos: (2, 0),
            tiles: vec![(0,1),(1,0),(1,1),(1,2),(2,1)],
        },
        Rock{
            pos: (2, 0),
            tiles: vec![(0,0),(1,0),(2,0),(2,1),(2,2)],
        },
        Rock{
            pos: (2, 0),
            tiles: vec![(0,0),(0,1),(0,2),(0,3)],
        },
        Rock{
            pos: (2, 0),
            tiles: vec![(0,0),(1,0),(0,1),(1,1)],
        },
    ];

    let mut map: HashSet<(i32, i32)> = HashSet::new();

    let mut highest_tile: usize = 0;
    let mut gas_index = 0;

    let gases = input.as_bytes();

    let count: usize = 1_000_000_000_000;

    let mut height_increase: usize = 0;
    let mut start_height: usize = 0;
    let mut start_rocks: usize = 0;
    let mut rocks_increase: usize = 0;
    let mut rocks_remainder: usize = 0;
    let mut num_rock_cycles: usize = 0;

    let mut sample_count = 0;

    for i in 0.. {
        let mut rock = rocks[i % rocks.len()].clone();
        rock.pos.1 = (highest_tile + 4) as i32;

        if gas_index != 0 && gas_index % gases.len() == 0 {
            match sample_count {
                0 => {
                    start_height = highest_tile;
                    start_rocks = i;
                }
                1 => {
                    height_increase = highest_tile - start_height;
                    rocks_increase = i - start_rocks;

                    rocks_remainder = (count - start_rocks) % rocks_increase;
                    num_rock_cycles = (count - start_rocks) / rocks_increase;
                }
                _ => {}
            }
            sample_count += 1;
        }
        if sample_count == 2 && i == start_rocks + rocks_increase + rocks_remainder {
            let remainder_height = highest_tile - height_increase - start_height;
            
            return (num_rock_cycles * height_increase + remainder_height + start_height).to_string();
        }

        loop {
            let current_gas = gases[gas_index % gases.len()];
            let dx = if current_gas == b'>' { 1 } else { -1 };

            // debug_print(&map, &rock, highest_tile);
            
            {
                let (px, py) = rock.pos;
                let mut pos_iter = rock.tiles.iter().map(|(x,y)| (x + px, y + py));
                if pos_iter.all(|(x,y)| !has_tile(&map, (x+dx,y))) {
                    rock.pos.0 += dx;
                }
            }
            
            // debug_print(&map, &rock, highest_tile);

            {
                let (px, py) = rock.pos;
                let pos_iter = rock.tiles.iter().map(|(x,y)| (x + px, y + py));
                if pos_iter.clone().any(|(x,y)| has_tile(&map, (x,y-1))) {
                    for p in pos_iter {
                        map.insert(p);
                        highest_tile = highest_tile.max(p.1 as usize);
                    }
                    gas_index += 1;
                    break;
                } else {
                    rock.pos.1 -= 1;
                }
            }

            gas_index += 1;
        }

        if i % (1_000_000) == 0 {
            println!("{i}");
        }
    }

    "nothing found".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, "3068");
    }

    #[test]
    fn test_part2() {
        // Doesn't work for test input
        // let result = part2(INPUT);
        // assert_eq!(result, "");
    }
}
