pub fn part1(input: &str) -> String {
    let (stacks_description, moves) = input.split_once("\n\n").unwrap();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    {
        let mut lines = stacks_description.lines();
        let num_stacks = (lines.nth(0).unwrap().len() + 1) / 4;
        stacks.resize(num_stacks, Vec::new());

        for line in stacks_description.lines() {
            if line.chars().nth(1).unwrap() == '1' {
                break;
            }

            for i in 0..num_stacks {
                let c = line.chars().nth(i*4 + 1).unwrap();

                if c != ' ' {
                    stacks[i].push(c)
                }
            }
        }

        for stack in stacks.iter_mut() {
            stack.reverse();
        }
    }

    for line in moves.lines() {
        let mut tokens = line.split(" ");
        let count = tokens.nth(1).unwrap().parse::<i32>().unwrap();
        let from = tokens.nth(1).unwrap().parse::<usize>().unwrap()-1;
        let to = tokens.nth(1).unwrap().parse::<usize>().unwrap()-1;

        for _ in 0..count {
            let c = stacks[from].pop().unwrap();
            stacks[to].push(c);
        }
    }

    let mut result = String::new();

    for stack in stacks {
        result.push(*stack.last().unwrap());
    }

    result
}

pub fn part2(input: &str) -> String {
    let (stacks_description, moves) = input.split_once("\n\n").unwrap();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    {
        let mut lines = stacks_description.lines();
        let num_stacks = (lines.nth(0).unwrap().len() + 1) / 4;
        stacks.resize(num_stacks, Vec::new());

        for line in stacks_description.lines() {
            if line.chars().nth(1).unwrap() == '1' {
                break;
            }

            for i in 0..num_stacks {
                let c = line.chars().nth(i*4 + 1).unwrap();

                if c != ' ' {
                    stacks[i].push(c)
                }
            }
        }

        for stack in stacks.iter_mut() {
            stack.reverse();
        }
    }

    let mut temp : Vec<char> = Vec::new();

    for line in moves.lines() {
        let mut tokens = line.split(" ");
        let count = tokens.nth(1).unwrap().parse::<usize>().unwrap();
        let from = tokens.nth(1).unwrap().parse::<usize>().unwrap()-1;
        let to = tokens.nth(1).unwrap().parse::<usize>().unwrap()-1;

        let from_len = stacks[from].len();
        let slice= &stacks[from][(from_len-count)..from_len];

        temp.extend_from_slice(slice);
        stacks[to].append(&mut temp);
        
        stacks[from].truncate(from_len-count);
    }

    let mut result = String::new();

    for stack in stacks {
        result.push(*stack.last().unwrap());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, "MCD");
    }
}
