use std::collections::*;
use itertools::Itertools; 

pub fn part1(input: &str) -> String {
    let mut items = HashSet::new();

    let mut sum: u32 = 0;

    for line in input.lines() {
        let line_bytes = line.as_bytes();

        items.reserve(line_bytes.len()/2);

        let (head, tail) = line_bytes.split_at(line_bytes.len()/2);

        for item in head {
            items.insert(*item);
        }

        for item in tail {
            if items.contains(item) {
                let (offset, init) = if item.is_ascii_uppercase() {
                    (b'A', 27)
                } else {
                    (b'a', 1)
                };

                sum += (item - offset) as u32 + init;
                break;
            }
        }
        items.clear();
    }

    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let mut items = HashSet::new();
    let mut item_counts = HashMap::new();

    let mut sum: u32 = 0;

    for (a,b,c) in input.lines().tuples() {
        for item in a.as_bytes() {
            items.insert(*item);
        }
        for item in &items {
            item_counts.insert(*item, 1);
        }

        items.clear();
        for item in b.as_bytes() {
            items.insert(*item);
        }
        for item in &items {
            if let Some(count) = item_counts.get_mut(&item) {
                *count += 1;
            }
        }

        items.clear();
        for item in c.as_bytes() {
            items.insert(*item);
        }
        for item in &items {
            if let Some(count) = item_counts.get_mut(&item) {
                *count += 1;
            }
        }

        for (item, count) in &item_counts {
            if *count == 3 {
                let (offset, init) = if item.is_ascii_uppercase() {
                    (b'A', 27)
                } else {
                    (b'a', 1)
                };

                sum += (*item - offset) as u32 + init;
                break;
            }
        }

        items.clear();
        item_counts.clear();
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, "70");
    }
}
