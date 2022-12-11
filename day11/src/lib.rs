use std::{fs, default};

pub struct Monkey {
    pub items: Vec<usize>,
    pub operation: fn(usize) -> usize,
    pub test_value: usize,
    pub true_monkey: usize,
    pub false_monkey: usize,
}

pub fn part1(mut monkeys: Vec<Monkey>) -> String {
    let mut activities: Vec<usize> = Vec::new();
    activities.resize(monkeys.len(), 0);
    
    let mut transfers: Vec<(usize, usize)> = Vec::new();
    for round in 0..20 {
        for i in 0..monkeys.len() {
            for item in monkeys[i].items.iter() {
                let new = (monkeys[i].operation)(*item) / 3;
                if new % monkeys[i].test_value == 0 {
                    transfers.push((monkeys[i].true_monkey, new));
                } else {
                    transfers.push((monkeys[i].false_monkey, new));
                }
            }
            activities[i] += monkeys[i].items.len();

            monkeys[i].items.clear();
            for (m, n) in transfers.iter() {
                monkeys[*m].items.push(*n);
            }
            transfers.clear();
        }

        // println!("After round {}", round+1);
        // for i in 0..monkeys.len() {
        //     println!("Monkey {}: {:?}", i, &monkeys[i].items);
        // }
        // println!();
    }
    
    activities.sort();
    let a = activities.pop().unwrap();
    let b = activities.pop().unwrap();

    (a*b).to_string()
}

pub fn part2(mut monkeys: Vec<Monkey>) -> String {
    let mut activities: Vec<usize> = Vec::new();
    activities.resize(monkeys.len(), 0);
    
    let common = monkeys.iter().fold(1, |acc, m| acc * m.test_value);

    let mut transfers: Vec<(usize, usize)> = Vec::new();
    for round in 1..=10000 {
        for i in 0..monkeys.len() {
            for item in monkeys[i].items.iter() {
                let new = (monkeys[i].operation)(*item) % common;
                if new % monkeys[i].test_value == 0 {
                    transfers.push((monkeys[i].true_monkey, new));
                } else {
                    transfers.push((monkeys[i].false_monkey, new));
                }
            }
            activities[i] += monkeys[i].items.len();

            monkeys[i].items.clear();
            for (m, n) in transfers.iter() {
                monkeys[*m].items.push(*n);
            }
            transfers.clear();
        }

        if round == 1 || round == 20 || round % 1000 == 0 {
            println!("After round {}", round);
            for i in 0..monkeys.len() {
                println!("Monkey {} inspected items {} times.", i, activities[i]);
            }
            println!();
        }
    }
    
    activities.sort();
    let a = activities.pop().unwrap();
    let b = activities.pop().unwrap();

    (a*b).to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let monkeys = get_test_monkeys();
        let result = part1(monkeys);
        assert_eq!(result, "10605");
    }

    #[test]
    fn test_part2() {
        let monkeys = get_test_monkeys();
        let result = part2(monkeys);
        assert_eq!(result, "2713310158");
    }

    fn get_test_monkeys() -> Vec<Monkey> {
        let mut monkeys: Vec<Monkey> = Vec::new();
        monkeys.push(Monkey {
            items: vec![79, 98],
            operation: |x| x * 19,
            test_value: 23,
            true_monkey: 2,
            false_monkey: 3,
        });
        monkeys.push(Monkey {
            items: vec![54, 65, 75, 74],
            operation: |x| x + 6,
            test_value: 19,
            true_monkey: 2,
            false_monkey: 0,
        });
        monkeys.push(Monkey {
            items: vec![79, 60, 97],
            operation: |x| x * x,
            test_value: 13,
            true_monkey: 1,
            false_monkey: 3,
        });
        monkeys.push(Monkey {
            items: vec![74],
            operation: |x| x + 3,
            test_value: 17,
            true_monkey: 0,
            false_monkey: 1,
        });
        monkeys
    }
}


pub fn get_input_monkeys() -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    monkeys.push(Monkey {
        items: vec![59, 65, 86, 56, 74, 57, 56],
        operation: |old| old * 17,
        test_value: 3,
        true_monkey: 3,
        false_monkey: 6,
    });
    monkeys.push(Monkey {
        items: vec![63, 83, 50, 63, 56],
        operation: |old| old + 2,
        test_value: 13,
        true_monkey: 3,
        false_monkey: 0,
    });
    monkeys.push(Monkey {
        items: vec![93, 79, 74, 55],
        operation: |old| old + 1,
        test_value: 2,
        true_monkey: 0,
        false_monkey: 1,
    });
    monkeys.push(Monkey {
        items: vec![86, 61, 67, 88, 94, 69, 56, 91],
        operation: |old| old + 7,
        test_value: 11,
        true_monkey: 6,
        false_monkey: 7,
    });
    monkeys.push(Monkey {
        items: vec![76, 50, 51],
        operation: |old| old * old,
        test_value: 19,
        true_monkey: 2,
        false_monkey: 5,
    });
    monkeys.push(Monkey {
        items: vec![77, 76],
        operation: |old| old + 8,
        test_value: 17,
        true_monkey: 2,
        false_monkey: 1,
    });
    monkeys.push(Monkey {
        items: vec![74],
        operation: |old| old * 2,
        test_value: 5,
        true_monkey: 4,
        false_monkey: 7,
    });
    monkeys.push(Monkey {
        items: vec![86, 85, 52, 86, 91, 95],
        operation: |old| old + 6,
        test_value: 7,
        true_monkey: 4,
        false_monkey: 5,
    });

    monkeys
}