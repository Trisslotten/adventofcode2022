use std::collections::HashMap;

enum Task<'a> {
    Val(i64),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
}

fn recursive_calculate(monkeys: &HashMap<&str, Task>, current_node: &str) -> i64 {
    let task = monkeys.get(current_node).unwrap();
    
    match *task {
        Task::Val(val) => val,
        Task::Add(left, right) => {
            let left_val = recursive_calculate(monkeys, left);
            let right_val = recursive_calculate(monkeys, right);
            left_val + right_val
        },
        Task::Sub(left, right) => {
            let left_val = recursive_calculate(monkeys, left);
            let right_val = recursive_calculate(monkeys, right);
            left_val - right_val
        },
        Task::Mul(left, right) => {
            let left_val = recursive_calculate(monkeys, left);
            let right_val = recursive_calculate(monkeys, right);
            left_val * right_val
        },
        Task::Div(left, right) => {
            let left_val = recursive_calculate(monkeys, left);
            let right_val = recursive_calculate(monkeys, right);
            left_val / right_val
        },
    }
}

pub fn part1(input: &str) -> String {
    let mut monkeys: HashMap<&str, Task> = HashMap::new();

    for line in input.lines() {
        let (id, action) = line.split_once(": ").unwrap();

        let task = if let Ok(num) = action.parse::<i64>() {
            Task::Val(num)
        } else if let Some((left, right)) = action.split_once(" + ") {
            Task::Add(left, right)
        } else if let Some((left, right)) = action.split_once(" - ") {
            Task::Sub(left, right)
        } else if let Some((left, right)) = action.split_once(" * ") {
            Task::Mul(left, right)
        } else if let Some((left, right)) = action.split_once(" / ") {
            Task::Div(left, right)
        } else {
            Task::Val(0)
        };
        
        monkeys.insert(id, task);
    }

    let result = recursive_calculate(&monkeys, "root");

    

    result.to_string()
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
        assert_eq!(result, "152");
    }
    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, "");
    }
    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

}
