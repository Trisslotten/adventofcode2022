struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && self.end >= other.start
    }

    fn from_str(str: &str) -> Range {
        let (start_str, end_str) = str.split_once("-").unwrap();
        Range {
            start: start_str.parse::<i32>().unwrap(),
            end: end_str.parse::<i32>().unwrap(),
        }
    }
}

pub fn part1(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        let (head, tail) = line.split_once(",").unwrap();

        let left_range = Range::from_str(head);
        let right_range = Range::from_str(tail);

        if left_range.contains(&right_range) || right_range.contains(&left_range) {
            sum += 1;
        }
    }

    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        let (head, tail) = line.split_once(",").unwrap();

        let left_range = Range::from_str(head);
        let right_range = Range::from_str(tail);

        if left_range.overlaps(&right_range) {
            sum += 1;
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, "4");
    }
}
