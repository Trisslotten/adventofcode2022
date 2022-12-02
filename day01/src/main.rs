use std::fs;


fn day01part1() {
    let input = fs::read_to_string("inputs/input01.txt").unwrap();

    let mut sum = 0;
    let mut largest = 0;

    for line in input.lines() {
        let len = line.len();
        
        if len != 0 {
            match line.parse::<i32>() {
                Ok(num) => {
                    sum += num;
                }
                Err(_) => {}
            }
        } else { 
            if sum > largest {
                largest = sum;
            }
            sum = 0;
        }
    }
    if sum > largest {
        largest = sum;
    }

    println!("Largest: {largest}");
}

fn day01part2() {
    let input = fs::read_to_string("inputs/input01.txt").unwrap();

    let mut sum = 0;
    let mut largest = [0; 4];

    for line in input.lines() {
        let len = line.len();
        
        if len != 0 {
            match line.parse::<i32>() {
                Ok(num) => {
                    sum += num;
                }
                Err(_) => {}
            }
        } else { 
            largest[0] = sum;
            largest.sort();
            sum = 0;
        }
    }
    largest[0] = sum;
    largest.sort();

    
    let sum_largest = largest[1] + largest[2] + largest[3];

    println!("sum: {sum_largest}");
}

fn main() {
    day01part1();
    day01part2();
}
