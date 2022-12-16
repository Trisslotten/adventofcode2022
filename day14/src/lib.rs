use std::collections::HashMap;

enum Tile {
    Wall,
    Sand
}

type PosInt = i32;
type Pos = (PosInt, PosInt);

pub fn part1(input: &str) -> String {
    let mut map: HashMap<Pos, Tile> = HashMap::new();
    
    let mut max_y = PosInt::MIN;

    for line in input.lines() {
        let mut prev: Option<Pos> = None;
        for coord in line.split(" -> ") {
            if let Some((x_str,y_str)) = coord.split_once(',') {
                let (x, y) = (x_str.parse::<PosInt>().unwrap(), y_str.parse::<PosInt>().unwrap());

                max_y = max_y.max(y);

                if let Some((px, py)) = prev {
                    let (mut dx, mut dy) = (x - px, y - py);
                    let len = dx.abs() + dy.abs();
                    dx = dx.clamp(-1, 1);
                    dy = dy.clamp(-1, 1);
                    let (mut cx, mut cy) = (px,py);
                    for _ in 0..=len {
                        map.insert((cx, cy), Tile::Wall);
                        cx += dx;
                        cy += dy;
                    }
                }
                prev = Some((x,y));
            }
        }
    }

    for i in 0.. {
        let mut x: PosInt = 500;
        let mut y: PosInt = 0;

        loop {
            // for my in 0..=12 {
            //     for mx in 493..=504 {
            //         let t = if let Some(v) = map.get(&(mx, my)) {
            //             match *v {
            //                 Tile::Sand => 'O',
            //                 Tile::Wall => '#'
            //             }
            //         } else {
            //             ' '
            //         };
            //         print!("{}", t);
            //     }
            //     println!();
            // }
            // println!();

            if !map.contains_key(&(x, y+1)) {
                y += 1;
            } else if !map.contains_key(&(x-1, y+1)) {
                x -= 1;
                y += 1
            } else if !map.contains_key(&(x+1, y+1)) {
                x += 1;
                y += 1;
            } else {
                map.insert((x,y), Tile::Sand);
                break;
            }
            if y > max_y {
                return i.to_string();
            }
        }
    }

    "".to_string()
}

fn has_tile(map: &HashMap<Pos, Tile>, max_y: PosInt, (x,y): Pos) -> bool {
    y >= max_y + 2 || map.contains_key(&(x,y))
}

pub fn part2(input: &str) -> String {
    let mut map: HashMap<Pos, Tile> = HashMap::new();
    
    let mut max_y = PosInt::MIN;

    for line in input.lines() {
        let mut prev: Option<Pos> = None;
        for coord in line.split(" -> ") {
            if let Some((x_str,y_str)) = coord.split_once(',') {
                let (x, y) = (x_str.parse::<PosInt>().unwrap(), y_str.parse::<PosInt>().unwrap());

                max_y = max_y.max(y);

                if let Some((px, py)) = prev {
                    let (mut dx, mut dy) = (x - px, y - py);
                    let len = dx.abs() + dy.abs();
                    dx = dx.clamp(-1, 1);
                    dy = dy.clamp(-1, 1);
                    let (mut cx, mut cy) = (px,py);
                    for _ in 0..=len {
                        map.insert((cx, cy), Tile::Wall);
                        cx += dx;
                        cy += dy;
                    }
                }
                prev = Some((x,y));
            }
        }
    }

    for i in 1.. {
        let mut x: PosInt = 500;
        let mut y: PosInt = 0;

        loop {
            // for my in 0..=12 {
            //     for mx in 493..=504 {
            //         let t = if let Some(v) = map.get(&(mx, my)) {
            //             match *v {
            //                 Tile::Sand => 'O',
            //                 Tile::Wall => '#'
            //             }
            //         } else {
            //             ' '
            //         };
            //         print!("{}", t);
            //     }
            //     println!();
            // }
            // println!();

            if !has_tile(&map, max_y, (x, y+1)) {
                y += 1;
            } else if !has_tile(&map, max_y, (x-1, y+1)) {
                x -= 1;
                y += 1
            } else if !has_tile(&map, max_y, (x+1, y+1)) {
                x += 1;
                y += 1;
            } else {
                if y == 0 {
                    return i.to_string();
                }
                println!("{}, {}", x, y);
                map.insert((x,y), Tile::Sand);
                break;
            }
        }
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, "24");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, "93");
    }
}
