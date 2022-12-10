use std::collections::HashSet;

pub fn part1(input: &str) -> String {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut head_pos = (0,0);
    let mut tail_pos = (0,0);

    visited.insert(tail_pos);

    for line in input.lines() {
        let (command, steps_str) = line.split_once(' ').unwrap();
        let steps = steps_str.parse::<i32>().unwrap();

        let dir = match command {
            "U" => (0,1),
            "D" => (0,-1),
            "R" => (1,0),
            "L" => (-1,0),
            _ => panic!()
        };

        for _ in 0..steps {
            head_pos.0 += dir.0;
            head_pos.1 += dir.1;

            let dx = head_pos.0 - tail_pos.0;
            let dy = head_pos.1 - tail_pos.1;
            if dx.abs() > 1 || dy.abs() > 1 {
                tail_pos.0 += dx.clamp(-1, 1);
                tail_pos.1 += dy.clamp(-1, 1);
            }

            visited.insert(tail_pos);
        }
    }

    visited.len().to_string()
}

pub fn part2(input: &str) -> String {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut positions = Vec::new();
    for _ in 0..10 {
        positions.push((0,0));
    }
    visited.insert(positions[0]);

    for line in input.lines() {
        let (command, steps_str) = line.split_once(' ').unwrap();
        let steps = steps_str.parse::<i32>().unwrap();

        let dir = match command {
            "U" => (0,-1),
            "D" => (0,1),
            "R" => (1,0),
            "L" => (-1,0),
            _ => panic!()
        };

        for _ in 0..steps {
            positions[0].0 += dir.0;
            positions[0].1 += dir.1;

            for i in 1..10 {
                let head_pos = positions[i-1];
                let tail_pos = &mut positions[i];

                let dx = head_pos.0 - tail_pos.0;
                let dy = head_pos.1 - tail_pos.1;
                if dx.abs() > 1 || dy.abs() > 1 {
                    tail_pos.0 += dx.clamp(-1, 1);
                    tail_pos.1 += dy.clamp(-1, 1);
                }
            }

            visited.insert(*positions.last().unwrap());
        }

        // print!("{}[2J", 27 as char);
        // let s = 40;
        // for y in -s..s {
        //     for x in -s..s {
                
        //         if positions[0].0 == x && positions[0].1 == y {
        //             print!("H");
        //         } else if positions[9].0 == x && positions[9].1 == y {
        //             print!("T");
        //         } else if positions.contains(&(x,y)) {
        //             print!("*");
        //         } else if visited.contains(&(x,y)) {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
        // println!();
    }

    visited.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
"R 4
U 4
L 3
D 1Ö
R 4
D 1
L 5
R 2";

    const INPUT2: &str =
"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn test_part2() {
        // let result = part2(INPUT);
        // assert_eq!(result, "1");

        let result = part2(INPUT2);
        assert_eq!(result, "36");
    }
}
