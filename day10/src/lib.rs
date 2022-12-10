use std::fs;

pub fn part1(input: &str) -> String {
    let mut cycles = 0;
    let mut x = 1;

    let values = vec![20, 60, 100, 140, 180, 220];

    let mut sum = 0;

    for line in input.lines() {
        if line == "noop" {
            cycles += 1;

            if values.contains(&cycles) {
                sum += cycles * x;
            }
        } else {
            let (_op, value) = line.split_once(' ').unwrap();

            if values.contains(&(cycles + 1)) {
                sum += (cycles + 1) * x;
            } else if values.contains(&(cycles + 2)) {
                sum += (cycles + 2) * x;
            }

            cycles += 2;
            x += value.parse::<i32>().unwrap();
        }
    }

    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let mut cycles = 0;
    let mut x: i32 = 1;

    let mut image = String::new();

    for line in input.lines() {
        if line == "noop" {
            if x.abs_diff(cycles % 40) <= 1 {
                image.push('#');
            } else {
                image.push('.');
            }
            cycles += 1;
            if cycles % 40 == 0 {
                image.push('\n');
            }
        } else {
            let (_op, value) = line.split_once(' ').unwrap();

            if x.abs_diff(cycles % 40) <= 1 {
                image.push('#');
            } else {
                image.push('.');
            }
            cycles += 1;
            if cycles % 40 == 0 {
                image.push('\n');
            }

            if x.abs_diff(cycles % 40) <= 1 {
                image.push('#');
            } else {
                image.push('.');
            }
            cycles += 1;
            if cycles % 40 == 0 {
                image.push('\n');
            }

            x += value.parse::<i32>().unwrap();
        }
    }

    image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").unwrap();
        let result = part1(&input);
        assert_eq!(result, "13140");
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("test_input.txt").unwrap();
        let result = part2(&input);
        assert_eq!(result,
"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
    }
}
