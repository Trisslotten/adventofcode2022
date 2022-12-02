pub fn part1(input: &str) -> String {
    let mut sum: u32 = 0;

    for str_line in input.lines() {
        let line = str_line.as_bytes();
        
        let opponent = line[0] - b'A';
        let me = line[2] - b'X';

        sum += me as u32 + 1;

        if opponent == me {
            sum += 3;
        } else if (me + 2) % 3 == opponent {
            sum += 6;
        }
    }

    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let mut sum: u32 = 0;

    for str_line in input.lines() {
        let line = str_line.as_bytes();
        
        let opponent = line[0] - b'A';
        let action = line[2] - b'X';

        let me = match action {
            0 => opponent + 2,
            1 => opponent,
            _ => opponent + 1,
        } % 3;

        sum += me as u32 + 1;

        if opponent == me {
            sum += 3;
        } else if (me + 2) % 3 == opponent {
            sum += 6;
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
"A Y
B X
C Z";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, "12");
    }
}
