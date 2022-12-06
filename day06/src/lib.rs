
fn all_different(w: &[u8], count: usize) -> bool {
    for i in 0..count {
        for j in (i+1)..count {
            if w[i] == w[j] {
                return false;
            }
        }
    }
    true
}

pub fn part1(input: &str) -> String {
    let mut count = 3;
    for w in input.as_bytes().windows(4) {
        count += 1;
        if all_different(w, 4) {
            break;
        }
    }

    count.to_string()
}

pub fn part2(input: &str) -> String {
    let mut count = 13;
    for w in input.as_bytes().windows(14) {
        count += 1;
        if all_different(w, 14) {
            break;
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT0: (&str, &str, &str) = ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", "7", "19");
    const INPUT1: (&str, &str, &str) = ("bvwbjplbgvbhsrlpgdmjqwftvncz", "5", "23");
    const INPUT2: (&str, &str, &str) = ("nppdvjthqldpwncqszvftbrmjlhg", "6", "23");
    const INPUT3: (&str, &str, &str) = ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "10", "29");
    const INPUT4: (&str, &str, &str) = ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "11", "26");

    const INPUTS: [(&str, &str, &str); 5] = [
        INPUT0,
        INPUT1,
        INPUT2,
        INPUT3,
        INPUT4,
    ];

    #[test]
    fn test_part1() {
        for (input, output, _) in INPUTS {
            let result = part1(input);
            assert_eq!(result, output);
        }
    }

    #[test]
    fn test_part2() {
        for (input, _, output) in INPUTS {
            let result = part2(input);
            assert_eq!(result, output);
        }
    }
}
