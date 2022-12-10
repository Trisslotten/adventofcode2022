use std::cmp;

pub fn part1(input: &str) -> String {
    let mut lines = input.lines().peekable();
    let dims = lines.peek().unwrap().len();

    let mut visibility_map = Vec::new();
    visibility_map.resize(dims * dims, false);

    let mut tree_map = Vec::new();

    for line in lines {
        for c in line.chars() {
            tree_map.push(c.to_digit(10).unwrap() as i32);
        }
    }

    let max_i = dims - 1;

    for i in 0..dims {
        let mut xr = -1;
        let mut xl = -1;
        let mut yu = -1;
        let mut yd = -1;
        for step in 0..dims {
            let xr_pos = step + i * dims;
            let xl_pos = max_i - step + i * dims;
            let yu_pos = i + (max_i - step) * dims;
            let yd_pos = i + step * dims;

            if tree_map[xr_pos] > xr {
                xr = tree_map[xr_pos];
                visibility_map[xr_pos] = true;
            }
            if tree_map[xl_pos] > xl {
                xl = tree_map[xl_pos];
                visibility_map[xl_pos] = true;
            }
            if tree_map[yu_pos] > yu {
                yu = tree_map[yu_pos];
                visibility_map[yu_pos] = true;
            }
            if tree_map[yd_pos] > yd {
                yd = tree_map[yd_pos];
                visibility_map[yd_pos] = true;
            }
        }
    }

    let count = visibility_map.iter().fold(0, |acc, x| {
        acc + match x {
            false => 0,
            true => 1,
        }
    });
    count.to_string()
}

pub fn part2(input: &str) -> String {
    let mut lines = input.lines().peekable();
    let dims = lines.peek().unwrap().len();

    let mut visibility_map = Vec::new();
    visibility_map.resize(dims * dims, false);

    let mut tree_map = Vec::new();

    for line in lines {
        for c in line.chars() {
            tree_map.push(c.to_digit(10).unwrap() as i32);
        }
    }

    let mut max_score = 0;

    for y in 0..dims {
        for x in 0..dims {
            let considered_tree = tree_map[x + y * dims];

            
            let mut score_xr = 0;
            for xr in (x + 1)..dims {
                score_xr += 1;
                let tree = tree_map[xr + y * dims];
                if tree >= considered_tree {
                    break;
                }
            }

            let mut score_xl = 0;
            for xl in (0..x).rev() {
                score_xl += 1;
                let tree = tree_map[xl + y * dims];
                if tree >= considered_tree {
                    break;
                }
            }

            let mut score_yd = 0;
            for yd in (y + 1)..dims {
                score_yd += 1;
                let tree = tree_map[x + yd * dims];
                if tree >= considered_tree {
                    break;
                }
            }

            let mut score_yu = 0;
            for yu in (0..y).rev() {
                score_yu += 1;
                let tree = tree_map[x + yu * dims];
                if tree >= considered_tree {
                    break;
                }
            }

            let current_score = score_xl * score_xr * score_yd * score_yu;
            max_score = cmp::max(current_score, max_score);

            // print!("{},", current_score)
        }
        // println!();
    }

    max_score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
"30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, "8");
    }
}
