use itertools::Itertools;

fn find_matching(s: &str) -> usize {
    let chars = s.chars();
    let mut depth = 0;
    for (i, c) in chars.enumerate() {
        let d = match c {
            '[' => 1,
            ']' => -1,
            _ => 0,
        };
        depth += d;
        if d == -1 && depth == 0 {
            return i;
        }
    }

    0
}

// enum Item<'a> {
//     List(&'a str),
//     Num
// }

fn in_order(left: &str, right: &str) -> bool {
    let mut left_chars = left.chars().peekable();
    let mut right_chars = right.chars().peekable();
    
    while left_chars.peek().is_some() && right_chars.peek().is_some(){
        let l = *left_chars.peek().unwrap();
        let r = *right_chars.peek().unwrap();

        if l == '[' && r == '[' {
            let left_str = left_chars.clone().collect::<String>();
            let left_end = find_matching(left_str.as_str());
            let left_subslice = left_str.get(1..left_end).unwrap();

            let right_str = right_chars.clone().collect::<String>();
            let right_end = find_matching(right_str.as_str());
            let right_subslice = right_str.get(1..right_end).unwrap();

            let result = in_order(left_subslice, right_subslice);
            if !result {
                return result;
            }

            left_chars.nth(left_end+1);
            right_chars.nth(right_end+1);
        } else if l == '[' {
            let left_str = left_chars.clone().collect::<String>();
            let left_end = find_matching(left_str.as_str());
            let left_subslice = left_str.get(1..left_end).unwrap();

            let result = in_order(left_subslice, right);
            if !result {
                return result;
            }

            left_chars.nth(left_end+1);
            right_chars.nth(usize::MAX);
        } else if r == '[' {
            let right_str = right_chars.clone().collect::<String>();
            let right_end = find_matching(right_str.as_str());
            let right_subslice = right_str.get(1..right_end).unwrap();

            let result = in_order(left, right_subslice);
            if !result {
                return result;
            }

            left_chars.nth(usize::MAX);
            right_chars.nth(right_end+1);
        } else {
            let mut left_str = String::new();
            while left_chars.peek() != Some(&',') && left_chars.peek().is_some() {
                left_str.push(left_chars.next().unwrap());
            }
            left_chars.next();

            let mut right_str = String::new();
            while right_chars.peek() != Some(&',') && right_chars.peek().is_some() {
                right_str.push(right_chars.next().unwrap());
            }
            right_chars.next();

            let bog = left_str.parse::<i32>();

            let left_num = bog.unwrap();
            let right_num = right_str.parse::<i32>().unwrap();

            if left_num > right_num {
                return false;
            }
        }
    }

    if left_chars.peek().is_none() && right_chars.peek().is_some() {
        return true; 
    } else if left_chars.peek().is_some() && right_chars.peek().is_none() {
        return false;
    }

    true
}

pub fn part1(input: &str) -> String {
    let mut sum = 0;

    for (i, (left, right, _)) in input.lines().tuples().enumerate() {
        if in_order(left, right) {
            sum += i;
        }
    }

    sum.to_string()
}

pub fn part2(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, "");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, "");
    }

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
 ";

}
